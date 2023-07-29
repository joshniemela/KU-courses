(ns db-manager.querier
  (:require [clojure.string :as str]))

; Postgres has both ordinary strings and double dollar strings,
; this function converts ordinary strings to double dollar strings to avoid SQL injection
(defn stringify [val]
  (str "$$" (clojure.string/replace val #"\$" "") "$$"))

(defn rm-empty [predicates]
  (filter #(seq %) predicates))

(defn generate-search-statements [search-statement]
  (let [category (case (str/lower-case (:category search-statement))
                   "title" "course.title"
                   "description" "course.raw_description"
                   "coordinator" "employee.full_name")
        query (str/lower-case (:query search-statement))]
    ; if category is course.raw_description, do not use %, but always use %> either way
    (str "( " category " %> " (stringify query) (when-not (= category "course.raw_description") (str " OR " category " % " (stringify query))) ")")))

; input example
;{
;  "block": [],
;  "study_level": [],
;  "schedule_group": [
;    "C"
;  ],
;  "examination_type": [],
;  "searches": [
;    {
;      "category": "Title",
;      "query": "linear algebra",
;      "fuzzy": true
;    },
;    {
;      "category": "Coordinator",
;      "query": "troels",
;      "fuzzy": true
;    }
;  ]
;}
; TODO: this should not be a function we need, but it is required since the parser and frontend are not in sync
(defn convert-exam [exam-type]
  (stringify (case (str/lower-case exam-type)
               "written" "written_examination"
               "oral" "oral_examination"
               "continuous assessment" "continuous_assessment"
               "assignment" "written_assignment")))

; Takes the input JSON and generates a SQL snippet
(defn generate-statements [predicates]
  (let [block (map stringify (:block predicates))
        study_level (map stringify (:study_level predicates))
        schedule_group (map stringify (:schedule_group predicates))
        department (map stringify (:department predicates))
        examination_type (map convert-exam (:examination_type predicates))
        searches (map generate-search-statements (:searches predicates))]
    (str/join " AND " (rm-empty (list (if (empty? block)
                                        ""
                                        (str "course.start_block IN ( " (str/join ", " block) " )"))
                                      (if (empty? study_level)
                                        ""
                                        (str "course.study_level IN ( " (str/join ", " study_level) " )"))
                                      (if (empty? schedule_group)
                                        ""
                                        (str "schedule.schedule_type IN ( " (str/join ", " schedule_group) " )"))
                                      (if (empty? examination_type)
                                        ""
                                        (str "exam.exam_type IN ( " (str/join ", " examination_type) " )"))
                                      (if (empty? department)
                                        ""
                                        (str "department.department_type IN ( " (str/join ", " department) " )"))
                                      (if (empty? searches)
                                        ""
                                        (str/join " AND " searches)))))))

; Takes the input JSON and generates a SQL query
(defn course-overview-template [where-clause]
  (str "SELECT
	course.title,
    course.course_id,
	course.study_level,
	course.start_block,
    course.course_language,
	course.credits,
	course.duration,
    course.raw_description,
    jsonb_agg(DISTINCT to_jsonb(exam) - 'course_id')::TEXT AS exams,
	jsonb_agg(DISTINCT to_jsonb(schedule) - 'course_id')::TEXT AS schedules
FROM
    course
JOIN
    coordinates ON course.course_id = coordinates.course_id
JOIN
    exam ON course.course_id = exam.course_id
JOIN
	schedule ON course.course_id = schedule.course_id
JOIN
	employee ON employee.email = coordinates.email
JOIN
    department ON course.course_id = department.course_id"
       (if (empty? (str/replace where-clause #"\(|\)" "")) ; improve this, just check if it is empty
         ""
         (str "\nWHERE " where-clause))
       " GROUP BY course.course_id;"))

(defn generate-overview-query [predicates]
  (let [prepared-predicate (generate-statements predicates)]
    (course-overview-template prepared-predicate)))

(defn course-query-template [where-clause]
  (str "SELECT
	course.title,
    course.course_id,
	course.study_level,
	course.start_block,
    course.course_language,
	course.credits,
	course.duration,
    course.description,
    jsonb_agg(DISTINCT to_jsonb(exam) - 'course_id')::TEXT AS exams,
    jsonb_agg(DISTINCT to_jsonb(employee))::TEXT AS employees,
	jsonb_agg(DISTINCT to_jsonb(schedule) - 'course_id')::TEXT AS schedules,
    jsonb_agg(DISTINCT to_jsonb(workload) - 'course_id')::TEXT AS workloads,
    jsonb_agg(DISTINCT to_jsonb(department) - 'course_id')::TEXT AS departments
FROM
    course
JOIN
    workload ON course.course_id = workload.course_id
JOIN
    coordinates ON course.course_id = coordinates.course_id
JOIN
    exam ON course.course_id = exam.course_id
JOIN
	schedule ON course.course_id = schedule.course_id
JOIN
	employee ON employee.email = coordinates.email
JOIN
    department ON course.course_id = department.course_id"
       (if (empty? (str/replace where-clause #"\(|\)" "")) ; improve this, just check if it is empty
         ""
         (str "\nWHERE " where-clause))
       " GROUP BY course.course_id;"))
(defn generate-course-by-id-query [course-id]
  (course-query-template (str "course.course_id = " (stringify course-id))))

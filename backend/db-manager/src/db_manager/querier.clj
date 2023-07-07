(ns db-manager.querier
  (:require [clojure.string :as str]))

(defn sanitise [val]
  ; psql has double dollar seprated strings, this is used to avoid injection, and therefore we need to remove any $ in the string
  (str "$$" (clojure.string/replace val #"\$" "") "$$"))

(defn sanitise-op [op]
  ;only allow =, <, >, <=, >=, <>, ~~, %>, %, %>>
  (condp = op
    "=" "="
    "<" "<"
    ">" ">"
    "<=" "<="
    ">=" ">="
    "<>" "<>"
    "~" "~"
    "~~" "~~"
    "%>" "%>"
    "%" "%"
    "%>>" "%>>"))

(defn rm-empty [predicates]
  (filter #(seq %) predicates))

(defn generate-predicate [predicate]
  (str (case (:key predicate)
         "schedule_group" "schedule.schedule_type"
         "email" "employee.email"
         "workload_type" "workload.workload_type"
         "exam_type" "exam.exam_type"
         "course_id" "course.course_id"
         "course_title" "course.title"
         "start_block" "course.start_block"
         "duration" "course.duration"
         "study_level" "course.study_level"
         "course_language" "course.course_language"
         "employee_name" "employee.full_name"
         "credits" "course.credits"
         "description" "course.raw_description")
       ; "employee_title" is a value in the future
       " " (sanitise-op (:op predicate)) " " (sanitise (:value predicate))))

(defn generate-inner [predicates]
  (str "(" (str/join " OR " (map generate-predicate (rm-empty predicates))) ")"))

(defn generate-outer [predicates]
  (str/join "\nAND " (map generate-inner (rm-empty predicates))))

(defn generate-courses-query [predicates]
  (let [prepared-predicate (generate-outer predicates)]
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
    jsonb_agg(DISTINCT to_jsonb(workload) - 'course_id')::TEXT AS workloads
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
	employee ON employee.email = coordinates.email"
         (if (empty? (str/replace prepared-predicate #"\(|\)" ""))
           ""
           (str "\nWHERE " prepared-predicate))
         " GROUP BY course.course_id;")))
; NEW CODE HERE
(defn generate-search-statements [search-statement]
  (let [category (case (str/lower-case (:category search-statement))
                   "title" "course.title"
                   "description" "course.raw_description"
                   "coordinator" "employee.full_name")
        query (str/lower-case (:query search-statement))
        ;fuzzy (:fuzzy search-statement)]
        ]
    ;TODO: implement exact search, right now it is always fuzzy
    (str "( " category " % " (sanitise query) " OR "
         category " <% " (sanitise query) " )")))

; input example
;{"block":[],"study_level":[],"schedule_group":["C"],"examination_type":[],"searches":[{"category":"Title","query":"linear algebra","fuzzy":true},{"category":"Coordinator","query":"troels","fuzzy":true}]}
(defn generate-statements [predicates]
  (let [block (map sanitise (:block predicates))
        study_level (map sanitise (:study_level predicates))
        schedule_group (map sanitise (:schedule_group predicates))
        examination_type (map sanitise (:examination_type predicates))
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
                                      (if (empty? searches)
                                        ""
                                        (str/join " AND " searches)))))))

(defn generate-courses-query-new [predicates]
  (let [prepared-predicate (generate-statements predicates)]
    (str "SELECT
	course.title,
    course.course_id,
	course.study_level,
	course.start_block,
    course.course_language,
	course.credits,
	course.duration,
    substring(course.description, 0, 200) AS description,
    jsonb_agg(DISTINCT to_jsonb(exam) - 'course_id')::TEXT AS exams,
    jsonb_agg(DISTINCT to_jsonb(employee))::TEXT AS employees,
	jsonb_agg(DISTINCT to_jsonb(schedule) - 'course_id')::TEXT AS schedules,
    jsonb_agg(DISTINCT to_jsonb(workload) - 'course_id')::TEXT AS workloads
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
	employee ON employee.email = coordinates.email"
         (if (empty? (str/replace prepared-predicate #"\(|\)" ""))
           ""
           (str "\nWHERE " prepared-predicate))
         " GROUP BY course.course_id;")))

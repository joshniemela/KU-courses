(ns db-manager.querier
  (:require [clojure.string :as str]
            [clojure.data.json :as json]))

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
         "schedule_type" "schedule.schedule_type"
         "email" "employee.email"
         "workload_type" "workload.workload_type"
         "exam_type" "exam.exam_type"
         "course_id" "course.course_id"
         "course_title" "course.title"
         "start_block" "course.start_block"
         "duration" "course.duration"
         "schedule_group" "course.schedule_group"
         "study_level" "course.study_level"
         "course_language" "course.course_language"
         "employee_name" "employee.full_name"
         "credits" "course.credits"
         "raw_desc" "course.raw_description")
       ; "employee_title" is a value in the future
       " " (sanitise-op (:op predicate)) " " (sanitise (:value predicate))))

(defn generate-inner [predicates]
  (str "(" (clojure.string/join " OR " (map generate-predicate (rm-empty predicates))) ")"))

(defn generate-outer [predicates]
  (clojure.string/join "\nAND " (map generate-inner (rm-empty predicates))))

(defn generate-courses-query [predicates]
  (let [prepared-predicate (generate-outer predicates)]
    (str "SELECT
	course.title,
  course.course_id, 
	course.study_level,
	course.start_block,
  course.course_language,
	course.credits,
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


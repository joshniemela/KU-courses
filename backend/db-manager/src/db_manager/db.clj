(ns db-manager.db
  (:require [datascript.core :as d]

            [clojure.walk :refer [postwalk]]))

(def many-ref {:db/valueType :db.type/ref
               :db/cardinality :db.cardinality/many})
(def one-ref {:db/valueType :db.type/ref
              :db/cardinality :db.cardinality/one})
(def unique {:db/unique :db.unique/identity})

(defn component [schema]
  (assoc schema :db/isComponent true))

(def schema
  {:course/id unique
   :course/title {}
   :course/ects {}

   :course/block many-ref

   :course/schedule many-ref

   :course/language many-ref
   :course/duration {:db/cardinality :db.cardinality/one}
   :course/degree many-ref
   :course/capacity {:db/cardinality :db.cardinality/one}
   :course/department many-ref
   :course/faculty one-ref
   :course/coordinator many-ref

   :course/workload (component many-ref)

   :course/exam (component many-ref)
   :course/content {}
   :course/learning-outcome {}
   :course/recommended-qualifications {}
   :course/statistics (component one-ref)

   :schedule-group/type unique
   :block/type unique
   :faculty/name unique
   :department/name unique
   :degree/name unique
   :language/name unique
   :employee/email unique
   :employee/name {}
   :statistics/mean {:db/cardinality :db.cardinality/one}
   :statistics/median {:db/cardinality :db.cardinality/one}
   :statistics/pass-rate {:db/cardinality :db.cardinality/one}})

(defn convert-coordinator
  "Convert a coordinator map from rust parser to a datascript map"
  [coordinator]
  (let [name (get coordinator "name")
        email (get coordinator "email")]
    {:employee/name name
     :employee/email email}))
(defn convert-workload
  "Convert a workload map from rust parser to a datascript map"
  [workload]
  (let [type (get workload "workload_type")
        hours (get workload "hours")]
    {:workload/type type
     :workload/hours hours}))
(defn convert-exam [exam]
  ; this can either be a string or a map, if its a string then it has no duration
  (if (string? exam)
    {:exam/type exam}
    ; the key is the exam type, the value is the duration
    ; ensure that the map is exactly 1 element
    (if (= 1 (count exam))
      (let [[exam-type duration] (first exam)]
        {:exam/type exam-type
         :exam/duration duration})
      (throw (Exception. "Exam map has more than 1 element, this should be impossible")))))

(defn remove-nils
  "As hinted by the name, it traverses the entire map and removes all fields with nils
    This is necessary because the rust parser returns a lot of nils, and datascript does not like nils
    Snippet from https://stackoverflow.com/questions/3937661/remove-nil-values-from-a-map"
  [m]
  (let [f (fn [[k v]] (when v [k v]))]
    (postwalk (fn [x] (if (map? x) (into {} (map f x)) x)) m)))

(defn course-to-transaction  [course-map]
  (let [id (get-in course-map ["info" "id"])
        title (get course-map "title")
        ects (get-in course-map ["info" "ects"])
        blocks (get-in course-map ["info" "block"])
        schedules (get-in course-map ["info" "schedule"])
        languages (get-in course-map ["info" "language"])
        duration (get-in course-map ["info" "duration"])
        degrees (get-in course-map ["info" "degree"])
        capacity (get-in course-map ["info" "capacity"])
        departments (get-in course-map ["logistics" "departments"])
        faculty (get-in course-map ["logistics" "faculty"])
        coordinators (map convert-coordinator (get-in course-map ["logistics" "coordinators"]))
        workloads (map convert-workload (get course-map "workloads"))
        exams (map convert-exam (get course-map "exams"))
        content (get-in course-map ["description" "content"])
        learning-outcome (get-in course-map ["description" "learning_outcome"])
        recommended-qualifications (get-in course-map ["description" "recommended_qualifications"])]
        ;content ""
        ;learning-outcome ""
        ;recommended-qualifications ""]
    (remove-nils {:course/id id
                  :course/title title
                  :course/ects ects
                  :course/blocks (mapv #(hash-map :block/type %) blocks)
                  :course/schedule (mapv #(hash-map :schedule/type %) schedules)
                  :course/language (mapv #(hash-map :language/name %) languages)
                  :course/duration duration
                  :course/degree (mapv #(hash-map :degree/name %) degrees)
                  :course/capacity capacity
                  :course/department (mapv #(hash-map :department/name %) departments)
                  :course/faculty (hash-map :faculty/name faculty)
                  :course/coordinator coordinators
                  :course/workload workloads
                  :course/exam exams
                  :course/content content
                  :course/learning-outcome learning-outcome
                  :course/recommended-qualifications (if (nil? recommended-qualifications) "" recommended-qualifications)})))

(defn courses-to-transactions [courses]
  (map course-to-transaction courses))

(defn get-course-ids [db]
  nil)
(defn get-course-by-id [db id]
  nil)
(defn get-courses [db]
  nil)

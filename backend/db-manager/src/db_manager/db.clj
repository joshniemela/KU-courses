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

   :schedule/type unique
   :block/type unique
   :faculty/name unique
   :department/name unique
   :degree/type unique
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

(defn remove-db-ids
  [coll]
  (postwalk (fn [x] (if (map? x) (dissoc x :db/id) x)) coll))

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
        recommended-qualifications (get-in course-map ["description" "recommended_qualifications"])
        summary (get-in course-map ["description" "summary"])]
    {:course/id id
     :course/title title
     :course/ects ects
     :course/block (mapv #(hash-map :block/type %) blocks)
     :course/schedule (mapv #(hash-map :schedule/type %) schedules)
     :course/language (mapv #(hash-map :language/name %) languages)
     :course/duration duration
     :course/degree (mapv #(hash-map :degree/type %) degrees)
     :course/capacity capacity
     :course/department (mapv #(hash-map :department/name %) departments)
     :course/faculty (hash-map :faculty/name faculty)
     :course/coordinator coordinators
     :course/workload workloads
     :course/exam exams
     :course/content content
     :course/learning-outcome learning-outcome
     :course/recommended-qualifications (if (nil? recommended-qualifications) "" recommended-qualifications)
     :course/summary summary}))

(defn courses-to-transactions [courses]
  (map course-to-transaction courses))

(defn get-course-ids [conn]
  (let [course-ids (d/q '[:find ?id
                          :where
                          [?e :course/id ?id]]
                        @conn)]
    ; this is a vector of vectors, we want a vector of strings
    (mapv first course-ids)))
(defn get-course-by-id
  "Find all the detailed information about a course by its id"
  [conn course-id]
  (let [course (d/pull @conn '[* {:course/schedule [*]
                                  :course/exam [*]
                                  :course/degree [*]
                                  :course/block [*]
                                  :course/faculty [*]
                                  :course/department [*]
                                  :course/coordinator [*]
                                  :course/workload [*]
                                  :course/language [*]
                                  :course/statistics [*]}]
                       [:course/id course-id])]
    ; remove summary since we already bring it along from content
    (remove-db-ids (dissoc course :course/summary))))

; denest a vector of vectors
(defn denest [v]
  (mapv first v))

(defn query-course-ids [conn predicate-map]
  (let [blocks (get predicate-map :blocks)
        schedules (get predicate-map :schedules)
        exams (get predicate-map :exams)
        degrees (get predicate-map :degrees)
        departments (get predicate-map :departments)]
    (denest (d/q (concat '[:find ?course-id :in $
                           :where
                           [?e :course/block ?block]
                           [?e :course/id ?course-id]
                           [?e :course/schedule ?schedule]
                           [?e :course/exam ?exam]
                           [?e :course/degree ?degree]
                           [?e :course/department ?department]]
                         (if (empty? blocks)
                           []
                           (list (cons 'or (mapv (fn [block] (vector '?block ':block/type block)) blocks))))

                         (if (empty? schedules)
                           []
                           (list (cons 'or (mapv (fn [schedule] (vector '?schedule ':schedule/type schedule)) schedules))))

                         (if (empty? exams)
                           []
                           (list (cons 'or (mapv (fn [exam] (vector '?exam ':exam/type exam)) exams))))

                         (if (empty? degrees)
                           []
                           (list (cons 'or (mapv (fn [degree] (vector '?degree ':degree/type degree)) degrees))))

                         (if (empty? departments)
                           []
                           (list (cons 'or (mapv (fn [department] (vector '?department ':department/name department)) departments)))))
                 @conn))))

(defn get-overviews-from-ids [conn ids]
  (d/pull-many @conn '[:course/id
                       :course/title
                       :course/ects
                       :course/summary
                       {:course/schedule [*]
                        :course/block [*]
                        :course/exam [*]
                        :course/degree [*]
                        :course/language [*]
                        :course/statistics [:statistics/mean
                                            :statistics/median
                                            :statistics/pass-rate]}]
               (mapv #(vector :course/id %) ids)))

(defn get-courses [conn predicate-map]
  (let [course-ids (query-course-ids conn predicate-map)]
    (map remove-db-ids (get-overviews-from-ids conn course-ids))))

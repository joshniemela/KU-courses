(ns db-manager.db
  (:require [datascript.core :as d]))

(def one {:db/cardinality :db.cardinality/one})
(def many {:db/cardinality :db.cardinality/many})

(def one-ref {:db/valueType :db.type/ref
              :db/cardinality :db.cardinality/one})
(def many-ref {:db/valueType :db.type/ref
               :db/cardinality :db.cardinality/many})
(defn unique [m]
  (assoc m :db/unique :db.unique/identity))

(def schema {:course/code (unique one)
             :course/content one
             :course/learning_outcomes one
             :course/recommended_qualifications one
             :course/title one
             :course/credits one
             :course/capacity one
             :course/languages one
             :course/faculty one
             :course/degrees many
             :course/departments many
             :course/schedules many
             :course/coordinators many-ref
             :course/exams many-ref
             :course/workloads many-ref

             :emplooyee/name one
             :emplooyee/email (unique one)

             :exam/type one
             :exam/minutes one

             :workload/type one
             :workload/hours one})

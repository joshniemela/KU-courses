(ns db-manager.db
  (:refer-clojure :exclude [filter for group-by into partition-by set update])
  (:require [next.jdbc :as jdbc]
            [next.jdbc.sql :as jdbc.sql]
            [honey.sql :as sql]
            [clojure.java.io :as io]
            [honey.sql.helpers :refer :all :as h]))

(defn nuke-db! [db]
  (jdbc/with-transaction [tx db]
    (jdbc/execute! tx [(slurp (io/resource "nuke.sql"))])
    (jdbc/execute! tx [(slurp (io/resource "types.sql"))])
    (jdbc/execute! tx [(slurp (io/resource "schema.sql"))])))


(defn insert-course! [ds course-map]
  (let [course-schema [:course_id :title :course_language
                       :description :start_block :duration
                       :schedule_group :credits :study_level
                       :url]]

    (jdbc.sql/insert! ds :course (select-keys course-map course-schema))))


(defn emp-fields [emp]
  (select-keys emp [:email :full_name]))

(defn insert-employees! [ds emps-map]
  (jdbc/execute! ds (-> (insert-into :employee)
                        (values (map emp-fields (:coordinators emps-map)))
                        (on-conflict :email)
                        do-nothing
                        (sql/format))))

(defn insert-coordinates! [ds course-emp-map]
  ; associate course_id with each coordinator

  (let [cid (:course_id course-emp-map)]

    ; select only the email from the coordinators and associate with course_id
    (jdbc.sql/insert-multi! ds :coordinates (map #(select-keys (assoc % :course_id cid) [:email
                                                                                         :course_id])
                                                 (:coordinators course-emp-map)))))
(defn insert-workloads! [ds course-emp-map]
  (let [workloads (:workloads course-emp-map)]
    (jdbc.sql/insert-multi! ds :workload (map #(select-keys (assoc % :course_id (:course_id course-emp-map))
                                                            [:course_id :workload_type :hours])
                                              workloads))))




(defn insert-course-emp! [db course-emp-map]
  (jdbc/with-transaction [tx db]
    (insert-course! tx course-emp-map)
    (insert-employees! tx course-emp-map)
    (insert-coordinates! tx course-emp-map)
    (insert-workloads! tx course-emp-map)))



(defn populate-courses! [db courses]
  (let [len (count courses)]
    (println (str "Populating " len " courses"))
    (doseq [course courses]
      (insert-course-emp! db course)
      (println (str "Inserted " (:course_id course))))))
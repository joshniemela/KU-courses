(ns db-manager.db
  (:refer-clojure :exclude [filter for group-by into partition-by set update])
  (:require [next.jdbc :as jdbc]
            [next.jdbc.sql :as jdbc.sql]
            [honey.sql :as sql]
            [clojure.java.io :as io]
            [honey.sql.helpers :refer :all :as h]
            [clojure.set :as set]))

(defn nuke-db! [db]
  (jdbc/with-transaction [tx db]
    (jdbc/execute! tx [(slurp (io/resource "nuke.sql"))])
    (jdbc/execute! tx [(slurp (io/resource "types.sql"))])
    (jdbc/execute! tx [(slurp (io/resource "schema.sql"))])))


(defn insert-course! [ds course-map]
  (let [course-schema [:course_id :title :course_language
                       :description :start_block :duration
                       :credits :study_level :url
                       :raw_description]]

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

(defn insert-schedule-groups! [ds course-emp-map]
  (let [schedule-groups (:schedules course-emp-map)]
    (jdbc.sql/insert-multi! ds :schedule (map #(select-keys (assoc % :course_id (:course_id course-emp-map))
                                                            [:course_id :schedule_type :minutes])
                                              schedule-groups))))

; add null minutes to exams if not present
(defn add-null-minutes [exam]
  (if (:minutes exam)
    exam
    (assoc exam :minutes nil)))
(defn insert-exams! [ds course-emp-map]
  (let [exams (map add-null-minutes (:exams course-emp-map))]
    (jdbc.sql/insert-multi! ds :exam (map #(select-keys (assoc % :course_id (:course_id course-emp-map))
                                                        [:course_id :exam_type :minutes])
                                          exams))))


(defn insert-course-emp! [db course-emp-map]
  (jdbc/with-transaction [tx db]
    (insert-course! tx course-emp-map)
    (insert-employees! tx course-emp-map)
    (insert-coordinates! tx course-emp-map)
    (insert-workloads! tx course-emp-map)
    (insert-schedule-groups! tx course-emp-map)
    (insert-exams! tx course-emp-map)))



(defn populate-courses! [db courses]
  (println (str "Populating database with " (count courses) " courses"))
  ; TODO, fix print race condition
  (doall (pmap #(do (insert-course-emp! db %)
                    (println (str "Inserted course " (:course_id %))))
               courses))
  (println "Done populating database"))


; queries section begins here

; SELECT full_name, similarity(full_name, '<name here>') AS search_similarity
; FROM employee
; ORDER BY search_similarity DESC
; LIMIT 1;"
(defn find-email-by-name [db name]
  (->
   (jdbc/execute! db ["SELECT email, full_name, similarity(full_name, ?) AS search_similarity
                          FROM employee
                          ORDER BY search_similarity DESC
                          LIMIT 1;" name])
   first
   ; rename the keys
   (set/rename-keys {:employee/email :email
                     :employee/full_name :full_name})))

; use honeysql
(defn get-courses [db]
  (jdbc/execute! db (-> (select :course-id)
                        (from :course)
                        (sql/format))))


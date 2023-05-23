(ns db-manager.core
  (:refer-clojure :exclude [filter for group-by into partition-by set update])
  (:require [next.jdbc :as jdbc] 
            [honey.sql :as sql] 
            [honey.sql.helpers :refer :all :as h] 
            [clojure.core :as c]))
                                             

(def db-config
  {:dbtype "postgresql"
   :dbname "admin"
   :host "localhost"
   :user "admin"
   :password "admin"})

(def db (jdbc/get-datasource db-config))


(def employee-table 
  [:employees [[:name [:varchar 255] [:not nil]]
               [:email [:varchar 50] [:not nil]]
               [:title [:varchar 255] [:not nil]]
               ;[:phone [:varchar 255]] might not be allowed by GDPR
               [[:primary-key :email]]]])

(def course-table 
  [:courses [[:course-id [:char 10] [:not nil]]
             [:placement [:char 3] [:not nil]]]])


(defn init-table! [table-name columns]
  (jdbc/execute! db (sql/format {:create-table [table-name :if-not-exists]
                                 :with-columns columns})))

(defn init-tables! [tables]
  (map #(init-table! (first %) (second %)) tables))



(defn -main []
  (println (jdbc/execute! db ["select version();"])))

(comment 
  (init-tables! [employee-table course-table])
  (jdbc/execute! db ["drop table employees;"])
  )
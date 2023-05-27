(ns db-manager.db 
  (:refer-clojure :exclude [filter for group-by into partition-by set update])
  (:require [next.jdbc :as jdbc]
            [honey.sql :as sql]
            [honey.sql.helpers :refer :all :as h]))


(def employee-schema
  [[:name [:varchar 255] [:not nil]] 
   [:email [:varchar 50] [:not nil]] 
   [:title [:varchar 255] [:not nil]]
  ;[:phone [:varchar 255]] ;might not be allowed by GDPR 
   [[:primary-key :email]]])

(def course-schema
  [[:course-id [:char 10] [:not nil]] 
   [:placement [:char 3] [:not nil]]])

; takes map of shape:
; [{:name "Erik" :email "foo@bar.dk" :title "CEO"}
;  {...}...]
(defn insert-employees [employees]
  (-> (insert-into :employees)
      ; values are the keys of the table
      (values (map #(select-keys % [:name :email :title]) employees))
      sql/format))



(defn nuke-replace-employees! [db employees]
  ; drop employees table if exists,
  ; then create it again
  ; then insert employees
  (jdbc/with-transaction [tx db]
    (jdbc/execute! tx (sql/format {:drop-table [:if-exists :employees]}))
    (jdbc/execute! tx (sql/format {:create-table [:employees :if-not-exists]
                                   :with-columns employee-schema}))
    (jdbc/execute! tx (insert-employees employees))))
  
  





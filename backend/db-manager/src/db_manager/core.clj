(ns db-manager.core
  (:refer-clojure :exclude [filter for group-by into partition-by set update])
  (:require [next.jdbc :as jdbc] 
            [honey.sql :as sql] 
            [honey.sql.helpers :refer :all :as h] 
            [clojure.core :as c]
            [clojure.data.json :as json]
            [clojure.java.io :as io]
            [muuntaja.core :as m]
            [reitit.ring :as ring] 
            [reitit.coercion.spec]
            [reitit.ring.coercion :as rrc]
            [reitit.ring.middleware.muuntaja :as muuntaja]
            [reitit.ring.middleware.parameters :as parameters]
            [org.httpkit.server :refer [run-server]]
            [db-manager.routes :refer [ping-route crud-routes]]))
                                             

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
               [:phone [:varchar 255]] ;might not be allowed by GDPR
               [[:primary-key :email]]]])

(def course-table 
  [:courses [[:course-id [:char 10] [:not nil]]
             [:placement [:char 3] [:not nil]]]])


(defn init-table! [table-name columns]
  (jdbc/execute! db (sql/format {:create-table [table-name :if-not-exists]
                                 :with-columns columns})))

(defn init-tables! [tables]
  (map #(init-table! (first %) (second %)) tables))

; read employed.json
(def employees (json/read-str (slurp "employed.json") :key-fn keyword))

; upsert employees
(defn upsert-employees! [employees]
  (-> (insert-into :employees)
      (values employees)
      (on-conflict :email)
      do-nothing
      sql/format))

(defn app []
  (ring/ring-handler
   (ring/router
    [["/api"
      ping-route
      crud-routes]]
    {:data {:coercion reitit.coercion.spec/coercion
            :muuntaja m/instance
            :middleware [parameters/parameters-middleware
                         muuntaja/format-middleware
                         rrc/coerce-exceptions-middleware
                         rrc/coerce-request-middleware
                         rrc/coerce-response-middleware]}})))


(defn -main []
  (println (jdbc/execute! db ["select version();"])))
  (run-server (app) {:port 3000})

(comment 
  (init-tables! [employee-table course-table])
  (jdbc/execute! db ["drop table employees;"])
  (jdbc/execute! db (upsert-employees! employees))
)

(ns db-manager.core
  (:require [next.jdbc :as jdbc]
            [honeysql.core :as sql]
            [honeysql.helpers :refer :all :as helpers]))


(def db-config
  {:dbtype "postgresql"
   :dbname "admin"
   :host "localhost"
   :user "admin"
   :password "admin"})

(def db (jdbc/get-datasource db-config))



(defn -main []
  (println (jdbc/execute! db ["select version();"])))


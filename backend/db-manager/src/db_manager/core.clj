(ns db-manager.core 
  (:require [clojure.core :as c]
            [clojure.data.json :as json]
            [clojure.java.io :as io]
            [muuntaja.core :as m]
            [reitit.ring :as ring]
            [reitit.coercion.spec]
            [reitit.ring.coercion :as rrc]
            [reitit.ring.middleware.muuntaja :as muuntaja]
            [reitit.ring.middleware.parameters :as parameters]
            [org.httpkit.server :refer [run-server]]
            [db-manager.routes :refer [ping-route crud-routes]]
            [db-manager.db :refer [nuke-db! insert-course-emp! populate-courses!]]
            [next.jdbc :as jdbc]
            [next.jdbc.types :refer [as-other]]
            [honey.sql :as sql]))
                                             

(def db-config
  {:dbtype "postgresql"
   :dbname "admin"
   :host "localhost"
   :user "admin"
   :password "admin"})

(def data-dir "../../data/")

(def json-dir (str data-dir "json_science/"))

(def db (jdbc/get-datasource db-config))

; read employed.json
(def employees (json/read-str (slurp (str data-dir "employed.json")) :key-fn keyword))


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


;(defn -main []
;  (println (jdbc/execute! db ["select version();"])))
;  (run-server (app) {:port 3000})

(def test-course {
                  :course_id "1234123412"
                  :title "test"
                  :course_language "da"
                  :description "test"
                  :start_block (as-other 1)
                  :duration (as-other 1)
                  :schedule_group (as-other "A")
                  :credits 7.5
                  :study_level "test"
                  :coordinators [{:email "josh@jniemela.dk" :full_name "Joshua Niemelä"} 
                                 {:email "jhaudfa" :full_name "foobar"}]
                  :workloads [{:workload_type (as-other "lectures") :hours 10} 
                              {:workload_type (as-other "exercises") :hours 10}]})
                  
(def another-test-course {
                          :course_id "blablabla1"
                          :title "test2"
                          :course_language "da"
                          :description "test"
                          :start_block (as-other 1)
                          :schedule_group (as-other "A")
                          :credits 7.5
                          :study_level "test"
                          :coordinators [{:email "josh@jniemela.dk" :full_name "Joshua Niemelä"} 
                                         {:email "potato" :full_name "afafjaf"}]
                          :workloads [{:workload_type (as-other "lectures") :hours 100} 
                                      {:workload_type (as-other "exercises") :hours 100}]})

; read every json in data-dir
(defn read-json [file]
  (json/read-str (slurp (str json-dir file)) :key-fn keyword))

; find all jsons
(def course-files (for [file (file-seq (io/file json-dir)) :when (.endsWith (.getName file) ".json")]
                     (.getName file)))

(def courses (map read-json course-files))


(defn -main []
  (nuke-db! db)
  (insert-course-emp! db test-course)
  (insert-course-emp! db another-test-course)
  (println (jdbc/execute! db ["SELECT * FROM Employee"])))

(defn merge-employees [employees]
  (let [grouped (group-by :email employees)]
    (map (fn [[email employees]]
           (reduce (fn [acc employee]
                     (assoc acc :title (str (:title acc) ", " (:title employee))))
                   (first employees)
                   (rest employees)))
         grouped)))

(comment 
  ; some employees have multiple titles, so we need to group them
   
  
  (println (merge-employees employees))
  (println (count employees))
  (println (count (merge-employees employees))
   (nuke-replace-employees! db (merge-employees employees)))
  (jdbc/execute! db ["drop table employees;"])
  ; find person with email back@di.ku.dk
  (println (jdbc/execute! db ["select * from employees;"])) 
  (println (jdbc/execute! db ["select * from employees where email = 'back@di.ku.dk';"])))


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
            [reitit.swagger-ui :as swagger-ui]
            [reitit.swagger :as swagger]
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
   :password "admin"
   :stringtype "unspecified"})

(def data-dir "../../data/")

(def json-dir (str data-dir "json_science/"))

(def db (jdbc/get-datasource db-config))

; read employed.json
(def employees (json/read-str (slurp (str data-dir "employed.json")) :key-fn keyword))



(defn app []
  (ring/ring-handler
   (ring/router
    [["/swagger.json"
      {:get {:no-doc true
             :swagger {:info {:title "DISKU backend API"}
                       :basePath "/"} ;; prefix for all paths
             :handler (swagger/create-swagger-handler)}}]
     ["/api"
      ping-route
      crud-routes]]
    {:data {:coercion reitit.coercion.spec/coercion
            :muuntaja m/instance
            :middleware [parameters/parameters-middleware
                         muuntaja/format-middleware
                         rrc/coerce-exceptions-middleware
                         rrc/coerce-request-middleware
                         rrc/coerce-response-middleware]}})
   (ring/routes
     (swagger-ui/create-swagger-ui-handler {:path "/swagger"})
     (ring/create-default-handler))))



(def test-course {:course_id "1234123412"
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

(def another-test-course {:course_id "blablabla1"
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


(def real (slurp (io/resource "NNEB18000U.json")))
(def real-course (json/read-str real :key-fn keyword))


(defn coerce-as-other [course-map]
  ; make schedule_group into "as-other"
  (-> course-map
      (assoc :schedule_group (as-other (:schedule_group course-map)))
      (assoc :start_block (as-other (:start_block course-map)))
      ; workloads is a vector of maps with :workload_type and :hours
      ; workload_types should have as-other
      (update :workloads #(map (fn [workload]
                                 (assoc workload :workload_type (as-other (:workload_type workload))))
                               %))
      ; exact same thing with schedule_groups
      (update :schedules #(map (fn [schedule_group]
                                 (assoc schedule_group :schedule_type (as-other (:schedule_type schedule_group))))
                               %))
      ; same with exams
      (update :exams #(map (fn [exam]
                             (assoc exam :exam_type (as-other (:exam_type exam))))
                           %))))



;(defn -main []
;  (println (jdbc/execute! db ["select version();"])))
;  
(defn -main []
  (nuke-db! db)
  (populate-courses! db [(coerce-as-other real-course)])
  (println (jdbc/execute! db ["SELECT * FROM Employee"]))
  (run-server (app) {:port 3000}))

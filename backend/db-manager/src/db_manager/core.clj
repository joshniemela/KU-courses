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
            [db-manager.routes :refer [ping-route api-routes]]
            [db-manager.db :refer [nuke-db! populate-courses!]]
            [db-manager.cli :refer [parse-cli scrape-courses!]]
            [next.jdbc :as jdbc]
            [next.jdbc.types :refer [as-other]]
            [honey.sql :as sql])
  (:gen-class))

(def db-config
  {:dbtype "postgresql"
   :dbname "admin"
   :host "localhost"
   :user "admin"
   :password "admin"
   :stringtype "unspecified"})

(def data-dir "../../data/")

(def json-dir (str data-dir "json/"))

(def db (jdbc/get-datasource db-config))

; https://andersmurphy.com/2022/03/27/clojure-removing-namespace-from-keywords-in-response-middleware.html
(defn transform-keys
  [t coll]
  (clojure.walk/postwalk (fn [x] (if (map? x) (update-keys x t) x)) coll))

(defn remove-namespace-keywords-in-response-middleware [handler & _]
  (fn [req]
    (let [resp (handler req)]
      (cond-> resp
        (comp map? :body) (update :body
                                  (partial transform-keys
                                           (comp keyword name)))))))

(def cors {"Access-Control-Allow-Origin" "*"
           "Access-Control-Allow-Headers" "Origin, Accept, Access-Control-Request-Method, Access-Control-Allow-Headers, Content-Type, *"})

(defn cors-handler
  [_]
  {:headers cors :status 200})

(defn cors-handler [handler]
  (fn [request]
    (let [response (handler request)]
      (assoc-in response [:headers "Access-Control-Allow-Origin"] "*"))))

(defn app []
  (ring/ring-handler
   (ring/router
    [["/swagger.json"
      {:get {:no-doc true
             :swagger {:info {:title "DISKU backend API"}
                       :basePath "/"} ;; prefix for all paths
             :handler (swagger/create-swagger-handler)}}]
     ["/api" {:middleware [remove-namespace-keywords-in-response-middleware]
              :options {:cors true
                        :handler cors-handler}}
      ping-route
      (api-routes db)
      ]]
    {:data {:coercion reitit.coercion.spec/coercion
            :muuntaja m/instance
            :middleware [
                         
                         parameters/parameters-middleware
                         muuntaja/format-middleware
                         rrc/coerce-exceptions-middleware
                         rrc/coerce-request-middleware
                         rrc/coerce-response-middleware 
                         ]}})
   (ring/routes
    (swagger-ui/create-swagger-ui-handler {:path "/swagger"})
    (ring/create-default-handler))))

; read every json in data-dir
(defn read-json [file]
  (json/read-str (slurp (str json-dir file)) :key-fn keyword))

; find all jsons
(def course-files (for [file (file-seq (io/file json-dir)) :when (.endsWith (.getName file) ".json")]
                    (.getName file)))

(def courses (map read-json course-files))

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

(def coerced-courses (pmap coerce-as-other courses))

(def main-config {:port 3000})

(defn -main [& args]
  (let [args (parse-cli args)]
    ; this runs if -s is passed
    (when (:scrape args)
      (println "Scraping courses from the web... (this may take a while)")
      (scrape-courses!))

    ; this runs if -f is passed
    (if (:force args)
      (do (println "Nuking database and repopulating with courses from" json-dir)
          (nuke-db! db)
          (populate-courses! db coerced-courses))
      (println "Starting database with existing data..."))
    (println "Starting server on port " (:port main-config))
    (run-server (app) main-config)))



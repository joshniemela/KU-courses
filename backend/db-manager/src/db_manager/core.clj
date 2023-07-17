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
            [ring.middleware.cors :refer [wrap-cors]]
            [io.staticweb.rate-limit.storage :as storage]
            [io.staticweb.rate-limit.middleware :refer [wrap-rate-limit ip-rate-limit]])
  (:gen-class))

(def db-config
  {:dbtype "postgresql"
   :dbname "admin"
   :host "db"
   :user "admin"
   :password "admin"
   :stringtype "unspecified"})
(def db (jdbc/get-datasource db-config))

(def storage (storage/local-storage))

; limit each IP to 1000 api calls per hour
(def limit (ip-rate-limit :limit-id 1000 (java.time.Duration/ofHours 1)))
(def rate-limit-config {:storage storage :limit limit})

(def data-dir "../../data/")
(def json-dir (str data-dir "json/"))

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

(defn app []
  (ring/ring-handler
   (ring/router
    [["/api/swagger.json"
      {:get {:no-doc true
             :swagger {:info {:title "DISKU backend API"}
                       :basePath "/"} ;; prefix for all paths
             :handler (swagger/create-swagger-handler)}}]
     ["/api" {:middleware [remove-namespace-keywords-in-response-middleware]}
      ping-route
      (api-routes db)]]
    {:data {:coercion reitit.coercion.spec/coercion
            :muuntaja m/instance
            ; TODO: fix the CORS middleware, it seems to not work for Chromium
            :middleware [[wrap-cors
                          :access-control-allow-origin [#".*"]
                          :access-control-allow-methods [:get :post]
                          :access-control-allow-headers #{"accept"
                                                          "accept-encoding"
                                                          "accept-language"
                                                          "authorization"
                                                          "content-type"
                                                          "origin"}]

                         #(wrap-rate-limit % rate-limit-config)
                         parameters/parameters-middleware
                         muuntaja/format-middleware
                         rrc/coerce-exceptions-middleware
                         rrc/coerce-request-middleware
                         rrc/coerce-response-middleware]}})
   (ring/routes
    (swagger-ui/create-swagger-ui-handler {:path "/api"
                                           :url "/api/swagger.json"})
    (ring/create-default-handler))))

; Read every json in data-dir
(defn read-json [file]
  (json/read-str (slurp (str json-dir file)) :key-fn keyword))

; find all jsons
(def course-files (for [file (file-seq (io/file json-dir)) :when (.endsWith (.getName file) ".json")]
                    (.getName file)))

(def courses (map read-json course-files))

; This is responsible for coercing the data into the enums expected by Postgres
(defn coerce-as-other [course-map]
  ; make schedule_group into "as-other"
  (-> course-map
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

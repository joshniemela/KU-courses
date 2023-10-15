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
            [db-manager.db :refer [course-to-transaction schema remove-nils]]
            [course-scraper.watcher :refer [sitemap-watcher scrape-course]]
            [statistics.core :refer [stats-watcher]]
            [ring.middleware.cors :refer [wrap-cors]]
            [io.staticweb.rate-limit.storage :as storage]
            [io.staticweb.rate-limit.middleware :refer [wrap-rate-limit ip-rate-limit]]
            [datascript.core :as d])
  (:gen-class))

(def conn (d/create-conn schema))

(def storage (storage/local-storage))

; limit each IP to 1000 api calls per hour
(def limit (ip-rate-limit :limit-id 1000 (java.time.Duration/ofHours 1)))
(def rate-limit-config {:storage storage :limit limit})

(def data-dir "../../data/")
(def json-dir (str data-dir "new_json/"))
(def stats-dir (str data-dir "statistics/"))

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
      (api-routes conn)]]
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

(defn try-finding-stats [course-id]
  (try
    ; stats file is in stats-dir
    (let [stats-file (str stats-dir course-id ".json")]
      (json/read-str (slurp stats-file)))
    (catch Exception e
      nil)))

(defn transform-stats [stats]
  (when-not (nil? (stats "exam"))
    (let [exam (stats "exam")
          pass-rate (exam "pass-rate")
          mean (exam "mean")
          median (exam "median")
          graded? (exam "graded")
          grades (exam "grades")]

      {:statistics/pass-rate pass-rate
       :statistics/mean mean
       :statistics/median median
       :statistics/graded? graded?
       :statistics/grades grades})))

(defn read-json-file [file-name]
  (let [file (slurp file-name)]
    (json/read-str file)))
(def courses (map read-json-file (drop 1 (file-seq (clojure.java.io/file json-dir)))))

(def transactions-w-stats (map (fn [course]
                                 (let [course-id (get-in course ["info" "id"])
                                       stats (try-finding-stats course-id)
                                       transacted-course (course-to-transaction course)]
                                   (remove-nils (if stats
                                                  (assoc transacted-course :course/statistics (transform-stats stats))
                                                  transacted-course))))
                               courses))

(def main-config {:port 3000})
(defn -main [& args]
; concurrently run sitemap-watcher scrape-course and stats-watcher so that they don't block the server
  (future (sitemap-watcher scrape-course))
  ;(future (stats-watcher))

  (println "Populating database...")
  (d/transact! conn transactions-w-stats)
  (println "Done!")

  (println "Starting server on port " (:port main-config))
  (run-server (app) main-config))

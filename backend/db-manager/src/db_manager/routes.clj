(ns db-manager.routes
  (:require [clojure.spec.alpha :as s]
            [db-manager.db :refer [get-course-ids
                                   get-course-by-id
                                   get-courses]]
            [clojure.data.json :as json]
            [db-manager.cache :refer [cache]]
            [org.httpkit.client :as http]))

; TODO: fix code duplication, this also apperas in core.clj

(def data-dir "../../data/")
(def json-dir (str data-dir "json/"))
(def stats-dir (str data-dir "statistics/"))

(defn try-finding-stats [course-id]
  (try
    ; stats file is in stats-dir
    (let [stats-file (str stats-dir course-id ".json")]
      (json/read-str (slurp stats-file)))
    (catch Exception e
      nil)))

(def ping-route
  ["/ping"
   {:name :ping
    :get (fn [_]
           {:status 200
            :body "pong"})}])

(defn api-routes [db]
  [["/get-all-course-ids" {:get {:parameters {}
                                 :responses {200 {:body [string?]}}
                                 :handler (fn [_]
                                            {:status 200
                                             :body (get-course-ids db)})}}]

   ; This route is used by the /course/:id route in the frontend, it returns a more detailed course
   ["/get-detailed-course-info" {:get {:parameters {:query {:id string?}}
                                       :responses {200 {:body map?}}
                                       :summary "Get a course by its id"
                                       :description "Returns a course with the given id"
                                       :handler (fn [{{{:keys [id]} :query} :parameters}]
                                                  {:status 200
                                                   :body (get-course-by-id db id)})}}]

   ; Better echo route, not used
   ["/echo" {:post {:parameters {:body map?}
                    :handler (fn [request]
                               (let [body (-> request :parameters :body)]
                                 {:status 200
                                  :body body}))}}]

   ; This route is used by the root route in the frontend, it returns an overview of all matching courses
   ; we expect a map of keys with vectors
   ["/find-course-overviews" {:post {:parameters {:body map?}
                                     :handler (fn [request]
                                                (let [predicates (-> request :parameters :body)]
                                                  {:status 200
                                                   ; make get-courses a partial without the db argument
                                                   :body (let [get-courses-partial (partial get-courses db)
                                                               courses (cache predicates get-courses-partial)]
                                                           {:count (count courses)
                                                            :courses courses})}))}}]
   ["/run-get-on-link" {:post {:parameters {:body map?}
                               :handler (fn [request]
                                          (let [body (-> request :parameters :body)
                                                link (get body :link)]
                                            (println link)
                                            (let [response @(http/get link)]
                                              (if (= (:status response) 200)
                                                (let [body (:body response)]
                                                  {:status 200
                                                   :body body})
                                                (do
                                                  (println response)
                                                  (throw (Exception. "Request failed")))))))}}]])

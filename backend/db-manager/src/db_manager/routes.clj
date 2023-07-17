(ns db-manager.routes
  (:require [clojure.spec.alpha :as s]
            [db-manager.db :refer [find-email-by-name
                                   get-course-ids
                                   get-course-by-id
                                   get-courses]]))

(def ping-route
  ["/ping"
   {:name :ping
    :get (fn [_]
           {:status 200
            :body "pong"})}])

(defn api-routes [db]
  ; make a route that takes json body with values "x" and "y", and returns the sum
  ; not used
  [;["/post" {:post {:parameters {:body {:x int?
  ;                                      :y int?}}
  ;                  :responses {200 {:body {:sum int?}}}

  ;                  :handler (fn [{{{:keys [x y]} :body} :parameters}]
  ;                             {:status 200
  ;                              :body {:sum (+ x y)}})}}]
   ; find email of coordinator in query, not used
   ["/find-email" {:get {:parameters {:query {:name string?}}
                         :responses {200 {:body {:email string?
                                                 :full_name string?
                                                 :search_similarity float?}}}
                         :handler (fn [{{{:keys [name]} :query} :parameters}]
                                    {:status 200
                                     :body (find-email-by-name db name)})}}]
  ; grab all course ids, not used
   ["/get-course-ids" {:get {:parameters {}
                             :responses {200 {:body [map?]}}
                             :handler (fn [_]
                                        {:status 200
                                         :body (get-course-ids db)})}}]

   ; This route is used by the /course/:id route in the frontend, it returns a more detailed course
   ["/get-course" {:get {:parameters {:query {:id string?}}
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
   ["/find-course-overviews" {:post {:parameters {:body map?}
                                     :handler (fn [request]
                                                (let [body (-> request :parameters :body)]
                                                  {:status 200
                                                   :body (let [courses (get-courses db body)]
                                                           {:count (count courses)
                                                            :keys (keys (first courses))
                                                            :courses courses})}))}}]])

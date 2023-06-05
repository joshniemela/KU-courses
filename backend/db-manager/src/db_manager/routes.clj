(ns db-manager.routes
  (:require [clojure.data.json :as json]
            [db-manager.db :refer [find-email-by-name
                                   get-courses
                                   get-course-combined]]))

(def ping-route
  ["/ping"
   {:name :ping
    :get (fn [_]
           {:status 200
            :body "pong"})}])

(defn api-routes [db]
  ; make a route that takes json body with values "x" and "y", and returns the sum
  ; not used
  [["/post" {:post {:parameters {:body {:x int?
                                        :y int?}}
                    :responses {200 {:body {:sum int?}}}

                    :handler (fn [{{{:keys [x y]} :body} :parameters}]
                               {:status 200
                                :body {:sum (+ x y)}})}}]
  ; echo json back, no longer used
   ["/echo" {:post {:parameters {:body {:json map?}}
                    :responses {200 {:body {:json map?}}}
                    :handler (fn [{{{:keys [json]} :body} :parameters}]
                               {:status 200
                                :body {:json json}})}}]
   ; find email of coordinator in query
   ["/find-email" {:get {:parameters {:query {:name string?}}
                         :responses {200 {:body {:email string?
                                                 :full_name string?
                                                 :search_similarity float?}}}
                         :handler (fn [{{{:keys [name]} :query} :parameters}]
                                    {:status 200
                                     :body (find-email-by-name db name)})}}]
  ; grab all courses
   ["/get-course-ids" {:get {:parameters {}
                             :responses {200 {:body [map?]}}
                             :handler (fn [_]
                                        {:status 200
                                         :body (get-courses db)})}}]

   ; grab all the details of one specific course
   ["/get-course" {:get {:parameters {:query {:course_id string?}}
                         :responses {200 {:body {:course_id string?
                                                 :title string?
                                                 :course_language string?
                                                 :description map? ; todo fix to map
                                                 :start_block string?
                                                 :duration int?
                                                 :credits number?
                                                 :study_level string?
                                                 :url string?
                                                 :coordinators [map?]
                                                 :workloads [map?]
                                                 :schedules [map?]}}}
                         :handler (fn [{{{:keys [course_id]} :query} :parameters}]
                                    {:status 200
                                     :body (->
                                            (get-course-combined db course_id)
                                               ; read the json in description
                                            (update
                                             :course/description
                                             json/read-str))})}}]])


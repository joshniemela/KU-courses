(ns db-manager.routes
  (:require [clojure.data.json :as json]
            [db-manager.db :refer [find-email-by-name]]))

(def ping-route
  ["/ping"
   {:name :ping
    :get (fn [_]
           {:status 200
            :body "pong"})}])

(defn api-routes [db]
  ; make a route that takes json body with values "x" and "y", and returns the sum
  [["/post" {:post {:parameters {:body {:x int?
                                       :y int?}}
                   :responses {200 {:body {:sum int?}}} 
                   
                   :handler (fn [{{{:keys [x y]} :body} :parameters}]
                              {:status 200
                               :body {:sum (+ x y)}})}}]
  ; echo json back
  ["/echo" {:post {:parameters {:body {:json map?}}
                   :responses {200 {:body {:json map?}}}
                   :handler (fn [{{{:keys [json]} :body} :parameters}]
                              {:status 200
                               :body {:json json}})}}]
   ; find email of coordinator in query
   ["/find-email" {:get {:parameters {:query {:name string?}}
                         :responses {200 {:body {:email string?}}}
                         :handler (fn [{{{:keys [name]} :query} :parameters}]
                                    {:status 200
                                     :body (find-email-by-name db name)})}}]])
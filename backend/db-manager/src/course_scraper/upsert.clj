
(ns course-scraper.upsert
  (:require [clojure.core :as c]
            [clojure.data.json :as json]
            [clojure.java.io :as io]
            [reitit.coercion.spec]
            [db-manager.db :refer [course-to-transaction remove-nils]]
            [datascript.core :as d])
  (:gen-class))

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

(defn try-finding-stats [stats-dir course-id]
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
          grades (exam "grades")
          absent (exam "absent")
          fail (exam "fail")
          pass (exam "pass")
          total (exam "total")]
      (if graded?
        {:statistics/pass-rate pass-rate
         :statistics/absent absent
         :statistics/fail fail
         :statistics/pass pass
         :statistics/total total
         :statistics/mean mean
         :statistics/median median
         :statistics/grades grades}
        {:statistics/pass-rate pass-rate
         :statistics/pass pass
         :statistics/absent absent
         :statistics/fail fail
         :statistics/total total}))))


(defn transactions-w-stats [stats-finder courses] (map (fn [course]
                                 (let [course-id (get-in course ["info" "id"])
                                       stats (stats-finder course-id)
                                       transacted-course (course-to-transaction course)]
                                   (remove-nils (if stats
                                                  (assoc transacted-course :course/statistics (transform-stats stats))
                                                  transacted-course))))
                               courses))

(defn read-json-file [file-name]
  (let [file (slurp file-name)]
    (json/read-str file)))

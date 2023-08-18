(ns course-scraper.watcher
  (:import (org.jsoup Jsoup)))

(def sitemap-url "http://www.coursera.org/sitemap~.xml")
(defn sitemap-watcher
  "Watches the course sitemap for last-mod newer than time"
  [time callback]
  (let [url (str sitemap-url time)
        response (http/get url)]
    (if (= (:status response) 200)
      (callback (:body response))
      (recur time callback))))

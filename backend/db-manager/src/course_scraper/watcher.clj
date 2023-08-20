(ns course-scraper.watcher
  (:require [clojure.zip :as zip]
            [clojure.xml :as xml]
            [clojure.java.io :as io]
            [org.httpkit.client :as http])
  (:import (javax.net.ssl SSLEngine SSLParameters SNIHostName)
           (java.net URI))

  (:gen-class))

(def data-dir "../../data/pages")

(defn grab-info-from-course [course]
  (let [content (:content course)
        loc (first (filter #(= (:tag %) :loc) content))
        lastmod (first (filter #(= (:tag %) :lastmod) content))
        ; convert YYYY-MM-DD to java.time
        ldt (java.time.LocalDate/parse (first (:content lastmod)))
        instant (.atStartOfDay ldt (java.time.ZoneId/of "Europe/Copenhagen"))
        timestamp (.toEpochSecond instant)]

    {:loc (first (:content loc))
     :id (last (clojure.string/split (first (:content loc)) #"/"))
     :lastmod (first (:content lastmod))
     :timestamp (* 1000 timestamp)}))

(defn grab-mod-date [course-id]
  ; grab the modification date of the file with the course-id as name
  (let [file (io/file data-dir (str course-id ".html"))]
    (if (.exists file)
      (.lastModified file)
      0)))

(defn sitemap-watcher
  "Watches the course sitemap for last-mod newer than time"
  [callback]
  (def sitemap-url "https://kurser.ku.dk/sitemap.xml")

  (def sitemap-zipper (zip/xml-zip (xml/parse sitemap-url)))

; skip the first element, which is the page index, then grab everything
  (def courses (-> sitemap-zipper
                   zip/down
                   zip/right
                   zip/rights))
  ; for every course, grab mod date and check if it's newer than the file
  ; if it is, grab the info from the course and pass it to the callback
  (doseq [course courses]
    (let [course-info (grab-info-from-course course)
          course-id (:id course-info)
          course-mod-date (grab-mod-date course-id)
          course-lastmod (:timestamp course-info)]
      (when (> course-lastmod course-mod-date)
        (callback course-info))))
  ; go to sleep for 30 minutes
  (Thread/sleep (* 1000 60 30))
  (recur callback))

(defn sni-configure
  [^SSLEngine ssl-engine ^URI uri]
  (let [^SSLParameters ssl-params (.getSSLParameters ssl-engine)]
    (.setServerNames ssl-params [(SNIHostName. (.getHost uri))])
    (.setSSLParameters ssl-engine ssl-params)))

(def client (http/make-client {:ssl-configurer sni-configure}))

(def options {:client client :timeout 10000})

(defn scrape-course [course]
  ; slurp loc
  (let [loc (:loc course)]
    (println "Scraping" loc)
    (http/get loc options
              (fn [{:keys [status headers body error]}] ;; asynchronous response handling
                (if error
                  (println "Failed, exception is " error)
                  (do
                    (println "Writing" loc)
                    (spit (str data-dir "/" (:id course) ".html") body))))))
  (Thread/sleep 300))

(ns course-scraper.watcher
  (:require [clojure.zip :as zip]
            [clojure.xml :as xml]
            [clojure.java.io :as io]
            [org.httpkit.client :as http]
            [clojure.java.shell :as shell])
  (:import (javax.net.ssl SSLEngine SSLParameters SNIHostName)
           (java.net URI))

  (:gen-class))

(def pages-dir "../../data/pages")
(def new-json-dir "../../data/new_json")

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

(defn grab-mod-date
  "Grabs the modification date of the file with the course-id as name or 0 if it doesn't exist"
  [course-id]
  (let [file (io/file pages-dir (str course-id ".html"))]
    (if (.exists file)
      (.lastModified file)
      0)))

(defn sitemap-watcher
  "Watches the course sitemap for last-mod newer than time"
  [callback]
  (let [newly-scraped (atom [])
        sitemap-url "https://kurser.ku.dk/sitemap.xml"
        sitemap-zipper (zip/xml-zip (xml/parse sitemap-url))
        courses (-> sitemap-zipper
                    zip/down
                    ; skip the first element, which is the page index, then grab everything
                    zip/right

                    zip/rights)]
    ; for every course, grab mod date and check if it's newer than the file
    ; if it is, grab the info from the course and pass it to the callback
    (println "[course scraper]: Scraping courses")
    (doseq [course courses]
      (let [course-info (grab-info-from-course course)
            course-id (:id course-info)
            course-mod-date (grab-mod-date course-id)
            course-lastmod (:timestamp course-info)]
        (when (> course-lastmod course-mod-date)
          (callback course-info newly-scraped))))
    ; go to sleep for 30 minutes and then do it again
    (println "[course scraper]: Finished scraping, going to sleep")
    (println "[course scraper]: Modified" (count @newly-scraped) "courses")

    (if-not (zero? (count @newly-scraped))
      (let [result (future (shell/sh "rust_parser" pages-dir new-json-dir))]
        (println "[course parser] Running rust parser...")
        (println "[course parser] Parser stderr: " (:err @result))
        (println "[course parser] Finished parsing courses"))
      (println "[course parser] No new courses, not running parser"))

    (reset! newly-scraped [])
    (Thread/sleep (* 1000 60 60)) ;; 1 hour, unit is ms
    (recur callback)))

; Magical snippet of code that allows us to use SNI with http-kit
; https://kumarshantanu.medium.com/using-server-name-indication-sni-with-http-kit-client-f7d92954e165
(defn sni-configure
  [^SSLEngine ssl-engine ^URI uri]
  (let [^SSLParameters ssl-params (.getSSLParameters ssl-engine)]
    (.setServerNames ssl-params [(SNIHostName. (.getHost uri))])
    (.setSSLParameters ssl-engine ssl-params)))

(def client (http/make-client {:ssl-configurer sni-configure}))
(def options {:client client :timeout (* 1000 60 5)})

(defn scrape-course
  "Scrapes the course page and writes it to disk, the 300ms sleep is to avoid DOSing KU"
  [course newly-scraped]
  (let [loc (:loc course)]
    (println "[course scraper]: Scraping" loc)
    (http/get loc options
              (fn [{:keys [status headers body error]}] ;; asynchronous response handling
                (if error
                  (println "[course scraper]: Failed, exception is " error)
                  (do
                    (println "[course scraper]: Writing " loc)
                    (spit (str pages-dir "/" (:id course) ".html") body)
                    (swap! newly-scraped conj course))))))
  (Thread/sleep 300))

(defn generate-url-combinations
  "KU has not given us any useful API and since the exams don't always correspond to the course's block
  we have to generate all combinations of Summer/Winter and the years from now to 2020"
  [course-id]
  (let [base-url "https://karakterstatistik.stads.ku.dk/Histogram/"]
    ; generate all combinations of year from now to 2020 and semester (summer, winter)
    (for [year (range (.getYear (java.time.LocalDate/now)) 2020 -1)
          semester ["Summer" "Winter"]]
      (str base-url course-id "/" semester "-" year))))

(ns statistics.core
  (:import (org.jsoup Jsoup))
  (:require
   [clojure.data.json :as json]
   [clojure.java.io :as io]
   [clojure.string :as str]
   [statistics.utils :refer [stats]])
  (:gen-class))

(def data-dir "../../data/")
(def json-dir (str data-dir "json/"))
(def out-dir (str data-dir "statistics/"))

; Read every json in data-dir
(defn read-json [file]
  (json/read-str (slurp (str json-dir file)) :key-fn keyword))

(defn generate-url-combinations [course-id]
  (let [base-url "https://karakterstatistik.stads.ku.dk/Histogram/"
        ; if the last letter is a U, replace it with an E
        exam-name (if (= \U (last course-id))
                    (str/replace course-id "U" "E")
                    course-id)]
    ; generate all combinations of year from now to 2020 and semester (summer, winter)
    (for [year (range (.getYear (java.time.LocalDate/now)) 2020 -1)
          semester ["Summer" "Winter"]]
      {:url (str base-url exam-name "/" semester "-" year)
       :course-id course-id
       :year year})))

(defn try-scraping
  "Tries to scrape the given url and returns nil if it fails,
  if the error code is 500 it returns nil, otherwise it throws an exception"
  [url]
  (println "[statistics] Trying: " (subs url (inc (.lastIndexOf url "/")) (count url)))
  (try (.get (Jsoup/connect url))
       (catch Exception e
         (let [status (.getStatusCode e)]
           (if (= 500 status)
             nil
             (do
               (println "[statistics] Error fetching: " url)
               (println "[statistics] Status code: " status)
               (throw e)))))))

(defn get-statistics-html
  "Takes a map with the course-id, year and url and associates the html with it if it exists,
  otherwise it returns nil"
  [course]
  (let [course-id (:course-id course)
        combinations (generate-url-combinations course-id)]
    (loop [combinations combinations]
      (when-not (empty? combinations)
        (let [combination (first combinations)
              url (:url combination)
              html (try-scraping url)]
          (if (nil? html)
            ; Sleep 200ms to be nice to the server
            (do (Thread/sleep 200)
                (recur (rest combinations)))
            (do
              (println "[statistics] Found exam for: " course-id)
              (assoc combination :html html))))))))

(defn existing-json? [course-info]
  (let [file (io/file (str out-dir (:course-id course-info) ".json"))]
    (if (.exists file)
      (let [data (json/read (io/reader file))]
        (if (not= (:year data) "2023")
          false
          (= (:re-exam data) nil)))
      true)))
; find all jsons
; TODO: filter out the ones that already exist
; TODO: refactor this since we arent using the start block anymore
(def course-infos-init (for [file (file-seq (io/file json-dir))
                             :when (.endsWith (.getName file) ".json")]
                         (let [course (read-json (.getName file))
                               course-id (:course_id course)
                               start-block (:start_block course)]
                           {:course-id course-id
                            :start-block start-block})))

(def course-infos (filter existing-json? course-infos-init))

(def html-seq (for [course course-infos]
                (get-statistics-html course)))

; HOW TO GENERATE THE COURSE STATISTICS PAGE URL:
; start with base https://karakterstatistik.stads.ku.dk/Histogram/
; add the course-id which also exists in each course map
; the course ID has a "U" at the end, this has to be changed to an "E" for exams
; add semester which is "Winter" or "Summer"
; add year which is the year of the exam
; EXAMPLE: Advanced Algorithms and Data Structures (AADS)
; NDAA09023U - SCIENCE
; =>
; https://karakterstatistik.stads.ku.dk/Histogram/NDAA09023E/Winter-2022

; scrape the table with the grades for reeksamen and the ordinary exam
; only important information is the total numebr of people who took were signed up, total attending, passed and then the table with exam grades where only the numbers are important since hte order and percentage are known

; Make one function that takes a map containing hte course code and the block name and returns the statistics map

; Checks for colspan tag in html
(defn contains-colspan? [elem]
  (let [attributes (.attributes elem)]
    (= "2" (.get attributes "colspan"))))
;TODO make sure both exam and reexam data is contained in HTML
(defn fetch-html [html]
  (filter contains-colspan? (-> (str html)
                                Jsoup/parse
                                (.getElementsByTag "td"))))

; check if the exam table exists
(defn empty-exam? [table]
  (not (< (count (.getElementsByTag table "td")) 3)))

(defn translate-grade [grade]
  (case (str/lower-case grade)
    "ej mødt" "Absent"
    "ikke bestået" "Failed"
    "bestået" "Passed"
    grade))

(defn grade-count-reducer [grades-list three-elems]
  (conj grades-list {:grade (translate-grade (.text (first three-elems)))
                     :count (Integer/parseInt (.text (second three-elems)))}))

(defn fetch-data [table]
  (if (empty-exam? table)
    (reduce grade-count-reducer [] (partition 3 (-> (second (.getElementsByTag table "tbody"))
                                                    (.getElementsByTag "td"))))
    nil))

(defn add-stats [exam-table]
  (when-not (nil? exam-table)
    (stats exam-table)))

(defn build-stats-json [tables]
  (let [exam-table (first tables)
        re-exam-table (second tables)]
    {:exam (add-stats (fetch-data exam-table))
     :re-exam (add-stats (fetch-data re-exam-table))}))

(defn to-json [tables course-id year]
  (spit (str out-dir course-id ".json") (json/write-str (assoc tables :course_id course-id :year year))))
; Parses all the html to json (ergo why the underscore is there)
(defn parse-html [html]
  (when (some? html)
    (to-json (build-stats-json (fetch-html (:html html))) (:course-id html) (:year html))))
; Goes over all html in html-seq and spits them out through parse_html TODO: make this only do one json at a time
(defn spit-all-to-json []
  (doseq [html html-seq]
    (parse-html html)))

(defn stats-watcher
  []
  (io/make-parents (str out-dir "anything here"))
  (spit-all-to-json)
  (Thread/sleep (* 1000 60 60 24))
  (recur))

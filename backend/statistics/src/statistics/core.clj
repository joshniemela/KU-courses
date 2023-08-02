(ns statistics.core
  (:import (org.jsoup Jsoup))
  (:require
   [clojure.data.json :as json]
   [clojure.java.io :as io]
   [clojure.string :as str])
  (:gen-class))

(def data-dir "../../data/")
(def json-dir (str data-dir "json/"))
(def out-dir (str data-dir "statistics/"))

; Read every json in data-dir
(defn read-json [file]
  (json/read-str (slurp (str json-dir file)) :key-fn keyword))

; each course has a key called "start_block", if it's 1 or 2 then it is winter,
; if it's 3 or 4 then it is summer
(defn get-semester [course]
  (let [start-block (:start-block course)]
    (if (or (= start-block 1) (= start-block 2))
      "Winter"
      "Summer")))

(defn get-statistics-html [course year]
  (let [course-id (:course-id course)
        semester (get-semester course)]
    (Thread/sleep 200) ; Be nice to KU's servers
    (try (.get (Jsoup/connect (str "https://karakterstatistik.stads.ku.dk/Histogram/"
                                   (str/replace course-id "U" "E")
                                   "/" semester "-" year)))
         (catch Exception e
           (let [status (.getStatusCode e)]
             (if (= 500 status)
               nil
               (do
                 (println "Error fetching statistics for course: " course-id)
                 (println "Status code: " status)
                 (throw e))))))))

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
(def course-infos-init (for [file (file-seq (io/file json-dir))
                        :when (.endsWith (.getName file) ".json")]
                    (let [course (read-json (.getName file))
                          course-id (:course_id course)
                          start-block (:start_block course)]
                      {:course-id course-id
                       :start-block start-block
                       :semester (get-semester course)})))

(def course-infos (filter existing-json? course-infos-init))

; Puts both html, year and course id in a single map
(defn html-id-map [course year]
  (let [html (get-statistics-html course year)
        course-id (:course-id course)]
    (println "Fetching statistics for course: " course-id " in year: " year "is it empty?" (nil? html))
    {:html html
     :year year
     :course-id course-id}))

(defn attempt-fetch-latest-years [course]
  (let [this-year (.getYear (java.time.LocalDate/now))]
    (loop [years (range this-year (- this-year 3) -1)]
      (when-not (empty? years)
        (let [year (first years)
              html-id (html-id-map course year)]
          (if (nil? (:html html-id))
            (recur (rest years))
            html-id))))))

(def html-seq (for [course course-infos]
                (attempt-fetch-latest-years course)))

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
  (case (lower-case grade)
    "ej mødt" "Absent"
    "ikke bestået" "Failed"
    "bestået" "Passed"
    :else grade))

(defn grade-count-reducer [grades-list three-elems]
  (conj grades-list {:grade (translate-grade (.text (first three-elems)))
                     :count (Integer/parseInt (.text (second three-elems)))}))

(defn fetch-data [table]
  (if (empty-exam? table)
    (reduce grade-count-reducer [] (partition 3 (-> (second (.getElementsByTag table "tbody"))
                                                    (.getElementsByTag "td"))))
    nil))

(defn build-stats-json [tables]
  (let [exam-table (first tables)
        re-exam-table (second tables)]
    {:exam (fetch-data exam-table)
     :re-exam (fetch-data re-exam-table)}))

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

(defn -main
  [& args]
  (println course-infos)
  (if (= (.exists (io/file out-dir)) true)
    (spit-all-to-json)
    ((.mkdir (io/file out-dir))
     (spit-all-to-json))))

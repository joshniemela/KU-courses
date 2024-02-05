(ns statistics.core
  (:import (org.jsoup Jsoup))
  (:require
   [clojure.data.json :as json]
   [clojure.java.io :as io]
   [clojure.string :as str]
   [statistics.utils :refer [stats]])
  (:gen-class))

(def data-dir "../../data/")
(def json-dir (str data-dir "new_json/"))
(def out-dir (str data-dir "statistics/"))


(defn parse-block [block]
  (case block
    "One" 1
    "Two" 2
    "Three" 3
    "Four" 4
    "Summer" 5
    0))

; take a list of blocks in strings "One", "Two", "Three", "Four" and find the smallest
(defn get-first-block [blocks]
  (->> blocks
       (map parse-block)
       (apply min)))

(defn read-json
  "Read a json file and return the data as a map"
  [file]
  (let [old-course (json/read-str (slurp (str json-dir file)) :key-fn keyword)]
    (let [temp (assoc old-course :course-id (get-in old-course [:info :id]))]
    (assoc temp :start-block (get-first-block (get-in old-course [:info :block]))))))

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
(defn generate-url-combinations [course-id]
  (let [base-url "https://karakterstatistik.stads.ku.dk/Histogram/"
        ; The courses end with a U, but the exams end with an E
        exam-name (if (= \U (last course-id))
                    (str/replace course-id "U" "E")
                    course-id)]
    ; Generate all combinations of year from now to 2020 and semester (summer, winter)
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

(defn existing-json? [course-info]
  (let [file (io/file (str out-dir (:course-id course-info) ".json"))]
    (if (.exists file)
      (let [data (json/read (io/reader file))]
        (if (not= (:year data) "2023")
          false
          (= (:re-exam data) nil)))
      true)))
; find all jsons
; TODO: refactor this since we arent using the start block anymore
(def course-infos-init (for [file (file-seq (io/file json-dir))
                             :when (.endsWith (.getName file) ".json")]
                         (let [course (read-json (.getName file))
                               course-id (:course-id course)
                               start-block (:start-block course)]
                           ; FIXME: this can be simplified
                           {:course-id course-id
                            :start-block start-block})))
;(def course-infos-init [{:course-id "NNEB19009U"}])

(println "number of courses: " (count course-infos-init))

; The exams  don't ever change, so we only need to fetch them once
; TODO: this should not be filtering out courses that haven't had their re-exam yet
(def course-infos (filter existing-json? course-infos-init))

; Checks for colspan tag in html, which indicates that the table contains the exam data
(defn contains-colspan? [elem]
  (let [attributes (.attributes elem)]
    (= "2" (.get attributes "colspan"))))
;TODO make sure both exam and reexam data is contained in HTML
(defn fetch-html [html]
  (filter contains-colspan? (-> (str html)
                                Jsoup/parse
                                (.getElementsByTag "td"))))

; Check if the exam table exists
(defn empty-exam? [table]
  (not (< (count (.getElementsByTag table "td")) 3)))

(defn translate-grade [grade]
  (case (str/lower-case grade)
    "ej mødt" "Absent"
    "ikke bestået" "Failed"
    "bestået" "Passed"
    grade))

; The exams are stored in html tables, where each row has three columns (grade, count, percentage)
; We only grab the count and grade
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

(defn save-exam [tables course-id year]
  (spit (str out-dir course-id ".json") (json/write-str (assoc tables :course_id course-id :year year))))

(defn parse-to-tables [html]
  (build-stats-json (fetch-html (:html html))))

(defn spit-all-to-json [exam-data-seq]
  (doseq [exam-data exam-data-seq]
    (when (some? exam-data)
      (let [course-id (:course-id exam-data)
            year (:year exam-data)
            tables (select-keys exam-data [:exam :re-exam])]
        (save-exam tables course-id year)))))



(defn get-statistics-data
  "Takes a map with the course-id, year and url it if it exists,
  otherwise it returns nil"
  [course]
  (let [course-id (:course-id course)
        combinations (generate-url-combinations course-id)]
    (loop [combinations combinations]
      (when-not (empty? combinations)
        (let [combination (first combinations)
              url (:url combination)
              html (try-scraping url)
              exam-data (try (parse-to-tables {:html html})
                                   (catch Exception e
                                     (println "[statistics] Error parsing: " url)
                                     nil))]
          (if (nil? (:exam exam-data))
            ; Sleep 200ms to be nice to the server
            (do (Thread/sleep 200)
                (recur (rest combinations)))
            (do
              (println "[statistics] Found exam for: " course-id)
              (merge combination exam-data))))))))

(def exam-data-seq (for [course course-infos]
                (get-statistics-data course)))



(defn stats-watcher
  []
  (io/make-parents (str out-dir "anything here"))
  (spit-all-to-json exam-data-seq)
  (Thread/sleep (* 1000 60 60 24))
  (recur))

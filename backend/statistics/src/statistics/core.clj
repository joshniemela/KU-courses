(ns statistics.core
  (:require [clojure.data.json :as json]
            [clojure.java.io :as io])
  (:gen-class))

(def data-dir "../../data/")
(def json-dir (str data-dir "json/"))

; Read every json in data-dir
(defn read-json [file]
  (json/read-str (slurp (str json-dir file)) :key-fn keyword))

; find all jsons
(def course-files (for [file (file-seq (io/file json-dir)) :when (.endsWith (.getName file) ".json")]
                    (.getName file)))

(def courses (map read-json course-files))
; each course has a key called "start_block", if it's 1 or 2 then it is winter,
; if it's 3 or 4 then it is summer

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
(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "Hello, World!"))

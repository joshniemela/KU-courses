(ns exam-scraper.core
  (:require [clojure.java.io :as io]
            [clojure.string :as string]
            [clojure.set :as set])
  (:import (java.io File)
          (org.apache.commons.cli DefaultParser)
          (technology.tabula CommandLineApp)))

(defn to-command-line [options]
  (let [parser (DefaultParser.)
        build-options (CommandLineApp/buildOptions)
        args (into-array String options)]
    (.parse parser build-options args)))

(def tabula-options ["-f" "TSV" "-g" "-p" "all"])

(defn convert-exam-pdf-to-tsv [pdf-file out-file]
  (let [cmd-line (to-command-line tabula-options)
        cli-app (CommandLineApp. System/out cmd-line)]
        (.extractFileInto cli-app pdf-file out-file)))

(defn get-itx-courses-from-file [pdf-file]
  ; the course code is on the first column, if the second column contains "ITX" anywheer in the row it's an ITX course
  ; start by converting to tsv at a temporary location
  (let [tsv-file (File/createTempFile "tabula" ".tsv")]
    (convert-exam-pdf-to-tsv pdf-file tsv-file)
    (let [tsv (slurp tsv-file)
            lines (string/split-lines tsv)
            itx-courses (filter #(string/includes? % "ITX") lines)]
        (map #(first (string/split % #"\t")) itx-courses))))

(defn get-itx-courses-from-dir [dir]
  (let [pdf-files (drop 1 (file-seq (io/file dir)))
        itx-courses (mapcat get-itx-courses-from-file pdf-files)]
    (distinct itx-courses)))


(defn to-itx [exams-list]
  ; exams-list is a vector of maps, each key has a key, if this key is "Written", change it to "ITX")
  (map (fn [exam]
         (if (map? exam)
           (set/rename-keys exam {"Written" "ITX"})
              exam)) exams-list))

; I mistankenly thought they were a vector of maps, but they are a vector of maps OR strings


; make a functio nthat only does this for a single course
(defn patch-course-exam [course itx-course-ids]
  (let [course-id (get-in course ["info" "id"])
        itx? (some #(= course-id %) itx-course-ids)]
    (if itx?
      (assoc course "exams" (to-itx (get course "exams")))
      course)))

(defn patch-courses-w-itx [courses itx-course-ids]
    (map #(patch-course-exam % itx-course-ids) courses))

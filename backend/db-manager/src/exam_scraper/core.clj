(ns exam-scraper.core
  (:require [clojure.java.io :as io]
            [clojure.string :as clojure.string])
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
            lines (clojure.string/split-lines tsv)
            itx-courses (filter #(clojure.string/includes? % "ITX") lines)]
        (map #(first (clojure.string/split % #"\t")) itx-courses))))

(defn get-itx-courses-from-dir [dir]
  (let [pdf-files (file-seq (io/file dir))
        itx-courses (mapcat get-itx-courses-from-file pdf-files)]
    (distinct itx-courses)))

(ns statistics.utils
  (:require
   [clojure.java.io :as io]
   [clojure.string :as str]))


;  "exam": [
;    {
;      "grade": "12",
;      "count": 13
;    },
;    {
;      "grade": "10",
;      "count": 24
;    },
;    {
;      "grade": "7",
;      "count": 30
;    },
;    {
;      "grade": "4",
;      "count": 22
;    },
;    {
;      "grade": "02",
;      "count": 13
;    },
;    {
;      "grade": "00",
;      "count": 11
;    },
;    {
;      "grade": "-3",
;      "count": 9
;    },
;    {
;      "grade": "Failed",
;      "count": 17
;    }

(def passing-grades ["Passed" "12" "10" "7" "4" "02"])
(def failing-grades ["00" "-3" "Failed" "Absent"])

(defn pass-fail-course? [grade]
  (contains? grade "Passed"))

(defn compute-stats [exam-map]
  
          



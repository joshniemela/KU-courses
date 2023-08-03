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

; convert {"grade": "12", "count": 13} to {"12" 13}
(defn transform-obj [obj]
  (into {} (map (fn [x] {(:grade x) (:count x)}) obj)))

(def passing-grades ["Passed" "12" "10" "7" "4" "02"])
(def failing-grades ["00" "-3" "Failed" "Absent"])

(def grade-steps ["12" "10" "7" "4" "02" "00" "-3"])

; if the sum of all the 7 grades is 0 then we can assume the course is a pass/fail course
; and not a graded course
(defn is-pass-fail? [exam-table]
  ; select the grades from the exam table that are in the 7 step scale
  (let [grades (select-keys (transform-obj exam-table) grade-steps)]
    (= 0 (apply + (vals grades)))))

; move this to the testing suite
(def test-table
  [{:grade "12", :count 13}
   {:grade "10", :count 24}
   {:grade "7", :count 30}
   {:grade "4", :count 22}
   {:grade "02", :count 13}
   {:grade "00", :count 11}
   {:grade "-3", :count 9}
   {:grade "Failed", :count 17}])

(def test-pass-fail [{:grade "Passed", :count 13}
                     {:grade "Failed", :count 17}
                     {:grade "02", :count 0}
                     {:grade "00", :count 0}
                     {:grade "-3", :count 0}])


(defn stats-pass-fail [exam-table]
  

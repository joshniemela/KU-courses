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
(def pass-fail-steps ["Passed" "Failed" "Absent"])

(defn grade-repeats [exam-table]
    ; find all the grade-steps that also exist in the exam-table
    ; and repeat them the number of times they appear in the exam-table
  (let [transformed (transform-obj exam-table)
        grades (select-keys transformed grade-steps)]
    (apply concat (map (fn [x] (repeat (transformed x) (Integer/parseInt x))) (keys grades)))))

; if the sum of all the 7 grades is 0 then we can assume the course is a pass/fail course
; and not a graded course
(defn is-pass-fail? [exam-table]
  ; select the grades from the exam table that are in the 7 step scale
  (let [grades (select-keys (transform-obj exam-table) grade-steps)]
    (= 0 (apply + (vals grades)))))

(defn total [exam-table]
  (apply + (vals (transform-obj exam-table))))

(defn pass-total [exam-table]
  (let [grades (select-keys (transform-obj exam-table) passing-grades)]
    (apply + (vals grades))))

(defn fail-total [exam-table]
  (let [grades (select-keys (transform-obj exam-table) failing-grades)]
    (apply + (vals grades))))

(defn pass-rate [exam-table]
  (let [total-pass (pass-total exam-table)
        total-fail (fail-total exam-table)]
    (/ total-pass (+ total-pass total-fail))))

(defn median [exam-table]
  (let [sorted-grades (sort (grade-repeats exam-table))
        total-count (count sorted-grades)]
    (defn nth-elem [n]
      (nth sorted-grades n))
    (if (odd? total-count)
      (nth-elem (/ total-count 2))
      (/ (+ (nth-elem (/ total-count 2)) (nth-elem (dec (/ total-count 2)))) 2))))

(defn stats-pass-fail [exam-table]
  {:pass-rate (pass-rate exam-table)
   :total (total exam-table)
   :pass (pass-total exam-table)
   :fail (fail-total exam-table)
   :absent ((transform-obj exam-table) "Absent")})

(defn stats-graded [exam-table]
  ; calculate std dev, mean, median, mode
    ; return a map of the form
    ; {:std-dev 0.0 :mean 0.0 :median 0.0 :mode 0.0}
  (let [grades (select-keys (transform-obj exam-table) grade-steps)
          ; parse each key as an int and multiply by the val
        sum (apply + (map (fn [x] (* (Integer/parseInt (key x)) (val x))) grades))
        total (total exam-table)
        mean (/ sum total)]
    {:mean mean
     :median (median exam-table)}))

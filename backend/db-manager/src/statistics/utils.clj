(ns statistics.utils)

; convert {"grade": "12", "count": 13} to {"12" 13}
(defn transform-obj [obj]
  (into {} (map (fn [x] {(:grade x) (:count x)}) obj)))

(def passing-grades ["Passed" "12" "10" "7" "4" "02"])
(def failing-grades ["00" "-3" "Failed" "Absent"])

(def grade-steps ["12" "10" "7" "4" "02" "00" "-3"])

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
    ; some weird courses like LNAK10082E have a single graded thing and otherwise pass
    (> 5 (apply + (vals grades)))))

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
  {:pass-rate-w-absent (pass-rate exam-table)
   ; remove the {:grade "Absent" :count x} from the exam-table which is a list of maps
   :pass-rate (pass-rate (filter (fn [x] (not= (:grade x) "Absent")) exam-table))
   :total (total exam-table)
   :pass (pass-total exam-table)
   :fail (fail-total exam-table)
   :absent ((transform-obj exam-table) "Absent")})

(defn squared-diff [x mean]
  (* (- x mean) (- x mean)))

(defn stats-graded [exam-table]
  (let [repeats (grade-repeats exam-table)
        sum (reduce + repeats)
        total (count repeats)
        mean (/ sum total)
        var (/ (reduce + (map (fn [x] (squared-diff x mean)) repeats)) (- total 1))]
    {:mean mean
     :median (median exam-table)
     :var var
     :grades exam-table}))

(defn stats [exam-table]
  (if (is-pass-fail? exam-table)
    (assoc (stats-pass-fail exam-table) :graded false)
    (assoc (merge (stats-pass-fail exam-table) (stats-graded exam-table)) :graded true)))

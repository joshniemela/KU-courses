(ns statistics.core-test
  (:require [clojure.test :refer :all]
            [statistics.core :refer :all]
            [statistics.utils :refer :all]))

(def graded-course [{:grade "12" :count 13}
                    {:grade "10" :count 24}
                    {:grade "7" :count 30}
                    {:grade "4" :count 22}
                    {:grade "02" :count 13}
                    {:grade "00" :count 11}
                    {:grade "-3" :count 9}
                    {:grade "Failed" :count 17}])

(def pass-fail-course [{:grade "Absent" :count 0}
                       {:grade "Passed" :count 13}
                       {:grade "Failed" :count 17}
                       {:grade "02" :count 0}
                       {:grade "00" :count 0}
                       {:grade "-3" :count 0}])

(deftest pass-fail-test
  (testing "Testing the is-pass-fail? function"
    (is (= true (is-pass-fail? pass-fail-course)))
    (is (= false (is-pass-fail? graded-course)))))

(deftest pass-rate-test
  (testing "Testing pass-rate function"
    (is (= 13/30 (pass-rate pass-fail-course)))
    (is (= 102/139 (pass-rate graded-course)))))

(deftest median-test
  (testing "Testing the median function"
    (is (= 9/2 (median [{:grade "02" :count 3} {:grade "7" :count 3}])))
    (is (= 7 (median [{:grade "02" :count 3} {:grade "7" :count 4}])))
    (is (= 2 (median [{:grade "02" :count 4} {:grade "7" :count 3}])))
    (is (= 7 (median [{:grade "02" :count 3} {:grade "7" :count 3} {:grade "10" :count 4}])))))

(deftest stats-test
  (testing "Testing the pass-fail stats function"
    ; the stats function returns a map of the form
    ; {:total 30 :pass-rate 13/30 :pass 13 :fail 17 :absent 0}
    (is (= {:total 30 :pass-rate 13/30 :pass 13 :fail 17 :absent 0} (stats-pass-fail pass-fail-course))))

  (testing "Testing the graded stats function"
    (is (= (stats-graded graded-course)
           {:mean 693/139
            :median 7}))))

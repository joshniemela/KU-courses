(ns statistics.core-test
  (:require [clojure.test :refer :all]
            [statistics.core :refer :all]
            [statistics.utils :refer :all]))

(deftest pass-fail-test
  (testing "Testing the is-pass-fail? function"
    (let [graded-course [{:grade "12" :count 13}
                         {:grade "10" :count 24}
                         {:grade "7" :count 30}
                         {:grade "4" :count 22}
                         {:grade "02" :count 13}
                         {:grade "00" :count 11}
                         {:grade "-3" :count 9}
                         {:grade "Failed" :count 17}]
          pass-fail-course [{:grade "Absent" :count 0}
                            {:grade "Passed" :count 13}
                            {:grade "Failed" :count 17}
                            {:grade "02" :count 0}
                            {:grade "00" :count 0}
                            {:grade "-3" :count 0}]]
      (is (= true (is-pass-fail? pass-fail-course)))
      (is (= false (is-pass-fail? graded-course))))))

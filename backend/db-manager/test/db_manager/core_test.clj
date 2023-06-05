(ns db-manager.core-test
  (:require [clojure.test :refer :all]
            [db-manager.core :refer :all]))

(deftest merge-test
  (let [test-employees [{:email "foo@bar.dk" :name "Erik" :title "CEO"}
                        {:email "foo@bar.dk" :name "Erik" :title "CTO"}
                        {:email "josh@jniemela.dk" :name "Josh" :title "Developer"}]]
    (is (= (merge-employees test-employees)
           [{:email "foo@bar.dk" :name "Erik" :title "CEO, CTO"}
            {:email "josh@jniemela.dk" :name "Josh" :title "Developer"}]))))


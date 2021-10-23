(ns project-euler.solutions.euler012
  (:require [project-euler.utils.numeric :refer [triangular]])
  (:require [project-euler.utils.primes :refer [divisors]]))

(defn euler012
  []
  (str (first (filter #(> (count (divisors %)) 500) (triangular)))))
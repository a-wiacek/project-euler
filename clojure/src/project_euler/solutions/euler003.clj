(ns project-euler.solutions.euler003
  (:require [project-euler.utils.primes :refer [factorization]]))

(defn euler003
  []
  (->> 600851475143
       (factorization)
       (keys)
       (apply max)
       (str)))

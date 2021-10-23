(ns project-euler.solutions.euler007
  (:require [project-euler.utils.primes :refer [primes]]))

(defn euler007
  []
  (str (nth (primes) 10000)))

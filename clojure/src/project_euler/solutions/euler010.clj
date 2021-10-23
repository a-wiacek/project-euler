(ns project-euler.solutions.euler010
  (:require [project-euler.utils.primes :refer [primes-up-to]]))

(defn euler010
  []
  (str (apply + (primes-up-to 2000000))))
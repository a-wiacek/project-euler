(ns project-euler.solutions.euler002
  (:require [project-euler.utils.numeric :refer [fib]]))

(defn euler002
  []
  (->> (fib)
       (take-while #(< % 4000000))
       (filter even?)
       (apply +)
       (str)))

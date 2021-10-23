(ns project-euler.solutions.euler016
  (:require [project-euler.utils.numeric :refer [exp]]))

(defn euler016
  []
  (->> (exp 2N 1000)
       (str)
       (map #(Character/digit % 10))
       (apply +)
       (str)))
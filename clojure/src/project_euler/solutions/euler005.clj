(ns project-euler.solutions.euler005
  (:require [project-euler.utils.numeric :refer [lcm]]))

(defn euler005
  []
  (str (reduce lcm (range 1 21))))

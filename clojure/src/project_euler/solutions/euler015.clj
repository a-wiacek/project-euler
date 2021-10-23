(ns project-euler.solutions.euler015
  (:require [project-euler.utils.numeric :refer [binom]]))

(defn euler015
  []
  (str (binom 40 20)))
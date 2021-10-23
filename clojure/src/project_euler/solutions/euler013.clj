(ns project-euler.solutions.euler013
  (:require [project-euler.utils.input :refer [get-input]]))

(defn euler013
  []
  (.substring (->> (get-input 13)
                   (clojure.string/split-lines)
                   (map bigint)
                   (apply +)
                   str)
              0 10))
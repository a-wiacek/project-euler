(ns project-euler.solutions.euler004)

(defn euler004
  []
  (->> (for [x (range 100 1000) y (range x 1000)] (* x y))
       (filter #(= (str %) (clojure.string/reverse (str %))))
       (apply max)
       (str)))
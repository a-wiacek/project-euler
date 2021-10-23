(ns project-euler.solutions.euler011
  (:require [project-euler.utils.input :refer [get-input]]))

(defn euler011
  []
  (let [input (get-input 11)
        parse-row (fn [row] (vec (map #(Integer/parseInt %) (clojure.string/split row #" "))))
        grid (vec (map parse-row (clojure.string/split-lines input)))
        at (fn [x y] ((grid y) x))
        r (range 0 17) rr (range 0 20) dr (range 0 4)
        horizontal (for [x r y rr] (for [dx dr] (at (+ x dx) y)))
        vertical (for [x rr y r] (for [dy dr] (at x (+ y dy))))
        diagonal-1 (for [x r y r] (for [d dr] (at (+ x d) (+ y d))))
        diagonal-2 (for [x r y r] (for [d dr] (at (+ x d) (- (+ y 3) d))))
        all-lines (concat horizontal vertical diagonal-1 diagonal-2)]
    (str (apply max (map (partial apply *) all-lines)))))
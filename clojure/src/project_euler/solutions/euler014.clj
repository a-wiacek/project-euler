(ns project-euler.solutions.euler014)

(def collatz
  (memoize (fn [n]
    (if (= 1 n)
      0
      (inc (collatz (if (even? n) (/ n 2) (+ (* 3 n) 1))))))))

(defn euler014
  []
  (str (apply max-key collatz (range 1 1000000))))
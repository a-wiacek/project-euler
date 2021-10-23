(ns project-euler.solutions.euler009)

(defn euler009
  []
  (let [r (range 1 444)]
    (str (first
      (for [x r y r :let [z (- 1000 x y)] :when (= (* z z) (+ (* x x) (* y y)))]
        (* x y z))))))


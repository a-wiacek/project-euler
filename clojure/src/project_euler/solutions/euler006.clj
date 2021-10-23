(ns project-euler.solutions.euler006)

(defn euler006
  []
  (let [r (range 1 101) square #(* % %)]
    (str (- (square (apply + r)) (apply + (map square r))))))
(ns project-euler.solutions.euler001)

(defn euler001
  []
  (let [bound 999
        sum-multiples-up-to (fn [k] (let [l (quot bound k)] (quot (* k l (+ l 1)) 2)))]
    (str (- (+ (sum-multiples-up-to 3)
               (sum-multiples-up-to 5))
            (sum-multiples-up-to 15)))))

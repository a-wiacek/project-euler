(ns project-euler.solutions.euler017)

(defn itos
  [n]
  (let [first-numbers ["one" "two" "three" "four" "five" "six" "seven" "eight"
                       "nine" "ten" "eleven" "twelve" "thirteen" "fourteen"
                       "fifteen" "sixteen" "seventeen" "eighteen" "nineteen"]
        dozens ["twenty" "thirty" "forty" "fifty" "sixty" "seventy" "eighty" "ninety"]]
    (cond
      (= n 1000) "onethousand"
      (< n 20) (first-numbers (dec n))
      (>= n 100) (let [prefix (str (first-numbers (dec (quot n 100))) "hundred")
                       rem (mod n 100)]
                    (if (zero? rem) prefix (str prefix "and" (itos rem))))
      :else (let [prefix (dozens (- (quot n 10) 2)) rem (mod n 10)]
              (if (zero? rem) prefix (str prefix (itos rem)))))))

(defn euler017
  []
  (str (apply + (map (comp count itos) (range 1 1001)))))
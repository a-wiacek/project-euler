(ns project-euler.utils.numeric)

(defn fib
  "Generate a lazy infinite sequence of Fibonacci numbers.
  Call with two arguments to manually give first two
  elements of the sequence or call with no arguments to
  start with default arguments: 0 and 1."
  ([] (fib 0 1))
  ([a b] (map first (iterate (fn [[a b]] [b (+ a b)]) [a b]))))

(defn triangular
  "Generate a lazy infinite sequence of triangular numbers: 1, 3, 6, 10, ..."
  []
  (map first (iterate (fn [[res ctr]] [(+ res ctr 1) (inc ctr)]) [1 1])))

(defn gcd
  "Compute greatest common divisor of two numbers."
  [a b]
  (if (zero? b) a (recur b (mod a b))))

(defn lcm
  "Compute last common multiple of two numbers."
  [a b]
  (/ (* a b) (gcd a b)))

(defn binom
  "Compute binomial coefficient (n choose k).
  It is assumed that 0 <= k <= n."
  [n k]
  (let [k (if (> (+ k k) n) (- n k) k)]
    (loop [acc 1 i 1]
      (if (> i k) acc (recur (/ (* acc (- (inc n) i)) i) (inc i))))))

(defn exp
  "Compute b^e or b^e mod m, using fast exponentiation algorithm.
  Assumes that e is nonnegative."
  ([b e] (exp b e nil))
  ([b e m] (if (zero? b) 0 (let [* (if (nil? m) * (fn [a b] (mod (* a b) m)))]
    (loop [b b e e acc (/ b b)] (if (zero? e) acc
      (recur (* b b) (quot e 2) (if (even? e) acc (* b acc)))))))))
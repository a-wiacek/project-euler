(ns project-euler.utils.primes)

;; https://stackoverflow.com/questions/960980/fast-prime-number-generation-in-clojure
(defn primes
  "Creates an infinite sequence of prime numbers."
  []
  (letfn [(reinsert [table x prime]
            (update-in table [(+ prime x)] conj prime))
          (primes-step [table d]
            (if-let [factors (get table d)]
              (recur (reduce #(reinsert %1 d %2) (dissoc table d) factors)
                     (inc d))
              (lazy-seq (cons d (primes-step (assoc table (* d d) (list d))
                                             (inc d))))))]
    (primes-step {} 2)))

(defn prime?
  "Checks if a number is prime."
  [n]
  (letfn [(find
            [[p & ps]]
            (case (compare p n)
              -1 (recur ps)
              0 true
              1 false))]
    (find (primes))))

(defn primes-vector-up-to
  "Given a number n, generate a vector of length (n + 1) such that
  k-th position is true if and only if k is prime."
  [n]
  (let [v (transient (into [false false] (repeat (dec n) true)))]
    (doseq [k (take-while #(>= n (* % %)) (iterate inc 2))]
      (when (v k) (doseq [l (range (* k k) (inc n) k)] (assoc! v l false))))
    (persistent! v)))

(defn primes-up-to
  "Given a number n, generate a sequence containing all primes up to n."
  [n]
  (->> n
       (primes-vector-up-to)
       (map-indexed vector)
       (filter second)
       (map first)))

(defn hard-div
  "Given two numbers n and p, return [r e] such that n = r * p^e
  and r is not divisible by p."
  [n p]
  (loop [r n e 0]
    (if (zero? (mod r p))
      (recur (/ r p) (inc e))
      [r e])))

(defn factorization
  "Given a number $n$, compute its factorization. The output is a map:
  keys are prime numbers and values are multiplicities."
  [n]
  (let [m (transient {})]
    (loop [[p & ps] (primes) n n]
      (if (== 1 n)
        (persistent! m)
        (let [[r e] (hard-div n p)]
          (when-not (zero? e) (assoc! m p e))
          (recur ps r))))))

(defn divisors
  "Given a positive number n, generate sorted sequence containing all its divisors."
  [n]
  (let [small-divisors (vec (filter #(zero? (mod n %)) (take-while #(<= (* % %) n) (iterate inc 1))))
        ld (reverse (map #(/ n %) small-divisors))
        large-divisors (let [[h & _] ld] (if (= (* h h) n) (rest ld) ld))]
    (concat small-divisors large-divisors)))
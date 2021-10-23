(ns project-euler.solutions.euler084)

(defrecord Square [n])
(def square-go (Square. 0))
(def square-r1 (Square. 5))
(def square-jail (Square. 10))
(def square-c1 (Square. 11))
(def square-e3 (Square. 24))
(def square-g2j (Square. 30))
(def square-h2 (Square. 39))
(def squares-cc #{(Square. 2) (Square. 17) (Square. 33)})
(def squares-ch #{(Square. 7) (Square. 22) (Square. 36)})

(defn step-by
  [{n :n} step]
  (Square. (mod (+ n step) 40)))

(defn next-r
  [{n :n}]
  (Square. (cond
    (and (> n 5) (< n 15)) 15
    (and (> n 15) (< n 25)) 25
    (and (> n 25) (< n 35)) 35
    :else 5)))

(defn next-u
  [{n :n}]
  (Square. (if (and (> n 12) (< n 28)) 28 12)))

(defrecord GameState [position visited-counter doubles community-deck chance-deck dice-sides])

(defn new-game-state
  [dice-sides]
  (GameState.
    square-go
    (transient (vec (repeat 40 0)))
    0
    (shuffle (into (vec (repeat 14 nil)) [:go :jail]))
    (shuffle (into (vec (repeat 6 nil)) [:go :jail :c1 :e3 :h2 :r1 :next-r :next-r :next-u :back-3]))
    dice-sides))

(defn roll-dice
  [{sides :dice-sides}]
  (inc (rand-int sides)))

(defn go-to-square
  [{ctr :visited-counter :as game-state} square]
  (let [{n :n :as position} (if (= square square-g2j) square-jail square)]
    (assoc game-state :position position :visited-counter (assoc! ctr n (inc (get ctr n))))))

(defn interpret-card
  [{pos :position :as game-state} card]
  (go-to-square game-state
    (case card
      :go square-go
      :jail square-jail
      :c1 square-c1
      :e3 square-e3
      :h2 square-h2
      :r1 square-r1
      :next-r (next-r pos)
      :next-u (next-u pos)
      :back-3 (step-by pos 37)
      pos)))

(defn draw-card
  [deck-key]
  (fn [{[card & cards] deck-key :as game-state}]
      (assoc (interpret-card game-state card) deck-key (into (vec cards) [card]))))

(def draw-community-card (draw-card :community-deck))
(def draw-chance-card (draw-card :chance-deck))

(defn move-pawn
  [game-state position]
  (condp some [position]
    squares-cc (draw-community-card (assoc game-state :position position))
    squares-ch (draw-chance-card (assoc game-state :position position))
    (go-to-square game-state position)))

(defn next-position
  [{:keys [position doubles] :as game-state}]
  (let [roll1 (roll-dice game-state)
        roll2 (roll-dice game-state)
        doubles (if (= roll1 roll2) (inc doubles) 0)]
    (if (= 3 doubles) [0 square-jail] [doubles (step-by position (+ roll1 roll2))])))

(defn game-step
  [game-state]
  (let [[doubles position] (next-position game-state)]
    (move-pawn (assoc game-state :doubles doubles) position)))

(defn simulate-game
  [game-state steps]
  (nth (iterate game-step game-state) steps))

(defn get-answer
  [game-state]
  (->> game-state
       :visited-counter
       persistent!
       (map-indexed (fn [i c] [i c]))
       (sort-by second)
       reverse
       (take 3)
       (apply (fn [[a _] [b _] [c _]] (format "%02d%02d%02d" a b c)))))

(defn euler084
  []
  (get-answer (simulate-game (new-game-state 4) 5000000)))
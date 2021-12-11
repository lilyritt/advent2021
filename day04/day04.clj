(ns day04
  (:require [clojure.string :refer [split split-lines]]))

(def moves
  (map read-string
       (-> (slurp "input")
           split-lines
           first
           (split #","))))

(def initial-boards
  (->> (split (slurp "input") #"\s")
       (rest)
       (remove empty?)
       (map read-string)
       (partition (* 5 5))))

(defn x? [x]
  (= x :x))

(defn full-row? [board]
  (some #(every? x? %) board))

(defn winner? [board]
  (let [board (partition 5 board)
        trans (apply map list board)]
    (some full-row? [board trans])))

(defn find-winner [boards]
  (first (filter winner? boards)))

(defn calculate-score [board move]
  (->> board
       (remove x?)
       (reduce +)
       (* move)))

(defn apply-move [acc move]
  (let [boards (map (partial map #(if (= move %) :x %)) (first acc))
        winner (find-winner boards)]
    [(remove winner? boards)
     (if winner
       (calculate-score winner move)
       (second acc))]))

(def solution
  (second (reduce apply-move [initial-boards 0] moves)))

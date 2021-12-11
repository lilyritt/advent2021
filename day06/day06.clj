(ns day06
  (:require [clojure.string :refer [split]]))

(defonce initial-fish
  (map read-string (split (slurp "input") #",")))

(defn simulate [fish _]
  (mapcat #(if (zero? %) [6 8] [(dec %)]) fish))

(defn solve [n]
  (count (reduce simulate initial-fish (range n))))

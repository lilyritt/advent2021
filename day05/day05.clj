(ns day05
  (:require [clojure.string :refer [split split-lines]]))

(defn read-vent [line]
  (mapv #(mapv read-string (split % #",")) (split line #" -> ")))

(defn read-vents [path]
  (->> (slurp path)
       (split-lines)
       (map read-vent)))

(defn not-diagonal? [vent]
  (let [[[x1 y1] [x2 y2]] vent]
    (or (= x1 x2) (= y1 y2))))

(defn vent-coords [vent]
  (let [[[x1 y1] [x2 y2]] vent]
    vent))

(defn apply-vent [grid vent]
  (merge-with + grid (zipmap (vent-coords vent) (repeat 1))))

(defn grid-score [grid]
  (count (filter #(> (val %) 1) grid)))

(defn solve [path]
  (let [vents (filter not-diagonal? (read-vents path))
        grid (reduce apply-vent {} vents)]
    (grid-score grid)))

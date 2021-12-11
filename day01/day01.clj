(ns day01
  (:require [clojure.string :refer [split-lines]]))

(defn parse-file [s]
  (map read-string (split-lines s)))

(defn sliding-sum [l]
  (map (partial reduce +) (partition 3 1 l)))

(defn larger [l]
  (count (filter (partial apply <) (partition 2 1 l))))

(defn -main []
  (->> (slurp "input")
       parse-file
       sliding-sum
       larger))

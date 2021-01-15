(ns part2
    (:require [clojure.string :as str]
              [clojure.java.io :as io]
              hasher))

(defn pad-left [n c s]
    (apply str (seq (concat (repeat (- n (count s)) c) s))))

(defn build-mem-block []
    (with-open [rdr (io/reader (.getPath (io/resource "input")))]
        (let [key (-> (line-seq rdr) first str/trim)]
            (->>
                (range 0 128)
                (map
                    (fn [line-num]
                        (->>
                            (str key "-" line-num)
                            (map #(int %))
                            ((fn [s] (concat s [17 31 73 47 23])))
                            (hasher/compute-hash)
                            (map #(pad-left 8 \0 (Integer/toBinaryString %)))
                            (str/join "")
                            (map (fn [c] { :color 0 :filled (= c \1)}))
                            (vec))))
                (vec)))))

(defn get-square [grid x y]
    ((grid y) x))

(defn set-square [grid x y v]
    (assoc grid y (assoc (grid y) x v)))

(defn get-adjacent-squares [x y]
    (filter
        (fn [[x y]]
            (and (< x 128) (>= x 0) (< y 128) (>= y 0)))
        [[(inc x) y]
         [(dec x) y]
         [x (inc y)]
         [x (dec y)]]))

(defn fill-region [start-grid start-x start-y color]
    (let [square (get-square start-grid start-x start-y)
          colored-square (assoc square :color color)]
        (reduce
            (fn [grid [x y]]
                (let [square (get-square grid x y)]
                    (if (and (:filled square) (= (:color square) 0))
                        (fill-region grid x y color)
                        grid)))
            (set-square start-grid start-x start-y colored-square)
            (get-adjacent-squares start-x start-y))))

(defn find-regions [start-grid]
    (loop [grid start-grid
           x 0
           y 0
           color 1]
        (let [square (get-square grid x y)
              fill-square? (and (:filled square) (= (:color square) 0))
              new-grid (if fill-square? (fill-region grid x y color) grid)
              new-color (if fill-square? (inc color) color)]
            (cond
                (and (= x 127) (= y 127)) (dec new-color)
                (= x 127) (recur new-grid 0 (inc y) new-color)
                :else (recur new-grid (inc x) y new-color)))))

(defn -main []
    (let [grid (build-mem-block)]
        (println (find-regions grid))))

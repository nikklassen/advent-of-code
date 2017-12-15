(ns part1
    (:require [clojure.string :as str]
              [clojure.java.io :as io]
              hasher))

(defn pad-left [n c s]
    (apply str (seq (concat (repeat (- n (count s)) c) s))))

(defn -main []
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
                            (filter #(= % \1))
                            (count))))
                (reduce +)
                (println)))))

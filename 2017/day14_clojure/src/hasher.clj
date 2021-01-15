(ns hasher)

(defn hash-step [state length]
    (let [num-len 256
          pos (:position state)
          to-take (- (min (+ pos length) num-len) pos)
          to-take-wrapped (- length to-take)
          new-list (if (< to-take length)
                      (let [[prefix md-hd] (split-at pos (:nums state))
                            [md-tl rest] (split-at to-take-wrapped prefix)
                            [rev-hd rev-tl] (split-at to-take (reverse (concat md-hd md-tl)))]
                         (concat rev-tl rest rev-hd))
                      (let [[hd rest] (split-at pos (state :nums))
                            [middle tl] (split-at length rest)]
                         (concat hd (reverse middle) tl)))]
       {:nums new-list
        :position (mod (+ pos length (:skip-size state)) num-len)
        :skip-size (inc (:skip-size state))}))

(defn new-hash-state []
    {:position 0
     :skip-size 0
     :nums (range 0 256)})

(defn compute-hash [lengths]
    (->>
        (range 0 64)
        (reduce (fn [state _] (reduce hash-step state lengths)) (new-hash-state))
        ((fn [state] (partition 16 (:nums state))))
        (map #(reduce bit-xor %))))

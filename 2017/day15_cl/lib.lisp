(defstruct gen-state a b count)

(defun range (start end)
   (loop for n from start below end collect n))

(defun generator (seed factor divisor pred)
    (let ((result (mod (* seed factor) divisor)))
        (if (= 0 (mod result pred))
            result
            (generator result factor divisor pred))))

(defun eql-bits? (value-a value-b)
    (= 0 (logxor (logand value-a #xffff) (logand value-b #xffff))))

(defun run-generators (iterations seed-a factor-a pred-a seed-b factor-b pred-b divisor)
    (reduce
        (lambda (state _)
            (let ((value-a (gen-state-a state))
                  (value-b (gen-state-b state))
                  (c (gen-state-count state)))
                (let ((next-a (generator value-a factor-a divisor pred-a))
                      (next-b (generator value-b factor-b divisor pred-b)))
                    (if (eql-bits? next-a next-b)
                        (make-gen-state :a next-a :b next-b :count (+ c 1))
                        (make-gen-state :a next-a :b next-b :count c)))))
        (range 0 iterations)
        :initial-value (make-gen-state :a seed-a :b seed-b :count 0)))

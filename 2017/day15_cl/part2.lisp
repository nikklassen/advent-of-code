(load "lib.lisp")

(defparameter *seed-a* 783)
; (defparameter *seed-a* 65)
(defparameter *seed-b* 325)
; (defparameter *seed-b* 8921)

(format t "~d~%" (gen-state-count (run-generators 5000000 *seed-a* 16807 4 *seed-b* 48271 8 2147483647)))

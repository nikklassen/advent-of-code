(load "lib.lisp")

(defparameter *seed-a* 783)
; (defparameter *seed-a* 65)
(defparameter *seed-b* 325)
; (defparameter *seed-b* 8921)

(format t "~d~%" (gen-state-count (run-generators 40000000 *seed-a* 16807 1 *seed-b* 48271 1 2147483647)))

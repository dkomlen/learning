#lang racket
(require racket/trace)

;; Thinking About Recursion workshop by Nada Amin and William Byrd at Kats Conf (http://www.katsconf.com)

;; accumulator-passing style
(trace-define fact
              (lambda (x y)
                (if (= y 0) x
                    (fact (* x y) (- y 1) ))))
(fact 1 5)

;; continuation-passing style
(define factk
   (lambda (n k)
     (if (= n 0) (k 1)
         (factk (- n 1) (lambda (r) (k (* n r)))))))

(factk 5 (lambda(x) x))

;; Fibonacci direct style
(trace-define fib (lambda (x)
              (if (< x 2) x
              (+ (fib (- x 1)) (fib (- x 2))))))

;; Lists

;; empty list '()
;; non-empty list: (cons head tail)
;; is the list xs empty?: (null? xs)
;; frist, rest

(define sum
  (lambda (xs)
    (if (null? xs) 0
        (+ (first xs) (sum (rest xs))))))

(sum '(1 2 3))

;; exercise
(define square-tree
  (lambda (xs)
    (if (null? xs) '()
        (if (list? xs)
        (cons (square-tree (first xs)) (square-tree (rest xs)))
        (* xs xs)))))

(square-tree '((3) (4 5)))
                   
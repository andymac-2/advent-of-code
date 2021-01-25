#!/usr/bin/env racket
#lang racket

(define number-to-find 2020)

(define (integer-list)
    (map string->number (port->lines)))

(define (find-2020 my-list)
    (match my-list
       [(list n ns ...) (let* (
            [mapper (lambda (x) (+ x n))]
            [found (member number-to-find (map mapper ns))])
            (if found 
                (* n (- number-to-find n))
                (find-2020 ns)))]
        [_ (error "Could not find two numbers that sum to 2020")]))

(displayln (find-2020 (integer-list)))
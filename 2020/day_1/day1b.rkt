#!/usr/bin/env racket
#lang racket

(define (integer-list)
    (map string->number (port->lines)))

(define (find-1-sum number number-list) (match (member number number-list)
    [(list n ...) (car n)]
    [_ #f]))

(define (find-2-sum number number-list) (match number-list
    [(list n ns ...) (match (find-1-sum (- number n) ns)
        [#f (find-2-sum number ns)]
        [product-1-sum (* product-1-sum n)])]
    [_ #f]))

(define (find-3-sum number number-list) (match number-list
    [(list n ns ...) (match (find-2-sum (- number n) ns)
        [#f (find-3-sum number ns)]
        [product-2-sum (* product-2-sum n)])]
    [_ (error "Could not find three numbers that sum to" number)]))

(displayln (find-3-sum 2020 (integer-list)))
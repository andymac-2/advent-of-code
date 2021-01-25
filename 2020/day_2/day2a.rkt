#!/usr/bin/env racket
#lang racket

(struct entry (low high char password) #:transparent)

(define (count-chars char string) 
  (define string-list (string->list string))
  (define (eq-to-char c) (equal? c char))
  (define only-chars (filter eq-to-char string-list))
  (length only-chars))

(define (valid-password my-entry)
  (match my-entry 
    [(entry low high char password)
     (define char-count (count-chars char password))
     (<= low char-count high)]))

(define (parse-line line)
  (define line-match (regexp-match #px"(\\d+)-(\\d+) (.): (.*)" line))
  (match-define (list _ low-string high-string char-string password) line-match)
  (define char (string-ref char-string 0))
  (define low (string->number low-string))
  (define high (string->number high-string))
  (entry low high char password))

(define (valid-line line)
  (valid-password (parse-line line)))

(define (count-valid-lines)
  (length (filter valid-line (port->lines))))

(displayln (count-valid-lines))

(module+ test
  (define-syntax-rule (assert expr)
    (unless expr
      (error "assertion failed:" (quote expr))))

  (assert (equal? (count-chars #\a "aaaa") 4))
  (assert (equal? (count-chars #\b "aabaabbaabaaa") 4))
  
  (assert (valid-password (entry 1 3 #\a "abcde")))
  (assert (not (valid-password (entry 1 3 #\a "aaaaa"))))
  
  (assert (equal? (parse-line "1-3 a: abcde") (entry 1 3 #\a "abcde")))
  (assert (equal? (parse-line "1-3 b: cdefg") (entry 1 3 #\b "cdefg"))))

#!/usr/bin/env racket
#lang racket

(struct entry (low high char password) #:transparent)

(define (valid-password my-entry)
  (match-define (entry low high char password) my-entry)
  (define first-char (string-ref password (- low 1)))
  (define second-char (string-ref password (- high 1)))
  (xor (equal? char first-char) (equal? char second-char)))

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
  
  (assert (valid-password (entry 1 3 #\a "abcde")))
  (assert (not (valid-password (entry 1 3 #\a "aaaaa"))))
  
  (assert (equal? (parse-line "1-3 a: abcde") (entry 1 3 #\a "abcde")))
  (assert (equal? (parse-line "1-3 b: cdefg") (entry 1 3 #\b "cdefg"))))

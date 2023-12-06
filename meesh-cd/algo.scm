(define home-env-val (getenv "HOME"))
(define cdpath-env-val (getenv "CDPATH"))

(define exit-success 0)
(define exit-failure 1)
(define error-no-spec "Error: Neither $HOME nor <directory> specified")


(define (error-out e c) (begin 
			(display e (current-error-port))
			(exit c)))
(define (set-cwd-exit dir) (begin
			      (chdir dir)
			      (exit exit-success)))


(set! current-path (getenv "OLDPWD"))
(set! eligible-paths (string-split cdpath-env-val #\,))

(define (posix-cd-algo (set-dir))
  (cond
    [((= (string-length set-dir) 0) 
      and (= (string-length home-env-val) 0)) 
     				(error-out error-no-spec exit-failure)]
    [((= (string-length set-dir) 0)
      and (> (string-length home-env-val) 0))
     				(set-cwd-exit set-dir)]))



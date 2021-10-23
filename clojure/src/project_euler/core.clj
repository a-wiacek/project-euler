(ns project-euler.core (:gen-class))

(defn run-euler
  [no]
  (let [fname (format "euler%03d" no)
        path (format "project-euler.solutions.%s" fname)
        _ (require (symbol path))
        f (load-string (format "%s/%s" path fname))]
    (println (format "Running problem %d" no))
    (println (time (f)))))

(defn -main
  ([] (println "Pass problem numbers that you want to run in arguments."))
  ([& args] (doseq [arg args] (run-euler (read-string arg)))))

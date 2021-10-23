(ns project-euler.core-test
  (:require [clojure.test :refer :all]
            [clojure.java.io :refer [file]]
            [project-euler.core :refer :all]))

(deftest a-test
  (let [answers (clojure.string/split-lines (slurp (file ".." "txt" "answers.txt")))]
    (doseq [file (rest (file-seq (file "src" "project_euler" "solutions")))]
      (let [fname (.substring (.getName file) 0 8)
            path (format "project-euler.solutions.%s" fname)
            _ (require (symbol path))
            computed ((load-string (format "%s/%s" path fname)))
            f-num (Integer/parseInt (.substring fname 5 8))
            expected (answers (dec f-num))]
        (testing fname (is (= computed expected)))))))


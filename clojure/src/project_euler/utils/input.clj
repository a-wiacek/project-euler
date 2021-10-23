(ns project-euler.utils.input)

(defn get-file
  [ext no]
  (slurp (clojure.java.io/file ".." ext (format "Euler%03d.%s" no ext))))

(defn get-input
  [no]
  (get-file "in" no))

(defn get-txt-solution
  [no]
  (last (clojure.string/split-lines (get-file "txt" no))))

(defn get-tex-solution
  [no]
  (second (reverse (clojure.string/split-lines (get-file "txt" no)))))
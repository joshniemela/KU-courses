(ns db-manager.cli
  (:require [clojure.tools.cli :refer [parse-opts]])
  (:import [java.io File]))

(def cli-options
  [["-f" "--force" "Force reset the database and delete all harvested data"]
   ["-s" "--scrape" "Scrape the data from the web"]
   ["-h" "--help" "Show help"]])

(def help-message "Usage: db-manager [options]
Options:
  -f, --force  Force reset the database and delete all harvested data
  -h, --help   Show help")

; code from https://stackoverflow.com/questions/14409014/piping-a-subprocesses-output-directly-to-stdout-java-clojure
; used to run python scripts and print their output to stdout
(defn- print-return-stream
  [stream]
  (let [stream-seq (->> stream
                        (java.io.InputStreamReader.)
                        (java.io.BufferedReader.)
                        line-seq)]
    (doall (reduce
            (fn [acc line]
              (println line)
              (if (empty? acc) line (str acc "\n" line)))
            ""
            stream-seq))))

(defn exec-stream
  "Executes a command in the given dir, streaming stdout and stderr to stdout,
    and once the exec is finished returns a vector of the return code, a string of
    all the stdout output, and a string of all the stderr output"
  [dir command & args]
  (let [runtime  (Runtime/getRuntime)
        proc     (.exec runtime (into-array (cons command args)) nil (File. dir))
        stdout   (.getInputStream proc)
        stderr   (.getErrorStream proc)
        outfut   (future (print-return-stream stdout))
        errfut   (future (print-return-stream stderr))
        proc-ret (.waitFor proc)]
    [proc-ret @outfut @errfut]))

(defn scrape-courses! []
  ; run exec stream and occasionally force an output
  (let [[ret out err] (exec-stream "../courses" "pipenv" "run" "python" "main.py")]
    (println "scrape.py returned" ret)
    (println "stdout:")
    (println out)
    (println "stderr:")
    (println err)))

; return the options that are relevant
(defn parse-cli [args]
  (let [parsed-opts (:options (parse-opts args cli-options))]
    (when (:help parsed-opts)
      (println help-message)
      (System/exit 0))
    parsed-opts))




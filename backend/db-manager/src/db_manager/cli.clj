(ns db-manager.cli
  (:require [clojure.tools.cli :refer [parse-opts]]))


(def cli-options
  [["-f" "--force" "Force reset the database and delete all harvested data"]
   ["-s" "--scrape" "Scrape the data from the web"]
   ["-h" "--help" "Show help"]])

(def help-message "Usage: db-manager [options]
Options:
  -f, --force  Force reset the database and delete all harvested data
  -h, --help   Show help")



; return the options that are relevant
(defn parse-cli [args]
  (let [parsed-opts (:options (parse-opts args cli-options))]
    (when (:help parsed-opts)
      (println help-message)
      (System/exit 0))
    parsed-opts))




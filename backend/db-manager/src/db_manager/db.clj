(ns db-manager.db 
  (:refer-clojure :exclude [filter for group-by into partition-by set update])
  (:require [next.jdbc :as jdbc]
            [honey.sql :as sql]
            [clojure.java.io :as io]
            [honey.sql.helpers :refer :all :as h]))

(defn init-db [db]
  (jdbc/with-transaction [tx db]
    (jdbc/execute! tx [(slurp (io/resource "nuke.sql"))])
    (jdbc/execute! tx [(slurp (io/resource "types.sql"))])
    (jdbc/execute! tx [(slurp (io/resource "schema.sql"))])))

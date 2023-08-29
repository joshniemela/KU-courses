(ns db-manager.cache)

(defn cache
  "This simply caches the result of a function call. It is used for memoizing the
  results of a data-source (for instance, a database query)."
  [hashable data-source]
  (let [cache-atom (atom {})
        hashed (hash hashable)
        result (get @cache-atom hashed)]
    (if result
      result
      (let [result (data-source hashable)]
        (swap! cache-atom assoc hashed result)
        result))))

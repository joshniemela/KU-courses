(ns db-manager.cache)

(defn cache [hashable data-source]
  (let [cache-atom (atom {})
        hashed (hash hashable)
        result (get @cache-atom hashed)]
    (if result
      result
      (let [result (data-source hashable)]
        (swap! cache-atom assoc hashed result)
        result))))

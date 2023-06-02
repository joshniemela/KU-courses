(defproject db-manager "0.1.0-SNAPSHOT"
  :description "FIXME: write description"
  :url "http://example.com/FIXME"
  :license {:name "EPL-2.0 OR GPL-2.0-or-later WITH Classpath-exception-2.0"
            :url "https://www.eclipse.org/legal/epl-2.0/"}
  :dependencies [[org.clojure/clojure "1.11.1"]
                 [com.github.seancorfield/next.jdbc "1.3.834"]
                 [org.postgresql/postgresql "42.2.10"]
                 [com.github.seancorfield/honeysql "2.3.928"]
                 [org.clojure/data.json "2.4.0"]
                 [http-kit "2.3.0"]
                 [ring "1.10.0"]
                 [metosin/reitit "0.6.0"]
                 [metosin/muuntaja "0.6.8"]]
  :main ^:skip-aot db-manager.core
  :resource-path "resources"
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all
                       :jvm-opts ["-Dclojure.compiler.direct-linking=true"]}})

(defproject db-manager "1.0.0"
  :description ""
  :url "https://github.com/joshniemela/disproject"
  :license {:name ""
            :url ""}
  :dependencies [[org.clojure/clojure "1.11.1"]
                 [com.github.seancorfield/next.jdbc "1.3.834"]
                 [org.postgresql/postgresql "42.2.10"]
                 [com.github.seancorfield/honeysql "2.3.928"]
                 [org.clojure/data.json "2.4.0"]
                 [http-kit "2.3.0"]
                 [ring "1.10.0"]
                 [metosin/reitit "0.6.0"]
                 [metosin/muuntaja "0.6.8"]
                 [metosin/reitit-swagger-ui "0.7.0-alpha4"]
                 [org.clojure/tools.cli "1.0.214"]
                 [ring-cors "0.1.13"]
                 [io.staticweb/rate-limit "1.1.0"]]
  :main ^:skip-aot db-manager.core
  :resource-path "resources"
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all
                       :jvm-opts ["-Dclojure.compiler.direct-linking=true"]}})

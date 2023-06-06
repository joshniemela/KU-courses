(ns db-manager.core
  (:require [clojure.core :as c]
            [clojure.data.json :as json]
            [clojure.java.io :as io]
            [muuntaja.core :as m]
            [reitit.ring :as ring]
            [reitit.coercion.spec]
            [reitit.ring.coercion :as rrc]
            [reitit.ring.middleware.muuntaja :as muuntaja]
            [reitit.ring.middleware.parameters :as parameters]
            [reitit.swagger-ui :as swagger-ui]
            [reitit.swagger :as swagger]
            [org.httpkit.server :refer [run-server]]
            [db-manager.routes :refer [ping-route api-routes]]
            [db-manager.db :refer [nuke-db! populate-courses! find-email-by-name
                                   get-course-combined]]
            [db-manager.cli :refer [parse-cli scrape-courses!]]
            [next.jdbc :as jdbc]
            [next.jdbc.types :refer [as-other]]
            [honey.sql :as sql]))

(def db-config
  {:dbtype "postgresql"
   :dbname "admin"
   :host "localhost"
   :user "admin"
   :password "admin"
   :stringtype "unspecified"})

(def data-dir "../../data/")

(def json-dir (str data-dir "json/"))

(def db (jdbc/get-datasource db-config))

; https://andersmurphy.com/2022/03/27/clojure-removing-namespace-from-keywords-in-response-middleware.html
(defn transform-keys
  [t coll]
  (clojure.walk/postwalk (fn [x] (if (map? x) (update-keys x t) x)) coll))

(defn remove-namespace-keywords-in-response-middleware [handler & _]
  (fn [req]
    (let [resp (handler req)]
      (cond-> resp
        (comp map? :body) (update :body
                                  (partial transform-keys
                                           (comp keyword name)))))))

(defn app []
  (ring/ring-handler
   (ring/router
    [["/swagger.json"
      {:get {:no-doc true
             :swagger {:info {:title "DISKU backend API"}
                       :basePath "/"} ;; prefix for all paths
             :handler (swagger/create-swagger-handler)}}]
     ["/api" {:middleware [remove-namespace-keywords-in-response-middleware]}
      ping-route
      (api-routes db)
      ]]
    {:data {:coercion reitit.coercion.spec/coercion
            :muuntaja m/instance
            :middleware [
                         
                         parameters/parameters-middleware
                         muuntaja/format-middleware
                         rrc/coerce-exceptions-middleware
                         rrc/coerce-request-middleware
                         rrc/coerce-response-middleware
                         ]}})
   (ring/routes
    (swagger-ui/create-swagger-ui-handler {:path "/swagger"})
    (ring/create-default-handler))))

; read every json in data-dir
(defn read-json [file]
  (json/read-str (slurp (str json-dir file)) :key-fn keyword))

; find all jsons
(def course-files (for [file (file-seq (io/file json-dir)) :when (.endsWith (.getName file) ".json")]
                    (.getName file)))

(def courses (map read-json course-files))

(defn coerce-as-other [course-map]
  ; make schedule_group into "as-other"
  (-> course-map
      (assoc :schedule_group (as-other (:schedule_group course-map)))
      (assoc :start_block (as-other (:start_block course-map)))
      ; workloads is a vector of maps with :workload_type and :hours
      ; workload_types should have as-other
      (update :workloads #(map (fn [workload]
                                 (assoc workload :workload_type (as-other (:workload_type workload))))
                               %))
      ; exact same thing with schedule_groups
      (update :schedules #(map (fn [schedule_group]
                                 (assoc schedule_group :schedule_type (as-other (:schedule_type schedule_group))))
                               %))
      ; same with exams
      (update :exams #(map (fn [exam]
                             (assoc exam :exam_type (as-other (:exam_type exam))))
                           %))))

(def coerced-courses (pmap coerce-as-other courses))

(def main-config {:port 3000})

(defn -main [& args]
  (let [args (parse-cli args)]
    ; this runs if -s is passed
    (when (:scrape args)
      (println "Scraping courses from the web... (this may take a while)")
      (scrape-courses!))

    ; this runs if -f is passed
    (if (:force args)
      (do (println "Nuking database and repopulating with courses from" json-dir)
          (nuke-db! db)
          (populate-courses! db coerced-courses))
      (println "Starting database with existing data..."))
    (println "Starting server on port " (:port main-config))
    (run-server (app) main-config)))

(comment
  (defn query [predicates] (str "SELECT
	course.title,
    course.course_id, 
    jsonb_agg(DISTINCT to_jsonb(exam) - 'course_id') AS exams,
    jsonb_agg(DISTINCT to_jsonb(coordinates) - 'course_id') AS coordinators,
	jsonb_agg(DISTINCT to_jsonb(schedule) - 'course_id') AS schedules,
    jsonb_agg(DISTINCT to_jsonb(workload) - 'course_id') AS workloads
FROM 
    exam
JOIN 
    workload ON exam.course_id = workload.course_id
JOIN 
    coordinates ON exam.course_id = coordinates.course_id
JOIN 
    course ON exam.course_id = course.course_id
JOIN
	schedule ON exam.course_id = schedule.course_id
WHERE " (clojure.string/join " AND " predicates) 
                                "GROUP BY course.course_id, course.title"))
  
  (def predicate-list ["schedule.schedule_type = 'A'" "coordinates.email = 'jn@di.ku.dk'"])
; use honeysql to apply predicate list
  (def response (jdbc/execute! db [(query predicate-list)]))
  ; read jsons in workloads field in response
  (json/read-str (.getValue (:workloads (first response))))
  
)


; input is a map with a key and a value which sohuld be destructured
(defn equality-query [{key :predicate val :value}]
    (case key
      :schedule_type (str "schedule.schedule_type = '" val "'")
      :email (str "coordinates.email = '" val "'")
      :workload_type (str "workload.workload_type = '" val "'")
      :exam_type (str "exam.exam_type = '" val "'")
      :course_id (str "course.course_id = '" val "'")
      :title (str "course.title = '" val "'")
      :start_block (str "course.start_block = '" val "'")
      :duration (str "course.duration = '" val "'")
      :schedule_group (str "course.schedule_group = '" val "'")
      :course_type (str "course.course_type = '" val "'")
      :course_language (str "course.course_language = '" val "'")))


; take a list of predicates and return a string of them joined by AND
(defn merge-list-with-or [predicates] 
  (str "(" (clojure.string/join " OR " (map equality-query predicates)) ")"))


; take a list of list of predicates and return a string of them joined by AND
(defn merge-lists-with-and [predicates]
  (clojure.string/join " AND " (map merge-list-with-or predicates)))


(def test-predicates [[{:predicate :schedule_type :value "A"} {:predicate :schedule_type :value "B"}]
                      [{:predicate :exam_type :value "written_examination"} {:predicate :exam_type :value "oral_examination"}]])

(println (merge-lists-with-and test-predicates))
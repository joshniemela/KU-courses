(ns db
  )




(def employee-table
  [:employees [[:name [:varchar 255] [:not nil]]
               [:email [:varchar 50] [:not nil]]
               [:title [:varchar 255] [:not nil]]
               [:phone [:varchar 255]] ;might not be allowed by GDPR
               [[:primary-key :email]]]])

(def course-table
  [:courses [[:course-id [:char 10] [:not nil]]
             [:placement [:char 3] [:not nil]]]])



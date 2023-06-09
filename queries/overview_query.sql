WITH final_table AS (
	-- Aggregates exam_types, schedule_type and start_block, so that we convert it to lists.
	WITH courses_exams_schedule AS (
		-- Joins courses, exams and schedules
		WITH courses_and_exams AS(
			-- Joins courses and exams
			WITH courses AS (
				-- Selects the course table
				SELECT 
					title, 
					course_id, 
					course_language, 
					study_level, 
					start_block, 
					credits, 
					description 
				FROM course
			)
			SELECT 
				title, 
				courses.course_id, 
				course_language, 
				study_level, 
				start_block, 
				description, 
				exam_type 
			FROM courses 
			JOIN exam ON courses.course_id = exam.course_id
		)
		SELECT 
			title, 
			courses_and_exams.course_id, 
			course_language, 
			study_level, 
			start_block, 
			exam_type, 
			description, 
			schedule_type 
		FROM courses_and_exams 
		JOIN schedule ON courses_and_exams.course_id = schedule.course_id
	)
	SELECT 
		title, 
		course_id, 
		course_language, 
		study_level, 
		ARRAY_AGG(DISTINCT start_block) AS start_block, 
		ARRAY_AGG(DISTINCT exam_type) AS exam_type, 
		ARRAY_AGG(DISTINCT schedule_type) AS schedule_type, 
		description 
	FROM courses_exams_schedule
	GROUP BY
		title,
		course_id,
		course_language,
		study_level,
		description
)
SELECT * FROM final_table
-- Filters could ex. be: (All courses that start in block 1 and has schedule_type 'C')
WHERE '1' = ANY(start_block) AND 'C' = ANY(schedule_type)

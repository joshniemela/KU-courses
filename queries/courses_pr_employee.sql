WITH course_count AS (
	SELECT count(email) as course_count, email FROM coordinates
	GROUP BY email
)
SELECT full_name, employee.email, course_count FROM employee 
JOIN course_count ON course_count.email = employee.email
ORDER BY course_count DESC;
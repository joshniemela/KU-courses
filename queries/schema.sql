-- Initialize database
BEGIN;

CREATE TABLE IF NOT EXISTS course (
	course_id char(10) PRIMARY KEY,
	primary_title text NOT NULL,
	course_language text,
	description text NOT NULL,
	start_block block_enum NOT NULL,
	duration int,
	schedule_group sch_group,
	credits numeric(3, 1),
	study_level text
);

CREATE TABLE IF NOT EXISTS exam (
	course_id char(10),
	exam_type exam_enum NOT NULL,
	duration_minutes int NOT NULL
);

CREATE TABLE IF NOT EXISTS employee (
	email text PRIMARY KEY,
	full_name text
);

CREATE TABLE IF NOT EXISTS title (
	email text NOT NULL,
	title text NOT NULL -- Potentially enum in the future
);

CREATE TABLE IF NOT EXISTS coordinates (
	email text NOT NULL,
	course_id char(10) NOT NULL
);

CREATE TABLE IF NOT EXISTS workload (
	course_id char(10) NOT NULL,
	workload_type work_enum NOT NULL
);

-- Add constraints
ALTER TABLE exam
    ADD CONSTRAINT fk_course_id FOREIGN KEY (course_id) REFERENCES course (course_id);
	
ALTER TABLE title
	ADD CONSTRAINT fk_email FOREIGN KEY (email) REFERENCES employee (email);
	
ALTER TABLE coordinates
	ADD CONSTRAINT fk_email FOREIGN KEY (email) REFERENCES employee (email);
	
ALTER TABLE coordinates
	ADD CONSTRAINT fk_course_id FOREIGN KEY (course_id) REFERENCES course (course_id);
	
ALTER TAbLE workload
	ADD CONSTRAINT fk_course_id FOREIGN KEY (course_id) REFERENCES course (course_id);


END;

-- Initialize types
BEGIN;
CREATE TYPE block_enum AS ENUM ('1', '2', '3', '4', '5');
CREATE TYPE schedule_enum AS ENUM ('A', 'B', 'C', 'D');
CREATE TYPE exam_enum AS ENUM (
	'oral_examination',
	'written_examination',
	'written_assignment',
	'continuous_assessment',
	'practical_written_examination',
	'practical_oral_examination',
	'oral_defence',
	'portfolio',
	'other'
);
CREATE TYPE work_enum AS ENUM (
	'e_learning',
	'exam',
	'laboratory',
	'study_groups',
	'theory_exercises',
	'field_work',
	'preparation',
	'exam_preparation',
	'excursions',
	'lectures',
	'practical_exercises',
	'project_work',
	'exercises',
	'guidance',
	'class_instruction',
	'practical_training',
	'seminar'
);
END;

-- Initialize types
BEGIN;

CREATE TYPE block_enum AS ENUM (
	'1',
	'2',
	'3',
	'4'
);

CREATE TYPE sch_group AS ENUM (
	'A',
	'B',
	'C',
	'D'
);

CREATE TYPE exam_enum AS ENUM (
	'itx',
	'written',
	'oral',
	'take_home'
);

CREATE TYPE work_enum AS ENUM ( 
	'lectures',
	'exercises',
	'preparation',
	'project_work',
	'exam',
	'other' -- Catch all
);

END;
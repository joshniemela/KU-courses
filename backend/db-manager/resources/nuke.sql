-- NUKE IT
BEGIN;

DROP TABLE IF EXISTS course, exam, title, coordinates, employee, workload;

DROP TYPE IF EXISTS sch_group, block_enum, exam_enum, work_enum;

END;
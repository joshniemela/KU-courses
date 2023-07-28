-- NUKE IT
BEGIN;
DROP TABLE IF EXISTS course,
schedule,
exam,
title,
coordinates,
employee,
workload,
department;
DROP TYPE IF EXISTS schedule_enum,
block_enum,
exam_enum,
work_enum,
study_enum;
END;

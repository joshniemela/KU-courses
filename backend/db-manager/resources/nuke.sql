-- NUKE IT
BEGIN;
DROP TABLE IF EXISTS course,
schedule,
exam,
title,
coordinates,
employee,
workload;
DROP TYPE IF EXISTS schedule_enum,
block_enum,
exam_enum,
work_enum;
END;
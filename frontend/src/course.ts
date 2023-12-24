// TYPES FOR COURSE
// TODO: make workload an enum
export type Workload = {
  hours: number;
  workload_type: string;
};
export type Employee = {
  full_name: string;
  email: string;
};
export type Schedule = {
  schedule_type: string;
};

export type Description = {
  // TODO: rename type and string since it is a reserved keyword
  type: string;
  string: string;
};

export type Exam = {
  minutes: number;
  exam_type: string;
};

export type Course = {
  course: string;
  title: string;
  start_block: number;
  study_level: string;
  duration: number;
  course_language: string;
  credits: number;

  employees: Employee[];
  schedules: Schedule[];
  workloads: Workload[];
  exams: Exam[];
  description: Description[];
  departments: string[];
};

export const empty_course: Course = {
  course: "",
  title: "",
  start_block: 0,
  study_level: "",
  duration: 0,
  course_language: "",
  credits: 0,

  employees: [],
  schedules: [],
  workloads: [],
  exams: [],
  description: [],
  departments: [],
};

export function total_hours(course: Course): number {
  let total = 0;
  course.workload.forEach((workload) => {
    total += workload.hours;
  });
  return total;
}

// Same as course but removed the employees and workloads and desc is just a string
export type Overview = {
  course: string;
  title: string;
  start_block: number;
  study_level: string;
  duration: number;
  course_language: string;
  credits: number;

  schedules: Schedule[];
  exams: Exam[];
  summary: string;
  statistics: Statistics;
};

export type Statistics = {
  mean: number;
  median: number;
  "pass-rate": number;
};

export const empty_overview: Overview = {
  course: "",
  title: "",
  start_block: 0,
  study_level: "",
  duration: 0,
  course_language: "",
  credits: 0,

  schedules: [],
  exams: [],
  summary: "",
  statistics: {
    mean: 0,
    median: 0,
    "pass-rate": 0,
  },
};

// TYPES FOR COURSE
// TODO: make workload an enum
export type Workload = {
  hours: number;
  type: string;
};
export type Employee = {
  full_name: string;
  email: string;
};
export type Schedule = {
  type: string;
};

export type Block = {
  type: string;
};

export type Language = {
  name: string;
};

export type Description = {
  // TODO: rename type and string since it is a reserved keyword
  type: string;
  string: string;
};

export type Exam = {
  duration: number;
  type: string;
};

export type Degree = {
  type: string;
};

export type Department = {
  name: string;
};

export type Faculty = {
  name: string;
};

export type Coordinator = {
  name: string;
  email: string;
};

export type Grade = {
  grade: string;
  count: number;
};

export type Statistics = {
  grades: Grade[];
  fail: number;
  mean: number;
  median: number;
  pass: number;
  absent: number;
  "pass-rate": number;
  total: number;
};

export type Course = {
  department: Department[];
  schedule: Schedule[];
  block: Block[];
  content: string;
  "learning-outcome": string;
  duration: string;
  faculty: Faculty[];
  title: string;
  statistics: Statistics | null;
  ects: number;
  coordinator: Coordinator[];
  language: Language[];
  exam: Exam[];
  id: string;
  degree: Degree[];
  "recommended-qualifications": string;
  workload: Workload[];
};

export const empty_course: Course = {
  department: [],
  schedule: [],
  block: [],
  content: "",
  "learning-outcome": "",
  duration: "",
  faculty: [],
  title: "",
  statistics: null,
  ects: 0,
  coordinator: [],
  language: [],
  exam: [],
  id: "",
  degree: [],
  "recommended-qualifications": "",
  workload: [],
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
  schedule: Schedule[];
  block: Block[];
  title: string;
  statistics: StatisticsOverview | null;
  summary: string;
  ects: number;
  language: Language[];
  exam: Exam[];
  id: string;
  degree: Degree[];
};

export type StatisticsOverview = {
  mean: number;
  median: number;
  "pass-rate": number;
};

export const empty_overview: Overview = {
  schedule: [],
  block: [],
  title: "",
  statistics: null,
  summary: "",
  ects: 0,
  language: [],
  exam: [],
  id: "",
  degree: [],
};

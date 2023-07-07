import { writable } from "svelte/store";
import type { Writable } from "svelte/store";
import { browser } from "$app/environment";

// Generic store functions
function setSessionStore<T>(key: string, value: T): void {
  sessionStorage.setItem(key, JSON.stringify(value));
}

function getSessionStore<T>(key: string): T | null {
  return JSON.parse(sessionStorage.getItem(key) || "null") as T;
}

// A generic writable store that persists to sessionStorage
export function writableSession<T>(key: string, value: T): Writable<T> {
  if (!browser) return writable(value); // Mock for SSR
  const sessionValue = getSessionStore<T>(key);
  if (!sessionValue) setSessionStore(key, value);

  const store = writable(sessionValue || value);
  store.subscribe(value => {
    setSessionStore(key, value);
  });

  return store;
}
//  END Generic store functions

// make a writableSession if we have a browser
const emptyQuery = {
  block: [],
  study_level: [],
  schedule_group: [],
  examination_type: [],
  searches: [],
};

export const queryStore = writableSession("query", emptyQuery);

export function clearAll() {
  // Cause the checkboxes to update
  queryStore.update((store) => {
    store.block = [];
    store.study_level = [];
    store.schedule_group = [];
    store.examination_type = [];
    store.searches = [];
    return store;
  });
}

// API URL
export function apiUrl() {
  // check NODE_ENV
  // check that window is defined
  if (typeof window === "undefined") {
    return "https://disku.jniemela.dk/api"; // SSR
  }

  let hostname = window.location.hostname;
  if (hostname == "localhost") {
    return "http://localhost:3000/api";
  }

  // if running on another host, assume we are in prod
  return "https://" + hostname + "/api";
}

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
  course_id: string;
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
};

export const empty_course: Course = {
  course_id: "",
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
};

export function total_hours(course: Course): number {
  let total = 0;
  course.workloads.forEach((workload) => {
    total += workload.hours;
  });
  return total;
}

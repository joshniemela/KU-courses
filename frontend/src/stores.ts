import { writable, derived, type Writable, type Readable } from "svelte/store"
import { browser } from "$app/environment"
// API URL
export function apiUrl() {
  // check NODE_ENV
  // check that window is defined
  if (typeof window === "undefined") {
    return "https://disku.jniemela.dk/api"
  }

  let hostname = window.location.hostname
  if (hostname == "localhost") {
    return "https://disku.jniemela.dk/api"
  }

  // if running on another host, assume we are in prod
  return "https://" + hostname + "/api"
}

export const SearchTypes = {
  courseTitle: "course_title",
  employeeName: "employee_name",
  description: "description",
} as const

export const StudyLevelTypes = {
  bachelor: "Bachelor",
  master: "Master",
} as const

export const ScheduleGroupTypes = {
  A: "A",
  B: "B",
  C: "C",
  D: "D",
} as const

export const BlockTypes = {
  one: "1",
  two: "2",
  three: "3",
  four: "4",
  five: "5",
} as const

export const ExamTypes = {
  oralExamination: "oral_examination",
  writtenExamination: "written_examination",
  writtenAssignment: "written_assignment",
  continousAssesment: "continuous_assessment",
  practicalWrittenExamination: "practical_written_examination",
  practicalOralExamination: "practical_oral_examination",
  oralDefence: "oral_defence",
  portfolio: "portfolio",
  other: "other",
} as const
/*
FILTER STORE.
Responsible for keeping track of all the currently applied filters.
*/
export const initialFilters: Filters = {
  searches: [],
  study_level: [],
  block: [],
  schedule_group: [],
  exam_type: [],
}


// TODO: why is searches a possible list of strings
export type Filters = {
  searches: { search: string[], type: string }[],
  study_level: string[],
  block: string[],
  schedule_group: string[],
  exam_type: string[],
}



/**
* Constructor function for the filter store. Executes some blackmagic that
* lets us work with the browser's localstorage despite being SSR (SvelteKit).
* Heavily inspired by the workaround provided by @sharath725 here:
* https://www.reddit.com/r/sveltejs/comments/p438og/how_to_access_localstorage_via_store_in_sveltekit/
* Mixed with the localstorage example from:
* https://developer.mozilla.org/en-US/docs/Learn/Tools_and_testing/Client-side_JavaScript_frameworks/Svelte_stores

* @function createFilters()

* OBS: Since we are interacting with local storage, it expects the json to be in
* string format when updating the store
*
* !TODO: Fix the above haha
*/
function createFilters(): Writable<string> {

  let filters: string | null = null
  if (browser) {
    filters = localStorage.getItem("diskuFilter")
  }

  const filterStore = writable(
    filters ?? JSON.stringify(initialFilters)
  )

  filterStore.subscribe(
    (val) => {
      if (browser) {
        localStorage.setItem("diskuFilter", val)
      }
    }
  )

  return filterStore
}

export const filters = createFilters()
/**
 * Small derived store, such that we can subscribe to changes in filters,
 * without having to unpack the string every time.
 */
export const filtersObj: Readable<Filters> = derived(filters, ($filters) => JSON.parse($filters))

type Predicate = { op: string; key: string; value: string }

/**
 * Derived store responsible converting our state to Josh' magical querying
 * language
 * @function joshMagic
 */
function constructPredicate(op: string, key: string, value: string): Predicate {
  return { op: op, key: key, value: value }
}

function searchToPredicate(searchItem: string, key: string): Predicate {
  return constructPredicate("%", key, searchItem)
}

function searchWordToPredicate(searchItem: string, key: string): Predicate {
  return constructPredicate("%>", key, searchItem)
}

function equalityToPredicate(value: string, key: string): Predicate {
  return constructPredicate("=", key, value)
}

// function regexToPredicate(value, key) {
//   return constructPredicate("~*", key, value);
// }

function addSearches(query: Query, state: Filters) {
  for (let i = 0; i < state.searches.length; i++) {
    let searchElem = state.searches[i]
    let andList: Predicate[] = []
    searchElem.search.map((x) => {
      andList.push(searchToPredicate(x, searchElem.type))
      andList.push(searchWordToPredicate(x, searchElem.type))
      //      andList.push(regexToPredicate(x, searchElem.type));
    })
    query = {
      //...query,
      predicates: [...query.predicates, andList],
    }
  }
  return query
}

// function countFilters(state) {
//   let count = 0
//   for (filter in Object.entries(state)) {
//     print(filter)
//   }
//   return count
// }

export const filterCount = derived(filtersObj, ($filtersObj) => {
  let count = 0
  for (let [key, val] of Object.entries($filtersObj)) {
    count = count + val.length
  }
  return count
})

function addStudyLevel(query: Query, state: Filters) {
  let studyLevelList: Predicate[] = []
  state.study_level.map((x) =>
    studyLevelList.push(equalityToPredicate(x, "study_level"))
  )
  query = {
    //...query,
    predicates: [...query.predicates, studyLevelList],
  }
  return query
}

function addBlock(query: Query, state: Filters) {
  let blockList: Predicate[] = []
  state.block.map((x) => blockList.push(equalityToPredicate(x, "start_block")))
  query = {
    //...query,
    predicates: [...query.predicates, blockList],
  }
  return query
}

function addScheduleGroup(query: Query, state: Filters) {
  let scheduleGroupList: Predicate[] = []
  state.schedule_group.map((x) =>
    scheduleGroupList.push(equalityToPredicate(x, "schedule_group"))
  )
  query = {
    //...query,
    predicates: [...query.predicates, scheduleGroupList],
  }
  return query
}

function addExamType(query: Query, state: Filters) {
  let examTypeList: Predicate[] = []
  state.exam_type.map((x) =>
    examTypeList.push(equalityToPredicate(x, "exam_type"))
  )
  query = {
    ...query,
    predicates: [...query.predicates, examTypeList],
  }
  return query
}

type Query = {
  predicates: any[],
}
function convertToQueryStructure(state: Filters) {
  let query: Query = {
    predicates: [],
  }

  // Add searches to predicates
  query = addSearches(query, state)

  // Add study level
  if (state.study_level.length > 0) {
    query = addStudyLevel(query, state)
  }

  // Add block
  if (state.block.length > 0) {
    query = addBlock(query, state)
  }

  // Add schedule group
  if (state.schedule_group.length > 0) {
    query = addScheduleGroup(query, state)
  }

  // Add exam type
  if (state.exam_type.length > 0) {
    query = addExamType(query, state)
  }

  console.log(query)

  return query
}

export const queryStore = derived(filtersObj, ($filtersObj) => {
  return convertToQueryStructure($filtersObj)
})




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

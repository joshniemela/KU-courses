import { writable, derived } from 'svelte/store';
import { browser } from "$app/environment"

// Currently supported search types
export const SearchTypes = {
    courseTitle: 'course_title',
    employeeName: 'employee_name'
}

export const StudyLevelTypes = {
    bachelor: 'Bachelor',
    master: 'Master'
}

export const ScheduleGroupTypes = {
    A: 'A',
    B: 'B',
    C: 'C',
    D: 'D'
}

export const BlockTypes = {
    'one': 1,
    'two': 2,
    'three': 3,
    'four': 4,
    'five': 5
}

export const ExamTypes = {
    oralExamination: 'oral_examination',
	writtenExamination: 'written_examination',
	writtenAssignment: 'written_assignment',
	continousAssesment: 'continuous_assessment',
	practicalWrittenExamination: 'practical_written_examination',
	practicalOralExamination: 'practical_oral_examination',
	oralDefence: 'oral_defence',
	portfolio: 'portfolio',
	other: 'other'
}
/*
FILTER STORE.
Responsible for keeping track of all the currently applied filters.
*/
export const initialFilters = {
    'searches': [
    ],
    'study_level': [],
    'block': [],
    'schedule_group': [],
    'exam_type': []
}

export function checkEmpty(state) {
    let empty = true;
    for (const [key, val] of Object.entries(state)) {
        if (state[key].length > 0) {
            empty = false
        }
    }
    return empty
}

// Helper functions to allow us to store our objects as strings
export const jsonToString = (val) => JSON.stringify(val, null, 2);
const toObj = JSON.parse;

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
function createFilters() {
    // Helper functions to allow us to store our objects as strings
    const toString = (val) => JSON.stringify(val, null, 2);
    const toObj = JSON.parse;

    const filterStore = writable(browser && localStorage.getItem("diskuFilter") || jsonToString(initialFilters));
    
    filterStore.subscribe((val) => {
        if (browser) return (localStorage.diskuFilter = val);
    });

    return filterStore
}


export const filters = createFilters();

/**
* Small derived store, such that we can subscribe to changes in filters,
* without having to unpack the string every time.
*/
export const filtersObj = derived(
    filters,
    $filters => toObj($filters)
);

/**
 * Derived store responsible converting our state to Josh' magical querying
 * language
 * @function joshMagic
*/

// const initialFilters = {
//     'searches': [
//         {
//             'search': ['LinAlg', 'Problem'],
//             'type': 'title',
//         },
//         {
//             'search': ['Jakob', 'Henrik'],
//             'type': 'employee'
//         }
//     ],
//     'study_level': [''],
//     'block': ['']
// }
//
//
function constructPredicate(op, key, value) {
    return {'op': op, 'key': key, 'value': value}
}

function searchToPredicate(searchItem, key) {
    return constructPredicate('%', key, searchItem)
}

function searchWordToPredicate(searchItem, key) {
    return constructPredicate('%>', key, searchItem)
}

function equalityToPredicate(value, key) {
    return constructPredicate('=', key, value)
}

function addSearches(query, state) {
    for (let i = 0; i < state.searches.length; i++) {
        let searchElem = state.searches[i]
        let andList = []
        searchElem.search.map(x => {
            andList.push(searchToPredicate(x, searchElem.type))
            andList.push(searchWordToPredicate(x, searchElem.type))
        })
        query = {
            ...query,
            'predicates': [
                ...query.predicates,
                andList
            ]
        }
    }
    return query
}

function addStudyLevel(query, state) {
    let studyLevelList = [];
    state.study_level.map(x => studyLevelList.push(equalityToPredicate(x, 'study_level')))
    query = {
        ...query,
        'predicates': [
            ...query.predicates,
            studyLevelList
        ]
    }
    return query
}

function addBlock(query, state) {
    let blockList = [];
    state.block.map(x => blockList.push(equalityToPredicate(x, 'start_block')))
    query = {
        ...query,
        'predicates': [
            ...query.predicates,
            blockList
        ]
    }
    return query
}

function addScheduleGroup(query, state) {
    let scheduleGroupList = [];
    state.schedule_group.map(x => scheduleGroupList.push(equalityToPredicate(x, 'schedule_group')))
    query = {
        ...query,
        'predicates': [
            ...query.predicates,
            scheduleGroupList
        ]
    }
    return query
}

function addExamType(query, state) {
    let examTypeList = [];
    state.exam_type.map(x => examTypeList.push(equalityToPredicate(x, 'exam_type')))
    query = {
        ...query,
        'predicates': [
            ...query.predicates,
            examTypeList
        ]
    }
    return query
}
function convertToQueryStructure(state) {
    let query = {
        'predicates': [
        ]
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
        query = addScheduleGroup(query,state)
    }

    // Add exam type
    if (state.exam_type.length > 0) {
        query = addExamType(query, state)
    }

    console.log(query)

    return query
}


export const queryStore = derived(
    filtersObj,
    $filtersObj => {
        let obj = $filtersObj
        return convertToQueryStructure(obj)
    }
)

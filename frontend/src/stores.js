import { writable, derived } from 'svelte/store';
import { browser } from "$app/environment"

// Currently supported search types
export const SearchTypes = ['course_title', 'employee_name']

/*
FILTER STORE.
Responsible for keeping track of all the currently applied filters.
*/
export const initialFilters = {
    'searches': [
    ],
    'study_level': [''],
    'block': ['']
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
    return constructPredicate('%>', key, searchItem)
}

function convertToQueryStructure(state) {
    let query = {
        'predicates': [
        ]
    }

    // Add searches to predicates
    for (let i = 0; i < state.searches.length; i++) {
        let searchElem = state.searches[i]
        let andList = []
        searchElem.search.map(x => {
            andList.push(searchToPredicate(x, searchElem.type))
        })
        query = {
            ...query,
            'predicates': [
                ...query.predicates,
                andList
            ]
        }
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

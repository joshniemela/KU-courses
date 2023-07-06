import { writable } from "svelte/store";
import type { Writable } from "svelte/store";
import { browser } from "$app/environment";


function setSessionStore<T>(key: string, value: T): void {
    sessionStorage.setItem(key, JSON.stringify(value));
}

function getSessionStore<T>(key: string): T | null {
    return JSON.parse(sessionStorage.getItem(key) || "null") as T;
}


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

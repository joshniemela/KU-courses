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

import { writable } from "svelte/store";
import type { Writable } from "svelte/store";

function setSessionStore<T>(key: string, value: T): void {
    sessionStorage.setItem(key, JSON.stringify(value));
}

export function writableSession<T>(key: string, value: T): Writable<T> {
    const sessionValue = JSON.parse(sessionStorage.getItem(key) || "null") as T;
    if (!sessionValue) setSessionStore(key, value);


    const store = writable(sessionValue || value);
    store.subscribe((value) => {
        setSessionStore(key, value);
    });

    return store;
}

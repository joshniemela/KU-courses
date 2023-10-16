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
const STORE_VERSION = 2;
const emptyQuery = {
  blocks: [],
  degrees: [],
  schedules: [],
  exams: [],
  searches: [],
  departments: [],
};


export const queryStore = writableSession("query", emptyQuery);

export function clearAll() {
  // Cause the checkboxes to update
  queryStore.update((store) => {
    store.blocks = [];
    store.degrees = [];
    store.schedules = [];
    store.exams = [];
    store.searches = [];
    store.departments = [];
    return store;
  });
}

export function checkStore() {
  // Check if the store is up to date
  if (sessionStorage.getItem("version") != STORE_VERSION.toString()) {
    // we want to not only clearAll but also remove the store
    sessionStorage.clear();
    clearAll();
    sessionStorage.setItem("version", STORE_VERSION.toString());
  }
}



// API URL
export function apiUrl() {
  // check that window is defined, this is used for checking if we are running in the browser
  if (typeof window === "undefined") {
    return "https://disku.jniemela.dk:3000/api"; // SSR
  }

  let hostname = window.location.hostname;
  if (hostname == "localhost") {
    return "http://localhost:3000/api";
  }

  // if running on another host, assume we are in prod
  return "https://" + hostname + "/api";
}

function xorString(str: string, key: number): string {
  return str.split("").map(char => String.fromCharCode(char.charCodeAt(0) ^ key)).join("");
}
// mail obfuscator/deobfuscator using XOR, this should return a function with no arguments that returns a string
export function obfuscateEmail(email: string): () => string {
  // generate the key by summing the char codes of the email and mod 256
  const key = email.split("").reduce((acc, char) => acc + char.charCodeAt(0), 0) % 256;
  const obfuscated = xorString(email, key);
  return () => xorString(obfuscated, key);
}

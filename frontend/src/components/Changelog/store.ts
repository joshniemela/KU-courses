import { writable } from "svelte/store";
import { browser } from "$app/environment";

function modalStoreFunctions() {
    const { subscribe, set } = writable<boolean>(false);

    return {
        subscribe,
        open: () => {
            console.log(document.body.scrollTop);
            set(true);
            if (browser) {
                document.body.classList.add("modal-open");
            }
        },
        close: () => {
            set(false);
            if (browser) {
                document.body.classList.remove("modal-open");
            }
        },
    };
}

export const modalStore = modalStoreFunctions();

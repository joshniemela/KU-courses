<script lang="ts">
    import { self } from "svelte/legacy";

    import { onMount } from "svelte";
    import CloseCross from "../../assets/CloseCross.svelte";
    import { modalStore } from "./store";

    const changelogItems = [
        {
            date: "2024-01-13",
            changes: [
                "Fixed opening in new tab and copying of course links.",
                "Fixed back button when coming from an external page.",
            ],
        },
        {
            date: "2024-02-05",
            changes: [
                "Fixed that performing a vector search and a normal filter will destroy results that should appear.",
                "Fixed that statistics weren't being updated.",
            ],
        },
        {
            date: "2024-06-24",
            changes: [
                "Switched to a quantised multilingual search so that results are more accurate, faster and work with all languages.",
            ],
        },
        {
            date: "2025-04-14",
            changes: [
                "Fixed double fetching",
                "Migrated to Svelte 5",
                "Migrated to SvelteKit 2",
            ],
        },

        {
            date: "2025-05-19",
            changes: [
                "Update styling of checkboxes.",
                "Fix unhandled exception when parsing exam types",
            ],
        },

        {
            date: "2025-05-21",
            changes: ["Added the ability to search by language."],
        },
    ];

    // Start with modal closed
    onMount(() => {
        modalStore.close();
    });
</script>

{#if $modalStore}
    <dialog
        class="absolute w-screen h-screen z-10 bg-black/40 flex justify-center items-center"
        onclick={self(modalStore.close)}
    >
        <div
            class="bg-white text-m font-normal h-fit mx-4 max-h-[75vh] md:max-h-[500px] overflow-y-scroll rounded"
        >
            <div
                class="flex justify-between mb-6 sticky top-0 bg-white pt-6 pb-4 border-b-2 px-6"
            >
                <h3 class="font-bold text-2xl">Changelog</h3>
                <button type="button" onclick={modalStore.close}>
                    <CloseCross classes="size-6" />
                </button>
            </div>
            <ul class="space-y-4 mb-6 mt-2 px-6">
                {#each changelogItems.reverse() as { date, changes }}
                    <li>
                        <p class="font-bold text-lg">{date}</p>
                        <ul>
                            {#each changes as change}
                                <li>{change}</li>
                            {/each}
                        </ul>
                    </li>
                {/each}
            </ul>
        </div>
    </dialog>
{/if}

<script lang="ts">
    import { writable } from "svelte/store";
    export const searchTypes = ["Title", "Description", "Coordinator"] as const;
    type SearchType = (typeof searchTypes)[number];
    type Search = {
        category: SearchType;
        query: string;
        fuzzy: boolean;
    };

    // Store for searches
    export let searches: Search[];

    const initialSearch: Search = {
        category: searchTypes[0],
        query: "",
        fuzzy: true,
    } as const;
    let currentSearch: Search = initialSearch;

    function pushCurrentSearch() {
        if (currentSearch.query === "") return; // Don't push empty searches
        // Push to searches and clear currentSearch
        searches = [...searches, currentSearch];

        // TODO: currentSearch = initialSearch; doesn't work
        currentSearch = {
            category: searchTypes[0],
            query: "",
            fuzzy: true,
        };
    }
    function removeSearch(index: number) {
        searches = searches.filter((_, i) => i !== index);
    }

    function formatPlaceholder(search: Search) {
        const uncased =
            (search.fuzzy ? "" : "exact ") +
            "match in " +
            search.category +
            "...";
        // lowercase everything except first letter which will be uppercase
        return uncased.charAt(0).toUpperCase() + uncased.slice(1).toLowerCase();
    }
</script>

<!-- Dropdown menu for categories -->
<div class="flex md:flex-row flex-col">
    <select class="" bind:value={currentSearch.category}>
        {#each searchTypes as type}
            <option value={type}>{type}</option>
        {/each}
    </select>

    <!-- Search bar -->
    <input
        type="text"
        bind:value={currentSearch.query}
        placeholder={formatPlaceholder(currentSearch)}
        on:keypress={(e) => e.key === "Enter" && pushCurrentSearch()}
    />

    <button
        class="bg-kuRed text-white font-bold py-2 px-4"
        on:click={pushCurrentSearch}>Search</button
    >
</div>
<!-- Button to trigger search -->

<!--Show each search in the searches list and make so each of them have a remvoe button-->
{#if searches.length}
    <span> Searching on: </span>
{/if}
<ul>
    {#each searches as search, index}
        <li class="flex">
            <span class="flex items-center">
                {search.category}: {search.query}
                <button
                    class="align-right"
                    on:click={() => removeSearch(index)}
                >
                    <svg
                        class="w-4 h-4 text-gray-800 dark:text-white ml-1"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 14 14"
                    >
                        <path
                            stroke="currentColor"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"
                        />
                    </svg>
                </button>
            </span>
        </li>
    {/each}
</ul>

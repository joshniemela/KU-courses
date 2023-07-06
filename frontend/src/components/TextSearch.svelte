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

    function formatPlaceholder(search: Search) {
        const uncased =
            (search.fuzzy ? "" : "exact ") +
            "match in " +
            search.category +
            "...";
        // lowercase everything except first letter which will be uppercase
        return uncased.charAt(0).toUpperCase() + uncased.slice(1).toLowerCase();
    }
    function removeSearch(index: number) {
        return () => {
            searches = searches.filter((_, i) => i !== index);
        };
    }

    function getIndex(search: Search) {
        return searches.findIndex((s) => s === search);
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
        class={"text-white font-bold py-2 px-4 bg-" +
            (currentSearch.query ? "kuRed" : "kuGray")}
        on:click={pushCurrentSearch}>Add</button
    >
</div>
<!-- Button to trigger search -->

<!--Show each search in the searches list and make so each of them have a remvoe button-->

<div class="flex flex-col md:flex-row md:space-x-4 space-y-4 md:space-y-0">
    {#each searchTypes as type}
        {#if searches.filter((search) => search.category === type).length > 0}
            <div>
                <h1 class="bg-kuGray font-bold px-4 text-white text-xl">
                    {type}
                </h1>
                {#each searches.filter((search) => search.category === type) as search, index}
                    <div class="flex flex-row justify-between">
                        <p>{search.query}</p>
                        <button
                            class="ml-4 text-kuRed font-bold"
                            on:click={removeSearch(getIndex(search))}
                        >
                            X
                        </button>
                    </div>
                {/each}
            </div>
        {/if}
    {/each}
</div>

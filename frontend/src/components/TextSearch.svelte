<script lang="ts">
    import { writable } from "svelte/store";
    const searchTypes = ["Title", "Description", "Coordinator"] as const;
    type SearchType = (typeof searchTypes)[number];
    type Search = {
        category: SearchType;
        query: string;
        fuzzy: boolean;
    };

    // Store for searches
    const searches = writable<Search[]>([]);

    const initialSearch: Search = {
        category: searchTypes[0],
        query: "",
        fuzzy: true,
    } as const;
    let currentSearch: Search = initialSearch;

    function pushCurrentSearch() {
        if (currentSearch.query === "") return; // Don't push empty searches
        // Push to searches and clear currentSearch
        searches.update((prevSearches) => [...prevSearches, currentSearch]);

        // TODO: currentSearch = initialSearch; doesn't work
        currentSearch = {
            category: searchTypes[0],
            query: "",
            fuzzy: true,
        };
    }
    function removeSearch(index: number) {
        searches.update((prevSearches) => {
            // Create a new array without the search at the specified index
            return prevSearches.filter((_, i) => i !== index);
        });
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
<div>
    <select bind:value={currentSearch.category}>
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
Searching on:
<ul>
    {#each $searches as search, index}
        <li class="flex justify-between">
            <span>{search.category}: {search.query}</span>
            <button on:click={() => removeSearch(index)}>
                <svg
                    class="w-6 h-6 text-gray-800 dark:text-white"
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
        </li>
    {/each}
</ul>

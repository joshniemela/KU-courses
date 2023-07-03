<script lang="ts">
    import FilterButton from "../FilterButton/FilterButton.svelte";
    import SearchIcon from "../../assets/SearchIcon.svelte";
    import theme from "../../theme";
    import { goto } from "$app/navigation";

    import {
        filters,
        initialFilters,
        filtersObj,
        SearchTypes,
    } from "../../stores";

    let currentType: string = SearchTypes.courseTitle;
    let searchInput = "";
    let searches = $filtersObj.searches;

    function switchType(newType: string) {
        currentType = newType;
    }

    /**
     * Submits filters and goes to browse
     * @function submiteAndReload
     */
    function submitAndReload(value: string): void {
        $filters = JSON.stringify({
            ...$filtersObj,
            searches: [
                ...$filtersObj.searches,
                {
                    search: value.split(","),
                    type: currentType,
                },
            ],
        });
        goto("/browse");
        location.reload();
    }

    /**
     * navigates to the /browse route and updates the search value.
     * @function submitOnKeydown
     * @param event the event emitted by the component on Keydown
     */
    function submitOnKeydown(event: KeyboardEvent) {
        if (event.key !== "Enter") {
            return;
        }

        if (searchInput.length > 0) {
            submitAndReload(searchInput);
        } else {
            goto("/browse");
        }
    }
    /**
     * navigates to the /browse route and updates the search value.
     * @function submitOnClick
     * @param event the event emitted by the component on click
     */
    function submitOnClick(event: MouseEvent) {
        if (searchInput.length > 0) {
            submitAndReload(searchInput);
        } else {
            goto("/browse");
        }
    }

    function convertExamToString(inputString: string) {
        return inputString.replace(/(\w)_(\w)/g, "$1 $2");
    }
</script>

<div class="root-search-container">
    <div class="search-container">
        <FilterButton />
        <input
            class="search"
            type="search"
            placeholder={searches.length > 0
                ? searches[searches.length - 1].search.join()
                : "Search"}
            style="
            --text-color: {theme.colors.brand[200]};
            --search-bg-color: {theme.colors.neutral[800]}
            "
            on:keydown={submitOnKeydown}
            bind:value={searchInput}
        />
        <SearchIcon on:click={submitOnClick} />
    </div>
    <div class="type-button-container">
        <button
            on:click={() => ($filters = JSON.stringify(initialFilters))}
            class="type-button"
            style="--text-color: {theme.colors.brand[200]}; --bg-color: {theme
                .colors.brand[800]};
            --hover-color: {theme.colors.brand[900]}; --hover-bg: {theme.colors
                .brand[500]}"
        >
            Clear filters
        </button>
        {#each Object.entries(SearchTypes) as [_, type]}
            {#if type == currentType}
                <button
                    class="type-button"
                    style="--text-color: {theme.colors
                        .neutral[900]}; --bg-color: {theme.colors.neutral[200]};
                        --hover-color: {theme.colors
                        .neutral[900]}; --hover-bg: {theme.colors.neutral[200]}"
                    on:click={() => switchType(type)}
                >
                    {convertExamToString(type)}
                </button>
            {:else}
                <button
                    class="type-button"
                    style="--text-color: {theme.colors
                        .neutral[200]}; --bg-color: {theme.colors.neutral[800]};
                        --hover-color: {theme.colors
                        .neutral[900]}; --hover-bg: {theme.colors.neutral[200]}"
                    on:click={() => switchType(type)}
                >
                    {convertExamToString(type)}
                </button>
            {/if}
        {/each}
    </div>
</div>

<style scoped>
    .root-search-container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: 100%;
        width: 100%;
    }
    .search-container {
        display: flex;
        flex-direction: row;
        justify-content: center;
        margin-bottom: 1vh;
        align-items: center;
        height: 100%;
        width: 100%;
    }

    .search {
        font-size: 1.5rem;
        border: 0;
        width: 100%;
        height: 100%;
        padding-left: 1vw;
        color: var(--text-color);
        background-color: var(--search-bg-color);
    }

    .search:focus {
        outline: none !important;
        border: 2px solid var(--text-color);
    }

    .type-button-container {
        display: flex;
        flex-direction: row;
        height: 100%;
        width: 100%;
        justify-content: center;
        align-items: center;
    }

    .type-button {
        background: none;
        border: 0;
        font-size: 1.2rem;
        padding-left: 1%;
        padding-right: 1%;
        padding-top: 0.5%;
        padding-bottom: 0.5%;
        margin-left: 1vw;
        margin-right: 1vw;
        border-color: var(--text-color);
        color: var(--text-color);
        height: 100%;
        background-color: var(--bg-color);
        transition: ease-in-out 0.1s;
    }

    .type-button:hover {
        color: var(--hover-color);
        background-color: var(--hover-bg);
    }
</style>

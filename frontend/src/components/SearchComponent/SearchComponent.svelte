<script>
import FilterButton from "../FilterButton/FilterButton.svelte";
import SearchIcon from "../../assets/SearchIcon.svelte";
import theme from "../../theme";
import { navigate } from "svelte-navigator";
import { filters, initialFilters, filtersObj, jsonToString, SearchTypes } from "../../stores";

let currentType = SearchTypes.courseTitle;
let searchInput = "";
let searches = $filtersObj.searches;

function switchType(newType) {
    currentType = newType
}

/** 
* Submits filters and goes to browse
* @function submiteAndReload
*/
function submitAndReload(value) {
    $filters = jsonToString({
        ...$filtersObj,
        'searches': [
            ...$filtersObj.searches,
            {
                'search': value.split(','),
                'type': currentType
            }
        ]
    })
    navigate('/browse')
    location.reload()
}

/**
* navigates to the /browse route and updates the search value.
* @function submit 
* @param {event} event event: the event emitted by the component on click / enter
*/
function submit(event) {
    if (searchInput.length > 0) {
            if (event.key === 'Enter') {
                submitAndReload(searchInput)
            } else if (event.type === 'clicked') {
                submitAndReload(searchInput)
            }
    } else {
        if (event.key === 'Enter') {
            navigate('/browse')
            location.reload()
        } else if (event.type === 'clicked') {
            navigate('/browse')
            location.reload()
        }
    }
}
</script>
<div class="root-search-container">
    <div class="search-container">
        <FilterButton />
        <input class="search" type="search" placeholder={searches.length > 0 ? searches[searches.length-1].search.join() : 'Search'}
            style="
            --text-color: {theme.colors.brand[200]};
            --search-bg-color: {theme.colors.neutral[800]}
            "
            on:keydown={submit}
            bind:value={searchInput}
        />
        <SearchIcon on:clicked={submit} />
    </div>
    <div class="type-button-container">
        <button on:click={() => $filters = jsonToString(initialFilters)}>Clear filters </button>
        {#each Object.entries(SearchTypes) as [_, type]}
            {#if type == currentType}
                <button
                    class="type-button"
                    style="--text-color: {theme.colors.brand[200]}; --bg-color: {theme.colors.brand[800]}"
                    on:click={() => switchType(type)}
                >
                    { type }
                </button>
            {:else}
                <button
                    class="type-button"
                    style="--text-color: {theme.colors.neutral[200]}; --bg-color: {theme.colors.neutral[800]}"
                    on:click={() => switchType(type)}
                >
                    { type }
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
    border:2px solid var(--text-color);
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
    border-color: var(--text-color);
    color: var(--text-color);
    height: 100%;
    background-color: var(--bg-color);
    transition: ease-in-out 0.1s;
}
</style>

<script>
import theme from '../theme'
import SearchIcon from '../assets/SearchIcon.svelte';
import FilterButton from '../components/FilterButton/FilterButton.svelte';
import { navigate } from 'svelte-navigator';
import { filters, filtersObj, jsonToString, SearchTypes, queryStore, initialFilters, checkEmpty} from '../stores';

let searches = $filtersObj.searches;
let currentType = SearchTypes.courseTitle;
let searchInput = "";
function consoleJosh() {
    console.log($queryStore)
}
function switchType(newType) {
    console.log('click')
    currentType = newType
}

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
    if (searchInput.length > 0 || !checkEmpty($filtersObj)) {
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
}

</script>
    <div class="content">
        <h1 class="title" style="--font-color: {theme.colors.brand[500]}">KU Courses (WIP)</h1>

        <!-- Container responsible for the search area --> 
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
        <p> {searchInput} </p>
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
        <a href="/browse">
        <button class="view-all-button"
            style="
                --bg-color: {theme.colors.neutral[900]};
                --text-color: {theme.colors.brand[200]}
            "
        >View all (WIP)</button>
        </a>
        {#each $filtersObj.searches as searchElem}
            <p>{searchElem.search} - {searchElem.type}</p>
        {/each}
        <button on:click={consoleJosh}> log </button>
</div>

<style scoped>
.title {
    color: var(--font-color);
    margin-bottom: 2vh;
}

.content {
    display: flex;
    height: 100vh;
    width: 100%;
    margin-right: 8vw; /* should be the same as navbar-container width */
    flex-direction: column;
    align-items: center;
    justify-content: center;
}

.search-container {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    margin-bottom: 2vh;
    height: 5vh;
    width: 30vw;
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

.view-all-button {
    background: none;
    font-size: 1rem;
    border: 0;
    color: var(--text-color);
    width: 8vw;
    background-color: var(--bg-color);
    transition: ease-in-out 0.1s;
}

.search-icon-ref {
    height: 5vh;
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

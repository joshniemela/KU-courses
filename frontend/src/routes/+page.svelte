<script>
import theme from '../theme'
import SearchIcon from '../assets/SearchIcon.svelte';
import FilterButton from '../components/FilterButton/FilterButton.svelte';
import SearchComponent from '../components/SearchComponent/SearchComponent.svelte';
import { navigate } from 'svelte-navigator';
import { filters, filtersObj, jsonToString, SearchTypes, queryStore, initialFilters } from '../stores';

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
    <div class="content">
        <h1 class="title" style="--font-color: {theme.colors.brand[500]}">KU Courses (WIP)</h1>

        <!-- Container responsible for the search area --> 
        <div class="search-container">
            <SearchComponent />
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
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 5vh;
    width: 30vw;
    margin-bottom: 2vh;
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


</style>

<script>
import theme from "../../theme";
import SearchIcon from "../../assets/SearchIcon.svelte";
import FilterButton from "../../components/FilterButton/FilterButton.svelte";
import { filters, filtersObj, jsonToString } from '../../stores';
import { onMount } from 'svelte';
import overview from "../../mocking/overview.json";
import OverviewCard from "../../components/OverviewCard/OverviewCard.svelte";

let loading = true;

let courses = [];

/**
* Event handler for submit on search 
*/
function submit(event) {
    console.log(event)
    if (event.key === 'Enter') {
        $filters = jsonToString({
            'search': event.target.value 
        })
    }
}

const fetchCourses = async () => {
    await new Promise(resolve => setTimeout(resolve, 1000));
    loading = false;
    courses = overview;
}


onMount(async () => {
    await fetchCourses();
})

</script>
<div class="browse-container">
    <div class="control-container">
        <FilterButton paddingLR={"2vw"} fontSize={"1.25rem"} />
        <input class="search" type="search" placeholder={$filtersObj.search.length > 0 ? $filtersObj.search : 'Search'}
            style="
            --text-color: {theme.colors.brand[200]};
            --search-bg-color: {theme.colors.neutral[800]}
            "
            value={$filtersObj.search.length > 0 ? $filtersObj.search : ''}
            on:keydown={submit}
        />
        <SearchIcon />
    </div>
    {#if loading}
        <p>loading ...</p>
    {:else}
        <div class="card-container">
            {#each courses as card, i}
                <OverviewCard stagger={i} data={card} />
            {/each}
        </div>
    {/if}
</div>

<style scoped>
.browse-container {
    height: 100vh;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
}

.control-container {
    margin-top: 1vh;
    height: 4vh;
    width: 100%;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
}

.search {
    height: 100%;
    width: 30vw;
    max-width: 800px;
    margin-left: 1vw;
    font-size: 1.25rem;
    padding-left: 1vw;
    border: 0;
    color: var(--text-color);
    background-color: var(--search-bg-color);
}

.search:focus {
    outline: none !important;
    border:2px solid var(--text-color);
}

.card-container {
    height: 100%;
    width:96%;
    margin: 2%;
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    grid-auto-rows: 33vh;
    justify-content: center;
    align-items: center;
    gap: 2vh;
}

</style>



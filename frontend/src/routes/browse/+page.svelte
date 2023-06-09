<script>
import theme from "../../theme";
import SearchIcon from "../../assets/SearchIcon.svelte";
import FilterButton from "../../components/FilterButton/FilterButton.svelte";
import Loader from "../../components/Loader/Loader.svelte";
import { filters, filtersObj, jsonToString, queryStore } from '../../stores';
import { onMount } from 'svelte';
import overview from "../../mocking/overview.json";
import OverviewCard from "../../components/OverviewCard/OverviewCard.svelte";

let loading = true;

let courses = [];

let API_URL = import.meta.env.PROD ? 'https://disku.jniemela/api' : 'http://localhost:3000';
console.log(API_URL)

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
    const filters = $queryStore;
    console.log(filters);
    const res = await fetch(`${API_URL}/find-courses`, {
        method: 'POST',
        headers: {
            'accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(filters)
    })
		
	const json = await res.json();
    console.log(json.courses[0]);
    loading = false;
    courses = json.courses;
}


onMount(async () => {
    await fetchCourses();
})

</script>
<div class="browse-container">
    <div class="control-container">
        <FilterButton paddingLR={"2vw"} fontSize={"1.25rem"} />
        <input class="search" type="search" placeholder={$filtersObj.searches.length > 0 ? $filtersObj.searches[0].search.join() : 'Search'}
            style="
            --text-color: {theme.colors.brand[200]};
            --search-bg-color: {theme.colors.neutral[800]}
            "
            value={$filtersObj.searches.length > 0 ? $filtersObj.searches[$filtersObj.searches.length - 1].search.join() : ''}
            on:keydown={submit}
        />
        <SearchIcon />
    </div>
    {#if loading}
        <Loader />
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



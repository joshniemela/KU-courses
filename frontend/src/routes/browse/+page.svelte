<script>
import SearchComponent from "../../components/SearchComponent/SearchComponent.svelte";
import Loader from "../../components/Loader/Loader.svelte";
import { apiUrl, queryStore } from '../../stores';
import { onMount } from 'svelte';
import OverviewCard from "../../components/OverviewCard/OverviewCard.svelte";

let loading = true;
let API_URL = apiUrl()
let courses = [];

/**
* Fetches the courses from the backend
* @function fetchCourses
*/
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
        <SearchComponent />
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
    margin-top: 3vh;
    margin-bottom: 1vh;
    height: 5rem;
    width: 30vw;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}

.card-container {
    height: 100%;
    width:96%;
    margin: 2%;
    display: grid;

    grid-template-columns: repeat(auto-fit, minmax(30em, 1fr));
    justify-content: center;
    align-items: center;
    gap: 2vh;
}
</style>



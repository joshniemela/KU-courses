<script lang="ts">
    import Loader from "../../components/Loader/Loader.svelte";
    import { apiUrl } from "../../stores";
    import { queryStore } from "../../newStore";
    import { onMount } from "svelte";
    import OverviewCard from "../../components/OverviewCard/OverviewCard.svelte";
    import type { Course } from "../../stores";

    let loading = true;
    let API_URL = apiUrl();
    let courses: Course[] = [];

    /**
     * Fetches the courses from the backend
     * @function fetchCourses
     */
    const fetchCourses = async () => {
        const filters = $queryStore;
        console.log(filters);
        const res = await fetch(`${API_URL}/find-courses-new`, {
            method: "POST",
            headers: {
                accept: "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify(filters),
        });

        const json = await res.json();
        console.log(json.courses[0]);
        loading = false;
        courses = json.courses;
    };

    onMount(async () => {
        await fetchCourses();
    });
</script>

<div class="browse-container">
    {#if loading}
        <Loader />
    {:else}
        <div class="card-container">
            {#each courses as card, i}
                <OverviewCard stagger={i} course={card} />
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

    .card-container {
        height: 100%;
        width: 96%;
        margin: 2%;
        display: grid;

        grid-template-columns: repeat(auto-fit, minmax(30em, 1fr));
        justify-content: center;
        align-items: center;
        gap: 2vh;
    }
</style>

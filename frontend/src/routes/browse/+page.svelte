<script lang="ts">
    import Loader from "../../components/Loader/Loader.svelte";
    import { apiUrl } from "../../stores";
    import { queryStore } from "../../stores";
    import { onMount } from "svelte";
    import OverviewCard from "../../components/OverviewCard/OverviewCard.svelte";
    import type { Overview } from "../../course";

    let loading = true;
    // grab time for testing performance
    let start = new Date().getTime();
    let API_URL = apiUrl();
    let courses: Overview[] = [];

    /**
     * Fetches the courses from the backend
     * @function fetchCourses
     */
    const fetchCourses = async () => {
        const filters = $queryStore;
        console.log(filters);
        const res = await fetch(`${API_URL}/find-course-overviews`, {
            method: "POST",
            headers: {
                accept: "application/json",
                "Content-Type": "application/json",
                // Add caching to save previous API calls
                "Cache-Control": "max-age=300",
            },
            body: JSON.stringify(filters),
        });

        const json = await res.json();
        console.log(json.courses[0]);
        loading = false;
        console.log(
            `Time taken to fetch courses: ${new Date().getTime() - start}ms`
        );
        courses = json.courses;
    };

    onMount(async () => {
        await fetchCourses();
    });
</script>

<div class="flex flex-col items-center h-screen">
    <!--Position button at the top centre of the page-->
    <button
        class="fixed top-0 mx-auto bg-kuRed hover:bg-blue-700 text-white font-bold py-2 px-6"
        on:click={() => {
            window.location.href = "/";
        }}
    >
        Back
    </button>
    {#if loading}
        <!--put the loader in the centre of the screen always----------------->
        <Loader />
    {:else}
        <div
            class="grid grid-flow-row grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 m-4 mt-12"
        >
            {#each courses as card, i}
                <OverviewCard stagger={i} course={card} />
            {/each}
        </div>

        {#if courses.length === 0}
            <h1 class="text-3xl text-center mt-10">
                No courses found, try broadening your search
            </h1>
        {/if}
    {/if}
</div>

<script lang="ts">
    import CheckboxMenu from "../components/CheckboxMenu.svelte";
    import BigCheckbox from "../components/BigCheckbox.svelte";
    import TextSearch from "../components/TextSearch.svelte";
    import Footer from "../components/Footer/Footer.svelte";
    import { queryStore, clearAll } from "../stores";

    // browse route content
    import Loader from "../components/Loader/Loader.svelte";
    import { apiUrl } from "../stores";
    import { onMount } from "svelte";
    import OverviewCard from "../components/OverviewCard/OverviewCard.svelte";
    import type { Overview } from "../course";
    import { browser } from "$app/environment";
    let loading = true;
    // grab time for testing performance
    let start = new Date().getTime();
    let API_URL = apiUrl();
    let courses: Overview[] = [];
    let collapsed: boolean = true;
    const fetchCourses = async () => {
        loading = true;
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
    const institutes: string[] = [
        "Department of Mathematics",
        "Department of Science Education",
        "Department of Pharmacy",
        "Department of Plant and Environmental Sciences",
        "The Natural History Museum",
        "Department of Sports Science and Clinical Biomechanics",
        "Department of Geoscience and Natural Resource Management",
        "Department of Food and Resource Economics",
        "Department of Neuroscience",
        "Department of Food Science",
        "The Niels Bohr Institute",
        "Department of Veterinary Disease Biology",
        "Department of Computer Science",
        "Department of Nutrition, Exercise and Sports",
        "Department of Biology",
        "Department of Drug Design and Pharmacology",
        "Department of Veterinary and Animal Sciences",
        "Department of Cellular and Molecular Medicine",
        "Department of Biomedical Sciences",
        "Department of Public Health",
        "Department of Media, Cognition and Communication",
        "Department of Chemistry",
    ];

    onMount(async () => {
        await fetchCourses();
    });

    // If the store changes, we should fetch new courses
    $: $queryStore, browser && fetchCourses();
    // Automatically open the collapsible menu if there are fewer than 100 courses
    $: if (courses.length != 0 && courses.length < 100) collapsed = false;

    // SEO
    const title = "DISKU - KU Courses 2.0";
    const description =
        "A more precise, user-friendly way to browse courses offered by University of Copenhagen which acutally gives you the information you were looking for";
    const url = "https://disku.jniemela.dk";
</script>

<svelte:head>
    <title>{title}</title>
    <meta name="description" content={description} />

    <!-- Facebook Meta Tags -->
    <meta property="og:url" content={url} />
    <meta property="og:type" content="website" />
    <meta property="og:title" content={title} />
    <meta property="og:description" content={description} />
    <meta property="og:image" content={`/assets/og-image.png`} />
    <meta property="og:image:alt" content="DISKU - KU Courses 2.0" />
    <meta property="og:image:width" content="1200" />
    <meta property="og:image:height" content="630" />

    <!-- Twitter Meta Tags -->
    <meta name="twitter:card" content="summary_large_image" />
    <meta property="twitter:domain" content="disku.jniemela.dk" />
    <meta property="twitter:url" content={url} />
    <meta name="twitter:title" content={title} />
    <meta name="twitter:description" content={description} />
    <meta name="twitter:image" content={`/assets/og-image.png`} />

    <link rel="canonical" href={url} />
</svelte:head>

<main class="flex flex-col items-center justify-center space-y-4 mt-10">
    <h1 class="text-brand-500 text-4xl font-bold">KU Courses 2.0</h1>

    <TextSearch bind:searches={$queryStore.searches} />
    <div>
        <div class="grid grid-cols-2 gap-4 md:grid-cols-4">
            <CheckboxMenu
                header_name="Block"
                options={["1", "2", "3", "4", "5"]}
                bind:selected={$queryStore.block}
            />

            <CheckboxMenu
                header_name="Study Level"
                options={["Bachelor", "Master"]}
                bind:selected={$queryStore.study_level}
            />

            <CheckboxMenu
                header_name="Schedule Group"
                options={["A", "B", "C", "D"]}
                bind:selected={$queryStore.schedule_group}
            />

            <CheckboxMenu
                header_name="Examination Type"
                options={[
                    "Written",
                    "Oral",
                    "Assignment",
                    "Continuous Assessment",
                ]}
                bind:selected={$queryStore.examination_type}
            />
        </div>

        <BigCheckbox
            header_name="Department"
            options={institutes}
            bind:selected={$queryStore.department}
        />
    </div>

    <button
        class="bg-brand-500 text-white px-4 py-0"
        on:click={() => {
            clearAll();
            collapsed = true;
        }}
    >
        Clear All
    </button>

    <!--make a collapsible menu that contains the text "foobar" which automatically opens if theres fewer than 100 courses-->
    <div class="flex flex-col w-full">
        <div class="flex flex-col w-full">
            <button
                class="bg-brand-500 text-white text-xl px-4 py-0"
                on:click={() => {
                    collapsed = !collapsed;
                }}
            >
                {collapsed ? "Show" : "Hide"} Courses
            </button>
            <div class="bg-kuGray text-center">
                <p class="text-white px-4 py-0">
                    {courses.length} courses found
                </p>
            </div>
        </div>
    </div>
    {#if !collapsed}
        <div class="flex flex-col items-center">
            {#if loading}
                <!--put the loader in the centre of the screen always----------------->
                <Loader />
            {:else}
                <div
                    class="grid grid-flow-row grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 m-4"
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
    {/if}
</main>

<Footer />

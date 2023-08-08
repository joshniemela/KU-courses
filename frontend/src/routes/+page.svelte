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
    let API_URL = apiUrl();
    let courses: Overview[] = [];
    let visibleCourses: Overview[] = [];
    let remainingCourses: Overview[] = [];
    const initialCourseNumber = 50;
    const batchLoadSize = 25;

    const loadMoreCourses = () => {
        if (remainingCourses.length > 0) {
            const nextBatch = remainingCourses.splice(0, batchLoadSize);
            visibleCourses = [...visibleCourses, ...nextBatch];
        }
    };
onMount(() => {
     window.addEventListener('scroll', handleScroll);
     return () => {
         window.removeEventListener('scroll', handleScroll);
     };
});


  const handleScroll = () => {
    const threshold = 800; // Adjust as needed
    const scrollPosition = window.scrollY || window.pageYOffset;
    const windowHeight = window.innerHeight;
    const contentHeight = document.body.offsetHeight;

    if (contentHeight - (scrollPosition + windowHeight) < threshold) {
      loadMoreCourses();
    }
  };
    const fetchCourses = async () => {
        loading = true;
        const filters = $queryStore;
        const res = await fetch(`${API_URL}/find-course-overviews`, {
            method: "POST",
            headers: {
                accept: "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify(filters),
        });

        const json = await res.json();
        loading = false;
        courses = json.courses;

  visibleCourses = courses.slice(0, initialCourseNumber); // Courses to show
  remainingCourses = courses.slice(initialCourseNumber); // Courses to load in batches
    };
    const institutes: string[] = [
        "Department of Geoscience and Natural Resource Management", // 190
        "Department of Mathematics", // 130
        "Department of Food and Resource Economics", // 95
        "Department of Biology", // 93
        "Department of Computer Science", // 92
        "The Niels Bohr Institute", // 90
        "Department of Plant and Environmental Sciences", // 75
        "Department of Chemistry", // 63
        "Department of Nutrition, Exercise and Sports", // 53
        "Department of Food Science", // 43
        "Department of Sports Science and Clinical Biomechanics", // 16
        "Department of Science Education", // 16
        "The Natural History Museum", // 14
        "Department of Veterinary and Animal Sciences", // 10
        //"Department of Drug Design and Pharmacology", // 4
        //"Department of Media, Cognition and Communication", // 3
        //"Department of Public Health", // 2
        //"Department of Pharmacy", // 2
        //"Department of Neuroscience", // 1
        //"Department of Veterinary Disease Biology", // 1
        //"Department of Cellular and Molecular Medicine", // 1
        //"Department of Biomedical Sciences", // 1
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


<div class="flex flex-col min-h-screen justify-between">
<main class="flex flex-col items-center space-y-4 mt-10">
    <h1 class="text-brand-500 text-4xl font-bold">KU Courses 2.0</h1>

    <TextSearch bind:searches={$queryStore.searches} />
    <div>
        <div class="grid grid-cols-2 gap-4 pb-2 md:grid-cols-4 md:pb-0">
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
        <div class="bg-kuGray text-center">
            <p class="text-white px-4 py-0">
                {courses.length} courses found
            </p>
        </div>
    </div>

    {#if loading}
        <!--put the loader in the centre of the screen always----------------->
        <Loader />
    {:else}


        <div class="flex flex-col items-center">
            <div
                class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 m-4"
            >

                {#each visibleCourses as card (card.course_id)}
                    <OverviewCard course={card} />
                {/each}
            </div>

            {#if courses.length === 0}
                <h1 class="text-3xl text-center mt-10">
                    No courses found, try broadening your search
                </h1>
            {/if}
        </div>
    {/if}
</main>

<Footer />
</div>

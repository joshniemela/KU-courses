<script lang="ts">
    import CheckboxMenu from "../components/CheckboxMenu.svelte";
    import BigCheckbox from "../components/BigCheckbox.svelte";
    import TextSearch from "../components/TextSearch.svelte";
    import Footer from "../components/Footer/Footer.svelte";
    import { queryStore, clearAll} from "../stores";

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
    const initialCourseNumber = 40;
    const batchLoadSize = 20;

    let expand_footer = false;

    const loadMoreCourses = () => {
        if (remainingCourses.length > 0) {
            const nextBatch = remainingCourses.splice(0, batchLoadSize);
            visibleCourses = [...visibleCourses, ...nextBatch];
        }
    };
    onMount(() => {
        window.addEventListener("scroll", handleScroll);
        return () => {
            window.removeEventListener("scroll", handleScroll);
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
        // prepreocess the filters so they are in the correct format
        // Convert blocks to longer format
        // 1, 2, 3, 4 => "One", "Two", "Three", "Four
        const blockMap = {
            "1": "One",
            "2": "Two",
            "3": "Three",
            "4": "Four",
            Summer: "Summer",
        };
        const departmentMap: { [key: string]: string } = {
            "Department of Geoscience and Natural Resource Management":
                "GeosciencesAndNaturalResourceManagement",
            "Department of Mathematics": "Mathematics",
            "Department of Food and Resource Economics":
                "FoodAndResourceEconomics",
            "Department of Biology": "Biology",
            "Department of Computer Science": "ComputerScience",
            "The Niels Bohr Institute": "NielsBohrInstitute",
            "Department of Chemistry": "Chemistry",
            "Department of Nutrition, Exercise and Sports":
                "NutritionExerciseAndSports",
            "Department of Food Science": "FoodScience",
            "Department of Science Education": "ScienceEducation",
            "The Natural History Museum": "NaturalHistoryMuseumOfDenmark",
        };

        let coerced_filters = {
            ...filters,
            blocks: filters.blocks.map((block) => blockMap[block]),
            departments: filters.departments.map(
                (department) => departmentMap[department]
            ),

            // Convert Continous Assessment to ContinuousAssessment
            exams: filters.exams.map((exam) =>
                exam === "Continuous Assessment" ? "ContinuousAssessment" : exam
            ),
        };

        const res = await fetch(`${API_URL}/find-course-overviews`, {
            method: "POST",
            headers: {
                accept: "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify(coerced_filters),
        });

        const json = await res.json();
        loading = false;
        courses = json.courses;
        // sort courses alphanumerically or by weights
        // if queryStore.search is empty, then sort otherwise do nothing
        if ($queryStore.search === "") {
            courses.sort((a, b) => {
                return a.title.localeCompare(b.title);
            });
        }

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
        "Department of Chemistry", // 63
        "Department of Nutrition, Exercise and Sports", // 53
        "Department of Food Science", // 43
        "Department of Science Education", // 16
        "The Natural History Museum", // 14
        //"Department of Drug Design and Pharmacology", // 4
        //"Department of Media, Cognition and Communication", // 3
        //"Department of Public Health", // 2
        //"Department of Pharmacy", // 2
        //"Department of Neuroscience", // 1
        //"Department of Veterinary Disease Biology", // 1
        //"Department of Cellular and Molecular Medicine", // 1
        //"Department of Biomedical Sciences", // 1
    ];

    // SEO
    const title = "DISKU - KU Courses 2.0";
    const description =
        "A more precise, user-friendly way to browse courses offered by University of Copenhagen which acutally gives you the information you were looking for";
    const url = "https://disku.jniemela.dk";

    console.log(visibleCourses);

    let search: string | null = queryStore.search;

    // If the store changes, we should fetch new courses
    $: debounceTimeout = ((() => {
        clearTimeout(debounceTimeout ?? undefined);
        return setTimeout(() => { $queryStore.search = search; }, 500);
    })()) || null;

    $: $queryStore, browser && fetchCourses()
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

<div class="flex flex-col min-h-screen justify-between relative">
    <main class="flex flex-col items-center space-y-4 mt-10">
        <h1 class="text-brand-500 text-4xl font-bold">KU Courses 2.0</h1>
        <div>
            <input type="text" placeholder="Search" bind:value={search} />
            <button
                class="bg-kuRed text-white p-2"
                on:click={() => (search = "")}
            >
                Clear search
            </button>
        </div>
        <div>
            <div class="grid grid-cols-2 gap-4 pb-2 md:grid-cols-4 md:pb-0">
                <CheckboxMenu
                    header_name="Block"
                    options={["1", "2", "3", "4", "Summer"]}
                    bind:selected={$queryStore.blocks}
                />

                <CheckboxMenu
                    header_name="Study Level"
                    options={["Bachelor", "Master"]}
                    bind:selected={$queryStore.degrees}
                />

                <CheckboxMenu
                    header_name="Schedule Group"
                    options={["A", "B", "C", "D"]}
                    bind:selected={$queryStore.schedules}
                />

                <CheckboxMenu
                    header_name="Examination Type"
                    options={[
                        "Written",
                        "Oral",
                        "Assignment",
                        "Continuous Assessment",
                        "Other",
                    ]}
                    bind:selected={$queryStore.exams}
                />
            </div>

            <BigCheckbox
                header_name="Department"
                options={institutes}
                bind:selected={$queryStore.departments}
            />
        </div>

        <button
            class="bg-brand-500 text-white px-4 py-0"
            on:click={() => {
                clearAll();
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
                    {#each visibleCourses as card (card.id)}
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

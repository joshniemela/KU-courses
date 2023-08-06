<script lang="ts">
    import { page } from "$app/stores";
    import { empty_course, total_hours } from "../../../course";
    import { onMount } from "svelte";
    import Loader from "../../../components/Loader/Loader.svelte";
    import SideCard from "../../../components/SideCard.svelte";
    import Footer from "../../../components/Footer/Footer.svelte";
    import { goto } from "$app/navigation";


    export let data;
    let { courseId, course, totalHours, loading} = data;

    /**
     * TODO: remove this since it is duplicated in the project multiple times
     */
    function convertExamToString(input: string) {
        return input.replace(/(\w)_(\w)/g, "$1 $2");
    }

    /**
     * This function takes an exam duration and changes the unit depending on the duration, e.g. 120 minutes -> 2 hours
     */
    function formatExamDuration(duration: number) {
        if (duration % 60 == 0) {
            if (duration % (60 * 24) == 0) {
                return `${duration / (60 * 24)}d`;
            } else {
                return `${duration / 60}h`;
            }
        } else {
            return `${duration}m`;
        }
    }

    function goBack() {
        goto("/browse");
    }

    // SEO
    const title = `${course.title} - DISKU`;

    const description =
        "A more precise, user-friendly way to browse courses offered by University of Copenhagen which acutally gives you the information you were looking for";
    const url = "https://disku.jniemela.dk/course/" + courseId;
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

{#if loading}
    <Loader />
{:else}
    <div class="mt-10 flex flex-col items-center">
        <button
            class="fixed top-0 mx-auto bg-kuRed text-white font-bold py-1 px-6"
            on:click={() => {
                window.history.back();
            }}
        >
            Back
        </button>
        <div class="items-left mb-5 px-4">
            <h1 class="text-2xl font-bold md:text-4xl">{course.title}</h1>
            <h2>{course.course_id} - SCIENCE</h2>
        </div>
        <div class="w-full flex flex-col md:flex-row justify-center">
            <div class="px-4">
                {#each course.description as de}
                    {#if de.type == "h1"}
                        <h1 class="text-xl font-bold">{de.string}</h1>
                    {:else if de.type == "li"}
                        <p>* {de.string}</p>
                    {:else}
                        <p>{de.string}</p>
                    {/if}
                {/each}
            </div>
            <div class="">
                <SideCard heading={"Coordinators"}>
                    {#each course.employees as emp}
                        <div class="">
                            <p class="">{emp.full_name}</p>
                        </div>
                        <p class="">{emp.email}</p>
                    {/each}
                </SideCard>

                <SideCard heading={"Exam"}>
                    {#each course.exams as exam}
                        <p class="">
                            {convertExamToString(exam.exam_type)}
                            {#if exam.minutes}
                                - ({formatExamDuration(exam.minutes)})
                            {/if}
                        </p>
                    {/each}
                </SideCard>

                <SideCard heading={"Course Info"}>
                    <p class="">{course.study_level} course</p>
                    <p class="">ECTS: {course.credits}</p>

                    <p class="">
                        Block: {course.start_block}
                        {#if Number(course.duration) > 1}
                            - {Number(course.start_block) +
                                Number(course.duration) -
                                1}
                        {/if}
                    </p>
                    <p class="">
                        Schedule group(s): {#each course.schedules as sch}
                            {#if sch != course.schedules[course.schedules.length - 1]}
                                {sch.schedule_type}, &nbsp
                            {:else}
                                {sch.schedule_type}
                            {/if}
                        {/each}
                    </p>

                    <p class="flex flex-col" />

                    <a
                        href={`https://kurser.ku.dk/course/${course.course_id}`}
                        class="text-kuRed font-bold"
                    >
                        Go to official page
                    </a>
                </SideCard>
                <SideCard heading={"Department(s)"}>
                    {#each course.departments as dep}
                        <p class="mb-2">
                            {dep.department_type}
                        </p>
                    {/each}
                </SideCard>

                <SideCard heading={"Workload"}>
                    <!--arrange in a table------------------------------------>
                    <table>
                        {#each course.workloads as wl}
                            <tr class="border-b-4 border-kuGray">
                                <td class=""> {wl.workload_type}</td>
                                <td class="pl-2">{wl.hours}h</td>
                            </tr>
                        {/each}
                    </table>
                    <p class="font-bold">Total: {totalHours}h</p>
                </SideCard>
            </div>
        </div>
    </div>

    <Footer />
{/if}

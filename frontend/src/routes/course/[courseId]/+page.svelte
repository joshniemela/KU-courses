<script lang="ts">
    import { page } from "$app/stores";
    import { empty_course, total_hours } from "../../../course";
    import { onMount } from "svelte";
    import theme from "../../../theme";
    import Loader from "../../../components/Loader/Loader.svelte";
    import { apiUrl } from "../../../stores";
    import { goto } from "$app/navigation";

    const courseId = $page.params.courseId;
    let API_URL = apiUrl();
    let loading = true;

    let totalHours = 0;
    let course = empty_course;

    const fetchCourse = async (courseId: string) => {
        const res = await fetch(`${API_URL}/get-course?id=${courseId}`, {
            method: "GET",
            headers: {
                accept: "application/json",
                "Content-Type": "application/json",
            },
        });
        const json = await res.json();
        console.log(json);
        console.log(json.employees[0].email);
        loading = false;
        return json;
    };

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

    onMount(async () => {
        const res = await fetchCourse(courseId);
        console.log(res.employees);
        course = res;
        totalHours = total_hours(course);
    });

    const buttonTextColor = theme.colors.brand[200];
    const buttonBgColor = theme.colors.brand[800];
    const buttonTextColorHover = theme.colors.brand[900];
    const buttonBgColorHover = theme.colors.brand[500];
</script>

{#if loading}
    <Loader />
{:else}
    <div class="main-container mt-10">
        <button
            class="fixed top-0 mx-auto bg-kuRed hover:bg-blue-700 text-white font-bold py-2 px-6"
            on:click={() => {
                window.history.back();
            }}
        >
            Back
        </button>
        <div class="content-container">
            <div class="content-container-left">
                <div class="header-container">
                    <div>
                        <h1 class="text-4xl font-bold">{course.title}</h1>
                        <h2>{course.course_id} - SCIENCE</h2>
                    </div>
                </div>
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
            <div class="content-container-right">
                <div
                    class="side-card"
                    style="
                        --bg-color: {theme.colors.neutral[800]};
                        --text-color: {theme.colors.neutral[200]};
                        --sub-title-color: {theme.colors.neutral[600]};
                        --brand-color: {theme.colors.brand[500]};
                        "
                >
                    <h3 class="side-card-heading">Coordinators</h3>
                    {#each course.employees as emp}
                        <div class="side-card-name-title">
                            <p class="side-card-name">{emp.full_name}</p>
                        </div>
                        <p class="side-card-clickable">{emp.email}</p>
                    {/each}
                </div>
                <div
                    class="side-card"
                    style="
                        --bg-color: {theme.colors.neutral[800]};
                        --text-color: {theme.colors.neutral[200]};
                        --sub-title-color: {theme.colors.neutral[600]};
                        --brand-color: {theme.colors.brand[500]};
                        "
                >
                    <h3 class="side-card-heading">Info</h3>
                    <p class="side-card-name">{course.study_level} course</p>
                    <p class="side-card-name">ECTS: {course.credits}</p>
                    <a href={`https://kurser.ku.dk/course/${course.course_id}`}>
                        <p class="side-card-clickable">
                            https://kurser.ku.dk/course/{course.course_id}
                        </p>
                    </a>
                </div>
                <div
                    class="side-card"
                    style="
                        --bg-color: {theme.colors.neutral[800]};
                        --text-color: {theme.colors.neutral[200]};
                        --sub-title-color: {theme.colors.neutral[600]};
                        --brand-color: {theme.colors.brand[500]};
                        "
                >
                    <h3 class="side-card-heading">Schedule</h3>
                    <p class="side-card-name">
                        Block: {course.start_block}
                        {#if Number(course.duration) > 1}
                            - {Number(course.start_block) +
                                Number(course.duration) -
                                1}
                        {/if}
                    </p>
                    <p class="side-card-name">
                        Schedule group(s): {#each course.schedules as sch}
                            {#if sch != course.schedules[course.schedules.length - 1]}
                                {sch.schedule_type}, &nbsp
                            {:else}
                                {sch.schedule_type}
                            {/if}
                        {/each}
                    </p>
                </div>
                <div
                    class="side-card"
                    style="
                        --bg-color: {theme.colors.neutral[800]};
                        --text-color: {theme.colors.neutral[200]};
                        --sub-title-color: {theme.colors.neutral[600]};
                        --brand-color: {theme.colors.brand[500]};
                        "
                >
                    <h3 class="side-card-heading">Workload</h3>
                    {#each course.workloads as wl}
                        <p class="side-card-name">
                            {convertExamToString(wl.workload_type)}: {wl.hours}h
                        </p>
                    {/each}
                    <p class="side-card-clickable">Total: {totalHours}h</p>
                </div>
                <div
                    class="side-card"
                    style="
                        --bg-color: {theme.colors.neutral[800]};
                        --text-color: {theme.colors.neutral[200]};
                        --sub-title-color: {theme.colors.neutral[600]};
                        --brand-color: {theme.colors.brand[500]};
                        "
                >
                    <h3 class="side-card-heading">Exam</h3>
                    {#each course.exams as exam}
                        <p class="side-card-name">
                            {convertExamToString(exam.exam_type)}
                            {#if exam.minutes}
                                - ({formatExamDuration(exam.minutes)})
                            {/if}
                        </p>
                    {/each}
                </div>
            </div>
        </div>
    </div>
{/if}

<style scoped>
    .main-container {
        display: flex;
        width: 100%;
        height: 100vh;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    .header-container {
        display: flex;
        width: 100%;
        flex-direction: row;
        justify-content: start;
        align-items: center;
    }

    .content-container {
        height: 100%;
        width: 100%;
        display: grid;
        grid-template: 1fr / 4fr 1fr;
    }

    .content-container-right {
        height: 92%;
        width: 92%;
        padding: 4%;
        display: flex;
        flex-direction: column;
        justify-content: start;
        align-items: center;
    }

    .content-container-left {
        height: 92%;
        width: 92%;
        padding: 4%;
    }

    .side-card {
        width: 90%;
        margin-bottom: 2vh;
        background-color: var(--bg-color);
        color: var(--text-color);
        padding: 2%;
    }

    .side-card-heading {
        font-size: 1.5rem;
        color: var(--text-color);
    }
    .side-card-name-title {
        display: flex;
        flex-direction: row;
        justify-content: start;
    }
    .side-card-name {
        font-size: 1rem;
        color: var(--text-color);
    }

    .side-card-clickable {
        font-size: 1rem;
        color: var(--brand-color);
        margin-bottom: 1vh;
    }
</style>

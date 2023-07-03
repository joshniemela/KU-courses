<script lang="ts">
    import { goto } from "$app/navigation";
    import Dk from "../../assets/Dk.svelte";
    import Gb from "../../assets/Gb.svelte";
    import theme from "../../theme.js";
    import { empty_course } from "../../stores";
    import type { Course } from "../../stores";
    export let stagger = 0;
    export let course: Course = structuredClone(empty_course);

    /**
     * Function to extract the first <charLimit> letters from the paragraphs to use for the summary.
     * @function extractSummary
     */
    function extractSummary(charLimit: number): string {
        return (
            course.description
                .map((x) => x.string)
                .join()
                .slice(0, charLimit) + "..."
        );
    }
    let summary = extractSummary(390);

    /**
     * Function to navigate to the course corresponding with the course_id
     * @function navigateToCourse
     */
    function navigateToCourse() {
        goto(`/course/${course.course_id}`);
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

    function convertExamToString(inputString: string) {
        return inputString.replace(/(\w)_(\w)/g, "$1 $2");
    }
</script>

<!--why not link !TODO!-->
<div
    class="card-container"
    on:click={navigateToCourse}
    on:keydown={navigateToCourse}
>
    <div
        class="card"
        style="
            --bg-color: {theme.colors.neutral[800]};
            --stagger: {stagger};
            "
    >
        <div class="card-header-container">
            <div class="card-title-container">
                <div class="title-container">
                    <a href={`/course/${course.course_id}`}>
                        <h1
                            class="card-title"
                            style="
                                --text-color: {theme.colors.neutral[200]};
                                --text-size: 24px
                                "
                        >
                            {course.title}
                        </h1>
                    </a>
                </div>
                <h2
                    class="card-subtitle"
                    style="--text-color: {theme.colors.neutral[600]}"
                >
                    {course.course_id} - SCIENCE
                </h2>
            </div>
            <table class="card-info-table">
                <tr>
                    <td class="card-td-left-top">{course.study_level}</td>
                    <td class="card-td-right-top">{course.credits} ECTS</td>
                </tr>
                <tr>
                    <td class="card-td-left-bot"
                        >Block {Number(course.start_block)}
                        {#if Number(course.duration) > 1}
                            - {Number(course.start_block) +
                                Number(course.duration) -
                                1}
                        {/if}
                    </td>
                    <td>
                        Group: {#each course.schedules as sch}
                            {#if sch != course.schedules[course.schedules.length - 1]}
                                {sch.schedule_type}, &nbsp
                            {:else}
                                {sch.schedule_type}
                            {/if}
                        {/each}
                    </td>
                </tr>
            </table>
        </div>
        <div class="card-description-container">
            <p class="card-description">{summary}</p>
        </div>
        <div
            class="card-exam-text-container"
            style="--bg-color: {theme.colors.neutral[300]}"
        >
            {#each course.exams as exam}
                <p
                    class="card-exam-text"
                    style="--text-color: {theme.colors.neutral[900]}"
                >
                    {convertExamToString(exam.exam_type)}
                    {#if exam.minutes}
                        ({formatExamDuration(exam.minutes)})
                    {/if}
                    {#if course.exams.length > 1 && exam != course.exams[course.exams.length - 1]}
                        &nbsp - &nbsp
                    {/if}
                </p>
            {/each}
            {#if course.course_language == "da"}
                <Dk />
            {:else}
                <Gb />
            {/if}
        </div>
    </div>
</div>

<style scoped>
    .card {
        display: flex;
        position: relative;
        flex-direction: column;
        justify-content: start;
        align-items: center;
        width: 100%;
        height: 33vh;
        background-color: var(--bg-color);
        border-radius: 10px;
        box-shadow: 0 2.8px 2.2px rgba(0, 0, 0, 0.15);
        transition: ease-in-out 0.2s;
        opacity: 0%;
        animation: fadeIn 0.5s calc(var(--stagger) * 0.05s);
        animation-fill-mode: forwards;
        overflow: auto;
    }

    .card:hover {
        scale: 1.02;
        box-shadow: 0 2.8px 2.2px rgba(0, 0, 0, 0.25);
        transition: ease-in-out 0.2s;
    }

    .card-header-container {
        height: 12vh;
        max-height: 12vh;
        overflow: hidden;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
        width: 96%;
    }

    .card-title-container {
        max-width: 70%;
        height: 100%;
        display: flex;
        flex-direction: column;
        justify-content: start;
        align-items: start;
    }

    .card-description-container {
        width: 96%;
        padding-left: 2%;
        padding-right: 2%;
        height: 80%;
        overflow: auto;
    }
    .card-exam-text-container {
        position: relative;
        background-color: var(--bg-color);
        bottom: 0;
        width: 100%;
        height: 6vh;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .card-exam-text {
        font-size: 1rem;
        color: var(--text-color);
    }

    .card-title {
        font-size: var(--text-size);
        color: var(--text-color);
    }

    .card-subtitle {
        font-size: 1rem;
        color: var(--text-color);
    }

    .card-description {
        font-size: 1rem;
        color: var(--text-color);
    }

    .card-info-table {
        width: 30%;
        text-align: center;
        border-spacing: 0;
    }

    .card-td-left-top {
        border-bottom: 1px solid;
        border-right: 1px solid;
    }

    .card-td-right-top {
        border-bottom: 1px solid;
    }

    .card-td-left-bot {
        border-right: 1px solid;
    }

    .title-container {
        height: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
        word-wrap: break-word;
    }

    @keyframes fadeIn {
        0% {
            opacity: 0;
        }
        100% {
            opacity: 1;
        }
    }
</style>

<script lang="ts">
    import { goto } from "$app/navigation";
    import Dk from "../../assets/Dk.svelte";
    import Gb from "../../assets/Gb.svelte";
    import { empty_overview } from "../../course";
    import type { Overview } from "../../course";
    export let stagger = 0;
    export let course: Overview = structuredClone(empty_overview);

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

<button
    class="w-full border-2 border-black animate-fadeIn flex flex-col justify-between hover:bg-gray-100"
    on:click={navigateToCourse}
>
    <div class="p-2">
        <div class="flex flex-row justify-between">
            <div class="flex flex-col w-full justify-start">
                <h1 class="text-l font-bold text-center">
                    {course.title}
                </h1>
                <h2>
                    {course.course_id} - SCIENCE
                </h2>
            </div>
            <table class="text-sm">
                <tr>
                    <td class="border-e border-b border-black px-1">
                        {course.study_level}</td
                    >
                    <td class="border-b border-black px-1">
                        {course.credits} ECTS</td
                    >
                </tr>
                <tr>
                    <td class="border-e border-black px-1">
                        Block {Number(course.start_block)}
                        {#if Number(course.duration) > 1}
                            - {Number(course.start_block) +
                                Number(course.duration) -
                                1}
                        {/if}
                    </td>
                    <td class="px-1">
                        Group{#if course.schedules.length > 1}s{/if}:
                        {#each course.schedules as sch}
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
        <div class="">
            <p class="">{course.summary}</p>
        </div>
    </div>
    <div class="bg-kuGray text-white">
        {#each course.exams as exam}
            <p class="">
                {convertExamToString(exam.exam_type)}
                {#if exam.minutes}
                    ({formatExamDuration(exam.minutes)})
                {/if}
            </p>
        {/each}
        {#if false}
            {#if course.course_language == "da"}
                <Dk />
            {:else}
                <Gb />
            {/if}
        {/if}
    </div>
</button>

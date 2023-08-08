<script lang="ts">
    import { goto } from "$app/navigation";
    import Dk from "../../assets/Dk.svelte";
    import Gb from "../../assets/Gb.svelte";
    import { empty_overview } from "../../course";
    import type { Overview } from "../../course";
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
    class="w-full border-2 border-black animate-fadeIn flex flex-col justify-betweenhover:bg-gray-100 relative"
    on:click={navigateToCourse}
>
    <div class="p-2">
        <div class="flex flex-row justify-between">
            <div class="w-full">
                <h1 class="text-l font-bold text-center z-10">
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
    <div class="bg-kuGray text-white flex flex-row">
        <div class="w-full items-center justify-center flex flex-col">
            {#each course.exams as exam}
                <p class="">
                    {convertExamToString(exam.exam_type)}
                    {#if exam.minutes}
                        ({formatExamDuration(exam.minutes)})
                    {/if}
                </p>
            {/each}
        </div>
        <!--stats table, contains pass_rate, median_grade, and avg_grade-->
        <table class="text-xs">
            <tr>
                <td class="border-e border-b border-white px-1"> Pass</td>
                <td class="border-b border-white px-1">
                    {course.pass_rate == null
                        ? "N/A"
                        : `${Math.round(course.pass_rate * 10000) / 100}%`}
                </td></tr
            >
            <tr>
                <td class="border-e border-white px-1"> Median</td>
                <td class="border-white px-1">
                    {course.median_grade == null ? "N/A" : course.median_grade}
                </td>
            </tr>
            <tr>
                <td class="border-e border-t border-white px-1"> Average </td>
                <td class="border-t border-white px-1">
                    {course.avg_grade == null ? "N/A" : course.avg_grade}
                </td>
            </tr>
        </table>
    </div>
    <!--put this relatively in the bottom right corner of the card-->
    <div class="w-8 h-8 absolute top-0 -top-px opacity-50">
        {#if course.course_language == "da"}
            <Dk />
        {:else}
            <Gb />
        {/if}
    </div>
</button>

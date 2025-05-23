<script lang="ts">
    import { goto } from "$app/navigation";
    import Dk from "../../assets/Dk.svelte";
    import Gb from "../../assets/Gb.svelte";
    import { empty_overview } from "../../course";
    import type { Overview } from "../../course";
    export let course: Overview = structuredClone(empty_overview);

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

    // this takes a vector of maps ex: [{:type "A"}, {:type "B"}] and returns a vector of strings ex: ["A", "B"]
    function denest_type_maps(map_vector: { type: string }[]): string[] {
        let type_vector: string[] = [];
        for (let i = 0; i < map_vector.length; i++) {
            type_vector.push(map_vector[i].type);
        }
        return type_vector;
    }

    function coerce_blocks_to_int(blocks: string[]): number[] {
        // blocks are written in One Two Three Four
        // this function converts them to "1" "2" "3" "4"
        let block_vector: number[] = [];
        for (let i = 0; i < blocks.length; i++) {
            switch (blocks[i]) {
                case "One":
                    block_vector.push(1);
                    break;
                case "Two":
                    block_vector.push(2);
                    break;
                case "Three":
                    block_vector.push(3);
                    break;
                case "Four":
                    block_vector.push(4);
                    break;
                case "Summer":
                    block_vector.push(5);
                    break;
            }
        }
        return block_vector;
    }

    const isCancelled =
        course.title.toLowerCase().includes("aflyst") ||
        course.title.toLowerCase().includes("cancelled");
</script>

<a
    class="w-full border-2 border-black animate-fadeIn flex flex-col justify-between hover:bg-gray-100 relative"
    href="course/{course.id}"
>
    <div class="p-2">
        <div class="flex flex-row justify-between overflow-x-auto">
            <div class="w-full">
                <h1
                    class="text-l font-bold text-center z-10 {isCancelled
                        ? 'text-red-500'
                        : ''}"
                >
                    {course.title}
                </h1>
                <h2>
                    {course.id} - SCIENCE
                </h2>
            </div>
            <table class="text-sm h-8 whitespace-nowrap">
                <tbody>
                    <tr>
                        <td class="border-e border-b border-black px-1">
                            {denest_type_maps(course.degree).join(", ")}
                        </td>
                        <td class="border-b border-black px-1">
                            ECTS: {course.ects}
                        </td>
                    </tr>
                    <tr>
                        <td class="border-e border-black p-1">
                            Block(s): {coerce_blocks_to_int(
                                denest_type_maps(course.block)
                            )
                                .sort((a, b) => a - b) // Ensure numeric sorting
                                .reduce((acc: number[][], curr, index, arr) => {
                                    // Convert consecutive numbers to ranges
                                    if (
                                        index === 0 ||
                                        curr - arr[index - 1] !== 1
                                    ) {
                                        acc.push([curr]); // Start a new range
                                    } else {
                                        acc[acc.length - 1][1] = curr; // Extend the current range
                                    }
                                    return acc;
                                }, [])
                                .map((range) => {
                                    if (range.length === 2) {
                                        return `${range[0]}-${range[1]}`;
                                    } else if (range[0] === 5) {
                                        // Map block 5 to "Summer"
                                        return "Summer";
                                    } else {
                                        return `${range[0]}`;
                                    }
                                })
                                // Format ranges or single values
                                .join(", ")}
                        </td>
                        <!--TODO: If this is an "other", this breaks and just shows object object-->
                        <td class="px-1">
                            Group(s): {denest_type_maps(course.schedule)
                                // TODO: acutally process the string schedules instead of calling them other
                                .map((x) =>
                                    typeof x === "object" ? "Other" : x
                                )
                                .map((x) =>
                                    x == "OutsideOfSchedule" ? "Other" : x
                                )
                                .sort()
                                .join(", ")}
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>

        <p class="break-all">
            {@html course.summary.length > 200
                ? course.summary.substring(0, 200) + "..."
                : course.summary}
        </p>
    </div>
    <div class="w-full bg-kuGray text-white flex flex-row">
        <div class="w-full items-center justify-center flex flex-col">
            {#each course.exam as exam}
                <p class="">
                    {exam.type == "ContinuousAssessment"
                        ? "Continuous Assesment"
                        : exam.type}
                    {#if exam.duration}
                        ({formatExamDuration(exam.duration)})
                    {/if}
                </p>
            {/each}
        </div>
        <!--stats table, contains pass_rate, median_grade, and avg_grade-->
        {#if course.statistics}
            <table class="text-xs whitespace-nowrap">
                <tbody>
                    <tr>
                        <td class="border-e border-b border-white px-1">
                            Pass</td
                        >

                        <td class="border-b border-white px-1">
                            {course.statistics["pass-rate"] == null
                                ? "N/A"
                                : `${
                                      Math.round(
                                          course.statistics["pass-rate"] * 10000
                                      ) / 100
                                  }%`}
                        </td>
                    </tr>
                    <tr>
                        <td class="border-e border-white px-1"> Median</td>
                        <td class="border-white px-1">
                            {course.statistics.median == null
                                ? "N/A"
                                : course.statistics.median}
                        </td>
                    </tr>
                    <tr>
                        <td class="border-e border-t border-white px-1">
                            Average
                        </td>
                        <td class="border-t border-white px-1">
                            {course.statistics.mean == null
                                ? "N/A"
                                : Math.round(course.statistics.mean * 100) /
                                  100}
                        </td>
                    </tr>
                </tbody>
            </table>
        {:else}
            <table class="text-xs whitespace-nowrap">
                <tbody>
                    <tr>
                        <td class="border-e border-b border-white px-1">
                            Pass</td
                        >

                        <td class="border-b border-white px-1">N/A</td>
                    </tr>
                    <tr>
                        <td class="border-e border-white px-1"> Median</td>
                        <td class="border-white px-1">N/A</td>
                    </tr>
                    <tr>
                        <td class="border-e border-t border-white px-1">
                            Average
                        </td>
                        <td class="border-t border-white px-1">N/A</td>
                    </tr>
                </tbody>
            </table>
        {/if}
    </div>
    <!--put this relatively in the bottom right corner of the card-->
    <div class="w-8 h-8 absolute top-0 -top-px opacity-50">
        {#if course.language.filter((lang) => lang.name == "Danish").length > 0}
            <Dk />
        {:else}
            <Gb />
        {/if}
    </div>
</a>

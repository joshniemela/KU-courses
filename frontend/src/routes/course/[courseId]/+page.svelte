<script lang="ts">
    import { page } from "$app/stores";
    import { empty_course, total_hours } from "../../../course";
    import { onMount } from "svelte";
    import Loader from "../../../components/Loader/Loader.svelte";
    import SideCard from "../../../components/SideCard.svelte";
    import Footer from "../../../components/Footer/Footer.svelte";

    import GradeGraph from "../../../components/GradeGraph/GradeGraph.svelte";

    let { data } = $props();
    let { courseId, course, totalHours, statistics, loading } = data;
    const isCancelled =
        course.title.toLowerCase().includes("aflyst") ||
        course.title.toLowerCase().includes("cancelled");

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

    // SEO
    const title = `${course.title} - KU Courses`;

    const description =
        "A more precise, user-friendly way to browse courses offered by University of Copenhagen which acutally gives you the information you were looking for";
    const url = "https://kucourses.dk/course/" + courseId;

    // To every li tag, add class="list-square list-inside" in content["learning-outcome"]

    let content = course.content
        ? course.content.replaceAll(
              "<li>",
              '<li class="list-square list-inside ml-4">'
          )
        : null;

    let learning_outcome = course["learning-outcome"]
        ? course["learning-outcome"].replaceAll(
              "<li>",
              '<li class="list-square list-inside ml-4">'
          )
        : null;

    // this takes a vector of maps ex: [{:type "A"}, {:type "B"}] and returns a vector of strings ex: ["A", "B"]
    function denest_type_maps(map_vector: any) {
        let type_vector: string[] = [];
        for (let i = 0; i < map_vector.length; i++) {
            type_vector.push(map_vector[i].type);
        }
        return type_vector;
    }

    function coerce_blocks_to_int(blocks: any) {
        // blocks are written in One Two Three Four
        // this function converts them to "1" "2" "3" "4"
        let block_vector: string[] = [];
        for (let i = 0; i < blocks.length; i++) {
            switch (blocks[i]) {
                case "One":
                    block_vector.push("1");
                    break;
                case "Two":
                    block_vector.push("2");
                    break;
                case "Three":
                    block_vector.push("3");
                    break;
                case "Four":
                    block_vector.push("4");
                    break;
                default:
                    block_vector.push(blocks[i]);
                    break;
            }
        }
        return block_vector;
    }
    function separate_capitals_letters(sentence: string) {
        return sentence.replace(/([A-Z])/g, " $1").trim();
    }

    function remove_repeated_br_tags(dom: Document) {
        let brs = dom.getElementsByTagName("br");
        for (let i = 0; i < brs.length; i++) {
            if (brs[i].nextSibling != null) {
                brs[i].nextSibling!.remove();
            }
        }
    }

    // If a p tag is immediately descendent of an li tag, we pop the text out of the p tag
    function pop_p_tags(dom: Document) {
        let lis = dom.getElementsByTagName("li");
        for (let i = 0; i < lis.length; i++) {
            let p = lis[i].getElementsByTagName("p");
            if (p.length > 0) {
                lis[i].innerHTML = p[0].innerHTML;
            }
        }
    }

    onMount(() => {
        remove_repeated_br_tags(document);
        pop_p_tags(document);
    });
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
    <meta property="og:image:alt" content="KU Courses" />
    <meta property="og:image:width" content="1200" />
    <meta property="og:image:height" content="630" />

    <!-- Twitter Meta Tags -->
    <meta name="twitter:card" content="summary_large_image" />
    <meta property="twitter:domain" content="kucourses.dk" />
    <meta property="twitter:url" content={url} />
    <meta name="twitter:title" content={title} />
    <meta name="twitter:description" content={description} />
    <meta name="twitter:image" content={`/assets/og-image.png`} />

    <link rel="canonical" href={url} />
</svelte:head>

{#if loading}
    <Loader />
{:else}
    <a
        class="block sticky top-0 left-1/2 -translate-x-1/2 mx-auto w-fit bg-kuRed text-white font-bold py-1 px-6"
        href="/"
    >
        Back
    </a>
    <div class="min-h-screen mx-auto px-10 lg:px-0 lg:w-[900px] mt-6">
        <div class="items-left mb-5 px-4 text-center">
            <h1
                class="text-2xl font-bold md:text-4xl {isCancelled
                    ? 'text-red-500'
                    : ''}
        "
            >
                {course.title}
            </h1>
            <h2>{course.id} - SCIENCE</h2>
        </div>

        <div class="block md:grid md:grid-cols-[auto_1fr] md:gap-x-10">
            <div class="md:col-span-2">
                {#if course["statistics"] != null && statistics != null}
                    <p>
                        Passed: {Math.round(statistics["pass-rate"] * 100)}%,
                        Average grade: {Math.round(statistics["mean"] * 100) /
                            100}, Median grade: {statistics["median"]}
                    </p>
                    <GradeGraph data={statistics.grades} />
                {/if}
            </div>
            <div>
                <h1 class="text-xl font-bold">Description</h1>

                {@html content}
                {@html learning_outcome}
                {#if course["recommended-qualifications"] != null}
                    <h2 class="text-l font-bold">Recommended qualifications</h2>
                    {@html course["recommended-qualifications"]}
                {/if}
            </div>
            <div>
                <SideCard heading={"Coordinators"}>
                    {#each course.coordinator as emp}
                        <div class="">
                            <p class="">{emp.name}</p>
                        </div>
                        <p class="">{emp.email}</p>
                    {/each}
                </SideCard>
                <SideCard heading={"Exam"}>
                    {#each course.exam as exam}
                        <p class="">
                            {separate_capitals_letters(exam.type)}
                            {#if exam.duration}
                                - ({formatExamDuration(exam.duration)})
                            {/if}
                        </p>
                    {/each}
                </SideCard>
                <SideCard heading={"Course Info"}>
                    <p class="">
                        Level: {denest_type_maps(course.degree).join("\n")}
                    </p>
                    <p class="">ECTS: {course.ects}</p>

                    <p class="">
                        Block(s): {coerce_blocks_to_int(
                            denest_type_maps(course.block)
                        )
                            .sort()
                            .join(", ")}
                    </p>
                    <p class="">
                        Group(s): {denest_type_maps(course.schedule)
                            .sort()
                            .join(", ")}
                    </p>

                    <p class="flex flex-col"></p>

                    <a
                        href={`https://kurser.ku.dk/course/${course.id}`}
                        class="text-kuRed font-bold"
                    >
                        Go to official page
                    </a>
                </SideCard>
                <SideCard heading={"Department(s)"}>
                    <ul class="list-square">
                        {#each course.department as dep}
                            <li class="">
                                {separate_capitals_letters(dep.name)}
                            </li>
                        {/each}
                    </ul>
                </SideCard>
                <SideCard heading={"Workload"}>
                    <table>
                        <tbody>
                            {#each course.workload as wl}
                                <tr class="border-b-4 border-kuGray">
                                    <td class="">
                                        {separate_capitals_letters(wl.type)}</td
                                    >
                                    <td class="pl-2">{wl.hours}h</td>
                                </tr>
                            {/each}
                        </tbody>
                    </table>
                    <p class="font-bold">Total: {totalHours}h</p>
                </SideCard>
            </div>
        </div>

        <Footer />
    </div>
{/if}

<script>
    import { onMount } from "svelte";
    import {
        filters,
        filtersObj,
        jsonToString,
        StudyLevelTypes,
        ScheduleGroupTypes,
        BlockTypes,
        ExamTypes,
    } from "../../stores.js";

    import theme from "../../theme.js";

    export let dialog;

    /**
     * Helper function to generate option objects based on the different filter types
     */
    function generateOptionsObject(TypeObject, field) {
        let obj = {};
        for (const [key, val] of Object.entries(TypeObject)) {
            obj[val] = $filtersObj[field].includes(val.toString());
        }
        return obj;
    }

    /**
     * Helper function to aggregate chosen options into array
     */
    function aggregateOptions(optionsObject) {
        let li = [];
        for (const [key, val] of Object.entries(optionsObject)) {
            if (val === true) {
                li.push(key);
            }
        }
        return li;
    }

    /**
     * Applies the new filters
     */
    function applyOptions() {
        $filters = jsonToString({
            ...$filtersObj,
            study_level: aggregateOptions(studyLevelOptions),
            schedule_group: aggregateOptions(scheduleGroupOptions),
            block: aggregateOptions(blockOptions),
            exam_type: aggregateOptions(examTypeOptions),
        });
        dialog.close();
    }

    function convertExamToString(inputString) {
        return inputString.replace(/(\w)_(\w)/g, "$1 $2");
    }

    let studyLevelOptions = generateOptionsObject(
        StudyLevelTypes,
        "study_level"
    );
    let scheduleGroupOptions = generateOptionsObject(
        ScheduleGroupTypes,
        "schedule_group"
    );
    let blockOptions = generateOptionsObject(BlockTypes, "block");
    let examTypeOptions = generateOptionsObject(ExamTypes, "exam_type");

    /**
     * Checks wether we are clicking on the background, and if so closes the dialog
     * @function closeOnClickOutside
     */
    function closeOnClickOutside(event) {
        if (event.target.id == "dialog") {
            dialog.close();
        }
    }

    // Styling handling for the apply button
    let buttonTextColor = theme.colors.brand[200];
    let buttonBgColor = theme.colors.brand[800];
    function handleHover(e) {
        buttonTextColor = theme.colors.brand[900];
        buttonBgColor = theme.colors.brand[500];
    }
    function handleHoverOut(e) {
        buttonTextColor = theme.colors.brand[200];
        buttonBgColor = theme.colors.brand[800];
    }

    onMount(() => {});
</script>

<dialog
    class="dialog"
    bind:this={dialog}
    id="dialog"
    on:click={closeOnClickOutside}
    on:close={() => {
        studyLevelOptions = generateOptionsObject(
            StudyLevelTypes,
            "study_level"
        );
        scheduleGroupOptions = generateOptionsObject(
            ScheduleGroupTypes,
            "schedule_group"
        );
        blockOptions = generateOptionsObject(BlockTypes, "block");
        examTypeOptions = generateOptionsObject(ExamTypes, "exam_type");
    }}
>
    <div class="filter-container">
        <h1 class="title" style="--color: {theme.colors.brand[500]}">
            Filters
        </h1>
        <div class="filter-content-container">
            <div class="filter-option-container">
                <div
                    class="filter-title-container"
                    style="--bg-color: {theme.colors.neutral[200]}"
                >
                    <h2
                        class="filter-option-title"
                        style="--text-color: {theme.colors.neutral[800]}"
                    >
                        Course level
                    </h2>
                </div>
                <div class="filter-choice-container">
                    {#each Object.entries(StudyLevelTypes) as [key, value]}
                        <div class="option-container">
                            <label class="option-label" for={key}>
                                <input
                                    type="checkbox"
                                    style="--color: {theme.colors.brand[500]}"
                                    bind:checked={studyLevelOptions[value]}
                                    id={key}
                                />
                                {value}
                            </label>
                        </div>
                    {/each}
                </div>
            </div>
            <div>
                <div
                    class="filter-title-container"
                    style="--bg-color: {theme.colors.neutral[200]}"
                >
                    <h2
                        class="filter-option-title"
                        style="--text-color: {theme.colors.neutral[800]}"
                    >
                        Schedule group
                    </h2>
                </div>
                <div class="filter-choice-container">
                    {#each Object.entries(ScheduleGroupTypes) as [key, value]}
                        <div class="option-container">
                            <label class="option-label" for={key}>
                                <input
                                    type="checkbox"
                                    style="--color: {theme.colors.brand[500]}"
                                    bind:checked={scheduleGroupOptions[value]}
                                    id={key}
                                />

                                {value}
                            </label>
                        </div>
                    {/each}
                </div>
            </div>
            <div>
                <div
                    class="filter-title-container"
                    style="--bg-color: {theme.colors.neutral[200]}"
                >
                    <h2
                        class="filter-option-title"
                        style="--text-color: {theme.colors.neutral[800]}"
                    >
                        Block
                    </h2>
                </div>
                <div class="filter-choice-container">
                    {#each Object.entries(BlockTypes) as [key, value]}
                        <div class="option-container">
                            <label class="option-label" for={key}>
                                <input
                                    type="checkbox"
                                    style="--color: {theme.colors.brand[500]}"
                                    bind:checked={blockOptions[value]}
                                    id={key}
                                />

                                {value}
                            </label>
                        </div>
                    {/each}
                </div>
            </div>
            <div>
                <div
                    class="filter-title-container"
                    style="--bg-color: {theme.colors.neutral[200]}"
                >
                    <h2
                        class="filter-option-title"
                        style="--text-color: {theme.colors.neutral[800]}"
                    >
                        Exam type
                    </h2>
                </div>
                <div class="filter-choice-container">
                    {#each Object.entries(ExamTypes) as [key, value]}
                        <div class="option-container">
                            <label class="option-label" for={key}>
                                <input
                                    type="checkbox"
                                    style="--color: {theme.colors.brand[500]}"
                                    bind:checked={examTypeOptions[value]}
                                    id={key}
                                />

                                {convertExamToString(value)}
                            </label>
                        </div>
                    {/each}
                </div>
            </div>
        </div>
        <button
            class="apply-button"
            style="
            --bg-color: {buttonBgColor};
            --text-color: {buttonTextColor};
            "
            on:mouseover={handleHover}
            on:mouseout={handleHoverOut}
            on:click={applyOptions}
        >
            Apply
        </button>
    </div>
</dialog>

<style scoped>
    .dialog {
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.4);
    }

    .title {
        color: var(--color);
    }

    .filter-container {
        background-color: #ffffff;
        width: 900px;
        height: 40vh;
        overflow: scroll;
        margin: auto;
        padding: 1%;
        margin-top: 30vh;
        display: flex;
        flex-direction: column;
        justify-content: start;
        align-items: center;
    }

    .filter-option-container {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
        justify-content: start;
        align-items: center;
    }

    .filter-choice-container {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
        justify-content: start;
        align-items: start;
    }

    .filter-content-container {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr 1fr;
        width: 100%;
        height: 100%;
    }

    .filter-title-container {
        width: 95%;
        display: flex;
        background-color: var(--bg-color);
        justify-content: center;
    }
    .option-container {
        width: 100%;
        margin-bottom: 2%;
        margin-top: 2%;
    }

    /* Check box styling heavily inspired by: https://moderncss.dev/pure-css-custom-checkbox-style/ */
    input[type="checkbox"] {
        /* Add if not using autoprefixer */
        -webkit-appearance: none;
        /* Remove most all native input styles */
        appearance: none;
        /* Not removed via appearance */
        margin-right: 0.25em;

        font: inherit;
        color: var(--color);
        width: 1.15em;
        height: 1.15em;

        border: 0.15em solid var(--color);
        border-radius: 0.15em;
        transform: translateY(-0.075em);
        display: grid;
        place-content: center;
    }

    input[type="checkbox"]::before {
        content: "";
        width: 0.65em;
        height: 0.65em;
        clip-path: polygon(14% 44%, 0 65%, 50% 100%, 100% 16%, 80% 0%, 43% 62%);
        transform: scale(0);
        transform-origin: bottom left;
        transition: 120ms transform ease-in-out;
        box-shadow: inset 1em 1em var(--color);
    }

    input[type="checkbox"]:checked::before {
        transform: scale(1);
    }

    .filter-option-title {
        font-size: 1.5rem;
        color: var(--text-color);
    }

    .option-label {
        display: flex;
        flex-direction: row;
        justify-content: start;
        align-items: center;
    }

    .apply-button {
        background: none;
        width: 15vw;
        height: 5vh;
        font-size: var(--font-size);
        border: 0;
        border-color: var(--text-color);
        color: var(--text-color);
        background-color: var(--bg-color);
        transition: ease-in-out 0.1s;
    }
</style>

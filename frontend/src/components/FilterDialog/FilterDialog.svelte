<script>
    import { onMount } from 'svelte';
import {
    filters,
    filtersObj,
    jsonToString,
    StudyLevelTypes,
    ScheduleGroupTypes,
    BlockTypes,
    ExamTypes
} from '../../stores.js';

export let dialog;

/**
* Helper function to generate option objects based on the different filter types
*/
function generateOptionsObject(TypeObject, field) {
    let obj = {};
    for (const [key, val] of Object.entries(TypeObject)) {
        obj[val] = ($filtersObj[field].includes(val.toString()))
    }
    return obj
}

/** 
* Helper function to aggregate chosen options into array
*/
function aggregateOptions(optionsObject) {
    let li = []
    for (const [key, val] of Object.entries(optionsObject)) {
        if (val === true) {
            li.push(key)
        }
    }
    return li
}

/**
* Applies the new filters
*/
function applyOptions() {
    $filters = jsonToString({
        ...$filtersObj,
        'study_level': aggregateOptions(studyLevelOptions),
        'schedule_group': aggregateOptions(scheduleGroupOptions),
        'block': aggregateOptions(blockOptions),
        'exam_type': aggregateOptions(examTypeOptions),
    })
    dialog.close();
}

function convertExamToString(inputString) {
    return inputString.replace(/(\w)_(\w)/g, "$1 $2");
}

let studyLevelOptions = generateOptionsObject(StudyLevelTypes, 'study_level')
let scheduleGroupOptions = generateOptionsObject(ScheduleGroupTypes, 'schedule_group')
let blockOptions = generateOptionsObject(BlockTypes, 'block')
let examTypeOptions = generateOptionsObject(ExamTypes, 'exam_type')

onMount(() => {
})
</script>

<dialog bind:this={dialog} on:close={() => {
    studyLevelOptions = generateOptionsObject(StudyLevelTypes, 'study_level')
    scheduleGroupOptions = generateOptionsObject(ScheduleGroupTypes, 'schedule_group')
    blockOptions = generateOptionsObject(BlockTypes, 'block')
    examTypeOptions = generateOptionsObject(ExamTypes, 'exam_type')
}}>
    <div class="filter-container">
    <button on:click={() => console.log($filtersObj)}> log </button>
    <button on:click={applyOptions}>Apply</button>
    <p>Course level</p>
    {#each Object.entries(StudyLevelTypes) as [key, value]}
        <div>
            <input
                type="checkbox"
                bind:checked={studyLevelOptions[value]}
                id={key}
            />
            <label for={key}>{value}</label>
        </div>
    {/each}

    <p>Schedule group</p>
    {#each Object.entries(ScheduleGroupTypes) as [key, value]}
        <div>
            <input
                type="checkbox"
                bind:checked={scheduleGroupOptions[value]}
                id={key}
            />
            <label for={key}>{value}</label>
        </div>
    {/each}

    <p>Block</p>
    {#each Object.entries(BlockTypes) as [key, value]}
        <div>
            <input
                type="checkbox"
                bind:checked={blockOptions[value]}
                id={key}
            />
            <label for={key}>{value}</label>
        </div>
    {/each}
    <p>Exam type</p>
    {#each Object.entries(ExamTypes) as [key, value]}
        <div>
            <input
                type="checkbox"
                bind:checked={examTypeOptions[value]}
                id={key}
            />
            <label for={key}>{convertExamToString(value)}</label>
        </div>
    {/each}
    </div>
</dialog>

<style scoped>
.filter-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
}
</style>

<script>
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
function generateOptionsObject(TypeObject) {
    let obj = {};
    for (const [_, val] of Object.entries(TypeObject)) {
        obj[val] = false
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

function applyOptions() {
    // Apply studyLevelOptions
    $filters = jsonToString({
        ...$filtersObj,
        'study_level': aggregateOptions(studyLevelOptions)
    })
}
let studyLevelOptions = generateOptionsObject(StudyLevelTypes)
</script>

<dialog bind:this={dialog} on:close>
    <button on:click={() => console.log(aggregateOptions(studyLevelOptions))}> log </button>
    <button on:click={applyOptions}>Apply</button>
    {#each Object.entries(StudyLevelTypes) as [key, value]}
        <input
            type="checkbox"
            bind:checked={studyLevelOptions[value]}
            id={key}
        />
        <label for={key}>{value}</label>
    {/each}
</dialog>

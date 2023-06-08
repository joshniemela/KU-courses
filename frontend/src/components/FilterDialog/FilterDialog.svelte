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
function generateOptionsObject(TypeObject, field) {
    let obj = {};
    for (const [key, val] of Object.entries(TypeObject)) {
        console.log($filtersObj[field])
        obj[val] = ($filtersObj[field].length > 0)
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
    // Apply studyLevelOptions
    $filters = jsonToString({
        ...$filtersObj,
        'study_level': aggregateOptions(studyLevelOptions),
        'scheduleGroupOptions': aggregateOptions(scheduleGroupOptions)
    })
}


let studyLevelOptions = generateOptionsObject(StudyLevelTypes, 'study_level')
let scheduleGroupOptions = generateOptionsObject(ScheduleGroupTypes, 'schedule_group')
</script>

<dialog bind:this={dialog} on:close>
    <div class="filter-container">
    <button on:click={() => console.log(aggregateOptions(studyLevelOptions))}> log </button>
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
    <p>Exam type</p>
    </div>
</dialog>

<style scoped>
.filter-container {
    display: flex;
    flex-direction: column;
}
</style>

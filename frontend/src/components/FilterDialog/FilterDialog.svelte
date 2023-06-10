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

/**
    * Checks wether we are clicking on the background, and if so closes the dialog 
    * @function closeOnClickOutside 
*/
function closeOnClickOutside(event) {
        if (event.target.id == 'dialog') {
            dialog.close()
        }
    }
onMount(() => {
})
</script>

<dialog class="dialog" bind:this={dialog}
    id="dialog"
    on:click={closeOnClickOutside}
    on:close={() => {
    studyLevelOptions = generateOptionsObject(StudyLevelTypes, 'study_level')
    scheduleGroupOptions = generateOptionsObject(ScheduleGroupTypes, 'schedule_group')
    blockOptions = generateOptionsObject(BlockTypes, 'block')
    examTypeOptions = generateOptionsObject(ExamTypes, 'exam_type')
}}>
    <div class="filter-container">
        <h1>Filters</h1>
        <div class="filter-content-container">
            <div>
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
            </div>
            <div>
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
            </div>
            <div>
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
            </div>
            <div>
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
        </div>
        <button on:click={() => console.log($filtersObj)}> log </button>
        <button on:click={applyOptions}>Apply</button>
    </div>
</dialog>

<style scoped>
.dialog {
    width:100%;
    height:100%;
    background-color:rgba(0,0,0,0.8);
}

.filter-container {
    background-color:#ffffff;
    width: 30%;
    height: 60%;
    overflow: scroll;
    margin: auto;
    padding: 1%;
    margin-top: 15vh;
    display: flex;
    flex-direction: column;
    justify-content: start;
    align-items: center;
}

.filter-content-container {
    display: grid;
    grid-template-columns: 1fr 1fr;
    background-color: yellow;
    width: 100%;
    height: 100%;
}
</style>

<script>
    import { navigate } from "svelte-navigator";
import Dk from "../../assets/Dk.svelte";
import Gb from "../../assets/Gb.svelte";
import overviewCardDefault from "./overviewCardDefault.json";
import theme from "../../theme.js";

export let stagger = 0;
export let data = overviewCardDefault;

/** 
* Function to extracct the first <charLimit> letters from the paragraphs to 
use for the summary.
* @function extractSummary
*/
function extractSummary(charLimit) {
    let summaryArray = []
    for (var i in data.description) {
        let elem = data.description[i]
        if (summaryArray.join().length <  charLimit && elem != data.description[data.description.length -1]) {
            if (elem.type == 'p') {
                summaryArray.push(elem.string)
            } 
        } else {
            let summ = summaryArray.join()
            return (summ.slice(0, charLimit) + "...")
        }
    }
}
let summary = extractSummary(390);

/**
* Function to scale the font sizes of the course titles based on their length
* @function calcFontSize
*/
function calcFontSize(string) {
    return (1 + 12/string.length)*16 + "px"
}
/**
* Function to navigate to the course corresponding with the course_id
* @function navigateToCourse
*/
function navigateToCourse() {
    navigate(`/course/${data.course_id}`);
    location.reload();
}


function convertExamToString(inputString) {
    return inputString.replace(/(\w)_(\w)/g, "$1 $2");
}

</script>
<div class="card-container" on:click={navigateToCourse}>
    <div class="card"
        style="
            --bg-color: {theme.colors.neutral[800]};
            --stagger: {stagger};
            "
    >
        <div class="card-header-container">
            <div class="card-title-container">
                <div class="title-container">
                    <a href={`/course/${data.course_id}`}>
                        <h1 class="card-title" 
                            style="
                                --text-color: {theme.colors.neutral[200]};
                                --text-size: {calcFontSize(data.title)}
                                "
                        >
                            {data.title}
                        </h1>
                    </a>
                </div>
                <h2 class="card-subtitle" style="--text-color: {theme.colors.neutral[600]}">{data.course_id} - SCIENCE</h2>
            </div>
            <table class="card-info-table">
                <tr>
                    <td class="card-td-left-top">{data.study_level}</td>
                    <td class="card-td-right-top">{data.credits} ECTS</td>
                </tr>
                <tr>
                    <td class="card-td-left-bot">Block {data.start_block}</td>
                    <td> Group: SCHEDULES</td>
                </tr>
            </table>
        </div>
        <div class="card-description-container">
            <p class="card-description">{ summary }</p>
        </div>
        <div class="card-exam-text-container"
            style="--bg-color: {theme.colors.neutral[300]}"
        >
            {#each data.exams as exam }
                <p class="card-exam-text"
                style="--text-color: {theme.colors.neutral[900]}"

                >
                    {convertExamToString(exam.exam_type)} {#if exam.minutes} ({exam.minutes}m) {/if}
                    {#if exam != data.exams[data.exams.length - 1] && data.exams.length > 1} -  &nbsp {/if}
                </p>
            {/each}
            {#if data.course_language == "da"}
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
    max-width: 30vw;
    background-color: var(--bg-color);
    border-radius: 10px;
    box-shadow: 0 2.8px 2.2px rgba(0, 0, 0, 0.15);
    transition: ease-in-out 0.2s;
    opacity: 0%;
    animation: fadeIn 0.5s calc(var(--stagger)*0.05s);
    animation-fill-mode: forwards;
    overflow: scroll;
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
    justify-content: space-between;
    align-items: start;
}

.card-description-container {
    width: 96%;
    padding-left: 2%;
    padding-right: 2%;
    height: 80%;
    overflow: scroll;
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
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    overflow: auto;
    word-wrap: break-word;
}

@keyframes fadeIn {
  0% { opacity: 0; }
  100% { opacity: 1; }
}

</style>

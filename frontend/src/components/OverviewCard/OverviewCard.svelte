<script>
import Dk from "../../assets/Dk.svelte";
import Gb from "../../assets/Gb.svelte";
import theme from "../../theme.js";

export let stagger = 0;
export let data = {
    "primary_title": "High Performance Programming and Systems (HPPS)",
    "course_id": "NDAB20001U",
    "course_language": "en",
    "faculty": "SCIENCE",
    "study_level": "Bachelor",
    "credits": "7.5",
    "start_block": "2",
    "schedule_group": "A",
    "description": "Kurset introducerer de studerende til emnerne computerarkitektur og datanetværk, hukommelsesarkitektur, styresystemer, task-parallelisme og samtidighed, samt til massivt data-paralleliserede arkitekturer. Der vil være fokus på effektiv data-processering (big data) og effektive beregninger (big compute).",
    "exam": "Skriftlig aflevering, 51 timer"
}

/**
* Function to scale the font sizes of the course titles based on their length
*/
function calcFontSize(string) {
    
    return (1 + 12/string.length)*16 + "px"
}

</script>
<div class="card-container">
    <div class="card"
        style="
            --bg-color: {theme.colors.neutral[800]};
            --stagger: {stagger};
            "
    >
        <div class="card-header-container">
            <div class="card-title-container">
                <div class="title-container">
                <h1 class="card-title" 
                    style="
                        --text-color: {theme.colors.neutral[200]};
                        --text-size: {calcFontSize(data.primary_title)}
                        "
                >
                    {data.primary_title}</h1>
                </div>
                <h2 class="card-subtitle" style="--text-color: {theme.colors.neutral[600]}">{data.course_id} - {data.faculty}</h2>
            </div>
            <table class="card-info-table">
                <tr>
                    <td class="card-td-left-top">{data.study_level}</td>
                    <td class="card-td-right-top">{data.credits} ECTS</td>
                </tr>
                <tr>
                    <td class="card-td-left-bot">Block {data.start_block}</td>
                    <td> Group {data.schedule_group}</td>
                </tr>
            </table>
        </div>
        <div class="card-description-container">
            <p class="card-description">{data.description}</p>
        </div>
        <div class="card-exam-text-container"
            style="--bg-color: {theme.colors.neutral[300]}"
        >
            <p class="card-exam-text"
            style="--text-color: {theme.colors.neutral[900]}"

            >
                {data.exam}
            </p>
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

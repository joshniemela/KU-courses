<script>
import { page } from '$app/stores';
import { onMount } from 'svelte';
import theme from '../../../theme';
import Loader from '../../../components/Loader/Loader.svelte';
import { LoremIpsum } from 'lorem-ipsum';
import overview from '../../../mocking/overview.json';

const courseId = $page.params.courseId;

let loading = true;

let course = {};

const desc = new LoremIpsum({
    sentencesPerParagraph: {
        max: 8,
        min: 4
    },
    wordsPerSentence: {
        max: 16,
        min: 4
    }
});

const matchId = (obj) => {
    console.log('matching id ' + obj.course_id)
    return obj.course_id === courseId
}

const fetchCourse = async (courseId) => {
    await new Promise(resolve => setTimeout(resolve, 1000));
    loading = false;
    const index = overview.findIndex(course => course.course_id === courseId)
    return overview[index]
}

onMount(async () => {
    const res = await fetchCourse(courseId);
    console.log(res)
    course = res
})

</script>
{#if loading}
    <Loader />
{:else}
    <div class="main-container">
        <div class="content-container">
            <div class="content-container-left">
                <div class="header-container">
                    <a href="/browse">
                        <p>Go back</p>
                    </a>
                    <div>
                        <h1>{course.primary_title}</h1>
                        <h2>{course.course_id} - SCIENCE </h2>
                    </div>
                </div>
                <h3>Description</h3>
                <p>{desc.generateParagraphs(1)}</p>
                <p>{desc.generateParagraphs(1)}</p>
                <p>{desc.generateParagraphs(1)}</p>
                <p>{desc.generateParagraphs(1)}</p>
            </div>
            <div class="content-container-right">
                <div class="side-card"
                    style="
                        --bg-color: {theme.colors.neutral[800]};
                        --text-color: {theme.colors.neutral[200]};
                        --sub-title-color: {theme.colors.neutral[600]};
                        --brand-color: {theme.colors.brand[500]};
                        "
                >
                    <h3 class="side-card-heading">Coordinators</h3>
                    <div class="side-card-name-title">
                        <p class="side-card-name">Jon Sporring</p>
                        <p class="side-card-title">(Professor)</p>
                    </div>
                    <p class="side-card-clickable">sporring@di.ku.dk</p>

                    <div class="side-card-name-title">
                        <p class="side-card-name">Josh Niemela</p>
                        <p class="side-card-title">(baller)</p>
                    </div>
                    <p class="side-card-clickable">rick@roll.com</p>
                </div>
                <div class="side-card"
                    style="
                        --bg-color: {theme.colors.neutral[800]};
                        --text-color: {theme.colors.neutral[200]};
                        --sub-title-color: {theme.colors.neutral[600]};
                        --brand-color: {theme.colors.brand[500]};
                        "
                >
                    <h3 class="side-card-heading">Info</h3>
                    <p class="side-card-name">Bachelor's course</p>
                    <p class="side-card-name">15 ECTS</p>
                    <p class="side-card-clickable">https://kurser.ku.dk/course/ndab15009u</p>
                </div>
                <div class="side-card"
                    style="
                        --bg-color: {theme.colors.neutral[800]};
                        --text-color: {theme.colors.neutral[200]};
                        --sub-title-color: {theme.colors.neutral[600]};
                        --brand-color: {theme.colors.brand[500]};
                        "
                >
                    <h3 class="side-card-heading">Schedule</h3>
                    <p class="side-card-name">Block: 1</p>
                    <p class="side-card-name">Group: A</p>
                </div>
                <div class="side-card"
                    style="
                        --bg-color: {theme.colors.neutral[800]};
                        --text-color: {theme.colors.neutral[200]};
                        --sub-title-color: {theme.colors.neutral[600]};
                        --brand-color: {theme.colors.brand[500]};
                        "
                >
                    <h3 class="side-card-heading">Workload</h3>
                    <p class="side-card-name">Lectures: 54h</p>
                    <p class="side-card-name">Preparation: 106h</p>
                    <p class="side-card-name">Project: 144h</p>
                    <p class="side-card-name">Exercises: 108h</p>
                    <p class="side-card-clickable">Total: 412h</p>
                </div>
                <div class="side-card"
                    style="
                        --bg-color: {theme.colors.neutral[800]};
                        --text-color: {theme.colors.neutral[200]};
                        --sub-title-color: {theme.colors.neutral[600]};
                        --brand-color: {theme.colors.brand[500]};
                        "
                >
                    <h3 class="side-card-heading">Exam</h3>
                    <p class="side-card-name">Ongoing evaluation</p>
                </div>
            </div>
        </div>
    </div>
{/if}

<style scoped>
.main-container {
    display: flex;
    width: 100%;
    height: 100vh;
    flex-direction: column;
    justify-content: start;
    align-items: center;
}

.header-container {
    display: flex;
    width: 100%;
    flex-direction: row;
    justify-content: start;
    align-items: center;
}

.content-container {
    height: 100%;
    width: 100%;
    display: grid;
    grid-template: 1fr / 4fr 1fr;
}

.content-container-right {
    height: 92%;
    width: 92%;
    padding: 4%;
    display: flex;
    flex-direction: column;
    justify-content: start;
    align-items: center;
}

.content-container-left {
    height: 92%;
    width: 92%;
    padding: 4%;
}

.side-card {
    width: 90%;
    margin-bottom: 2vh;
    background-color: var(--bg-color);
    color: var(--text-color);
    padding: 2%;
    border-radius: 10px;
}

.side-card-heading {
    font-size: 1.5rem;
    color: var(--text-color);
}
.side-card-name-title {
    display: flex;
    flex-direction: row;
    justify-content: start;
}
.side-card-name {
    font-size: 1rem;
    color: var(--text-color);
}

.side-card-title {
    font-size: 1rem;
    color: var(--sub-title-color);
    margin-left: 0.5vw;
}

.side-card-clickable {
    font-size: 1rem;
    color: var(--brand-color);
    margin-bottom: 1vh;
}

</style>

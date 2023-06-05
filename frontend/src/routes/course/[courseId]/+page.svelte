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
                    <div class="coordinator-name-title">
                        <p class="coordinator-name">Jon Sporring</p>
                        <p class="coordinator-title">(Professor)</p>
                    </div>
                    <p class="coordinator-email">sporring@di.ku.dk</p>

                    <div class="coordinator-name-title">
                        <p class="coordinator-name">Josh Niemela</p>
                        <p class="coordinator-title">(baller)</p>
                    </div>
                    <p class="coordinator-email">rick@roll.com</p>
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
    grid-template: 1fr / 3fr 1fr;
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
.coordinator-name-title {
    display: flex;
    flex-direction: row;
    justify-content: start;
}
.coordinator-name {
    font-size: 1rem;
    color: var(--text-color);
}

.coordinator-title {
    font-size: 1rem;
    color: var(--sub-title-color);
    margin-left: 0.5vw;
}

.coordinator-email {
    font-size: 1rem;
    color: var(--brand-color);
    margin-bottom: 1vh;
}

</style>

<script>
import { page } from '$app/stores';
import { onMount } from 'svelte';
import overview from '../../../mocking/overview.json';

const courseId = $page.params.courseId;

let loading = true;

let course = {};


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
    <p>loading</p>
{:else}
    <div class="main-container">
        <div class="header-container">
            <h1>{course.primary_title}</h1>
            <h2>{course.course_id}</h2>
        </div>
        <div class="content-container">
            <div class="content-container-left">
                <p>Description</p>
            </div>
            <div class="content-container-right">
                <p>Workload</p>
                <p>Coordinators (with title)</p>
                <p>ECTS</p>
                <p>Schedule group</p>
                <p>Block</p>
                <p>Faculty</p>
                <p>Level</p>
                <p>Exam</p>
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
    background-color: yellow;
}

.header-container {
    display: flex;
    width: 100%;
    flex-direction: column;
    justify-content: start;
    align-items: start;
}

.content-container {
    height: 100%;
    width: 100%;
    display: grid;
    grid-template: 1fr / 3fr 2fr;
}

.content-container-right {
    height: 100%;
    width: 100%;
    background-color: red;
}

.content-container-left {
    height: 100%;
    width: 100%;
    background-color: grey;
}
</style>

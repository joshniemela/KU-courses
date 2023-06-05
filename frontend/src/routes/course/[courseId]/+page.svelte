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
    <h1>{course.primary_title}</h1>
    <p>not loading</p>
{/if}

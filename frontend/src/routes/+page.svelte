<script>
import theme from '../theme'
import SearchComponent from '../components/SearchComponent/SearchComponent.svelte';
import { filtersObj } from '../stores';

function convertExamToString(inputString) {
    return inputString.replace(/(\w)_(\w)/g, "$1 $2");
}

</script>
    <div class="content">
        <h1 class="title" style="--font-color: {theme.colors.brand[500]}">KU Courses (WIP)</h1>

        <!-- Container responsible for the search area --> 
        <div class="search-container">
            <SearchComponent />
        </div>
        {#if $filtersObj.searches.length > 0}
            <p> Current search: </p>
        {/if}
        {#each $filtersObj.searches as searchElem}
            <p>
            {convertExamToString(searchElem.type)} contains: 
            {#each searchElem.search as s}
                {#if searchElem.search.length == 1}
                    {s}
                {:else if s != searchElem.search[searchElem.search.length -1]}
                    {s} OR
                {:else}
                    {s}
                {/if}
            {/each}
            </p>
            {#if $filtersObj.searches.length > 1}
                {#if searchElem != $filtersObj.searches[$filtersObj.searches.length -1]}
                    <p> AND </p>
                {/if}
            {/if}
        {/each}
</div>

<style scoped>
.title {
    color: var(--font-color);
    margin-bottom: 2vh;
}

.content {
    display: flex;
    height: 100vh;
    width: 100%;
    margin-right: 8vw; /* should be the same as navbar-container width */
    flex-direction: column;
    align-items: center;
    justify-content: center;
}

.search-container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 5vh;
    width: 30vw;
    margin-bottom: 3vh;
}

.view-all-button {
    background: none;
    font-size: 1rem;
    border: 0;
    color: var(--text-color);
    width: 8vw;
    background-color: var(--bg-color);
    transition: ease-in-out 0.1s;
}


</style>

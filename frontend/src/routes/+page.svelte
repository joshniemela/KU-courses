<script lang="ts">
    import theme from "../theme";
    import SearchComponent from "../components/SearchComponent/SearchComponent.svelte";
    import { filtersObj } from "../stores";

    import CheckboxMenu from "../components/CheckboxMenu.svelte";
    let selected_values: string[] = [];

    function convertExamToString(inputString: string) {
        return inputString.replace(/(\w)_(\w)/g, "$1 $2");
    }
    type firstFiveInts = 1 | 2 | 3 | 4 | 5;
</script>

<div class="content">
    <h1 class="title" style="--font-color: {theme.colors.brand[500]}">
        KU Courses
    </h1>

    <!-- Container responsible for the search area -->
    <div class="search-container">
        <SearchComponent />
    </div>
    {#if $filtersObj.searches.length > 0}
        <p>Current search:</p>
    {/if}
    {#each $filtersObj.searches as searchElem}
        <p>
            {convertExamToString(searchElem.type)} contains:
            {#each searchElem.search as s}
                {#if searchElem.search.length == 1}
                    {s}
                {:else if s != searchElem.search[searchElem.search.length - 1]}
                    {s} OR
                {:else}
                    {s}
                {/if}
            {/each}
        </p>
        {#if $filtersObj.searches.length > 1}
            {#if searchElem != $filtersObj.searches[$filtersObj.searches.length - 1]}
                <p>AND</p>
            {/if}
        {/if}
    {/each}

    // bind selected_values to the selected values of the checkbox menu
    <CheckboxMenu
        header_name="Test Menu"
        options={["1", "2", "3", "4", "5"]}
        bind:selected = {selected_values}
    />
    Selected: {selected_values.join(", ")}

</div>

<style scoped>
    .title {
        color: var(--font-color);
        margin-bottom: 2vh;
    }

    .content {
        display: flex;
        height: 98vh;
        width: 100%;
        margin-right: 8vw; /* should be the same as navbar-container width */
        flex-direction: column;
        align-items: center;
        justify-content: center;
        overflow: auto;
    }

    .search-container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        width: 30vw;
        margin-bottom: 3vh;
    }

    /*     .view-all-button {
        background: none;
        font-size: 1rem;
        border: 0;
        color: var(--text-color);
        width: 8vw;
        background-color: var(--bg-color);
        transition: ease-in-out 0.1s;
    } */
</style>

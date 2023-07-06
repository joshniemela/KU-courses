<script lang="ts">
    import theme from "../theme";
    import SearchComponent from "../components/SearchComponent/SearchComponent.svelte";
    import { filtersObj } from "../stores";

    import CheckboxMenu from "../components/CheckboxMenu.svelte";
    import TextSearch from "../components/TextSearch.svelte";
    let selected_values: string[] = [];

    function convertExamToString(inputString: string) {
        return inputString.replace(/(\w)_(\w)/g, "$1 $2");
    }
    type firstFiveInts = 1 | 2 | 3 | 4 | 5;
</script>

<main class="flex flex-col items-center justify-center space-y-4">
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
    <TextSearch />
    <div
        class="flex flex-col justify-center space-y-4 md:space-y-0 md:flex-row md:space-x-4"
    >
        <CheckboxMenu
            header_name="Block"
            options={["1", "2", "3", "4", "5"]}
            bind:selected={selected_values}
        />

        <CheckboxMenu
            header_name="Study Level"
            options={["Bachelor", "Master", "PhD"]}
            bind:selected={selected_values}
        />

        <CheckboxMenu
            header_name="Schedule Group"
            options={["A", "B", "C", "D"]}
            bind:selected={selected_values}
        />

        <CheckboxMenu
            header_name="Examination Type"
            options={["Written", "Oral", "Assignment", "Continuous Assessment"]}
            bind:selected={selected_values}
        />
    </div>
</main>

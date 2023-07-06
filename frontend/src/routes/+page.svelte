<script lang="ts">
    import theme from "../theme";
    import SearchComponent from "../components/SearchComponent/SearchComponent.svelte";
    import CheckboxMenu from "../components/CheckboxMenu.svelte";
    import TextSearch from "../components/TextSearch.svelte";
    import { writableSession } from "../newStore";

    function convertExamToString(inputString: string) {
        return inputString.replace(/(\w)_(\w)/g, "$1 $2");
    }
    // make a writableSession if we have a browser
    const emptyQuery = {
        block: [],
        study_level: [],
        schedule_group: [],
        examination_type: [],
        searches: [],
    };

    const queryStore = writableSession("query", emptyQuery);

    function clearAll() {
        // Cause the checkboxes to update
        queryStore.update((store) => {
            store.block = [];
            store.study_level = [];
            store.schedule_group = [];
            store.examination_type = [];
            store.searches = [];
            return store;
        });
    }
</script>

<main class="flex flex-col items-center justify-center space-y-4 mt-40">
    <h1 class="text-brand-500 text-4xl font-bold">
        KU Courses 2.0 the electric boogaloo (WIP! everything is broken)
    </h1>

    <!-- Container responsible for the search area -->
    <TextSearch bind:searches={$queryStore.searches} />
    <div
        class="flex flex-col justify-center space-y-4 md:space-y-0 md:flex-row md:space-x-4"
    >
        <CheckboxMenu
            header_name="Block"
            options={["1", "2", "3", "4", "5"]}
            bind:selected={$queryStore.block}
        />

        <CheckboxMenu
            header_name="Study Level"
            options={["Bachelor", "Master", "PhD"]}
            bind:selected={$queryStore.study_level}
        />

        <CheckboxMenu
            header_name="Schedule Group"
            options={["A", "B", "C", "D"]}
            bind:selected={$queryStore.schedule_group}
        />

        <CheckboxMenu
            header_name="Examination Type"
            options={["Written", "Oral", "Assignment", "Continuous Assessment"]}
            bind:selected={$queryStore.examination_type}
        />
    </div>
    <button
        class="bg-brand-500 text-white px-4 py-0"
        on:click={() => {
            clearAll();
        }}
    >
        Clear All
    </button>
</main>

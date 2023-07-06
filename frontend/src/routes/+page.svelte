<script lang="ts">
    import theme from "../theme";
    import SearchComponent from "../components/SearchComponent/SearchComponent.svelte";

    import CheckboxMenu from "../components/CheckboxMenu.svelte";
    import TextSearch from "../components/TextSearch.svelte";

    let selected_values: string[] = [];

    function convertExamToString(inputString: string) {
        return inputString.replace(/(\w)_(\w)/g, "$1 $2");
    }

    let query = {
        blocks: [],
        study_levels: [],
        schedule_groups: [],
        examination_types: [],
        searches: [],
    };
</script>

<main class="flex flex-col items-center justify-center space-y-4 mt-40">
    <h1 class="text-brand-500 text-4xl font-bold">KU Courses</h1>

    <!-- Container responsible for the search area -->
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

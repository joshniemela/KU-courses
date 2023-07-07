<script lang="ts">
    import CheckboxMenu from "../components/CheckboxMenu.svelte";
    import TextSearch from "../components/TextSearch.svelte";
    import { queryStore, clearAll } from "../stores";
</script>

<main class="flex flex-col items-center justify-center space-y-4 mt-40">
    <h1 class="text-brand-500 text-4xl font-bold">
        KU Courses 2.0 the electric boogaloo (WIP! everything is broken)
    </h1>

    <TextSearch bind:searches={$queryStore.searches} />
    <!--Search button, this sends us to /browse------------------------------->
    <button class="bg-blue-500 text-white px-4 py-0">
        <a href="/browse">Search</a>
    </button>
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
            options={["Bachelor", "Master"]}
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

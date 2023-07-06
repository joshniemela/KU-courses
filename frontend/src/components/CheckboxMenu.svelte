<script lang="ts">
    // Create a function that takes a list of possible values and returns an object with the
    // possible values as keys and the values as booleans from checkboxes
    export let header_name: string;
    export let options: string[];
    export let selected: string[];
</script>

<div class="">
    <div class="bg-kuRed text-white text-center px-2 py-1">
        <h1>{header_name}</h1>
    </div>
    <div class="flex flex-col">
        {#each options as option}
            <!--Align label before checkbox-->
            <label class="flex items-center relative cursor-pointer py-1.5">
                <input
                    class="text-kuRed"
                    type="checkbox"
                    bind:group={selected}
                    name="header_name"
                    value={option}
                />
                <span class="text-center w-full absolute">
                    {option}
                </span>
            </label>
        {/each}
        <button
            class="bg-kuGray text-white text-center px-1 text-xs mx-2 {selected.length
                ? 'visible'
                : 'invisible'}"
            on:click={() => (selected = [])}>Clear all</button
        >
    </div>
</div>

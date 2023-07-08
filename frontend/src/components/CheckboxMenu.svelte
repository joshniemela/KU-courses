<script lang="ts">
    // Create a function that takes a list of possible values and returns an object with the
    // possible values as keys and the values as booleans from checkboxes
    export let header_name: string;
    export let options: string[];
    export let selected: string[];
</script>

<div class="">
    <div class="bg-kuGray text-white text-center px-2 py-1 w-full">
        <h1>{header_name}</h1>
    </div>

    <div class="flex flex-col">
        <button
            class="bg-kuRed text-white text-center text-xs {selected.length
                ? 'visible'
                : 'invisible'}"
            on:click={() => (selected = [])}>Clear</button
        >
        {#each options as option}
            <!--Align label before checkbox-->
            <label
                class="flex items-center relative cursor-pointer py-1.5 ml-2"
            >
                <input
                    class="text-kuRed"
                    type="checkbox"
                    bind:group={selected}
                    name="header_name"
                    value={option}
                />
                <span class="text-left w-full absolute leading-none ml-6">
                    {option}
                </span>
            </label>
        {/each}
    </div>
</div>

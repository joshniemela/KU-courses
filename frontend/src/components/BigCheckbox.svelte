<script lang="ts">
    // Create a function that takes a list of possible values and returns an object with the
    // possible values as keys and the values as booleans from checkboxes
    export let header_name: string;
    export let options: string[];
    export let selected: string[];

    export let show = false;
</script>

<div class="flex flex-col w-full text-sm md:text-base">
    <div
        class="bg-kuRed text-white text-center px-2 py-1"
        on:click={() => (show = !show)}
    >
        <h2>
            {show ? "Hide" : "Show"}
            {header_name}s {selected.length ? `(${selected.length})` : ""}
        </h2>
    </div>

    <div class="flex flex-col {show ? 'visible' : 'collapse'}">
        <button
            class="bg-kuRed text-white mt-2 text-center text-xs {selected.length
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

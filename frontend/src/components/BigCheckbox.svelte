<script lang="ts">
    // Create a function that takes a list of possible values and returns an object with the

    interface Props {
        // possible values as keys and the values as booleans from checkboxes
        header_name: string;
        options: string[];
        selected: string[];
        show?: boolean;
    }

    let {
        header_name,
        options,
        selected = $bindable(),
        show = $bindable(false),
    }: Props = $props();
</script>

<div class="flex flex-col w-full text-sm md:text-base">
    <button
        type="button"
        class="bg-kuRed text-white text-center px-2 py-1"
        onclick={() => (show = !show)}
    >
        <h2>
            {show ? "Hide" : "Show"}
            {header_name}s {selected.length ? `(${selected.length})` : ""}
        </h2>
    </button>

    <div class="flex flex-col {show ? 'visible' : 'hidden'}">
        <button
            class="bg-kuRed text-white mt-2 text-center text-xs {selected.length
                ? 'visible'
                : 'invisible'}"
            onclick={() => (selected = [])}>Clear</button
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

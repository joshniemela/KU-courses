<script lang="ts">
    import theme from "../../theme";
    import FilterDialog from "../FilterDialog/FilterDialog.svelte";
    import { filterCount } from "../../stores";

    // Props
    export let paddingLR = "1vw";
    export let fontSize = "1.25rem";
    let dialog: HTMLDialogElement;

    // Button on hover animation
    const buttonTextColor = theme.colors.brand[200];
    const buttonBgColor = theme.colors.brand[800];
    const buttonTextColorHover = theme.colors.brand[900];
    const buttonBgColorHover = theme.colors.brand[500];
</script>

<button
    class="filter-button"
    style="
    --bg-color: {buttonBgColor};
    --text-color: {buttonTextColor};
    --bg-color-Hover: {buttonTextColorHover};
    --text-color-Hover: {buttonBgColorHover};
    --padding: {paddingLR};
    --font-size: {fontSize}
    "
    on:click={() => dialog.showModal()}
    >Filter{#if $filterCount > 0}s ({$filterCount}){/if}</button
>
<FilterDialog bind:dialog on:close={() => console.log("closed")} />

<style scoped>
    .filter-button {
        background: none;
        font-size: var(--font-size);
        border: 0;
        padding-right: var(--padding);
        padding-left: var(--padding);
        border-color: var(--text-color);
        color: var(--text-color);
        height: 100%;
        white-space: nowrap;
        background-color: var(--bg-color);
        transition: ease-in-out 0.1s;
    }

    .filter-button:hover {
        border-color: var(--text-color-Hover);
        background-color: var(--bg-color-Hover);
        color: var(--text-color-Hover);
    }
</style>

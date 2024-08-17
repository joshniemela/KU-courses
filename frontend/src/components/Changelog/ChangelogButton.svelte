<script lang="ts">
    import { modalStore } from "./store";
</script>

<button type="button" class="text-kuRed/75 underline md:no-underline md:text-sm relative" on:click={modalStore.open}>
    <span class="p-2">
        View Changelog
    </span>

    <span class="absolute block h-full w-full bg-kuRed top-0 left-0 -z-10">
    </span>
</button>

<style>
    button span {
        height: 0%;
        transition-duration: 300ms;
    }

    button:hover span {
        height: 100%;
    }

    button:hover {
        color: white;
    }
</style>
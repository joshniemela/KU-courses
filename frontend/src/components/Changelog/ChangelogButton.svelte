<script lang="ts">
    import { modalStore } from "./store";
</script>

<button type="button" class="p-2 text-kuRed/75 underline md:no-underline md:text-sm" on:click={modalStore.open}>
    View Changelog
</button>
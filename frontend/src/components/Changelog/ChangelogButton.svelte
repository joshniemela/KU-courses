<script lang="ts">
    import { modalStore } from "./store";
</script>

<button
    type="button"
    class="text-kuRed/75 underline md:no-underline md:text-sm relative"
    onclick={modalStore.open}
>
    View Changelog
</button>

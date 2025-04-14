<script lang="ts">
    interface Props {
        heading: string;
        children?: import("svelte").Snippet;
    }

    let { heading, children }: Props = $props();
</script>

<!--sidecard component-------------------------------------------------------->
<div class="my-4">
    <h3 class="bg-kuGray text-white font-bold px-4">{heading}</h3>
    <div class="px-4">
        {@render children?.()}
    </div>
</div>

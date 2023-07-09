<script lang="ts">
    export let heading: string;
</script>

<!--sidecard component-------------------------------------------------------->
<div class="my-4">
    <h3 class="bg-kuGray text-white font-bold px-4">{heading}</h3>
    <div class="px-4">
        <slot />
    </div>
</div>

<script lang="ts">
    import "../app.css";
    import { browser } from "$app/environment";
    import { page } from "$app/stores";

    $: if (browser && (window as any).goatcounter) {
        (window as any).goatcounter.count({
            path: $page.url.pathname,
        });
    }
</script>

<svelte:head>
    <script
        data-goatcounter-settings="{'{"allow-local":true, "no_onload": true}'}"
        data-goatcounter="https://disku.goatcounter.com/count"
        async
        src="//gc.zgo.at/count.js"
    ></script>
</svelte:head>

<div class="main">
    <div class="slot-container content-offset">
        <slot />
    </div>
</div>

<script lang="ts">
    import "../app.css";
    import { browser } from "$app/environment";
    import { page } from "$app/stores";

    $: if (browser && window.goatcounter) {
        window.goatcounter.count({
            path: $page.url.pathname,
        });
        console.log("counting", $page.url.pathname);
    }
</script>

<svelte:head>
    <!-- analtyics -->
    <!-- analtyics -->
    <!-- analtyics -->
    <script
        data-goatcounter-settings={'{"allow-local":true, "no_onload": true}'}
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

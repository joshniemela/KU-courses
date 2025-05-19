<script lang="ts">
    import { run } from "svelte/legacy";

    import "../app.css";
    import { browser } from "$app/environment";
    import { page } from "$app/stores";
    interface Props {
        children?: import("svelte").Snippet;
    }

    let { children }: Props = $props();

    run(() => {
        if (browser && (window as any).goatcounter) {
            (window as any).goatcounter.count({
                path: $page.url.pathname,
            });
        }
    });
</script>

<svelte:head>
    <script
        data-goatcounter-settings={'{"allow-local":true, "no_onload": true}'}
        data-goatcounter="https://kucourses.goatcounter.com/count"
        async
        src="//gc.zgo.at/count.js"
    ></script>
</svelte:head>

<div class="main">
    <div class="slot-container content-offset">
        {@render children?.()}
    </div>
</div>

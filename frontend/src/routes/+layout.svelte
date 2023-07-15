<script lang="ts">
    import Navbar from "../components/Navbar/Navbar.svelte";
    import "../app.css";
    import { afterNavigate } from "$app/navigation";
    import { browser } from "$app/environment";
    import { page } from "$app/stores";
    // <Navbar>
    //     <slot name="navbar" />
    // </Navbar>

    $: if (browser && window.goatcounter) {
        window.goatcounter.count({
          path: $page.url.pathname
        });
        console.log("counting", $page.url.pathname)
      }
</script>

<svelte:head>

<!-- analtyics -->
	<script 
    data-goatcounter-settings={'{"allow-local":true, "no_onload": true}'}
		data-goatcounter="https://disku.goatcounter.com/count"
		async src="//gc.zgo.at/count.js"
	></script>
</svelte:head>

<div class="main">
    <div class="slot-container content-offset">
        <slot />
    </div>
</div>

<script lang="ts">
    import Navbar from "../components/Navbar/Navbar.svelte";
    import "../app.css";
    import { afterNavigate } from "$app/navigation";
    // <Navbar>
    //     <slot name="navbar" />
    // </Navbar>
</script>

<svelte:head>

<!-- analtyics -->
    <script>
        afterNavigate(({ to }) => {
            if (window.goatcounter) {
                window.goatcounter.count({
                    path: to.path
                });
            }
        };
    </script>
	<script 
		async
		src='//gc.zgo.at/count.js'
		data-goatcounter='https://disku.goatcounter.com/count'
        data-goatcounter-settings='{"allow_local": true}'
	></script>
</svelte:head>

<div class="main">
    <div class="slot-container content-offset">
        <slot />
    </div>
</div>

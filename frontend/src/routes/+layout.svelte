<script lang="ts">
    import Navbar from "../components/Navbar/Navbar.svelte";
    import "../app.css";
    import Analytics from "../components/Analytics.svelte";

    // <Navbar>
    //     <slot name="navbar" />
    // </Navbar>
</script>

<svelte:head>

<!-- analtyics -->
	<script 
		async
		src='//gc.zgo.at/count.js'
		data-goatcounter='https://jnimela.goatcounter.com/count'
	></script>
</svelte:head>

<div class="main">
    <div class="slot-container content-offset">
        <slot />
    </div>
</div>

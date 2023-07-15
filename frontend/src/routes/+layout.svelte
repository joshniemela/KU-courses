<script lang="ts">
    import Navbar from "../components/Navbar/Navbar.svelte";
    import "../app.css";

    // <Navbar>
    //     <slot name="navbar" />
    // </Navbar>
</script>

<svelte:head>

<!-- analtyics -->
	<script 
		async
		src='//gc.zgo.at/count.js'
		data-goatcounter='https://disku.goatcounter.com/count'
	></script>
</svelte:head>

<div class="main">
    <div class="slot-container content-offset">
        <slot />
    </div>
</div>

<script lang="ts">
    import Navbar from "../components/Navbar/Navbar.svelte";
    import "../app.css";
    import Analytics from "../components/Analytics.svelte";

    // <Navbar>
    //     <slot name="navbar" />
    // </Navbar>
</script>

<Analytics />

<div class="main">
    <div class="slot-container content-offset">
        <slot />
    </div>
</div>

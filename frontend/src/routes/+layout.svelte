<script lang="ts">
    import Navbar from "../components/Navbar/Navbar.svelte";
    import "../app.css";
</script>

<div class="main">
    <Navbar>
        <slot name="navbar" />
    </Navbar>
    <div class="slot-container content-offset">
        <slot />
    </div>
</div>

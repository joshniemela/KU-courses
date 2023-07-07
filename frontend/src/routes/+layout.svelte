<script lang="ts">
    import Navbar from "../components/Navbar/Navbar.svelte";
    import "../app.css";

    // <Navbar>
    //     <slot name="navbar" />
    // </Navbar>
</script>

<div class="main">
    <div class="slot-container content-offset">
        <slot />
    </div>
</div>

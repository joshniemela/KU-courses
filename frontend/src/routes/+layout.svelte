<script>
import Navbar from "../components/Navbar/Navbar.svelte";
</script>

<div class="main">
    <Navbar />
    <div class="slot-container">
        <slot />
    </div>
</div>

<style scoped>
:global(*) {
    margin: 0;
}
.main {
    display: flex;
    flex-direction: row;
    align-items: center;
}

.slot-container {
    width: 100%;
    height: 100;
    overflow: scroll;
}
</style>

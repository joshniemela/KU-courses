<script>
import Navbar from "../components/Navbar/Navbar.svelte";
</script>

<div class="main">
    <Navbar />
    <slot />
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

</style>

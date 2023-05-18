<script>
import DbConnectionTester from "../components/DbConnectionTester/DbConnectionTester.svelte";
import Navbar from "../components/Navbar/Navbar.svelte";
import theme from "../theme.js"
</script>

<div class="main-container">
    <Navbar />
    <h1 style="--theme: {theme.colors.neutral[100]}">Velkommen til projektet bois</h1>
    <DbConnectionTester />
</div>

<style scoped>

:global(body) {
    margin: 0;
}
h1 {
    color: var(--theme);
    margin: 1vh;
}

.main-container {
    display: flex;
    flex-direction: row;
    align-items: center;
}
</style>

<script>
import theme from '../theme'
import Navbar from '../components/Navbar/Navbar.svelte';
import DbConnectionTester from '../components/DbConnectionTester/DbConnectionTester.svelte';
</script>
<div class="main-container">
    <Navbar />
    <div class="content">
        <h1 class="title" style="--font-color: {theme.colors.brand[500]}">KU Courses (WIP)</h1>
        <div class="search-container">
    <button class="filter-button">Filter</button>
            <input type="search" placeholder="Search"/>
        </div>
        <button>View all (WIP)</button>
    </div>
</div>

<style scoped>
:global(*) {
    margin: 0px;
}

.title {
    color: var(--font-color);
    margin-bottom: 2vh;
}

.content {
    display: flex;
    height: 100vh;
    width: 100%;
    margin-right: 8vw; /* should be the same as navbar-container width */
    flex-direction: column;
    align-items: center;
    justify-content: center;
}

.search-container {
    display: flex;
    flex-direction: row;
    margin-bottom: 2vh;
}

.filter-button {
    background: none;
    border: 1;
}

.main-container {
    display: flex;
    flex-direction: row;
}

</style>

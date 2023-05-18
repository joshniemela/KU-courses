<script>
import theme from "../../theme.js"
</script>
<div class="nav-container" style="--bg: {theme.colors.neutral[800]}">
    <p class="nav-title" style="--theme: {theme.colors.brand[500]}">DIKU Course planner</p>
</div>

<style scoped>
.nav-title {
    color: var(--theme)
}

.nav-container {
    background-color:var(--bg);
    height: 100vh;
    width: 15vw;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: start;
}
</style>

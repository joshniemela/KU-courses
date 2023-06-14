<script>
    import theme from "../../theme"
    import Github from "../../assets/Github.svelte";
    import Logo from "../../assets/Logo.svelte";
</script>
<div class="navbar-container" style="--bg: {theme.colors.neutral[200]}">
    <a href="/">
        <div class="title-container">
            <Logo />
            <h1 style="--font-color: {theme.colors.brand[900]}"> DISKU </h1>
        </div>
    </a>
    <a href="https://github.com/joshniemela/KU-courses" style="--color: {theme.colors.brand[900]}">
    <div class="git-container">
        <Github />
        <p class="social-text" >/KU-courses </p>
    </div>
    </a>
</div>

<style scoped>
.navbar-container {
    display: flex;
    flex-direction: column;
    background-color: var(--bg);
    height: 100vh;
    width: 8vw;
    min-width: 8vw;
    align-items: center;
    justify-content: space-between;
}

.title-container {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
}

.social-text {
    font-size: 1vw;
}
.git-container {
    display: flex;
    flex-direction: row;
    width: fit-content;
    padding: 5%;
    align-items: center;
    justify-content: center;
    margin-bottom: 1vh;
}

h1 {
    font-size: 1vw;
    color: var(--font-color);
    margin-left: 0.2vw; /* Coordinated with the logo width */
}
a {
    color: var(--color);
}

</style>

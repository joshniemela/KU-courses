<script lang="ts">
    import Github from "../../assets/Github.svelte";
    import Logo from "../../assets/Logo.svelte";
    import MenuIcon from "../../assets/MenuIcon.svelte";
    import { onMount } from "svelte";
    import { Drawer } from "flowbite-svelte";
    import { sineIn } from "svelte/easing";
    let hidden1 = true;
    let transitionParams = {
        x: -200,
        duration: 300,
        easing: sineIn,
    };
    import SearchComponent from "../../components/SearchComponent/SearchComponent.svelte";

    // check if window is defined (so if in the browser or in node.js)
    let browsableRoutes = ["/browse", "/course"];
    let isBrowser = typeof window !== "undefined";
</script>

<nav class="fixed inset-x-0 top-0 z-50 backdrop-blur-sm flex items-center">
    <a href="/">
        <div class="object-cover">
            <Logo />
        </div>
    </a>

    <div>
        <button on:click={() => (hidden1 = false)}>
            <div>
                <MenuIcon />
            </div>
        </button>
    </div>
    {#if isBrowser && browsableRoutes.includes(window.location.pathname)}
        <SearchComponent />
    {/if}
</nav>

<Drawer
    transitionType="fly"
    {transitionParams}
    bind:hidden={hidden1}
    id="sidebar1"
>
    <div class="flex items-center">
        <div>
            <a href="/">
                <p>Root placeholder</p>
            </a>
            <a href="/about">
                <p>About placeholder</p>
            </a>

            <a href="https://http.cat/">
                <p>Donation placeholder</p>
            </a>

            <a href="mailto:foo@bar.baz">
                <p>Contact placeholder</p>
            </a>

            <a href="https://github.com/joshniemela/KU-courses">
                <div class="git-container">
                    <Github />
                    <p class="social-text">/KU-courses</p>
                </div>
            </a>
        </div>
        <button on:click={() => (hidden1 = true)} class="mb-4 dark:text-white">
            Close
        </button>
    </div>
</Drawer>
<!--
<style scoped>
    .navbar-container {
        /*
            Ensures navbar will "follow" when the user scrolls. https://stackoverflow.com/a/13337664
            Content may not fit correctly unless .content-offset is used on the content.
        */
        position: fixed;
        overflow-y: hidden;
        top: 0;
        bottom: 0;
        display: flex;

        flex-direction: column;
        background-color: var(--bg);
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
-->

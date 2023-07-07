<script lang="ts">
    import Github from "../../assets/Github.svelte";
    import Logo from "../../assets/Logo.svelte";
    import MenuIcon from "../../assets/MenuIcon.svelte";
    import { Drawer } from "flowbite-svelte";
    import { sineIn } from "svelte/easing";
    let hidden1 = true;
    let transitionParams = {
        x: -200,
        duration: 300,
        easing: sineIn,
    };
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

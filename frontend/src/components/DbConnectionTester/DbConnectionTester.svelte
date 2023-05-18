<script>
import theme from '../../theme'
import { fade } from 'svelte/transition';
// Make a HTTP Request upon clicking a button
async function testConnection() {
    clicked = !clicked
    error = !error
    const response = await fetch('http://localhost:5000/postgres');
    const data = await response.json();
    console.log(data);
}

let clicked = false
$: color = clicked  ? theme.colors.neutral[800] : theme.colors.neutral[900]

let error = false

// Build up a simple animation from svelte's example
function animate(node, { duration }) {
    return {
        duration,
        css: (timer, u) => {
            if (timer <= 0.5) {
                return `transform: translateY(100px)`
            } else {
                return `transform: translateY(${100*u*2}px)`
            }
        }
    }
}

</script>
<div>
    <button on:click={testConnection} style="--theme: {color}">Test Connection</button>
    {#if error}
        <p
        class="red-error"
        style="
            --font-color: {theme.colors.red[200]};
            --bg-color: {theme.colors.red[900]}
        "
        transition:animate={{duration: 300}}
        > Something went wrong!</p>
        <p
        class="orange-error"
        style="
            --font-color: {theme.colors.orange[200]};
            --bg-color: {theme.colors.orange[900]}
        "
        > Something went wrong!</p>
    {/if}
</div>

<style scoped>
button {
    background-color: var(--theme);
    height: 2rem;
    transition: ease-in-out 0.5s;
}

.red-error {
    position: absolute;
    bottom: 0;
    right: 0;
    padding: 1vh;
    margin: 0;
    color: var(--font-color);
    background-color: var(--bg-color);
}

.orange-error {
    position: absolute;
    bottom: 0;
    left: 0;
    padding: 1vh;
    margin: 0;
    color: var(--font-color);
    background-color: var(--bg-color);
    animation: slide-up 1s ease-in-out;
}

@keyframes slide-up {
    0% {
    opacity: 0;
    transform: translateY(100px);
    }
    50% {
        opacity: 1;
        transform: translateY(100px);
    }
    100% {
    transform: translateY(0);
    }
}

</style>


<script>
import theme from '../../theme'
// Make a HTTP Request upon clicking a button
async function testConnection() {
    clicked = !clicked
    const response = await fetch('http://localhost:5000/postgres');
    const data = await response.json();
    console.log(data);
}

let clicked = false
$: color = clicked  ? theme.colors.neutral[800] : theme.colors.neutral[900]

</script>

<button on:click={testConnection} style="--theme: {color}">Test Connection</button>

<style scoped>
button {
    background-color: var(--theme);
    height: 2rem;
    transition: ease-in-out 0.5s;
}
</style>


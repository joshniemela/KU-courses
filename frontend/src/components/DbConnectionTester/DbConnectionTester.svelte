<script>
// Make a HTTP Request upon clicking a button
async function testConnection() {
    const response = await fetch('http://localhost:5000/postgres');
    const data = await response.json();
    console.log(data);
}
</script>

<button on:click={testConnection}>Test Connection</button>


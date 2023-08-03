<script lang="ts">
import GraphComponent from "../../components/GraphComponent/GraphComponent.svelte";
</script>

<div class="container">
    <h1> Graph example </h1>
    <p>A small GraphJS example 🚀</p>
    <div class="graphs">
        <div class="graph-sub-div">
            <GraphComponent />
        </div>
        <div class="graph-sub-div">
            <GraphComponent />
        </div>
    </div>
</div>


<style scoped>
.container {
    height: 100vh;
    width: 100vw;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    padding: 40px;
}

.graphs {
    height: 30vh;
    width: 100%;
    display: flex;
    flex-direction: row;
    gap: 20px;
    justify-content: center;
    align-items: center;
    flex-wrap: wrap;
}

.graph-sub-div {
    width: 500px;
    height: 100%;
}

h1 {
    font-size: 3rem;
    font-weight: 600;
}

p {
    font-size: 2rem;
    font-weight: 400;
}

</style>



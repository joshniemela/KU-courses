<script lang="ts">
import Chart, { type ChartConfiguration, type ChartItem, type DatasetChartOptions } from "chart.js/auto";
import { onMount } from "svelte";
import jsonData from "./NFYK18005U.json";
console.log(jsonData)

let graph: HTMLCanvasElement;
onMount(()=> {
const ctx = graph.getContext('2d');
// Initialize chart using default config set
if (ctx) {
    new Chart(ctx, {
        type: 'bar',
        data: {
            labels: jsonData.map(row => row.grade).reverse(),
            datasets: [
                {
                    label: "Exam grades",
                    data: jsonData.map(row => row.count).reverse(),
                }
            ]
        },
        options: {
            plugins: {
                title: {
                    display: true,
                    text: "Exam grades"
                }
            },
            maintainAspectRatio: false
        }
    });
}
})

</script>
<div class="chart-container">
    <canvas bind:this={graph} />
</div>

<style scoped>
.chart-container {
    position: relative;
    width: 100%;
    height: 100%;
}
</style>

<script lang="ts">
import Chart, { type ChartConfiguration, type ChartItem, type DatasetChartOptions } from "chart.js/auto";
import { onMount } from "svelte";
import jsonData from "./sample.json";
console.log(jsonData)

// Types
type GradeObject = {
    grade: string;
    count: number;
}

// Props
export let data: GradeObject[] = jsonData
export let legend: string = "Explainer"
export let title: string = "Title"


let graph: HTMLCanvasElement;
onMount(()=> {

// We grab the canvas context
const ctx = graph.getContext('2d');

// If the canvas context != null, we can create our bar chart
if (ctx) {
    new Chart(ctx, {
        type: 'bar',
        data: {
            // X-Axis labels
            labels: data.map(row => row.grade).reverse(),
            datasets: [
                {
                    label: legend,
                    // Student counts
                    data: data.map(row => row.count).reverse(),
                }
            ]
        },
        options: {
            // Here we can define customization and options for our chart
            plugins: {
                title: {
                    display: true,
                    text: title
                }
            },
            maintainAspectRatio: false // Important to achieve responsiveness
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
    position: relative; /* Important as otherwise it won't be responsive */
    width: 100%;
    height: 100%;
}
</style>

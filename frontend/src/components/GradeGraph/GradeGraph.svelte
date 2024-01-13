<script lang="ts">
    import Chart, {
        type ChartConfiguration,
        type ChartItem,
        type DatasetChartOptions,
    } from "chart.js/auto";
    import type { Grade } from "../../course";
    import { onMount } from "svelte";
    import jsonData from "./sample.json";
    console.log(jsonData);

    // Props
    export let data: Grade[] = jsonData;
    export let legend: string = "Explainer";
    export let title: string = "Title";

    const total = data.reduce((acc, row) => acc + row.count, 0);

    let graph: HTMLCanvasElement;
    onMount(() => {
        // We grab the canvas context
        const ctx = graph.getContext("2d");

        // If the canvas context != null, we can create our bar chart
        if (ctx) {
            new Chart(ctx, {
                type: "bar",
                data: {
                    // X-Axis labels
                    labels: data.map((row) => row.grade).reverse(),
                    datasets: [
                        {
                            label: legend,
                            // Student counts
                            data: data
                                .map((row) => row.count / total)
                                .reverse(),
                            backgroundColor: "rgba(200, 56, 60, 0.6)",
                            barPercentage: 0.4,
                        },
                    ],
                },
                options: {
                    // make a tooltip that shows !!! as wel
                    // Here we can define customization and options for our chart
                    plugins: {
                        tooltip: {
                            callbacks: {
                                // Add percentage and count to tooltips (value is in percent)
                                label: (context) => {
                                    const value = context.dataset.data[
                                        context.dataIndex
                                    ] as number;
                                    // round to whole numbers
                                    return `${(value * 100).toFixed(
                                        2,
                                    )}% (${Math.round(value * total)})`;
                                },
                            },
                        },
                        title: {
                            display: false,
                            text: title,
                        },
                    },
                    scales: {
                        y: {
                            ticks: {
                                font: {
                                    weight: "bolder",
                                },
                                format: {
                                    style: "percent",
                                },
                            },
                        },
                        x: {
                            ticks: {
                                font: {
                                    weight: "bolder",
                                },
                            },
                        },
                    },
                    maintainAspectRatio: false, // Important to achieve responsiveness
                },
            });
        }
    });
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

<script lang="ts">
    import Chart, {
        type ChartConfiguration,
        type ChartItem,
        type DatasetChartOptions,
    } from "chart.js/auto";
    import ChartDataLabels from "chartjs-plugin-datalabels";
    import type { Grade } from "../../course";
    import { onMount } from "svelte";
    import jsonData from "./sample.json";
    Chart.register(ChartDataLabels);

    interface Props {
        // Props
        data?: Grade[];
        legend?: string;
    }

    let { data = jsonData, legend = "Explainer" }: Props = $props();

    const total = data.reduce((acc, row) => acc + row.count, 0);

    let graph: HTMLCanvasElement = $state();
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
                            barPercentage: 1.0,
                        },
                    ],
                },
                options: {
                    // make a tooltip that shows !!! as wel
                    // Here we can define customization and options for our chart
                    plugins: {
                        legend: {
                            display: false,
                        },
                        tooltip: {
                            callbacks: {
                                // Add percentage and count to tooltips (value is in percent)
                                label: (context) => {
                                    const value = context.dataset.data[
                                        context.dataIndex
                                    ] as number;
                                    // round to whole numbers
                                    return `${(value * 100).toFixed(
                                        2
                                    )}% (${Math.round(value * total)})`;
                                },
                            },
                        },
                        datalabels: {
                            anchor: "end",
                            align: "end",
                            font: {
                                weight: "bold",
                                size: 14,
                            },
                            offset: -4,
                            formatter: (value, context) => {
                                return Math.round(value * total);
                            },
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
                                    size: 14,
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

<div class="h-44">
    <canvas bind:this={graph}></canvas>
</div>

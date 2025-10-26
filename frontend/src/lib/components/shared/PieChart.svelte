<script lang="ts">
	import Chart from 'chart.js/auto';
	import { onMount } from 'svelte';

	let {
		data = [],
		labels = [],
		backgroundColor = ['#4ade80', '#f87171', '##a3a3a3'],
		title = ''
	}: {
		data: number[];
		labels: string[];
		backgroundColor?: string[];
		title?: string;
	} = $props();

	let chartElement: HTMLCanvasElement;
	let chart: Chart;

	onMount(() => {
		chart = new Chart(chartElement, {
			type: 'doughnut',
			data: {
				labels,
				datasets: [
					{
						data,
						backgroundColor,
						borderWidth: 1
					}
				]
			},
			options: {
				responsive: true,
				maintainAspectRatio: false,
				plugins: {
					legend: {
						position: 'right'
					},
					title: {
						display: !!title,
						text: title
					}
				}
			}
		});

		updateChart();

		return () => {
			chart.destroy();
		};
	});

	function updateChart() {
		if (!chart || !data) return;

		chart.data.labels = labels;
		chart.data.datasets[0].data = data;
		chart.data.datasets[0].backgroundColor = backgroundColor;
		chart.update();
	}
</script>

<div class="h-64 w-full">
	<canvas bind:this={chartElement}></canvas>
</div>

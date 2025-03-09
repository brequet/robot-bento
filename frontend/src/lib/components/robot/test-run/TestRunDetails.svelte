<script lang="ts">
	import PieChart from '$lib/components/shared/PieChart.svelte';
	import SuccessRateProgressBar from '$lib/components/shared/SuccessRateProgressBar.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import * as Card from '$lib/components/ui/card/index.js';
	import { formatDate } from '$lib/services/date';
	import type { TestRunResponse } from '$lib/types/generated';
	import { AlertCircle, CalendarDays, Code } from 'lucide-svelte';

	let {
		testRun
	}: {
		testRun: TestRunResponse;
	} = $props();

	let totalStatistic = $derived(testRun.statistics.find((stat) => stat.statType === 'total'));
	let totalTests = $derived(
		totalStatistic
			? totalStatistic.passCount + totalStatistic.failCount + totalStatistic.skipCount
			: 0
	);
	let passPercentage = $derived(
		totalStatistic ? Math.round((totalStatistic.passCount / totalTests) * 100) : 0
	);
	let errorCount = $derived(testRun.errors.length);
	let chartData = $derived(
		totalStatistic
			? [totalStatistic.passCount, totalStatistic.failCount, totalStatistic.skipCount]
			: []
	);
	let chartLabels = ['Passed', 'Failed', 'Skipped'];
</script>

<div class="space-y-4">
	<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
		<Card.Root>
			<Card.Header class="pb-2">
				<Card.Title class="text-xl font-semibold">Test Run Information</Card.Title>
			</Card.Header>
			<Card.Content>
				<div class="space-y-4">
					<div class="flex items-center space-x-2">
						<Badge variant="outline" class="font-medium">{testRun.id}</Badge>
						{#if testRun.rpa}
							<Badge class="bg-purple-500">RPA</Badge>
						{/if}
					</div>

					<div class="grid grid-cols-1 gap-3">
						<div class="flex items-center text-sm">
							<Code class="mr-2 h-4 w-4 text-gray-500" />
							<span class="w-24 font-medium text-gray-700">Generator:</span>
							<span class="text-gray-900">{testRun.generator}</span>
						</div>

						<div class="flex items-center text-sm">
							<CalendarDays class="mr-2 h-4 w-4 text-gray-500" />
							<span class="w-24 font-medium text-gray-700">Generated:</span>
							<span class="text-gray-900">{formatDate(testRun.generatedDate)}</span>
						</div>

						<div class="flex items-center text-sm">
							<CalendarDays class="mr-2 h-4 w-4 text-gray-500" />
							<span class="w-24 font-medium text-gray-700">Imported:</span>
							<span class="text-gray-900">{formatDate(testRun.importedDate)}</span>
						</div>
					</div>
				</div>
			</Card.Content>
		</Card.Root>

		<Card.Root>
			<Card.Header class="pb-2">
				<Card.Title class="text-xl font-semibold">Test Results</Card.Title>
			</Card.Header>
			<Card.Content>
				{#if totalStatistic}
					<div class="space-y-4">
						<div class="flex items-center justify-between">
							<div class="text-2xl font-bold">{passPercentage}%</div>
							<div class="text-sm text-gray-500">Pass Rate</div>
						</div>

						<SuccessRateProgressBar
							passedCount={totalStatistic.passCount}
							skippedCount={totalStatistic.skipCount}
							failedCount={totalStatistic.failCount}
						/>
					</div>
				{:else}
					<div class="text-gray-500">No test statistics available</div>
				{/if}
			</Card.Content>
		</Card.Root>
	</div>

	<Card.Root>
		<Card.Header class="pb-2">
			<Card.Title class="text-xl font-semibold">Test Distribution</Card.Title>
		</Card.Header>
		<Card.Content>
			<div class="grid grid-cols-1 items-center gap-6 md:grid-cols-2">
				<div>
					{#if totalStatistic}
						<PieChart data={chartData} labels={chartLabels} />
					{:else}
						<div class="mx-auto flex h-64 w-64 items-center justify-center">
							<span class="text-gray-500">No data available</span>
						</div>
					{/if}
				</div>

				<div>
					<div class="space-y-4">
						<div class="flex items-center space-x-3 rounded-lg bg-amber-50 p-4">
							<AlertCircle class="h-6 w-6 text-amber-500" />
							<div>
								<div class="text-sm font-medium text-amber-700">Errors Found</div>
								<div class="text-2xl font-bold">{errorCount}</div>
							</div>
						</div>

						<div class="space-y-1">
							<div class="flex items-center space-x-2">
								<div class="h-3 w-3 rounded-full bg-green-500"></div>
								<span class="text-sm"
									>Passed: {totalStatistic ? totalStatistic.passCount : 0} tests</span
								>
							</div>
							<div class="flex items-center space-x-2">
								<div class="h-3 w-3 rounded-full bg-red-500"></div>
								<span class="text-sm"
									>Failed: {totalStatistic ? totalStatistic.failCount : 0} tests</span
								>
							</div>
							{#if totalStatistic && totalStatistic.skipCount > 0}
								<div class="flex items-center space-x-2">
									<div class="h-3 w-3 rounded-full bg-gray-400"></div>
									<span class="text-sm">Skipped: {totalStatistic.skipCount} tests</span>
								</div>
							{/if}
						</div>

						{#if testRun.suites && testRun.suites.length > 0}
							<div class="mt-4 text-sm font-medium text-gray-700">
								Total Suites: {testRun.suites.length}
							</div>
						{/if}
					</div>
				</div>
			</div>
		</Card.Content>
	</Card.Root>
</div>

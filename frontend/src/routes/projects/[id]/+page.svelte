<script lang="ts">
	import { page } from '$app/state';
	import SuccessRateProgressBar from '$lib/components/shared/SuccessRateProgressBar.svelte';
	import * as Alert from '$lib/components/ui/alert/index.js';
	import Badge from '$lib/components/ui/badge/badge.svelte';
	import * as Card from '$lib/components/ui/card/index.js';
	import { formatDate as prettyFormatDate } from '$lib/services/date';
	import { getProjectById } from '$lib/services/projects';
	import type { ProjectResponse } from '$lib/types/generated';
	import { onMount } from 'svelte';

	let project: ProjectResponse | null = $state(null);
	let error: string | null = $state(null);

	onMount(async () => {
		project = await getProjectById(page.params.id);
		if (!project) error = 'Failed to load project details.';
	});

	let lastRun = $derived((project as ProjectResponse | null)?.testRunsSummaries[0]);
	let passRate = $derived(lastRun ? (lastRun.passedTests / lastRun.totalTests) * 100 : 0);
</script>

<main class="bg-background min-h-screen px-6 py-12">
	{#if error}
		<Alert.Root variant="destructive" class="mb-6">
			<Alert.Description>{error}</Alert.Description>
		</Alert.Root>
	{/if}

	{#if project}
		<div class="space-y-6">
			<div class="flex items-center justify-between">
				<div>
					<h1 class="text-3xl font-bold">{project.name}</h1>
					<p class="text-muted-foreground">Created {prettyFormatDate(project.createDate)}</p>
				</div>
				<div class="text-right">
					<p class="text-2xl font-semibold">{project.testRunCount}</p>
					<p class="text-muted-foreground">Total Test Runs</p>
				</div>
			</div>

			{#if lastRun}
				<a class="block" href={`/test-run/${lastRun.testRunId}`}>
					<Card.Root class="duration p-4 transition-all hover:bg-gray-50 hover:shadow-sm">
						<Card.Header>
							<Card.Title>Latest Test Run Summary</Card.Title>
						</Card.Header>
						<Card.Content>
							<div class="flex justify-between">
								<div class="col-span-2">
									<p class="text-muted-foreground text-sm">Run Date</p>
									<p class="text-2xl font-semibold">{prettyFormatDate(lastRun.testRunDate)}</p>
								</div>
								<div>
									<p class="text-muted-foreground text-sm">Duration</p>
									<p class="text-2xl font-semibold">{lastRun.elapsedTime}</p>
								</div>
								<div>
									<p class="text-muted-foreground text-sm">App Version</p>
									<p class="text-2xl font-semibold">{lastRun.appVersion}</p>
								</div>
								<div>
									<p class="text-muted-foreground text-sm">Errors</p>
									<p class="text-2xl font-semibold">{lastRun.errorCount}</p>
								</div>
								<div class="col-span-2">
									<div class="flex justify-between">
										<p class="text-muted-foreground text-sm">Pass Rate</p>
										<p class="text-foreground text-sm">{passRate.toFixed(1)}%</p>
									</div>
									<div class="mt-2">
										<SuccessRateProgressBar
											passedCount={lastRun.passedTests}
											failedCount={lastRun.failedTests}
											skippedCount={lastRun.skippedTests}
										/>
									</div>
								</div>
							</div>
						</Card.Content>
					</Card.Root>
				</a>

				<Card.Root>
					<Card.Header>
						<Card.Title>Test Run History</Card.Title>
					</Card.Header>
					<Card.Content>
						<div class="space-y-4">
							{#each project.testRunsSummaries as run}
								<a
									class="grid grid-cols-[minmax(150px,auto)_minmax(100px,auto)_auto_auto] items-center gap-4 rounded-lg border p-4 transition-all duration-300 hover:bg-gray-50 hover:shadow-sm active:scale-95"
									href={`/test-run/${run.testRunId}`}
								>
									<p class="font-medium">{prettyFormatDate(run.testRunDate)}</p>
									<Badge class="justify-self-start">{run.appVersion}</Badge>
									<div class="text-right">
										<p class="text-muted-foreground text-sm">Duration</p>
										<p class="font-medium">{run.elapsedTime}</p>
									</div>
									<SuccessRateProgressBar
										passedCount={run.passedTests}
										failedCount={run.failedTests}
										skippedCount={run.skippedTests}
									/>
								</a>
							{/each}
						</div>
					</Card.Content>
				</Card.Root>
			{/if}
		</div>
	{/if}
</main>

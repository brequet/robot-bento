<script lang="ts">
	import { page } from '$app/state';
	import TestRunProgress from '$lib/components/project/TestRunProgress.svelte';
	import * as Alert from '$lib/components/ui/alert/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { getProjectById } from '$lib/services/projects';
	import type { ProjectResponse } from '$lib/types/generated';
	import { onMount } from 'svelte';

	let project: ProjectResponse | null = $state(null);
	let error: string | null = $state(null);

	onMount(async () => {
		project = await getProjectById(page.params.id);
		if (!project) error = 'Failed to load project details.';
	});

	let lastRun = $derived(project?.testRunsSummaries[0]);
	let passRate = $derived(lastRun ? (lastRun.passedTests / lastRun.totalTests) * 100 : 0);

	function formatDate(dateString: string) {
		const date = new Date(dateString);
		const months = [
			'Jan',
			'Feb',
			'Mar',
			'Apr',
			'May',
			'Jun',
			'Jul',
			'Aug',
			'Sep',
			'Oct',
			'Nov',
			'Dec'
		];
		const month = months[date.getMonth()];
		const day = date.getDate();
		const year = date.getFullYear();
		const hours = date.getHours().toString().padStart(2, '0');
		const minutes = date.getMinutes().toString().padStart(2, '0');
		return `${month} ${day}, ${year} at ${hours}:${minutes}`;
	}
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
					<p class="text-muted-foreground">Created {formatDate(project.createDate)}</p>
				</div>
				<div class="text-right">
					<p class="text-2xl font-semibold">{project.testRunCount}</p>
					<p class="text-muted-foreground">Total Test Runs</p>
				</div>
			</div>

			{#if lastRun}
				<Card.Root>
					<Card.Header>
						<Card.Title>Latest Test Run Summary</Card.Title>
					</Card.Header>
					<Card.Content>
						<div class="flex justify-between">
							<div class="col-span-2">
								<p class="text-muted-foreground text-sm">Run Date</p>
								<p class="text-2xl font-semibold">{formatDate(lastRun.testRunDate)}</p>
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
									<TestRunProgress
										passedCount={lastRun.passedTests}
										failedCount={lastRun.failedTests}
										skippedCount={lastRun.skippedTests}
									/>
								</div>
							</div>
						</div>
					</Card.Content>
				</Card.Root>

				<Card.Root>
					<Card.Header>
						<Card.Title>Test Run History</Card.Title>
					</Card.Header>
					<Card.Content>
						<div class="space-y-4">
							{#each project.testRunsSummaries as run}
								<div class="flex items-center justify-between rounded-lg border p-4">
									<div>
										<p class="font-medium">{formatDate(run.testRunDate)}</p>
										<p class="text-muted-foreground text-sm">Version {run.appVersion}</p>
									</div>
									<div class="flex items-center gap-4">
										<div class="text-right">
											<p class="text-muted-foreground text-sm">Duration</p>
											<p class="font-medium">{run.elapsedTime}</p>
										</div>
										<div class="text-right">
											<TestRunProgress
												passedCount={run.passedTests}
												failedCount={run.failedTests}
												skippedCount={run.skippedTests}
											/>
										</div>
									</div>
								</div>
							{/each}
						</div>
					</Card.Content>
				</Card.Root>
			{/if}
		</div>
	{/if}
</main>

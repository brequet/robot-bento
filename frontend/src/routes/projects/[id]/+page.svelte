<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { getProjectById } from '$lib/services/projects';
	import type { Project } from '$lib/types/project';
	import { Alert, AlertTitle, AlertDescription } from '$lib/components/ui/alert';
	import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Progress } from '$lib/components/ui/progress';
	import type { Stat } from '$lib/types/robot';

	let project: Project | null = null;
	let error: string | null = null;

	$: projectId = $page.params.id;

	onMount(async () => {
		project = await getProjectById(projectId);
		if (!project) error = 'Failed to load project details.';
	});

	function getLatestStats(stats: Stat[]) {
		return stats.find((s) => s.stat_type === 'Total') ?? null;
	}

	function calculatePassRate(stats: Stat) {
		const total = stats.pass_count + stats.fail_count + stats.skip_count;
		return total > 0 ? (stats.pass_count / total) * 100 : 0;
	}

	function formatDate(dateStr: string) {
		return new Date(dateStr).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}
</script>

<main class="bg-background min-h-screen px-6 py-12">
	<div class="mx-auto max-w-6xl space-y-6">
		{#if error}
			<Alert variant="destructive">
				<AlertTitle>Error</AlertTitle>
				<AlertDescription>{error}</AlertDescription>
			</Alert>
		{:else if project}
			<div class="flex items-center justify-between">
				<h1 class="text-4xl font-bold tracking-tight">{project.name}</h1>
				<Badge variant="outline">
					{project.test_run_count} Test Runs
				</Badge>
			</div>

			{#if project.test_runs.length > 0}
				{@const latestRun = project.test_runs[0]}
				{@const stats = getLatestStats(latestRun.statistics)}

				<h2 class="text-2xl font-bold">Latest Run</h2>

				<div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
					<Card>
						<CardHeader>
							<CardTitle>Overview</CardTitle>
						</CardHeader>
						<CardContent>
							<div class="space-y-4">
								<div>
									<p class="text-muted-foreground text-sm">Generated</p>
									<p class="text-lg font-medium">{formatDate(latestRun.generated_date)}</p>
								</div>
								<div>
									<p class="text-muted-foreground text-sm">Application</p>
									<p class="text-lg font-medium">{latestRun.app_name}@{latestRun.app_version}</p>
								</div>
							</div>
						</CardContent>
					</Card>

					{#if stats}
						<Card>
							<CardHeader>
								<CardTitle>Test Results</CardTitle>
							</CardHeader>
							<CardContent>
								<div class="space-y-4">
									<Progress value={calculatePassRate(stats)} />
									<div class="grid grid-cols-3 gap-4 text-center">
										<div>
											<p class="text-muted-foreground text-sm">Passed</p>
											<p class="text-lg font-medium text-green-600">{stats.pass_count}</p>
										</div>
										<div>
											<p class="text-muted-foreground text-sm">Failed</p>
											<p class="text-lg font-medium text-red-600">{stats.fail_count}</p>
										</div>
										<div>
											<p class="text-muted-foreground text-sm">Skipped</p>
											<p class="text-lg font-medium text-yellow-600">{stats.skip_count}</p>
										</div>
									</div>
								</div>
							</CardContent>
						</Card>
					{/if}

					<Card>
						<CardHeader>
							<CardTitle>Environment</CardTitle>
						</CardHeader>
						<CardContent>
							<div class="space-y-4">
								<div>
									<p class="text-muted-foreground text-sm">RPA</p>
									<p class="text-lg font-medium">{latestRun.rpa ? 'Yes' : 'No'}</p>
								</div>
								<div>
									<p class="text-muted-foreground text-sm">Schema Version</p>
									<p class="text-lg font-medium">{latestRun.schema_version}</p>
								</div>
							</div>
						</CardContent>
					</Card>
				</div>
			{/if}

			<Card>
				<CardHeader>
					<CardTitle>Test Runs</CardTitle>
				</CardHeader>
				<CardContent>
					<div class="space-y-4">
						{#each project.test_runs as testRun}
							<div
								class="hover:bg-muted/50 flex items-center justify-between rounded-lg border p-4"
							>
								<div>
									<p class="font-medium">{formatDate(testRun.generated_date)}</p>
								</div>
							</div>
						{/each}
					</div>
				</CardContent>
			</Card>
		{/if}
	</div>
</main>

<style>
	:global(.badge-success) {
		@apply bg-green-100 text-green-800 hover:bg-green-100/80;
	}
</style>

<script lang="ts">
	import SuccessRateProgressBar from '$lib/components/shared/SuccessRateProgressBar.svelte';
	import * as Card from '$lib/components/ui/card/index.js';
	import { formatDate } from '$lib/services/date';
	import type { ProjectOverviewResponse } from '$lib/types/generated';

	export let project: ProjectOverviewResponse;

	let pass_rate =
		(project.lastTestRunSummary?.totalTests ?? 0) > 0
			? Math.round(
					(project.lastTestRunSummary!.passedTests / project.lastTestRunSummary!.totalTests) * 100
				)
			: 0;
</script>

<a href={`/projects/${project.id}`}>
	<Card.Root
		class="h-full w-full cursor-pointer p-4 transition-all duration-300 hover:bg-gray-50 hover:shadow-lg active:scale-95"
	>
		<Card.Header>
			<Card.Title class="text-lg font-semibold">{project.name}</Card.Title>
		</Card.Header>

		<Card.Content class="space-y-3">
			<div class="text-sm text-gray-500">
				<span class="font-medium">{project.testRunCount}</span> runs saved
			</div>

			<div class="space-y-1 text-sm text-gray-500">
				Last Run:
				{#if project.lastTestRunSummary}
					<span class="font-medium">
						{formatDate(project.lastTestRunSummary.testRunDate)}
						<br />(ran for {project.lastTestRunSummary.elapsedTime})
					</span>
					<div>
						<div class="flex justify-between text-xs text-gray-600">
							<span>Status</span>
							<span>{pass_rate}% Passed</span>
						</div>
						<SuccessRateProgressBar
							passedCount={project.lastTestRunSummary.passedTests}
							failedCount={project.lastTestRunSummary.failedTests}
							skippedCount={project.lastTestRunSummary.skippedTests}
						/>
					</div>
				{:else}
					<span class="text-gray-400">No Runs</span>
				{/if}
			</div>
		</Card.Content>
	</Card.Root>
</a>

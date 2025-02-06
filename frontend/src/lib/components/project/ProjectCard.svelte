<script lang="ts">
	import { goto } from '$app/navigation';
	import * as Card from '$lib/components/ui/card/index.js';
	import { Progress } from '$lib/components/ui/progress/index.js';
	import type { ProjectOverviewResponse } from '$lib/types/project';

	export let project: ProjectOverviewResponse;

	let pass_rate =
		(project.lastTestRunSummary?.totalTests ?? 0) > 0
			? Math.round(
					(project.lastTestRunSummary!.passedTests / project.lastTestRunSummary!.totalTests) * 100
				)
			: 0;

	function navigateToProject() {
		goto(`/projects/${project.id}`);
	}
</script>

<Card.Root
	class="cursor-pointer p-4 transition-all duration-300 hover:bg-gray-50 hover:shadow-lg active:scale-95"
	onclick={navigateToProject}
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
				<span class="font-medium"
					>{new Date(project.lastTestRunSummary.lastTestRunDate).toLocaleDateString()}</span
				>
				<div>
					<div class="flex justify-between text-xs text-gray-600">
						<span>Status</span>
						<span>{pass_rate}% Passed</span>
					</div>
					<Progress value={pass_rate} class="h-2 bg-gray-200" />
				</div>
			{:else}
				<span class="text-gray-400">No Runs</span>
			{/if}
		</div>
	</Card.Content>
</Card.Root>

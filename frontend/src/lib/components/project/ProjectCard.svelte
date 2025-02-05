<script lang="ts">
	import type { ProjectOverview } from '$lib/types/project';
	import { Progress } from '$lib/components/ui/progress/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { goto } from '$app/navigation';

	export let project: ProjectOverview;

	let pass_rate =
		(project.last_test_run?.total_tests ?? 0) > 0
			? Math.round((project.last_test_run!.passed_tests / project.last_test_run!.total_tests) * 100)
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
			<span class="font-medium">{project.test_run_count}</span> runs saved
		</div>

		<div class="space-y-1 text-sm text-gray-500">
			Last Run:
			{#if project.last_test_run}
				<span class="font-medium"
					>{new Date(project.last_test_run.last_test_run_date).toLocaleDateString()}</span
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

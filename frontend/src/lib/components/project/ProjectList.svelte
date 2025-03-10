<script lang="ts">
	import type { ProjectOverviewResponse } from '$lib/types/generated';
	import * as Card from '$lib/components/ui/card/index.js';
	import ProjectCard from './ProjectCard.svelte';

	export let projects: ProjectOverviewResponse[];
</script>

<div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
	{#if projects.length === 0}
		<Card.Root class="h-full w-full p-4">
			<Card.Header>
				<Card.Title class="text-lg font-semibold">No projects found</Card.Title>
			</Card.Header>

			<Card.Content class="space-y-3">
				<div class="text-sm text-gray-500">
					No projects were found. Create a new project to get started.
				</div>
			</Card.Content>
		</Card.Root>
	{:else}
		{#each projects as project}
			<ProjectCard {project} />
		{/each}
	{/if}
</div>

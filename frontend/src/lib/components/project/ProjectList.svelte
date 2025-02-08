<script lang="ts">
	import type { ProjectOverviewResponse } from '$lib/types/generated';
	import ProjectCard from './ProjectCard.svelte';

	export let projects: ProjectOverviewResponse[];
</script>

<div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
	{#each projects as project}
		<ProjectCard {project} />
	{/each}
</div>

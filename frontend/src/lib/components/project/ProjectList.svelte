<script lang="ts">
	import type { ProjectOverview } from '$lib/types/project';
	import ProjectCard from './ProjectCard.svelte';

	export let projects: ProjectOverview[];
</script>

<div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
	{#each projects as project}
		<ProjectCard {project} />
	{/each}
</div>

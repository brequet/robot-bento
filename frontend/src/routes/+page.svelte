<script lang="ts">
	import ProjectList from '$lib/components/project/ProjectList.svelte';
	import { getAllProjects } from '$lib/services/projects';
	import type { ProjectOverviewResponse } from '$lib/types/generated';
	import { onMount } from 'svelte';

	let projects: ProjectOverviewResponse[] = [];
	let error: string | null = null;

	onMount(async () => {
		try {
			projects = await getAllProjects();
		} catch (err) {
			error = 'Failed to load projects.';
		}
	});
</script>

<main class="min-h-screen bg-gray-50 py-10">
	<div class="mx-auto max-w-5xl px-4">
		<h1 class="mb-6 text-3xl font-bold">Projects</h1>

		{#if error}
			<p class="text-red-500">{error}</p>
		{:else}
			<ProjectList {projects} />
		{/if}
	</div>
</main>

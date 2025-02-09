<script lang="ts">
	import { page } from '$app/state';
	import * as Alert from '$lib/components/ui/alert/index.js';
	import { getTestRunById } from '$lib/services/robot';
	import type { TestRun } from '$lib/types/robot';
	import { onMount } from 'svelte';

	let testRun: TestRun | null = $state(null);
	let error: string | null = $state(null);

	onMount(async () => {
		testRun = await getTestRunById(page.params.id);
		if (!testRun) error = 'Failed to load project details.';
	});
</script>

<main class="bg-background min-h-screen px-6 py-12">
	{#if error}
		<Alert.Root variant="destructive" class="mb-6">
			<Alert.Description>{error}</Alert.Description>
		</Alert.Root>
	{/if}

	{JSON.stringify(testRun)}
</main>

<script lang="ts">
	import { page } from '$app/state';
	import * as Alert from '$lib/components/ui/alert/index.js';
	import { onMount } from 'svelte';

	let error: string | null = $state(null);

	let testRunId = page.params.id;

	onMount(async () => {});
</script>

<main class="bg-background min-h-screen px-6 py-12">
	{#if error}
		<Alert.Root variant="destructive" class="mb-6">
			<Alert.Description>{error}</Alert.Description>
		</Alert.Root>
	{/if}

	{testRunId}
</main>

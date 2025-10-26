<script lang="ts">
	import type { ApiSuite, ApiTest } from '$lib/types/generated';

	let {
		breadcrumbs,
		handleElementSelect
	}: {
		breadcrumbs: Array<{ name: string; element: ApiSuite | ApiTest }>;
		handleElementSelect: (suite: ApiSuite | ApiTest) => void;
	} = $props();
</script>

<!-- TODO: can use shadcn breadcrumb component -->
<div class="text-muted-foreground flex text-sm">
	{#each breadcrumbs as { name, element }, i}
		<button class="hover:text-primary" onclick={() => handleElementSelect(element)}>
			{name}
		</button>
		{#if i < breadcrumbs.length - 1}
			<span>.</span>
		{/if}
	{/each}
</div>

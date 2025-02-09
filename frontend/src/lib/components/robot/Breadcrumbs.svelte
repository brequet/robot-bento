<script lang="ts">
	let { path, onNavigate } = $props<{
		path: { name: string; id: number }[];
		onNavigate: (id: number) => void;
	}>();
</script>

<nav class="text-sm text-gray-600">
	{#each path as { name, id }, i}
		<span class="cursor-pointer hover:underline" onclick={() => onNavigate(id)}>
			{name}
		</span>
		{#if i < path.length - 1}
			<span class="mx-1">/</span>
		{/if}
	{/each}
</nav>

<script lang="ts">
	let { tags }: { tags: string[] } = $props();
</script>

{#if tags.length > 0}
	<div class="flex space-x-2 text-sm">
		<span class="font-semibold">Tags:</span>
		<span>{tags.join(', ')}</span>
	</div>
{/if}

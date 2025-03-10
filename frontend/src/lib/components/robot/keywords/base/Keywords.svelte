<script lang="ts">
	import type { RobotBaseBody } from '$lib/types/robot';
	import BaseBody from '../BaseBody.svelte';

	let {
		keywords,
		shouldOpenOnlyLastKeyword = false
	}: {
		keywords: RobotBaseBody[];
		shouldOpenOnlyLastKeyword?: boolean;
	} = $props();
</script>

{#if keywords.length > 0}
	<div class="space-y-1">
		{#each keywords as subKeyword, i}
			{#if shouldOpenOnlyLastKeyword && i !== keywords.length - 1}
				<BaseBody baseBody={subKeyword} preventUnwrap={true} />
			{:else}
				<BaseBody baseBody={subKeyword} />
			{/if}
		{/each}
	</div>
{/if}

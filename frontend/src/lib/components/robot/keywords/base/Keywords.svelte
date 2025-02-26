<script lang="ts">
	import type { RobotBaseBody } from '$lib/types/robot';
	import BaseBody from '../baseBody.svelte';

	let { keywords }: { keywords: RobotBaseBody[] } = $props();
</script>

{#if keywords.length > 0}
	<div class="space-y-1">
		{#each keywords as subKeyword}
			<BaseBody baseBody={subKeyword} />
		{/each}
	</div>
{/if}

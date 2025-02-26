<script lang="ts">
	import type { RobotIf } from '$lib/types/robot';
	import Keywords from './base/Keywords.svelte';
	import GenericKeyword from './GenericKeyword.svelte';

	let { ifKw }: { ifKw: RobotIf } = $props();
</script>

{#each ifKw.branches as branch}
	<GenericKeyword
		name={`${branch.type_} ${branch.condition}`}
		type={branch.type_}
		status={branch.status}
	>
		{#snippet markupName()}
			<div class="text-left">
				<span>{branch.type_}</span>
				{#if branch.condition}
					<span class="font-semibold">{branch.condition}</span>
				{/if}
			</div>
		{/snippet}

		<Keywords keywords={branch.children} />
	</GenericKeyword>
{/each}

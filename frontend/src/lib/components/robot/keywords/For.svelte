<script lang="ts">
	import type { RobotFor } from '$lib/types/robot';
	import GenericKeyword from './GenericKeyword.svelte';
	import IterationItem from './IterationItem.svelte';

	let { forKw }: { forKw: RobotFor } = $props();

	function getKeywordName(forKw: RobotFor) {
		if (forKw.flavor === 'IN') {
			const args = forKw.vars.map((v) => v).join(', ');
			const values = forKw.values.map((v) => v).join(', ');
			return `${args} IN ${values}`;
		}
	}
</script>

<GenericKeyword name={getKeywordName(forKw)} type={'FOR'} status={forKw.status}>
	<div class="space-y-1">
		{#each forKw.iters as iter}
			<IterationItem {iter} />
		{/each}
	</div>
</GenericKeyword>

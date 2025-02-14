<script lang="ts">
	import type { RobotIf } from '$lib/types/robot';
	import Keywords from '../base/Keywords.svelte';
	import GenericKeyword from './GenericKeyword.svelte';

	let { ifKw }: { ifKw: RobotIf } = $props();
</script>

{#each ifKw.branches as branch}
	<GenericKeyword
		name={`${branch.type_} ${branch.condition}`}
		type={branch.type_}
		status={branch.status}
	>
		<Keywords keywords={branch.children} />
	</GenericKeyword>
{/each}

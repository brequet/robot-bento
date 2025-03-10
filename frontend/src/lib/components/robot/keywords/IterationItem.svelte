<script lang="ts">
	import type { RobotForIter } from '$lib/types/robot';
	import Keywords from './base/Keywords.svelte';
	import GenericKeyword from './GenericKeyword.svelte';

	let { iter }: { iter: RobotForIter } = $props();

	function getKeywordName(iter: RobotForIter) {
		return iter.vars.map((v) => `${v.name} = ${v.value}`).join(', ');
	}
</script>

<GenericKeyword type={'ITERATION'} name={getKeywordName(iter)} status={iter.status}>
	<Keywords keywords={iter.children} />
</GenericKeyword>

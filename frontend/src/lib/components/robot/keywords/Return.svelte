<script lang="ts">
	import type { RobotReturn } from '$lib/types/robot';
	import StatusTime from './base/StatusTime.svelte';
	import GenericKeyword from './GenericKeyword.svelte';

	let { returnKw }: { returnKw: RobotReturn } = $props();
</script>

<GenericKeyword name={returnKw.value.join(', ')} type={'RETURN'} status={returnKw.status}>
	<StatusTime status={returnKw.status} />
</GenericKeyword>

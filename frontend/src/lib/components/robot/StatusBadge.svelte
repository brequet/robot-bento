<script lang="ts">
	import { Badge } from '$lib/components/ui/badge/index.js';
	import clsx from 'clsx';

	let { status, type }: { status: string; type: 'SUITE' | 'TEST' } = $props();
</script>

<Badge
	class={clsx(
		'px-2 ',
		status === 'FAIL' && 'bg-red-400 hover:bg-red-500',
		status === 'PASS' && 'bg-green-400 hover:bg-green-500',
		status === 'SKIP' && 'bg-ÿellow-400 hover:bg-ÿellow-500'
	)}
	style="font-size:0.65rem"
>
	{type}
</Badge>

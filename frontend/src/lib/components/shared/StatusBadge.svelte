<script lang="ts">
	import { Badge } from '$lib/components/ui/badge/index.js';
	import clsx from 'clsx';

	let { status, text }: { status: string; text: string } = $props();
</script>

<div>
	<Badge
		class={clsx(
			'px-2 font-semibold text-white',
			status === 'FAIL' && 'bg-red-400 hover:bg-red-500',
			status === 'PASS' && 'bg-green-400 hover:bg-green-500',
			status === 'SKIP' && 'bg-yellow-400 hover:bg-yellow-500',
			status === 'NOT RUN' && 'bg-neutral-400 hover:bg-neutral-500'
		)}
		style="font-size:0.65rem"
	>
		{text}
	</Badge>
</div>

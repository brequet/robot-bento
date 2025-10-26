<script lang="ts">
	import type { RobotMessage } from '$lib/types/robot';
	import clsx from 'clsx';

	let { message }: { message: RobotMessage } = $props();
</script>

<li class="flex flex-col gap-1 px-4 pb-0 pt-3">
	<div class="flex items-start justify-between">
		<span class="text-muted-foreground font-mono text-sm">
			{message.timestamp}
		</span>
		<span
			class={clsx('rounded-full px-2 py-0.5 text-xs font-medium', {
				'bg-blue-100 text-blue-700': message.level === 'INFO',
				'bg-red-100 text-red-700': message.level === 'ERROR' || message.level === 'FAIL',
				'bg-yellow-100 text-yellow-700': message.level === 'WARN'
			})}
		>
			{message.level}
		</span>
	</div>
	<p class="whitespace-pre-wrap font-mono text-xs">{message.value}</p>
</li>

<script lang="ts">
	import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
	import * as Accordion from '$lib/components/ui/accordion/index.js';
	import { formatRobotElapsedTime } from '$lib/services/date';
	import type { RobotStatus } from '$lib/types/robot';

	let {
		name,
		type,
		status,
		children
	}: {
		name: string;
		type: string | undefined;
		status: RobotStatus | undefined;
		children: () => any;
	} = $props();
</script>

<Accordion.Root type="single" class="rounded-lg border p-2 text-sm">
	<Accordion.Item value="item-1" class="border-b-0">
		<Accordion.Trigger class="py-2">
			<div class="flex w-full flex-row items-center justify-between space-x-2">
				<span class="text-left font-medium">{name}</span>
				{#if status}
					<div class="flex items-center space-x-2">
						<span>
							{formatRobotElapsedTime(status.start_time, status.end_time)}
						</span>
						<StatusBadge status={status.status} text={type ?? 'KEYWORD'} />
					</div>
				{/if}
			</div>
		</Accordion.Trigger>
		<Accordion.Content>
			<div class="ml-2 space-y-1 text-base">
				{@render children?.()}
			</div>
		</Accordion.Content>
	</Accordion.Item>
</Accordion.Root>

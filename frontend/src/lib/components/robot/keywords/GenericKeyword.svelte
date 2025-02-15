<script lang="ts">
	import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
	import * as Accordion from '$lib/components/ui/accordion/index.js';
	import { formatRobotElapsedTime } from '$lib/services/date';
	import type { RobotStatus } from '$lib/types/robot';
	import type { Snippet } from 'svelte';

	let {
		name,
		markupName,
		type,
		status,
		children
	}: {
		name?: string;
		markupName?: Snippet;
		type?: string;
		status?: RobotStatus;
		children: Snippet;
	} = $props();
</script>

<Accordion.Root type="single" class="rounded-lg border p-2 text-sm">
	<Accordion.Item value="item-1" class="border-b-0">
		<Accordion.Trigger class="py-2 ">
			<div class="flex w-full flex-row justify-between space-x-2">
				<div class="flex space-x-2 no-underline">
					{#if status}
						<StatusBadge status={status!.status} text={type ?? 'KEYWORD'} />
					{/if}

					{#if markupName}
						{@render markupName()}
					{:else}
						<span class="text-left font-medium">{name}</span>
					{/if}
				</div>
				{#if status}
					<span>
						{formatRobotElapsedTime(status.start_time, status.end_time)}
					</span>
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

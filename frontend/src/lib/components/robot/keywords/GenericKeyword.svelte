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
		children,
		preventUnwrap = false
	}: {
		name?: string;
		markupName?: Snippet;
		type?: string;
		status?: RobotStatus;
		children: Snippet;
		preventUnwrap?: boolean;
	} = $props();

	let isOpen = $state((!preventUnwrap && status && status.status === 'FAIL') || false);
</script>

<Accordion.Root
	type="single"
	class="rounded-lg border p-2 py-1 text-sm"
	value={status && status.status === 'FAIL' ? 'item-1' : undefined}
	onValueChange={(value) => (isOpen = value === 'item-1')}
>
	<Accordion.Item value="item-1" class="border-b-0">
		<Accordion.Trigger class="py-2">
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
			{#if isOpen}
				<div class="ml-2 space-y-1 text-base">
					{@render children?.()}
				</div>
			{/if}
		</Accordion.Content>
	</Accordion.Item>
</Accordion.Root>

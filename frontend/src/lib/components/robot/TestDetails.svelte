<script lang="ts">
	import * as Card from '$lib/components/ui/card/index.js';
	import { formatDate, formatElapsedTime } from '$lib/services/date';
	import type { ApiTest } from '$lib/types/generated';
	import { Clock, FileText, FolderTree } from 'lucide-svelte';
	import StatusBadge from './StatusBadge.svelte';

	let { test }: { test: ApiTest } = $props();
</script>

<Card.Root class="h-full w-full">
	<Card.Header>
		<div class="flex items-center justify-between">
			<div class="flex items-center gap-2">
				<FolderTree class="text-muted-foreground h-5 w-5" />
				<Card.Title>{test.name}</Card.Title>
			</div>
			<StatusBadge status={test.status} type={'TEST'} />
		</div>
		<Card.Description>ID: {test.identifier}</Card.Description>
	</Card.Header>

	<Card.Content>
		<div class="space-y-6">
			<div class="grid grid-cols-3 gap-4">
				<div class="flex items-center justify-center gap-2">
					<Clock class="text-muted-foreground h-4 w-4" />
					<div class="text-sm">
						<div class="font-medium">Start Time</div>
						<div class="text-muted-foreground">{formatDate(test.startTime)}</div>
					</div>
				</div>
				<div class="flex items-center justify-center gap-2">
					<Clock class="text-muted-foreground h-4 w-4" />
					<div class="text-sm">
						<div class="font-medium">End Time</div>
						<div class="text-muted-foreground">{formatDate(test.endTime)}</div>
					</div>
				</div>
				<div class="flex items-center justify-center gap-2">
					<Clock class="text-muted-foreground h-4 w-4" />
					<div class="text-sm">
						<div class="font-medium">Duration</div>
						<div class="text-muted-foreground">
							{formatElapsedTime(test.startTime, test.endTime)}
						</div>
					</div>
				</div>
			</div>

			{#if test.doc}
				<div class="flex items-center gap-2">
					<FileText class="h-4 w-4 min-w-4" />
					<div class="text-sm">
						<div class="font-medium">Documentation</div>
						<div class="text-muted-foreground">{test.doc}</div>
					</div>
				</div>
			{/if}
		</div>
	</Card.Content>
</Card.Root>

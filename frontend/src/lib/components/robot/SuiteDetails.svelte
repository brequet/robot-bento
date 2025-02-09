<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { formatDate, formatElapsedTime } from '$lib/services/date';
	import type { ApiStatistic, ApiSuite } from '$lib/types/generated';
	import { Clock, Code, FileText, FolderTree } from 'lucide-svelte';
	import TestRunProgress from '../project/TestRunProgress.svelte';
	import StatusBadge from './StatusBadge.svelte';

	let { suite, stats }: { suite: ApiSuite; stats: ApiStatistic | undefined } = $props();
</script>

<Card.Root class="h-full w-full">
	<Card.Header>
		<div class="flex items-center justify-between">
			<div class="flex items-center gap-2">
				<FolderTree class="text-muted-foreground h-5 w-5" />
				<Card.Title>{suite.name}</Card.Title>
			</div>
			<StatusBadge status={suite.status} type={'SUITE'} />
		</div>
		<Card.Description>ID: {suite.identifier}</Card.Description>
	</Card.Header>

	<Card.Content>
		<div class="space-y-6">
			{#if stats}
				<TestRunProgress
					passedCount={stats.passCount}
					skippedCount={stats.skipCount}
					failedCount={stats.failCount}
				/>
			{/if}

			<div class="grid grid-cols-3 gap-4">
				<div class="flex items-center justify-center gap-2">
					<Clock class="text-muted-foreground h-4 w-4" />
					<div class="text-sm">
						<div class="font-medium">Start Time</div>
						<div class="text-muted-foreground">{formatDate(suite.startTime)}</div>
					</div>
				</div>
				<div class="flex items-center justify-center gap-2">
					<Clock class="text-muted-foreground h-4 w-4" />
					<div class="text-sm">
						<div class="font-medium">End Time</div>
						<div class="text-muted-foreground">{formatDate(suite.endTime)}</div>
					</div>
				</div>
				<div class="flex items-center justify-center gap-2">
					<Clock class="text-muted-foreground h-4 w-4" />
					<div class="text-sm">
						<div class="font-medium">Duration</div>
						<div class="text-muted-foreground">
							{formatElapsedTime(suite.startTime, suite.endTime)}
						</div>
					</div>
				</div>
			</div>

			{#if suite.source || suite.doc}
				<div class="flex gap-4">
					{#if suite.source}
						<div class="flex items-center gap-2">
							<Code class="h-4 w-4" />
							<div class="text-sm">
								<div class="font-medium">Source</div>
								<div class="text-muted-foreground">{suite.source}</div>
							</div>
						</div>
					{/if}
					{#if suite.doc}
						<Button variant="outline" size="sm" class="flex items-center gap-2">
							<FileText class="h-4 w-4" />
							Documentation
						</Button>
					{/if}
				</div>
			{/if}
		</div>
	</Card.Content>
</Card.Root>

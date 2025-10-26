<script lang="ts">
	import SuccessRateProgressBar from '$lib/components/shared/SuccessRateProgressBar.svelte';
	import * as Card from '$lib/components/ui/card/index.js';
	import { formatDate, formatElapsedTime } from '$lib/services/date';
	import { getSuiteKeywords } from '$lib/services/robot';
	import type { ApiStatistic, ApiSuite } from '$lib/types/generated';
	import { Clock, FileText, FolderTree, LoaderCircle } from 'lucide-svelte';
	import StatusBadge from '../shared/StatusBadge.svelte';
	import Keyword from './keywords/Keyword.svelte';

	let { suite, stats }: { suite: ApiSuite; stats: ApiStatistic | undefined } = $props();

	let suiteKeywordsPromise = $derived(getSuiteKeywords(suite.id));
</script>

<Card.Root class="h-full w-full">
	<Card.Header>
		<div class="flex items-center justify-between">
			<div class="flex items-center gap-2">
				<FolderTree class="text-muted-foreground h-5 w-5" />
				<Card.Title>{suite.name}</Card.Title>
			</div>
			<StatusBadge status={suite.status} text={'SUITE'} />
		</div>
		<Card.Description>ID: {suite.identifier}</Card.Description>
	</Card.Header>

	<Card.Content>
		<div class="space-y-2">
			{#if stats}
				<SuccessRateProgressBar
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

			{#if suite.doc}
				<div class="flex items-center gap-2">
					<FileText class="h-4 w-4 min-w-4" />
					<div class="text-sm">
						<div class="font-medium">Documentation</div>
						<div class="text-muted-foreground">{suite.doc}</div>
					</div>
				</div>
			{/if}

			{#await suiteKeywordsPromise}
				Loading suite keywords..

				<LoaderCircle class="animate-spin" />
			{:then suiteKeywords}
				{#if suiteKeywords}
					<div class="space-y-1">
						{#if suiteKeywords.setupKeyword}
							<Keyword keyword={suiteKeywords.setupKeyword} />
						{/if}
						{#if suiteKeywords.teardownKeyword}
							<Keyword keyword={suiteKeywords.teardownKeyword} />
						{/if}
					</div>
				{:else}
					<div>No keywords found</div>
				{/if}
			{/await}
		</div>
	</Card.Content>
</Card.Root>

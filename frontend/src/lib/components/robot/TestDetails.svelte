<script lang="ts">
	import * as Accordion from '$lib/components/ui/accordion/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { formatDate, formatElapsedTime } from '$lib/services/date';
	import { getTestKeywords } from '$lib/services/robot';
	import type { ApiTest } from '$lib/types/generated';
	import { Clock, FileText, LoaderCircle, TestTube } from 'lucide-svelte';
	import StatusBadge from '../shared/StatusBadge.svelte';
	import BaseBody from './keywords/BaseBody.svelte';

	let { test }: { test: ApiTest } = $props();

	let testKeywordsPromise = $derived(getTestKeywords(test.id));
</script>

<Card.Root class="h-full w-full">
	<Card.Header>
		<div class="flex items-center justify-between">
			<div class="flex items-center gap-2">
				<TestTube class="text-muted-foreground h-5 w-5" />
				<Card.Title>{test.name}</Card.Title>
			</div>
			<StatusBadge status={test.status} text={'TEST'} />
		</div>
		<Card.Description>ID: {test.identifier}</Card.Description>
	</Card.Header>

	<Card.Content>
		<div class="space-y-2">
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
				<Accordion.Root type="single">
					<Accordion.Item value="item-1">
						<Accordion.Trigger>
							<div class="flex flex-row items-center space-x-2">
								<FileText class="h-4 w-4" />
								<div class="font-medium">Documentation</div>
							</div>
						</Accordion.Trigger>
						<Accordion.Content>
							<p class="text-muted-foreground whitespace-pre-wrap text-sm">{test.doc}</p>
						</Accordion.Content>
					</Accordion.Item>
				</Accordion.Root>
			{/if}

			{#await testKeywordsPromise}
				Loading test keywords..

				<LoaderCircle class="animate-spin" />
			{:then testKeywords}
				{#if testKeywords && testKeywords.length > 0}
					<div class="space-y-1">
						{#each testKeywords as keyword}
							<BaseBody baseBody={keyword} />
						{/each}
					</div>
				{:else}
					<div>No keywords found</div>
				{/if}
			{/await}
		</div>
	</Card.Content>
</Card.Root>

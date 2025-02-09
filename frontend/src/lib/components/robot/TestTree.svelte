<script lang="ts">
	import type { ApiSuite, ApiTest } from '$lib/types/generated';
	import { ChevronRight, ChevronDown } from 'lucide-svelte';
	import Self from './TestTree.svelte';
	import StatusBadge from './StatusBadge.svelte';

	let {
		suites,
		selectedSuite,
		selectedTest,
		handleSuiteSelect,
		handleTestSelect,
		level = 0
	}: {
		suites: ApiSuite[];
		selectedSuite: ApiSuite | null;
		selectedTest: ApiTest | null;
		handleSuiteSelect: (suite: ApiSuite) => void;
		handleTestSelect: (suite: ApiTest) => void;
		level?: number;
	} = $props();

	// TODO: init expandedSuites for failing suites and tests
	let expandedSuites = $state(new Set<number>());

	function selectSuite(suite: ApiSuite) {
		toggleSuite(null, suite);
		handleSuiteSelect(suite);
	}

	function toggleSuite(e: MouseEvent | null, suite: ApiSuite) {
		e?.stopPropagation();
		const newSet = new Set(expandedSuites);

		if (newSet.has(suite.id)) {
			newSet.delete(suite.id);
		} else {
			newSet.add(suite.id);
		}

		expandedSuites = newSet;
	}
</script>

<ul class="space-y-1">
	{#each suites as suite}
		<li>
			<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
			<div
				class="hover:bg-muted flex cursor-pointer items-center gap-2 rounded p-1"
				class:bg-muted={selectedSuite?.id === suite.id}
				style="padding-left: {level * 1.5}rem"
				onclick={() => selectSuite(suite)}
			>
				<button
					class="hover:bg-muted-foreground/20 rounded p-1"
					onclick={(e) => toggleSuite(e, suite)}
				>
					{#if suite.suites.length + suite.tests.length > 0}
						{#if expandedSuites.has(suite.id)}
							<ChevronDown size={16} />
						{:else}
							<ChevronRight size={16} />
						{/if}
					{/if}
				</button>
				<StatusBadge status={suite.status} type="SUITE" />
				<span class="ml-0 flex-1">
					{suite.name}
				</span>
			</div>

			{#if expandedSuites.has(suite.id)}
				{#if suite.suites.length > 0}
					<Self
						suites={suite.suites}
						{selectedSuite}
						{selectedTest}
						{handleSuiteSelect}
						{handleTestSelect}
						level={level + 1}
					/>
				{/if}
				{#if suite.tests.length > 0}
					<ul class="space-y-1">
						{#each suite.tests as test}
							<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_noninteractive_element_interactions -->
							<li
								class="hover:bg-muted flex cursor-pointer items-center gap-2 rounded p-1"
								class:bg-muted={selectedTest?.id === test.id}
								style="padding-left: {(level + 1) * 1.5}rem"
								onclick={() => handleTestSelect(test)}
							>
								<StatusBadge status={test.status} type="TEST" />
								<span class="flex-1">{test.name}</span>
							</li>
						{/each}
					</ul>
				{/if}
			{/if}
		</li>
	{/each}
</ul>

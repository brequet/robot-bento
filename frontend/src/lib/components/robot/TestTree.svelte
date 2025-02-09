<script lang="ts">
	import type { ApiSuite, ApiTest } from '$lib/types/generated';
	import { ChevronRight, ChevronDown } from 'lucide-svelte';
	import Self from './TestTree.svelte';

	let {
		suites,
		selectedSuite,
		selectedTest,
		handleSuiteSelect,
		handleTestSelect,
		level = 0
	} = $props<{
		suites: ApiSuite[];
		selectedSuite: ApiSuite | null;
		selectedTest: ApiTest | null;
		handleSuiteSelect: (suite: ApiSuite) => void;
		handleTestSelect: (suite: ApiTest) => void;
		level?: number;
	}>();

	let expandedSuites = $state(new Set<number>());

	function toggleSuite(suite: ApiSuite) {
		const newSet = new Set(expandedSuites);

		if (newSet.has(suite.id)) {
			newSet.delete(suite.id);
		} else {
			newSet.add(suite.id);
		}

		expandedSuites = newSet;
	}

	function getStatusColor(status: string) {
		switch (status.toLowerCase()) {
			case 'pass':
				return 'text-green-500';
			case 'fail':
				return 'text-red-500';
			case 'skip':
				return 'text-yellow-500';
			default:
				return 'text-muted-foreground';
		}
	}
</script>

<ul class="space-y-1">
	{#each suites as suite}
		<li>
			<div
				class="hover:bg-muted flex cursor-pointer items-center gap-2 rounded p-1"
				class:bg-muted={selectedSuite?.id === suite.id}
				style="padding-left: {level * 1.5}rem"
			>
				<button class="hover:bg-muted-foreground/20 rounded p-1" onclick={() => toggleSuite(suite)}>
					{#if suite.suites.length + suite.tests.length > 0}
						{#if expandedSuites.has(suite.id)}
							<ChevronDown size={16} />
						{:else}
							<ChevronRight size={16} />
						{/if}
					{/if}
				</button>
				<span class="flex-1" onclick={() => handleSuiteSelect(suite)}>
					{suite.name}
				</span>
				<span class={getStatusColor(suite.status)}>
					{suite.status}
				</span>
			</div>

			{#if expandedSuites.has(suite.id)}
				{#if suite.suites.length > 0}
					<Self
						suites={suite.suites}
						{selectedSuite}
						{selectedTest}
						level={level + 1}
						on:suiteSelect
						on:testSelect
					/>
				{/if}
				{#if suite.tests.length > 0}
					<ul class="space-y-1">
						{#each suite.tests as test}
							<li
								class="hover:bg-muted flex cursor-pointer items-center gap-2 rounded p-1"
								class:bg-muted={selectedTest?.id === test.id}
								style="padding-left: {(level + 1) * 1.5}rem"
								onclick={() => handleTestSelect(test)}
							>
								<span class="flex-1">{test.name}</span>
								<span class={getStatusColor(test.status)}>
									{test.status}
								</span>
							</li>
						{/each}
					</ul>
				{/if}
			{/if}
		</li>
	{/each}
</ul>

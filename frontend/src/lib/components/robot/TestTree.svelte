<script lang="ts">
	import type { ApiSuite, ApiTest } from '$lib/types/generated';
	import { LucideChevronDown, LucideChevronRight, LucideFileText } from 'lucide-svelte';
	import Self from './TestTree.svelte';

	let { suites, onSelect } = $props<{
		suites: ApiSuite[];
		onSelect: (item: ApiSuite | ApiTest) => void;
	}>();

	let expandedSuites = $state(new Set<number>());

	function toggleExpand(id: number) {
		console.log('toggleExpand', id, expandedSuites, expandedSuites.has(id));
		if (expandedSuites.has(id)) {
			expandedSuites.delete(id);
		} else {
			expandedSuites.add(id);
		}
	}
</script>

<ul class="space-y-2">
	{#each suites as suite}
		<li>
			<button
				class="flex cursor-pointer items-center space-x-2 rounded p-2 hover:bg-gray-100"
				onclick={() => toggleExpand(suite.id)}
			>
				{#if suite.suites.length > 0}
					{suite.id} in {expandedSuites} = {expandedSuites.has(suite.id)}
					{#if expandedSuites.has(suite.id)}
						<LucideChevronDown class="h-4 w-4" />
					{:else}
						<LucideChevronRight class="h-4 w-4" />
					{/if}
				{/if}
				<span class="font-medium" onclick={() => onSelect(suite)}>{suite.name}</span>
			</button>
			{#if expandedSuites.has(suite.id)}
				<ul class="ml-4 border-l border-gray-300 pl-2">
					<Self {suites} {onSelect} />
					{#each suite.tests as test}
						<li
							class="flex cursor-pointer items-center space-x-2 rounded p-1 hover:bg-gray-100"
							onclick={() => onSelect(test)}
						>
							<LucideFileText class="h-4 w-4 text-blue-500" />
							<span>{test.name}</span>
						</li>
					{/each}
				</ul>
			{/if}
		</li>
	{/each}
</ul>

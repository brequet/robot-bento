<script lang="ts">
	import Breadcrumbs from '$lib/components/robot/Breadcrumbs.svelte';
	import TestTree from '$lib/components/robot/TestTree.svelte';
	import { getTestRunById } from '$lib/services/robot';
	import { onMount } from 'svelte';
	// import SuiteDetails from '$lib/components/robot/SuiteDetails.svelte';
	// import TestDetails from '$lib/components/robot/TestDetails.svelte';
	import type { ApiSuite, ApiTest, TestRunResponse } from '$lib/types/generated';

	let { testRunId }: { testRunId: number } = $props();

	let testRun: TestRunResponse | null = $state(null);

	let selectedSuite: ApiSuite | null = $state(null);
	let selectedTest: ApiTest | null = $state(null);

	let selectedItem: ApiSuite | ApiTest | null = $state(null);
	let path = $state<{ name: string; id: number }[]>([]);

	onMount(async () => {
		testRun = await getTestRunById(testRunId);
	});

	function handleSuiteSelect(suite: ApiSuite) {
		console.log('handleSuiteSelect', suite);
		selectedTest = null;
		selectedSuite = suite;
		// updateBreadcrumbs(suite);
	}

	function handleTestSelect(test: ApiTest) {
		console.log('handleTestSelect', test);
		selectedTest = test;
	}

	function handleSelect(item: ApiSuite | ApiTest) {
		console.log('handleSelect', item);
		selectedItem = item;
		path = [...path, { name: item.name, id: item.id }];
	}

	function navigateBreadcrumb(id: number) {
		const index = path.findIndex((p) => p.id === id);
		path = path.slice(0, index + 1);
		selectedItem = testRun?.suites.find((s) => s.id === id) || null;
	}
</script>

<main class="flex h-screen">
	<!-- Sidebar: Test Suite Tree -->
	<aside class="w-1/4 overflow-y-auto border-r bg-gray-50 p-4">
		<h2 class="mb-4 text-lg font-semibold">Test Suites</h2>
		{#if testRun}
			<TestTree
				suites={testRun.suites}
				{selectedSuite}
				{selectedTest}
				{handleSuiteSelect}
				{handleTestSelect}
			/>
		{/if}
	</aside>

	<!-- Main Content -->
	<section class="flex-1 overflow-y-auto p-6">
		<Breadcrumbs {path} onNavigate={navigateBreadcrumb} />

		{#if selectedItem}
			{#if 'suites' in selectedItem}
				suite details
				<!-- <SuiteDetails suite={selectedItem as ApiSuite} /> -->
			{:else}
				test details
				<!-- <TestDetails test={selectedItem as ApiTest} /> -->
			{/if}
		{/if}
	</section>
</main>

<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import Breadcrumbs from '$lib/components/robot/Breadcrumbs.svelte';
	import SuiteDetails from '$lib/components/robot/SuiteDetails.svelte';
	import TestDetails from '$lib/components/robot/TestDetails.svelte';
	import TestTree from '$lib/components/robot/TestTree.svelte';
	import * as Resizable from '$lib/components/ui/resizable/index.js';
	import { getTestRunById } from '$lib/services/robot';
	import type { ApiStatistic, ApiSuite, ApiTest, TestRunResponse } from '$lib/types/generated';
	import { onMount } from 'svelte';

	let testRun: TestRunResponse | null = $state(null);
	let error: String | null = $state(null); // TODO: use error

	let selectedSuite: ApiSuite | null = $state(null);
	let selectedTest: ApiTest | null = $state(null);

	// TODO: scroll to selected test/suite
	$effect(() => {
		if (selectedTest !== null) {
			goto(`?test=${selectedTest.identifier}`);
		} else if (selectedSuite !== null) {
			goto(`?suite=${selectedSuite.identifier}`);
		}
	});

	let idToStats: Map<string, ApiStatistic> = $derived.by(() => {
		if (!testRun) {
			return new Map<string, ApiStatistic>();
		}

		return testRun.statistics
			.filter((stats) => stats.statType === 'suite')
			.reduce((map, stat) => {
				map.set(stat.identifier ?? '', stat);
				return map;
			}, new Map<string, ApiStatistic>());
	});

	let selectedSuiteStats = $derived.by(() => {
		return selectedSuite !== null ? idToStats.get(selectedSuite.identifier) : undefined;
	});

	onMount(async () => {
		testRun = await getTestRunById(Number(page.params.id));
		if (!testRun) {
			error = 'Failed to load test run details.';
			return;
		}

		let queriedTestIdentifier = page.url.searchParams.get('test');
		if (queriedTestIdentifier !== null) {
			let test = findTestByIdentifier(testRun.suites, queriedTestIdentifier);
			if (test) {
				selectedTest = test;
				return;
			}
		}

		let queriedSuiteIdentifier = page.url.searchParams.get('suite');
		if (queriedSuiteIdentifier !== null) {
			let suite = findSuiteByIdentifier(testRun.suites, queriedSuiteIdentifier);
			if (suite) {
				selectedSuite = suite;
				return;
			}
		}

		if (!selectedSuite && testRun && testRun.suites.length > 0) {
			selectedSuite = testRun.suites[0];
		}
	});

	function handleSuiteSelect(suite: ApiSuite) {
		selectedTest = null;
		selectedSuite = suite;
	}

	function handleTestSelect(test: ApiTest) {
		selectedSuite = null;
		selectedTest = test;
	}

	function handleElementSelect(element: ApiSuite | ApiTest) {
		if ('suites' in element) {
			handleSuiteSelect(element);
		} else {
			handleTestSelect(element);
		}
	}

	let breadcrumbs = $derived(buildBreadcrumbs(testRun?.suites ?? [], selectedSuite, selectedTest));

	function buildBreadcrumbs(
		rootSuites: ApiSuite[],
		selectedSuite: ApiSuite | null,
		selectedTest: ApiTest | null
	): Array<{ name: string; element: ApiSuite | ApiTest }> {
		const breadcrumbs: Array<{ name: string; element: ApiSuite | ApiTest }> = [];

		if (!selectedSuite && !selectedTest) return breadcrumbs;

		function findPathToSuite(
			suites: ApiSuite[],
			targetId: number,
			currentPath: ApiSuite[] = []
		): ApiSuite[] | null {
			for (const suite of suites) {
				const newPath = [...currentPath, suite];

				if (suite.id === targetId) {
					return newPath;
				}

				const foundInSubSuites = findPathToSuite(suite.suites, targetId, newPath);
				if (foundInSubSuites) return foundInSubSuites;
			}
			return null;
		}

		function findPathToTest(
			suites: ApiSuite[],
			testId: number,
			currentPath: ApiSuite[] = []
		): { path: ApiSuite[]; test: ApiTest } | null {
			for (const suite of suites) {
				const newPath = [...currentPath, suite];

				const test = suite.tests.find((t) => t.id === testId);
				if (test) {
					return { path: newPath, test };
				}

				const foundInSubSuites = findPathToTest(suite.suites, testId, newPath);
				if (foundInSubSuites) return foundInSubSuites;
			}
			return null;
		}

		if (selectedTest) {
			const result = findPathToTest(rootSuites, selectedTest.id);
			if (result) {
				breadcrumbs.push(
					...result.path.map((suite) => ({
						name: suite.name,
						element: suite
					}))
				);
				breadcrumbs.push({
					name: result.test.name,
					element: result.test
				});
			}
		} else if (selectedSuite) {
			const path = findPathToSuite(rootSuites, selectedSuite.id);
			if (path) {
				breadcrumbs.push(
					...path.map((suite) => ({
						name: suite.name,
						element: suite
					}))
				);
			}
		}

		return breadcrumbs;
	}

	function findTestByIdentifier(suites: ApiSuite[], testIdentifier: string): ApiTest | null {
		for (const suite of suites) {
			for (const test of suite.tests) {
				if (testIdentifier === test.identifier) {
					return test;
				}
			}
			const foundTest = findTestByIdentifier(suite.suites, testIdentifier);
			if (foundTest) {
				return foundTest;
			}
		}
		return null;
	}

	function findSuiteByIdentifier(suites: ApiSuite[], suiteIdentifier: string): ApiSuite | null {
		for (const suite of suites) {
			if (suiteIdentifier === suite.identifier) {
				return suite;
			}
			const foundSuite = findSuiteByIdentifier(suite.suites, suiteIdentifier);
			if (foundSuite) {
				return foundSuite;
			}
		}
		return null;
	}
</script>

<main class="h-screen">
	<Resizable.PaneGroup direction="horizontal" class="  rounded-lg border">
		<!-- Sidebar: Test Suite Tree -->
		<Resizable.Pane defaultSize={30}>
			<div class="flex h-full flex-col overflow-auto whitespace-nowrap">
				<h2 class="p-4 text-lg font-semibold">Test Suites</h2>
				{#if testRun}
					<TestTree
						suites={testRun.suites}
						{selectedSuite}
						{selectedTest}
						{handleSuiteSelect}
						{handleTestSelect}
					/>
				{/if}
			</div>
		</Resizable.Pane>
		<Resizable.Handle withHandle />
		<!-- Main Content -->
		<Resizable.Pane defaultSize={75}>
			<div class="flex h-full flex-col space-y-2 overflow-y-auto p-4">
				<Breadcrumbs {breadcrumbs} {handleElementSelect} />

				<div class="flex-1">
					{#if selectedTest}
						<TestDetails test={selectedTest} />
					{:else if selectedSuite}
						<SuiteDetails suite={selectedSuite} stats={selectedSuiteStats} />
					{/if}
				</div>
			</div>
		</Resizable.Pane>
	</Resizable.PaneGroup>
</main>

<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import Breadcrumbs from '$lib/components/robot/Breadcrumbs.svelte';
	import ErrorsDetails from '$lib/components/robot/ErrorsDetails.svelte';
	import NavigationFooter from '$lib/components/robot/NavigationFooter.svelte';
	import SuiteDetails from '$lib/components/robot/SuiteDetails.svelte';
	import TestRunDetails from '$lib/components/robot/test-run/TestRunDetails.svelte';
	import TestDetails from '$lib/components/robot/TestDetails.svelte';
	import TestTree from '$lib/components/robot/TestTree.svelte';
	import Badge from '$lib/components/ui/badge/badge.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Resizable from '$lib/components/ui/resizable/index.js';
	import {
		findSuiteByIdentifier,
		findTestByIdentifier,
		getFailedTestIdentifiers,
		getTestRunById
	} from '$lib/services/robot';
	import type {
		ApiError,
		ApiStatistic,
		ApiSuite,
		ApiTest,
		TestRunResponse
	} from '$lib/types/generated';
	import { ArrowLeft } from 'lucide-svelte';
	import { onMount } from 'svelte';

	let testRun: TestRunResponse | null = $state(null);
	let error: String | null = $state(null); // TODO: use error

	let selectedSuite: ApiSuite | null = $state(null);
	let selectedTest: ApiTest | null = $state(null);

	type Selection =
		| { type: 'suite'; suite: ApiSuite }
		| { type: 'test'; test: ApiTest }
		| { type: 'errors'; errors: ApiError[] }
		| null;

	let selected: Selection = $state(null);

	$effect(() => {
		switch (selected?.type) {
			case 'suite':
				goto(`?suite=${selected.suite.identifier}`);
				break;
			case 'test':
				goto(`?test=${selected.test.identifier}`);
				break;
			case 'errors':
				goto(`?errors`);
				break;
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
		return selected && selected.type === 'suite'
			? idToStats.get(selected.suite.identifier)
			: undefined;
	});

	let failedTestIdentifiers = $derived(getFailedTestIdentifiers(testRun?.suites ?? []));

	onMount(async () => {
		testRun = await getTestRunById(Number(page.params.id));

		selected = computeSelection(testRun);
	});

	function computeSelection(testRun: TestRunResponse | null): Selection | null {
		if (!testRun) {
			error = 'Failed to load test run details.';
			return null;
		}

		let queriedTestIdentifier = page.url.searchParams.get('test');
		if (queriedTestIdentifier !== null) {
			let test = findTestByIdentifier(testRun.suites, queriedTestIdentifier);
			if (test) {
				selectedTest = test;
				return { type: 'test', test };
			}
		}

		let queriedSuiteIdentifier = page.url.searchParams.get('suite');
		if (queriedSuiteIdentifier !== null) {
			let suite = findSuiteByIdentifier(testRun.suites, queriedSuiteIdentifier);
			if (suite) {
				selectedSuite = suite;
				return { type: 'suite', suite };
			}
		}

		let queriedErrors = page.url.searchParams.get('errors');
		if (queriedErrors !== null) {
			return { type: 'errors', errors: testRun.errors };
		}

		return null;
	}

	function handleSuiteSelect(suite: ApiSuite) {
		selectedTest = null;
		selectedSuite = suite;
		selected = { type: 'suite', suite };
	}

	function handleTestSelect(test: ApiTest) {
		selectedSuite = null;
		selectedTest = test;
		selected = { type: 'test', test };
	}

	function handleErrorsSelect(errors: ApiError[]) {
		selectedSuite = null;
		selectedTest = null;
		selected = { type: 'errors', errors };
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

	function goToTestByIdentifier(identifier: string) {
		const test = findTestByIdentifier(testRun?.suites ?? [], identifier);
		if (test) {
			handleTestSelect(test);
		}
	}

	function goToTestRunSection() {
		goto(`/test-run/${testRun?.id}`);
		selected = null;
		selectedSuite = null;
		selectedTest = null;
	}
</script>

<main class="h-screen">
	<Resizable.PaneGroup direction="horizontal" class="  rounded-lg border">
		<!-- Sidebar: Test Suite Tree -->
		<Resizable.Pane defaultSize={30}>
			<div class="flex h-full flex-col whitespace-nowrap">
				<div class="flex items-center justify-between border-b p-4">
					<!-- TODO: go back to project page -->
					<div>
						<Button
							variant="ghost"
							size="icon"
							onclick={() => goto(`/projects/${testRun?.projectId}`)}
						>
							<ArrowLeft />
						</Button>
						<button
							class="hover:bg-accent hover:text-accent-foreground flex-1 rounded-md p-2 text-left text-xl font-semibold transition-colors"
							onclick={goToTestRunSection}
						>
							Test run {testRun?.id}
						</button>
					</div>

					{#if testRun && testRun.errors.length > 0}
						<div>
							<Badge
								variant="destructive"
								class="cursor-pointer"
								onclick={() => handleErrorsSelect(testRun?.errors ?? [])}
							>
								{testRun.errors.length} errors
							</Badge>
						</div>
					{/if}
				</div>
				<div class="flex-1 overflow-auto">
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
			</div>
		</Resizable.Pane>
		<Resizable.Handle withHandle />
		<!-- Main Content -->
		<Resizable.Pane defaultSize={75} class="flex flex-col">
			<div class="flex flex-1 flex-col space-y-2 overflow-y-auto p-3">
				<Breadcrumbs {breadcrumbs} {handleElementSelect} />

				<div class="flex-1">
					{#if selected == null && testRun != null}
						<TestRunDetails {testRun} />
					{:else if selected?.type === 'errors'}
						<ErrorsDetails errors={selected.errors} />
					{:else if selected?.type === 'test'}
						<TestDetails test={selected.test} />
					{:else if selected?.type === 'suite'}
						<SuiteDetails suite={selected.suite} stats={selectedSuiteStats} />
					{/if}
				</div>
			</div>

			{#if selected != null}
				<NavigationFooter {failedTestIdentifiers} {selectedTest} {goToTestByIdentifier} />
			{/if}
		</Resizable.Pane>
	</Resizable.PaneGroup>
</main>

<script lang="ts">
	import { page } from '$app/state';
	import TestRunPage from '$lib/components/robot/TestRunPage.svelte';

	let testRunId = Number(page.params.id);

	// let testRun: TestRunResponse | null = $state(null);
	// let selectedItem: ApiSuite | ApiTest | null = $state(null);
	// let error: String | null = $state(null);

	// onMount(async () => {
	// 	testRun = await getTestRunById(Number(page.params.id));
	// 	if (!testRun) error = 'Failed to load test run details.';
	// });
</script>

<TestRunPage {testRunId} />

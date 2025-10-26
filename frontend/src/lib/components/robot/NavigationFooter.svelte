<script lang="ts">
	import type { ApiTest } from '$lib/types/generated';
	import { ArrowLeft, ArrowRight } from 'lucide-svelte';
	import Button from '../ui/button/button.svelte';

	let {
		failedTestIdentifiers,
		selectedTest,
		goToTestByIdentifier
	}: {
		failedTestIdentifiers: string[];
		selectedTest: ApiTest | null;
		goToTestByIdentifier: (identifier: string) => void;
	} = $props();

	let lastNonNullSelectedIndex: number = $state(-1);

	let selectedTestIndex = $derived(failedTestIdentifiers.indexOf(selectedTest?.identifier ?? ''));

	let isPreviousTestAvailable = $derived(selectedTestIndex > 0);
	let isNextTestAvailable = $derived(selectedTestIndex < failedTestIdentifiers.length - 1);

	$effect(() => {
		if (selectedTestIndex > -1) {
			lastNonNullSelectedIndex = selectedTestIndex;
		}
	});

	function goToPreviousFailedTest() {
		if (isPreviousTestAvailable) {
			goToTestByIdentifier(failedTestIdentifiers[lastNonNullSelectedIndex - 1]);
		}
	}

	function goToNextFailedTest() {
		if (isNextTestAvailable) {
			goToTestByIdentifier(failedTestIdentifiers[lastNonNullSelectedIndex + 1]);
		}
	}
</script>

<div class="flex h-16 w-full flex-row items-center justify-between border-t px-4">
	<div>
		<span class="mr-1 font-bold">
			[{lastNonNullSelectedIndex + 1} / {failedTestIdentifiers.length}]
		</span>
	</div>
	<div class="flex gap-2">
		<Button
			variant="outline"
			size="icon"
			disabled={!isPreviousTestAvailable}
			onclick={goToPreviousFailedTest}
		>
			<ArrowLeft class="h-4 w-4" />
		</Button>
		<Button
			variant="outline"
			size="icon"
			disabled={!isNextTestAvailable}
			onclick={goToNextFailedTest}
		>
			<ArrowRight class="h-4 w-4" />
		</Button>
	</div>
</div>

<script lang="ts">
	let {
		passedCount,
		failedCount,
		skippedCount
	}: {
		passedCount: number;
		failedCount: number;
		skippedCount: number;
	} = $props();

	let totalCount = $derived(passedCount + failedCount + skippedCount);

	let passedRate = $derived(Math.round((passedCount / totalCount) * 100));
	let failedRate = $derived(Math.round((failedCount / totalCount) * 100));
	let skippedRate = $derived(Math.round((skippedCount / totalCount) * 100));
</script>

<div class="w-full">
	<div class="h-2 w-full overflow-hidden rounded-full bg-gray-200">
		<div class="flex h-full">
			<div
				class="h-full bg-green-400 transition-all duration-300"
				style={`width: ${passedRate}%`}
			></div>
			<div
				class="h-full bg-red-400 transition-all duration-300"
				style={`width: ${failedRate}%`}
			></div>
			<div
				class="h-full bg-gray-400 transition-all duration-300"
				style={`width: ${skippedRate}%`}
			></div>
		</div>
	</div>

	<div class="mt-2 grid min-w-[250px] max-w-full grid-cols-3 text-xs text-gray-600">
		<div class="flex items-center gap-0.5">
			<div class="h-3 w-3 rounded-full bg-green-400"></div>
			<span>Passed: {passedCount}</span>
		</div>
		<div class="flex items-center gap-0.5">
			<div class="h-3 w-3 rounded-full bg-red-400"></div>
			<span>Failed: {failedCount}</span>
		</div>
		<div class="flex items-center gap-0.5">
			<div class="h-3 w-3 rounded-full bg-gray-400"></div>
			<span>Skipped: {skippedCount}</span>
		</div>
	</div>
</div>

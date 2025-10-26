<script lang="ts">
	import { formatRobotElapsedTime } from '$lib/services/date';
	import type { RobotStatus } from '$lib/types/robot';

	let { status }: { status: RobotStatus | undefined } = $props();
</script>

{#if status}
	<div class="flex space-x-2 text-sm">
		<span class="font-semibold">Start / End / Elapsed:</span>
		<span>
			{status.start_time} /
			{status.start_time} /
			{formatRobotElapsedTime(status.start_time, status.end_time)}
		</span>
	</div>
{/if}

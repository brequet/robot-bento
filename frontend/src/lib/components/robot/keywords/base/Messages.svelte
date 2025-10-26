<script lang="ts">
	import type { RobotMessage } from '$lib/types/robot';
	import Message from '../Message.svelte';

	let { messages }: { messages: RobotMessage[] } = $props();
</script>

{#if messages.length > 0}
	<div class="space-y-1">
		{#each messages as message}
			<Message {message} />
		{/each}
	</div>
{/if}

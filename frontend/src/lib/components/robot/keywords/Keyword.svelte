<script lang="ts">
	import * as Accordion from '$lib/components/ui/accordion/index.js';
	import type { RobotKeyword } from '$lib/types/robot';
	import BaseBody from './BaseBody.svelte';
	import Message from './Message.svelte';

	let { keyword }: { keyword: RobotKeyword } = $props();
</script>

<Accordion.Root type="single">
	<Accordion.Item value="item-1">
		<Accordion.Trigger>
			<div class="flex flex-row items-center space-x-2">
				<div class="font-medium">{keyword.name}</div>
				<div>{keyword.status?.status}</div>
			</div>
		</Accordion.Trigger>
		<Accordion.Content>
			<div class="space-y-2">
				{#if keyword.doc}
					<div class="text-muted-foreground whitespace-pre-wrap text-sm">{keyword.doc}</div>
				{/if}
				{#if keyword.msg.length > 0}
					<div class="space-y-1">
						{#each keyword.msg as message}
							<Message {message} />
						{/each}
					</div>
				{/if}
				{#if keyword.keywords.length > 0}
					<div class="space-y-1">
						{#each keyword.keywords as subKeyword}
							<BaseBody baseBody={subKeyword} />
						{/each}
					</div>
				{/if}
			</div>
		</Accordion.Content>
	</Accordion.Item>
</Accordion.Root>

<style>
	.text-muted-foreground {
		@apply text-gray-500;
	}
</style>

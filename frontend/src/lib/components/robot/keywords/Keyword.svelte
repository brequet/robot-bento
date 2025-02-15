<script lang="ts">
	import Tags from '$lib/components/robot/base/Tags.svelte';
	import type { RobotKeyword } from '$lib/types/robot';
	import Doc from '../base/Doc.svelte';
	import Keywords from '../base/Keywords.svelte';
	import Messages from '../base/Messages.svelte';
	import StatusTime from '../base/StatusTime.svelte';
	import GenericKeyword from './GenericKeyword.svelte';

	let { keyword }: { keyword: RobotKeyword } = $props();
</script>

<GenericKeyword type={keyword.type_} status={keyword.status}>
	{#snippet markupName()}
		<div class="text-left">
			{#if keyword.var.length > 0}
				<span>{keyword.var.join(', ')} =</span>
			{/if}
			{#if keyword.library}
				<span class="text-xs font-medium">{keyword.library}.</span>
			{/if}
			{#if keyword.name}
				<span class="font-semibold">{keyword.name}</span>
			{/if}
			{#if keyword.args.length > 0}
				<span>{keyword.args.join(' ')}</span>
			{/if}
		</div>
	{/snippet}

	<Doc doc={keyword.doc} />
	<StatusTime status={keyword.status} />
	<Tags tags={keyword.tags} />

	<Messages messages={keyword.msg} />

	<Keywords keywords={keyword.keywords} />
</GenericKeyword>

<script lang="ts">
	import Tags from '$lib/components/robot/base/Tags.svelte';
	import type { RobotKeyword } from '$lib/types/robot';
	import Doc from '../base/Doc.svelte';
	import Keywords from '../base/Keywords.svelte';
	import Messages from '../base/Messages.svelte';
	import StatusTime from '../base/StatusTime.svelte';
	import GenericKeyword from './GenericKeyword.svelte';

	let { keyword }: { keyword: RobotKeyword } = $props();

	function buildKeywordName(keyword: RobotKeyword): string {
		let result = '';
		if (keyword.var.length > 0) {
			result += `${keyword.var.join(', ')} =`;
		}
		if (keyword.library) {
			result += `${keyword.library}.`;
		}
		if (keyword.name) {
			result += keyword.name;
		}
		if (keyword.args.length > 0) {
			result += ` ${keyword.args.join(' ')}`;
		}
		return result;
	}
</script>

<GenericKeyword name={buildKeywordName(keyword)} type={keyword.type_} status={keyword.status}>
	<Doc doc={keyword.doc} />
	<StatusTime status={keyword.status} />
	<Tags tags={keyword.tags} />

	<Messages messages={keyword.msg} />

	<Keywords keywords={keyword.keywords} />
</GenericKeyword>

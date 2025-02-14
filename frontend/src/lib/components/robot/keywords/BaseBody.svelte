<script lang="ts">
	import type { KeywordType, RobotBaseBody } from '$lib/types/robot';
	import Break from './Break.svelte';
	import Continue from './Continue.svelte';
	import For from './For.svelte';
	import Group from './Group.svelte';
	import If from './If.svelte';
	import Keyword from './Keyword.svelte';
	import Message from './Message.svelte';
	import Return from './Return.svelte';
	import Try from './Try.svelte';
	import Var from './Var.svelte';
	import While from './While.svelte';

	let { baseBody }: { baseBody: RobotBaseBody } = $props();

	let keywordType: KeywordType = getKeywordType(baseBody);

	function getKeywordType(keyword: RobotBaseBody): KeywordType {
		if ('kw' in keyword) return 'RobotKeyword';
		if ('for' in keyword) return 'RobotFor';
		if ('while' in keyword) return 'RobotWhile';
		if ('group' in keyword) return 'RobotGroup';
		if ('if' in keyword) return 'RobotIf';
		if ('try' in keyword) return 'RobotTry';
		if ('variable' in keyword) return 'RobotVar';
		if ('return' in keyword) return 'RobotReturn';
		if ('continue' in keyword) return 'RobotContinue';
		if ('break' in keyword) return 'RobotBreak';
		if ('message' in keyword) return 'RobotMessage';
		throw new Error('Unknown keyword type');
	}
</script>

{#if keywordType === 'RobotKeyword'}
	<Keyword keyword={baseBody.kw} />
{:else if keywordType === 'RobotFor'}
	<For {...baseBody.for} />
{:else if keywordType === 'RobotWhile'}
	<While {...baseBody.while} />
{:else if keywordType === 'RobotGroup'}
	<Group {...baseBody.group} />
{:else if keywordType === 'RobotIf'}
	<If ifKw={baseBody.if} />
{:else if keywordType === 'RobotTry'}
	<Try {...baseBody.try} />
{:else if keywordType === 'RobotVar'}
	<Var {...baseBody.variable} />
{:else if keywordType === 'RobotReturn'}
	<Return {...baseBody.return} />
{:else if keywordType === 'RobotContinue'}
	<Continue {...baseBody.continue} />
{:else if keywordType === 'RobotBreak'}
	<Break {...baseBody.break} />
{:else if keywordType === 'RobotMessage'}
	<Message {...baseBody.message} />
{/if}

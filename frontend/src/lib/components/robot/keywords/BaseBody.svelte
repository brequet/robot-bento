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

	let {
		baseBody,
		preventUnwrap = false
	}: {
		baseBody: RobotBaseBody;
		preventUnwrap?: boolean;
	} = $props();

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
	<Keyword keyword={baseBody.kw} {preventUnwrap} />
{:else if keywordType === 'RobotFor'}
	<For forKw={baseBody.for} />
{:else if keywordType === 'RobotWhile'}
	<While />
{:else if keywordType === 'RobotGroup'}
	<Group />
{:else if keywordType === 'RobotIf'}
	<If ifKw={baseBody.if} />
{:else if keywordType === 'RobotTry'}
	<Try />
{:else if keywordType === 'RobotVar'}
	<Var />
{:else if keywordType === 'RobotReturn'}
	<Return returnKw={baseBody.return} />
{:else if keywordType === 'RobotContinue'}
	<Continue />
{:else if keywordType === 'RobotBreak'}
	<Break />
{:else if keywordType === 'RobotMessage'}
	<Message message={baseBody.message} />
{/if}

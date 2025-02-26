<script lang="ts">
	let { doc }: { doc: string | undefined } = $props();
</script>

{#if doc}
	<div class="text-muted-foreground whitespace-pre-wrap text-sm">{doc}</div>
{/if}

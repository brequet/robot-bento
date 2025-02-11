<script lang="ts">
	import * as Card from '$lib/components/ui/card/index.js';
	import type { ApiError } from '$lib/types/generated';

	let { errors }: { errors: ApiError[] } = $props();
</script>

<Card.Root class="h-full w-full">
	<Card.Header>
		<div class="flex items-center justify-between">
			<div class="flex items-center gap-2">
				<Card.Title>Test Run Errors</Card.Title>
			</div>
		</div>
	</Card.Header>

	<Card.Content>
		<ul class="space-y-2">
			{#each errors as error}
				<li class="flex flex-col gap-1.5 rounded-lg border p-4">
					<div class="flex items-start justify-between gap-4">
						<span class="text-muted-foreground font-mono text-sm">
							{new Date(error.timestamp).toLocaleString()}
						</span>
						<span
							class="rounded-full px-2 py-0.5 text-xs font-medium capitalize"
							class:bg-red-100={error.level === 'ERROR'}
							class:text-red-700={error.level === 'ERROR'}
							class:bg-yellow-100={error.level === 'WARN'}
							class:text-yellow-700={error.level === 'WARN'}
						>
							{error.level}
						</span>
					</div>
					<p class="whitespace-pre-wrap text-sm">{error.content}</p>
				</li>
			{/each}
		</ul>
	</Card.Content>
</Card.Root>

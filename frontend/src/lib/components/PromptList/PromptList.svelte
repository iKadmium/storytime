<script lang="ts">
	import type { Prompt } from '$lib/models/prompt.js';
	import ItemCard from '$lib/components/ItemCard/ItemCard.svelte';
	import ItemListCard from '$lib/components/ItemListCard/ItemListCard.svelte';

	interface Props {
		prompts: Prompt[];
		onEdit: (prompt: Prompt) => void;
		onDelete: (prompt: Prompt) => void;
		onView: (prompt: Prompt) => void;
		onTest?: (prompt: Prompt) => void;
		isLoading?: boolean;
	}

	let { prompts, onEdit, onDelete, onView, onTest, isLoading = false }: Props = $props();
</script>

<div class="space-y-4">
	{#if isLoading}
		<div class="py-8 text-center">
			<div class="placeholder animate-pulse"></div>
		</div>
	{:else if prompts.length === 0}
		<div class="py-8 text-center">
			<div class="text-lg opacity-75">No prompts found.</div>
			<p class="mt-2 opacity-50">Create your first prompt to get started!</p>
		</div>
	{:else}
		<!-- Desktop Grid View -->
		<div class="hidden gap-6 md:grid md:grid-cols-2 lg:grid-cols-3">
			{#each prompts as prompt (prompt.title)}
				<ItemCard
					title={prompt.title}
					description={prompt.description}
					executeLabel="Test"
					fields={[
						{ label: 'Context', value: prompt.context },
						{ label: 'Setup', value: prompt.setup.join('; ') },
						{ label: 'Audio', value: (prompt.create_audio ?? false) ? 'Yes' : 'No' },
						{ label: 'Images', value: (prompt.create_images ?? false) ? 'Yes' : 'No' }
					]}
					onView={() => onView(prompt)}
					onEdit={() => onEdit(prompt)}
					onDelete={() => onDelete(prompt)}
					onExecute={onTest ? () => onTest(prompt) : undefined}
				/>
			{/each}
		</div>

		<!-- Mobile List View -->
		<div class="space-y-4 md:hidden">
			{#each prompts as prompt (prompt.title)}
				<ItemListCard
					title={prompt.title}
					description={prompt.description}
					executeLabel="Test"
					onView={() => onView(prompt)}
					onEdit={() => onEdit(prompt)}
					onDelete={() => onDelete(prompt)}
					onExecute={onTest ? () => onTest(prompt) : undefined}
				/>
			{/each}
		</div>

		<!-- Prompt Count -->
		<div class="mt-6 text-center">
			<p class="text-sm opacity-50">
				{prompts.length} prompt{prompts.length === 1 ? '' : 's'}
			</p>
		</div>
	{/if}
</div>

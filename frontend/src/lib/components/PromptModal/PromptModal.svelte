<script lang="ts">
	import type { Prompt } from '$lib/models/prompt.js';
	import DetailModal from '$lib/components/DetailModal/DetailModal.svelte';
	import FieldDisplay from '$lib/components/FieldDisplay/FieldDisplay.svelte';

	interface Props {
		prompt?: Prompt;
		isOpen: boolean;
		onClose: () => void;
		onEdit?: (prompt: Prompt) => void;
		onDelete?: (prompt: Prompt) => void;
		onTest?: (prompt: Prompt) => void;
	}

	let { prompt, isOpen, onClose, onEdit, onDelete, onTest }: Props = $props();
</script>

{#if prompt}
	<DetailModal
		title={prompt.title}
		{isOpen}
		{onClose}
		onEdit={onEdit ? () => onEdit(prompt) : undefined}
		onDelete={onDelete ? () => onDelete(prompt) : undefined}
		onTest={onTest ? () => onTest(prompt) : undefined}
		editLabel="Edit Prompt"
		deleteLabel="Delete Prompt"
		testLabel="Test Prompt"
	>
		{#snippet children()}
			<FieldDisplay
				fields={[
					{
						icon: '📝',
						label: 'Description',
						value: prompt.description
					},
					{
						icon: '🎬',
						label: 'Context',
						value: prompt.context
					},
					{
						icon: '⚙️',
						label: 'Setup',
						value: '• ' + prompt.setup.join('\n• ')
					},
					{
						icon: '🎵',
						label: 'Create Audio',
						value: (prompt.create_audio ?? false) ? 'Yes' : 'No'
					},
					{
						icon: '🖼️',
						label: 'Create Images',
						value: (prompt.create_images ?? false) ? 'Yes' : 'No'
					}
				]}
			/>
		{/snippet}
	</DetailModal>
{/if}

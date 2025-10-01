<script lang="ts">
	import type { Job } from '$lib/models/job.js';
	import DetailModal from '$lib/components/DetailModal/DetailModal.svelte';
	import FieldDisplay from '$lib/components/FieldDisplay/FieldDisplay.svelte';
	import { formatCadence } from '$lib/utils/cron';

	interface Props {
		job?: Job;
		isOpen: boolean;
		onClose: () => void;
		onEdit?: (job: Job, filename: string) => void;
		onDelete?: (job: Job) => void;
		onTest?: (job: Job) => void;
		filename?: string;
	}

	let { job, isOpen, onClose, onEdit, onDelete, onTest, filename = '' }: Props = $props();
</script>

{#if job}
	<DetailModal
		title={`${job.character} → ${job.prompt}`}
		{isOpen}
		{onClose}
		onEdit={onEdit ? () => onEdit(job, filename) : undefined}
		onDelete={onDelete ? () => onDelete(job) : undefined}
		onTest={onTest ? () => onTest(job) : undefined}
		editLabel="Edit Job"
		deleteLabel="Delete Job"
		testLabel="Test Job"
	>
		<FieldDisplay
			fields={[
				{
					icon: '👤',
					label: 'Character',
					value: job.character
				},
				{
					icon: '💬',
					label: 'Prompt',
					value: job.prompt
				},
				{
					icon: '⏰',
					label: 'Schedule',
					value: formatCadence(job.cadence)
				},
				{
					icon: '🔧',
					label: 'Cron Expression',
					value: job.cadence
				},
				{
					icon: '📝',
					label: 'Prompt Override',
					value: job['prompt-override'] || 'None (uses default prompt template)'
				}
			]}
		/>
	</DetailModal>
{/if}

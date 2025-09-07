<script lang="ts">
	import type { Job } from '$lib/models/job.js';
	import DetailModal from '$lib/components/DetailModal/DetailModal.svelte';
	import FieldDisplay from '$lib/components/FieldDisplay/FieldDisplay.svelte';

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

	function formatCadence(cadence: string): string {
		// Simple cron expression formatter for display
		const parts = cadence.split(' ');
		if (parts.length === 5) {
			const [min, hour, day, month, dow] = parts;

			// Format day of week
			let dowDisplay = dow;
			if (dow === '1-5') dowDisplay = 'Weekdays';
			else if (dow === '0,6') dowDisplay = 'Weekends';
			else if (dow === '*') dowDisplay = 'Daily';

			// Format time
			let timeDisplay = '';
			if (hour.includes(',')) {
				const hours = hour.split(',');
				timeDisplay = hours.map((h) => `${h}:${min.padStart(2, '0')}`).join(', ');
			} else if (hour !== '*') {
				timeDisplay = `${hour}:${min.padStart(2, '0')}`;
			}

			return timeDisplay && dowDisplay !== 'Daily' ? `${timeDisplay} on ${dowDisplay}` : cadence;
		}
		return cadence;
	}
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
		{#snippet children()}
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
		{/snippet}
	</DetailModal>
{/if}

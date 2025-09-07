<script lang="ts">
	import type { Job } from '$lib/models/job.js';
	import ItemCard from '$lib/components/ItemCard/ItemCard.svelte';
	import ItemListCard from '$lib/components/ItemListCard/ItemListCard.svelte';

	interface Props {
		jobs: Job[];
		onEdit: (job: Job, filename: string) => void;
		onDelete: (job: Job) => void;
		onView: (job: Job, filename: string) => void;
		onExecute?: (job: Job) => void;
		isLoading?: boolean;
		generateFilename: (character: string, prompt: string) => string;
	}

	let {
		jobs,
		onEdit,
		onDelete,
		onView,
		onExecute,
		isLoading = false,
		generateFilename
	}: Props = $props();

	function handleEdit(job: Job) {
		const filename = generateFilename(job.character, job.prompt);
		onEdit(job, filename);
	}

	function handleDelete(job: Job) {
		onDelete(job);
	}

	function handleView(job: Job) {
		const filename = generateFilename(job.character, job.prompt);
		onView(job, filename);
	}

	function handleExecute(job: Job) {
		if (onExecute) {
			onExecute(job);
		}
	}

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

<div class="space-y-4">
	{#if isLoading}
		<div class="py-8 text-center">
			<div class="placeholder animate-pulse"></div>
		</div>
	{:else if jobs.length === 0}
		<div class="py-8 text-center">
			<div class="text-lg opacity-75">No jobs found.</div>
			<p class="mt-2 opacity-50">Create your first job to get started!</p>
		</div>
	{:else}
		<!-- Desktop Grid View -->
		<div class="hidden gap-6 md:grid md:grid-cols-2 lg:grid-cols-3">
			{#each jobs as job (`${job.character}-${job.prompt}`)}
				<ItemCard
					title={`${job.character} → ${job.prompt}`}
					description={job['prompt-override'] || 'No custom prompt override'}
					badge={formatCadence(job.cadence)}
					badgeColor="tertiary"
					fields={[
						{ label: 'Character', value: job.character },
						{ label: 'Prompt', value: job.prompt },
						{ label: 'Cron Expression', value: job.cadence }
					]}
					onView={() => handleView(job)}
					onEdit={() => handleEdit(job)}
					onDelete={() => handleDelete(job)}
					onExecute={onExecute ? () => handleExecute(job) : undefined}
				/>
			{/each}
		</div>

		<!-- Mobile List View -->
		<div class="space-y-4 md:hidden">
			{#each jobs as job (`${job.character}-${job.prompt}`)}
				<ItemListCard
					title={`${job.character} → ${job.prompt}`}
					description={job['prompt-override'] || 'No custom prompt override'}
					badge={formatCadence(job.cadence)}
					badgeColor="tertiary"
					onView={() => handleView(job)}
					onEdit={() => handleEdit(job)}
					onDelete={() => handleDelete(job)}
					onExecute={onExecute ? () => handleExecute(job) : undefined}
				/>
			{/each}
		</div>

		<!-- Job Count -->
		<div class="mt-6 text-center">
			<p class="text-sm opacity-50">
				{jobs.length} job{jobs.length === 1 ? '' : 's'}
			</p>
		</div>
	{/if}
</div>

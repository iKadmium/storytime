<script lang="ts">
	import { onMount } from 'svelte';
	import Button from '$lib/components/Button/Button.svelte';
	import JobForm from '$lib/components/JobForm/JobForm.svelte';
	import JobList from '$lib/components/JobList/JobList.svelte';
	import JobModal from '$lib/components/JobModal/JobModal.svelte';
	import DetailModal from '$lib/components/DetailModal/DetailModal.svelte';

	import type { Job, CreateJobRequest, UpdateJobRequest } from '$lib/models/job';
	import type { Message } from '$lib/models/chat';
	import { fetchJobs, createJob, updateJob, deleteJob, executeJob } from '$lib/services/job-service';
	import { jobSlug } from '$lib/utils/slug';
	import { fetchCharacters } from '$lib/services/character-service';
	import { fetchPrompts } from '$lib/services/prompt-service';

	// State management
	let jobs = $state<Job[]>([]);
	let availableCharacters = $state<string[]>([]);
	let availablePrompts = $state<string[]>([]);
	let isLoading = $state(false);
	let error = $state<string | null>(null);
	let isSubmitting = $state(false);

	// UI state
	let showForm = $state(false);
	let showModal = $state(false);
	let editingJob = $state<Job | undefined>(undefined);
	let viewingJob = $state<Job | undefined>(undefined);
	let editingJobFilename = $state<string>('');
	let _executingJobId = $state<string | null>(null);
	let executionResult = $state<Message | null>(null);
	let showExecutionResult = $state(false);

	// Load data when component mounts
	onMount(() => {
		loadJobs();
		loadAvailableOptions();
	});

	async function loadJobs() {
		try {
			isLoading = true;
			error = null;
			jobs = await fetchJobs();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load jobs';
			console.error('Error loading jobs:', err);
		} finally {
			isLoading = false;
		}
	}

	async function loadAvailableOptions() {
		try {
			// Load available characters
			const characters = await fetchCharacters();
			availableCharacters = characters.map((c) => c.name);

			// Load available prompts
			const prompts = await fetchPrompts();
			availablePrompts = prompts.map((p) => p.title);
		} catch (err) {
			console.error('Error loading available options:', err);
			// Don't show error for this as it's not critical
		}
	}

	function handleCreateNew() {
		editingJob = undefined;
		showForm = true;
	}

	function handleEdit(job: Job, filename: string) {
		editingJob = job;
		editingJobFilename = filename;
		showForm = true;
	}

	function handleView(job: Job, filename: string) {
		viewingJob = job;
		editingJobFilename = filename;
		showModal = true;
	}

	async function handleDeleteJob(job: Job) {
		if (!confirm('Are you sure you want to delete this job? This action cannot be undone.')) {
			return;
		}

		try {
			isSubmitting = true;
			error = null;
			await deleteJob(job.character, job.prompt);
			await loadJobs(); // Reload the list
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete job';
			console.error('Error deleting job:', err);
		} finally {
			isSubmitting = false;
		}
	}

	async function handleExecuteJob(job: Job) {
		const jobId = `${job.character}-${job.prompt}`;
		try {
			_executingJobId = jobId;
			error = null;
			const result = await executeJob(job, true); // Save to chat history for regular execution
			executionResult = result;
			showExecutionResult = true;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to execute job';
			console.error('Error executing job:', err);
		} finally {
			_executingJobId = null;
		}
	}

	async function handleTestJob2(job: Job) {
		const jobId = `${job.character}-${job.prompt}`;
		try {
			_executingJobId = jobId;
			error = null;
			const result = await executeJob(job, false); // Don't save to chat history for tests
			executionResult = result;
			showExecutionResult = true;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to test job';
			console.error('Error testing job:', err);
		} finally {
			_executingJobId = null;
		}
	}

	async function handleTestJob(jobData: CreateJobRequest) {
		try {
			error = null;
			// Convert CreateJobRequest to Job for testing (without saving to chat history)
			const testJob: Job = {
				character: jobData.character,
				prompt: jobData.prompt,
				cadence: jobData.cadence,
				'prompt-override': jobData['prompt-override']
			};
			const result = await executeJob(testJob, false); // Don't save to chat history for tests
			executionResult = result;
			showExecutionResult = true;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to test job';
			console.error('Error testing job:', err);
		}
	}

	async function handleFormSubmit(jobData: CreateJobRequest) {
		try {
			isSubmitting = true;
			error = null;

			if (editingJob) {
				// Update existing job
				const updates: UpdateJobRequest = {
					character: jobData.character,
					prompt: jobData.prompt,
					cadence: jobData.cadence,
					'prompt-override': jobData['prompt-override']
				};
				await updateJob(editingJob.character, editingJob.prompt, updates);
			} else {
				// Create new job
				await createJob(jobData);
			}

			// Refresh the job list and close form
			await loadJobs();
			handleFormCancel();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to save job';
			console.error('Error saving job:', err);
		} finally {
			isSubmitting = false;
		}
	}

	function handleFormCancel() {
		showForm = false;
		editingJob = undefined;
		editingJobFilename = '';
	}

	function handleModalClose() {
		showModal = false;
		viewingJob = undefined;
		editingJobFilename = '';
	}

	function handleModalEdit(job: Job, filename: string) {
		handleModalClose();
		handleEdit(job, filename);
	}

	function handleModalDelete(job: Job) {
		handleModalClose();
		handleDeleteJob(job);
	}
</script>

<div class="container mx-auto max-w-7xl px-4 py-8">
	<!-- Header -->
	<div class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
		<div>
			<h1 class="gradient-heading h1">Jobs</h1>
			<p class="opacity-75">Manage automated interaction jobs that run on a schedule.</p>
		</div>

		{#if !showForm}
			<Button onclick={handleCreateNew} disabled={isLoading}>+ Create Job</Button>
		{/if}
	</div>

	<!-- Error Display -->
	{#if error}
		<aside class="alert preset-filled-error mb-6">
			<div class="alert-message">
				<h3 class="h3">Error</h3>
				<p>{error}</p>
			</div>
			<div class="alert-actions">
				<Button size="sm" preset="ghost" color="error" onclick={() => (error = null)}>Dismiss</Button>
			</div>
		</aside>
	{/if}

	<!-- Job Form -->
	{#if showForm}
		<div class="mb-8">
			<JobForm
				job={editingJob}
				onSubmit={handleFormSubmit}
				onCancel={handleFormCancel}
				onTest={handleTestJob}
				{isSubmitting}
				submitLabel={editingJob ? 'Update Job' : 'Create Job'}
				{availableCharacters}
				{availablePrompts}
			/>
		</div>
	{/if}

	<!-- Job List -->
	{#if !showForm}
		<JobList {jobs} onEdit={handleEdit} onDelete={handleDeleteJob} onView={handleView} onExecute={handleExecuteJob} {isLoading} generateFilename={jobSlug} />
	{/if}

	<!-- Refresh Button -->
	{#if !showForm && !isLoading && jobs.length > 0}
		<div class="mt-8 text-center">
			<Button color="secondary" onclick={loadJobs}>Refresh Jobs</Button>
		</div>
	{/if}
</div>

<!-- Job Details Modal -->
<JobModal
	job={viewingJob}
	isOpen={showModal}
	onClose={handleModalClose}
	onEdit={handleModalEdit}
	onDelete={handleModalDelete}
	onTest={handleTestJob2}
	filename={editingJobFilename}
/>

<!-- Job Execution Result Modal -->
<DetailModal isOpen={showExecutionResult} onClose={() => (showExecutionResult = false)} title="Job Execution Result">
	{#if executionResult}
		<div class="space-y-4">
			<div>
				<h4 class="mb-2 h4">Generated Text:</h4>
				{#each executionResult.text as textItem, index (index)}
					<p class="mb-2 rounded bg-surface-200-800 p-3 text-sm">{textItem}</p>
				{/each}
			</div>

			{#if executionResult.audio && executionResult.audio.length > 0}
				<div>
					<h4 class="mb-2 h4">Audio Files:</h4>
					<ul class="list-inside list-disc text-sm">
						{#each executionResult.audio as audioFile, index (index)}
							<li>{audioFile}</li>
						{/each}
					</ul>
				</div>
			{/if}

			{#if executionResult.images && executionResult.images.length > 0}
				<div>
					<h4 class="mb-2 h4">Image Files:</h4>
					<ul class="list-inside list-disc text-sm">
						{#each executionResult.images as imageFile, index (index)}
							<li>{imageFile}</li>
						{/each}
					</ul>
				</div>
			{/if}
		</div>
	{/if}
</DetailModal>

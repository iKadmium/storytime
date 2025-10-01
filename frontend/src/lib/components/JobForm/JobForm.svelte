<script lang="ts">
	import type { CreateJobRequest, Job } from '$lib/models/job.js';
	import Button from '$lib/components/Button/Button.svelte';
	import MultiSelect from '$lib/components/MultiSelect/MultiSelect.svelte';
	import { formatCadence } from '$lib/utils/cron';

	interface Props {
		job?: Job;
		onSubmit: (job: CreateJobRequest) => void;
		onCancel: () => void;
		onTest?: (job: CreateJobRequest) => void;
		isSubmitting?: boolean;
		submitLabel?: string;
		availableCharacters?: string[];
		availablePrompts?: string[];
	}

	let { job, onSubmit, onCancel, onTest, isSubmitting = false, submitLabel = 'Create Job', availableCharacters = [], availablePrompts = [] }: Props = $props();

	// Form fields
	let characters = $state(job?.characters || []);
	let prompts = $state(job?.prompts || []);
	let cadence = $state(job?.cadence || '');
	let promptOverride = $state(job?.['prompt-override'] || '');
	let usePromptOverride = $state(!!job?.['prompt-override']);

	// Validation state
	let charactersError = $state('');
	let promptsError = $state('');
	let cadenceError = $state('');
	let promptOverrideError = $state('');

	let cadencePlain = $derived(formatCadence(cadence));

	function handleTest() {
		if (onTest && characters.length > 0 && prompts.length > 0) {
			const testJob: CreateJobRequest = {
				characters: characters,
				prompts: prompts,
				cadence: cadence.trim() || '0 0 * * *', // Default cadence for testing
				'prompt-override': usePromptOverride ? promptOverride.trim() || null : null
			};
			onTest(testJob);
		}
	}

	function validateForm(): boolean {
		let isValid = true;

		// Reset errors
		charactersError = '';
		promptsError = '';
		cadenceError = '';
		promptOverrideError = '';

		// Characters validation
		if (characters.length === 0) {
			charactersError = 'At least one character is required';
			isValid = false;
		}

		// Prompts validation
		if (prompts.length === 0) {
			promptsError = 'At least one prompt is required';
			isValid = false;
		}

		// Cadence validation (basic cron expression check)
		if (!cadence.trim()) {
			cadenceError = 'Cadence is required';
			isValid = false;
		} else {
			const cronParts = cadence.trim().split(/\s+/);
			if (cronParts.length !== 5) {
				cadenceError = 'Cadence must be a valid cron expression (5 parts)';
				isValid = false;
			}
		}

		// Prompt override validation (only if checkbox is checked)
		if (usePromptOverride && !promptOverride.trim()) {
			promptOverrideError = 'Prompt override is required when enabled';
			isValid = false;
		}

		return isValid;
	}

	function handleSubmit(e: Event) {
		e.preventDefault();

		if (!validateForm()) {
			return;
		}

		const jobData: CreateJobRequest = {
			characters: characters,
			prompts: prompts,
			cadence: cadence.trim(),
			'prompt-override': usePromptOverride ? promptOverride.trim() : null
		};

		onSubmit(jobData);
	}
</script>

<form class="space-y-4" onsubmit={handleSubmit}>
	<!-- Character Field -->
	<MultiSelect
		label="Characters"
		bind:selected={characters}
		options={availableCharacters}
		placeholder="Select characters..."
		required={true}
		disabled={isSubmitting}
		error={charactersError}
		minItems={1}
	/>

	<!-- Prompt Field -->
	<MultiSelect
		label="Prompts"
		bind:selected={prompts}
		options={availablePrompts}
		placeholder="Select prompts..."
		required={true}
		disabled={isSubmitting}
		error={promptsError}
		minItems={1}
	/>

	<!-- Cadence Field -->
	<div class="form-group">
		<label class="label" for="cadence">
			<span>Cadence (Cron Expression) <span class="text-red-500">*</span></span>
		</label>
		<input
			id="cadence"
			type="text"
			class="input"
			class:input-error={cadenceError}
			bind:value={cadence}
			placeholder="e.g., 30 9,10,11 * * 1-5"
			disabled={isSubmitting}
		/>
		{#if cadenceError}
			<div class="mt-1 text-sm text-error-500">{cadenceError}</div>
		{:else}
			<div class="mt-1 text-sm text-surface-500">
				Format: minute hour day month day-of-week (e.g., "30 9,10,11 * * 1-5" = 9:30, 10:30, 11:30 AM on weekdays)
			</div>
			<div class="mt-1 text-sm text-surface-500">Current parsed value: "{cadencePlain || 'N/A'}"</div>
		{/if}
	</div>

	<!-- Prompt Override Checkbox -->
	<div class="form-group">
		<label class="flex items-center space-x-2">
			<input type="checkbox" class="checkbox" bind:checked={usePromptOverride} disabled={isSubmitting} />
			<span>Use custom prompt override</span>
		</label>
		<div class="mt-1 text-sm text-surface-500">Enable this to override the default prompt template for this specific job</div>
	</div>

	<!-- Prompt Override Field (only shown when checkbox is checked) -->
	{#if usePromptOverride}
		<div class="form-group">
			<label class="label" for="prompt-override">
				<span>Prompt Override <span class="text-red-500">*</span></span>
			</label>
			<textarea
				id="prompt-override"
				class="textarea"
				class:input-error={promptOverrideError}
				bind:value={promptOverride}
				placeholder="Enter the custom prompt text to use for this job"
				rows="3"
				disabled={isSubmitting}
			></textarea>
			{#if promptOverrideError}
				<div class="mt-1 text-sm text-error-500">{promptOverrideError}</div>
			{:else}
				<div class="mt-1 text-sm text-surface-500">This text will override the default prompt template for this specific job</div>
			{/if}
		</div>
	{/if}

	<!-- Action Buttons -->
	<div class="flex justify-end space-x-2 pt-4">
		<Button type="button" preset="ghost" onclick={onCancel} disabled={isSubmitting}>Cancel</Button>
		{#if onTest}
			<Button type="button" preset="outlined" color="secondary" onclick={handleTest} disabled={isSubmitting || characters.length === 0 || prompts.length === 0}>
				Test
			</Button>
		{/if}
		<Button type="submit" preset="filled" disabled={isSubmitting}>
			{submitLabel}
		</Button>
	</div>
</form>

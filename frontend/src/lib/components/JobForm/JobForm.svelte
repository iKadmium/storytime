<script lang="ts">
	import type { CreateJobRequest, Job } from '$lib/models/job.js';
	import Button from '$lib/components/Button/Button.svelte';

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
	let character = $state(job?.character || '');
	let prompt = $state(job?.prompt || '');
	let cadence = $state(job?.cadence || '');
	let promptOverride = $state(job?.['prompt-override'] || '');
	let usePromptOverride = $state(!!job?.['prompt-override']);

	// Validation state
	let characterError = $state('');
	let promptError = $state('');
	let cadenceError = $state('');
	let promptOverrideError = $state('');

	function handleTest() {
		if (onTest && character.trim() && prompt.trim()) {
			const testJob: CreateJobRequest = {
				character: character.trim(),
				prompt: prompt.trim(),
				cadence: cadence.trim() || '0 0 * * *', // Default cadence for testing
				'prompt-override': usePromptOverride ? promptOverride.trim() || null : null
			};
			onTest(testJob);
		}
	}

	function validateForm(): boolean {
		let isValid = true;

		// Reset errors
		characterError = '';
		promptError = '';
		cadenceError = '';
		promptOverrideError = '';

		// Character validation
		if (!character.trim()) {
			characterError = 'Character is required';
			isValid = false;
		}

		// Prompt validation
		if (!prompt.trim()) {
			promptError = 'Prompt is required';
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
			character: character.trim(),
			prompt: prompt.trim(),
			cadence: cadence.trim(),
			'prompt-override': usePromptOverride ? promptOverride.trim() : null
		};

		onSubmit(jobData);
	}
</script>

<form class="space-y-4" onsubmit={handleSubmit}>
	<!-- Character Field -->
	<div class="form-group">
		<label class="label" for="character">
			<span>Character <span class="text-red-500">*</span></span>
		</label>
		{#if availableCharacters.length > 0}
			<select id="character" class="select" class:input-error={characterError} bind:value={character} disabled={isSubmitting}>
				<option value="">Select a character...</option>
				{#each availableCharacters as char, index (index)}
					<option value={char}>{char}</option>
				{/each}
			</select>
		{:else}
			<input
				id="character"
				type="text"
				class="input"
				class:input-error={characterError}
				bind:value={character}
				placeholder="Enter character name"
				disabled={isSubmitting}
			/>
		{/if}
		{#if characterError}
			<div class="mt-1 text-sm text-error-500">{characterError}</div>
		{/if}
	</div>

	<!-- Prompt Field -->
	<div class="form-group">
		<label class="label" for="prompt">
			<span>Prompt <span class="text-red-500">*</span></span>
		</label>
		{#if availablePrompts.length > 0}
			<select id="prompt" class="select" class:input-error={promptError} bind:value={prompt} disabled={isSubmitting}>
				<option value="">Select a prompt...</option>
				{#each availablePrompts as promptOption, index (index)}
					<option value={promptOption}>{promptOption}</option>
				{/each}
			</select>
		{:else}
			<input
				id="prompt"
				type="text"
				class="input"
				class:input-error={promptError}
				bind:value={prompt}
				placeholder="Enter prompt name"
				disabled={isSubmitting}
			/>
		{/if}
		{#if promptError}
			<div class="mt-1 text-sm text-error-500">{promptError}</div>
		{/if}
	</div>

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
			<Button type="button" preset="outlined" color="secondary" onclick={handleTest} disabled={isSubmitting || !character.trim() || !prompt.trim()}>
				Test
			</Button>
		{/if}
		<Button type="submit" preset="filled" disabled={isSubmitting}>
			{submitLabel}
		</Button>
	</div>
</form>

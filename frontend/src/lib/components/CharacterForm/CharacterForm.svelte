<script lang="ts">
	import type { CreateCharacterRequest, Character, Voice } from '$lib/models/character.js';
	import Button from '$lib/components/Button/Button.svelte';

	interface Props {
		character?: Character;
		onSubmit: (character: CreateCharacterRequest) => void;
		onCancel: () => void;
		isSubmitting?: boolean;
		submitLabel?: string;
		referenceVoiceFiles?: string[];
	}

	let {
		character,
		onSubmit,
		onCancel,
		isSubmitting = false,
		submitLabel = 'Create Character',
		referenceVoiceFiles = []
	}: Props = $props();

	// Form fields
	let name = $state(character?.name || '');
	let description = $state(character?.description || '');
	let personality = $state(character?.personality || '');
	let background = $state(character?.background || '');

	// Voice fields
	let voiceName = $state(character?.voice?.voiceName || '');
	let temperature = $state(character?.voice?.temperature || 0.7);
	let exaggeration = $state(character?.voice?.exaggeration || 0.5);
	let cfgWeight = $state(character?.voice?.cfgWeight || 1.0);
	let speedFactor = $state(character?.voice?.speedFactor || 1.0);

	// Validation state
	let nameError = $state('');
	let descriptionError = $state('');
	let personalityError = $state('');
	let backgroundError = $state('');

	function validateForm(): boolean {
		let isValid = true;

		// Reset errors
		nameError = '';
		descriptionError = '';
		personalityError = '';
		backgroundError = '';

		// Name validation (only if provided)
		if (name.trim() && name.trim().length < 2) {
			nameError = 'Name must be at least 2 characters';
			isValid = false;
		}

		// Description validation (only if provided)
		if (description.trim() && description.trim().length < 10) {
			descriptionError = 'Description must be at least 10 characters';
			isValid = false;
		}

		// Personality validation (only if provided)
		if (personality.trim() && personality.trim().length < 10) {
			personalityError = 'Personality must be at least 10 characters';
			isValid = false;
		}

		// Background validation (only if provided)
		if (background.trim() && background.trim().length < 10) {
			backgroundError = 'Background must be at least 10 characters';
			isValid = false;
		}

		return isValid;
	}

	function handleSubmit(event: Event) {
		event.preventDefault();

		if (!validateForm()) {
			return;
		}

		// Create voice object if any voice fields are filled
		let voice: Voice | undefined = undefined;
		if (voiceName.trim()) {
			voice = {
				voiceName: voiceName.trim(),
				temperature,
				exaggeration,
				cfgWeight,
				speedFactor
			};
		}

		const characterData: CreateCharacterRequest = {
			name: name.trim(),
			description: description.trim(),
			personality: personality.trim(),
			background: background.trim(),
			voice
		};

		onSubmit(characterData);
	}
</script>

<div class="card preset-glass-surface p-6">
	<h2 class="h2 mb-6">
		{character ? 'Edit Character' : 'Create New Character'}
	</h2>

	<form onsubmit={handleSubmit} class="space-y-4">
		<!-- Name Field -->
		<label class="label">
			<span>Name</span>
			<input
				type="text"
				class="input"
				bind:value={name}
				disabled={isSubmitting || !!character}
				placeholder="Enter character name"
			/>
			{#if nameError}
				<small class="text-error-500">{nameError}</small>
			{/if}
		</label>

		<!-- Description Field -->
		<label class="label">
			<span>Description</span>
			<textarea
				class="textarea"
				bind:value={description}
				disabled={isSubmitting}
				rows="3"
				placeholder="Describe the character's appearance and basic traits"
			></textarea>
			{#if descriptionError}
				<small class="text-error-500">{descriptionError}</small>
			{/if}
		</label>

		<!-- Personality Field -->
		<label class="label">
			<span>Personality</span>
			<textarea
				class="textarea"
				bind:value={personality}
				disabled={isSubmitting}
				rows="3"
				placeholder="Describe the character's personality traits, mannerisms, and behavior"
			></textarea>
			{#if personalityError}
				<small class="text-error-500">{personalityError}</small>
			{/if}
		</label>

		<!-- Background Field -->
		<label class="label">
			<span>Background</span>
			<textarea
				class="textarea"
				bind:value={background}
				disabled={isSubmitting}
				rows="3"
				placeholder="Describe the character's history, origin story, and past experiences"
			></textarea>
			{#if backgroundError}
				<small class="text-error-500">{backgroundError}</small>
			{/if}
		</label>

		<!-- Voice Configuration (Optional) -->
		<div class="card preset-glass-surface p-4">
			<h3 class="h3 mb-4">Voice Configuration (Optional)</h3>

			<!-- Voice Name Field -->
			<label class="label mb-4">
				<span>Voice Name</span>
				<select class="select" bind:value={voiceName} disabled={isSubmitting}>
					<option value="">No voice selected</option>
					{#each referenceVoiceFiles as voiceFile}
						<option value={voiceFile}>{voiceFile}</option>
					{/each}
				</select>
				{#if referenceVoiceFiles.length === 0}
					<small class="text-warning-500"
						>No reference voice files available. Upload some to the TTS server to enable voice
						selection.</small
					>
				{/if}
			</label>

			{#if voiceName}
				<!-- Voice Parameters -->
				<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
					<!-- Temperature -->
					<label class="label">
						<span>Temperature: {temperature}</span>
						<input
							type="range"
							min="0.1"
							max="1.5"
							step="0.1"
							bind:value={temperature}
							disabled={isSubmitting}
							class="range"
						/>
						<small class="text-surface-500">Controls randomness and creativity in speech</small>
					</label>

					<!-- Exaggeration -->
					<label class="label">
						<span>Exaggeration: {exaggeration}</span>
						<input
							type="range"
							min="0.0"
							max="1.0"
							step="0.1"
							bind:value={exaggeration}
							disabled={isSubmitting}
							class="range"
						/>
						<small class="text-surface-500">Controls emotional emphasis in speech</small>
					</label>

					<!-- CFG Weight -->
					<label class="label">
						<span>CFG Weight: {cfgWeight}</span>
						<input
							type="range"
							min="0.5"
							max="2.0"
							step="0.1"
							bind:value={cfgWeight}
							disabled={isSubmitting}
							class="range"
						/>
						<small class="text-surface-500">Controls adherence to voice characteristics</small>
					</label>

					<!-- Speed Factor -->
					<label class="label">
						<span>Speed Factor: {speedFactor}</span>
						<input
							type="range"
							min="0.5"
							max="2.0"
							step="0.1"
							bind:value={speedFactor}
							disabled={isSubmitting}
							class="range"
						/>
						<small class="text-surface-500">Controls speech speed</small>
					</label>
				</div>
			{/if}
		</div>

		<!-- Form Actions -->
		<div class="flex gap-4 pt-4">
			<Button type="submit" disabled={isSubmitting} preset="filled" color="primary">
				{#if isSubmitting}
					Saving...
				{:else}
					{submitLabel}
				{/if}
			</Button>
			<Button type="button" onclick={onCancel} disabled={isSubmitting} preset="outlined">
				Cancel
			</Button>
		</div>
	</form>
</div>

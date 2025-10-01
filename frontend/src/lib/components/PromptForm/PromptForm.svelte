<script lang="ts">
	import type { CreatePromptRequest, Prompt } from '$lib/models/prompt.js';
	import Button from '$lib/components/Button/Button.svelte';
	import FormField from '$lib/components/FormField/FormField.svelte';
	import FormTextarea from '$lib/components/FormTextarea/FormTextarea.svelte';

	interface Props {
		prompt?: Prompt;
		onSubmit: (prompt: CreatePromptRequest) => void;
		onCancel: () => void;
		onTest?: (prompt: CreatePromptRequest, character: string) => void;
		isSubmitting?: boolean;
		submitLabel?: string;
		availableCharacters?: string[];
	}

	let { prompt, onSubmit, onCancel, onTest, isSubmitting = false, submitLabel = 'Create Prompt', availableCharacters = [] }: Props = $props();

	// Form fields
	let title = $state(prompt?.title || '');
	let description = $state(prompt?.description || '');
	let context = $state(prompt?.context || '');
	let setupItems = $state(prompt?.setup || ['']);
	let createAudio = $state(prompt?.create_audio ?? false);
	let createImages = $state(prompt?.create_images ?? false);

	// Test functionality
	let selectedCharacter = $state('');

	// Validation state
	let titleError = $state('');
	let descriptionError = $state('');
	let contextError = $state('');
	let setupErrors = $state<string[]>([]);

	function validateForm(): boolean {
		let isValid = true;

		// Reset errors
		titleError = '';
		descriptionError = '';
		contextError = '';
		setupErrors = [];

		// Title validation (only check if provided)
		if (title.trim() && title.trim().length < 2) {
			titleError = 'Title must be at least 2 characters';
			isValid = false;
		}

		// All other fields are now optional - no validation required

		return isValid;
	}

	function handleSubmit(event: Event) {
		event.preventDefault();

		if (!validateForm()) {
			return;
		}

		const promptData: CreatePromptRequest = {
			title: title.trim() || '',
			description: description.trim() || '',
			context: context.trim() || '',
			setup: setupItems.map((item) => item.trim()).filter((item) => item.length > 0),
			create_audio: createAudio,
			create_images: createImages
		};

		onSubmit(promptData);
	}

	function handleTest() {
		if (!onTest || !selectedCharacter.trim()) {
			return;
		}

		const promptData: CreatePromptRequest = {
			title: title.trim() || 'Test Prompt',
			description: description.trim() || '',
			context: context.trim() || '',
			setup: setupItems.map((item) => item.trim()).filter((item) => item.length > 0),
			create_audio: createAudio,
			create_images: createImages
		};

		onTest(promptData, selectedCharacter.trim());
	}

	function addSetupItem() {
		setupItems = [...setupItems, ''];
	}

	function removeSetupItem(index: number) {
		setupItems = setupItems.filter((_, i) => i !== index);
		// If we removed all items, add one empty item back
		if (setupItems.length === 0) {
			setupItems = [''];
		}
	}

	function updateSetupItem(index: number, value: string) {
		setupItems = setupItems.map((item, i) => (i === index ? value : item));
	}
</script>

<div class="preset-glass-surface card p-6">
	<h2 class="mb-6 h2">
		{prompt ? 'Edit Prompt' : 'Create New Prompt'}
	</h2>

	<form onsubmit={handleSubmit} class="space-y-4">
		<!-- Title Field -->
		<FormField
			label="Title"
			bind:value={title}
			onInput={(value) => (title = value)}
			placeholder="Enter prompt title"
			disabled={isSubmitting || !!prompt}
			error={titleError}
		/>

		<!-- Description Field -->
		<FormTextarea
			label="Description"
			bind:value={description}
			onInput={(value) => (description = value)}
			placeholder="Brief description of what this prompt does"
			disabled={isSubmitting}
			error={descriptionError}
			rows={3}
		/>

		<!-- Context Field -->
		<FormTextarea
			label="Context"
			bind:value={context}
			onInput={(value) => (context = value)}
			placeholder="Context or background information for the prompt"
			disabled={isSubmitting}
			error={contextError}
			rows={4}
		/>

		<!-- Setup Fields -->
		<div class="space-y-2">
			<div class="flex items-center justify-between">
				<span class="label font-medium">Setup Items</span>
				<Button type="button" onclick={addSetupItem} disabled={isSubmitting} preset="outlined" size="sm">Add Item</Button>
			</div>
			{#each setupItems as _setupItem, index (index)}
				<div class="flex items-start gap-2">
					<div class="flex-1">
						<FormTextarea
							label=""
							bind:value={setupItems[index]}
							onInput={(value) => updateSetupItem(index, value)}
							placeholder={`Setup item ${index + 1}`}
							disabled={isSubmitting}
							error={setupErrors[index] || ''}
							rows={3}
						/>
					</div>
					{#if setupItems.length > 1}
						<Button type="button" onclick={() => removeSetupItem(index)} disabled={isSubmitting} preset="outlined" size="sm" color="surface">Remove</Button>
					{/if}
				</div>
			{/each}
		</div>

		<!-- Output Options -->
		<div class="space-y-4">
			<h3 class="h4">Output Options</h3>
			<div class="flex flex-col gap-4">
				<label class="flex items-center gap-2">
					<input type="checkbox" bind:checked={createAudio} disabled={isSubmitting} class="checkbox" />
					<span>Create Audio</span>
				</label>
				<label class="flex items-center gap-2">
					<input type="checkbox" bind:checked={createImages} disabled={isSubmitting} class="checkbox" />
					<span>Create Images</span>
				</label>
			</div>
		</div>

		<!-- Test Section -->
		{#if onTest && availableCharacters.length > 0}
			<div class="rounded border border-surface-500/20 bg-surface-50/50 p-4 dark:bg-surface-900/50">
				<h4 class="mb-3 h4">Test Prompt</h4>
				<p class="mb-3 text-sm opacity-75">Test this prompt with a character before saving</p>
				<div class="flex items-end gap-3">
					<div class="flex-1">
						<label for="character-select" class="mb-1 block text-sm font-medium"> Select Character </label>
						<select id="character-select" bind:value={selectedCharacter} disabled={isSubmitting} class="select w-full">
							<option value="">-- Select Character --</option>
							{#each availableCharacters as character, index (index)}
								<option value={character}>{character}</option>
							{/each}
						</select>
					</div>
					<Button type="button" preset="filled" color="secondary" onclick={handleTest} disabled={isSubmitting || !selectedCharacter.trim() || !title.trim()}>
						Test
					</Button>
				</div>
			</div>
		{/if}

		<!-- Form Actions -->
		<div class="flex gap-4 pt-4">
			<Button type="submit" disabled={isSubmitting} preset="filled" color="primary">
				{#if isSubmitting}
					Saving...
				{:else}
					{submitLabel}
				{/if}
			</Button>
			<Button type="button" onclick={onCancel} disabled={isSubmitting} preset="outlined">Cancel</Button>
		</div>
	</form>
</div>

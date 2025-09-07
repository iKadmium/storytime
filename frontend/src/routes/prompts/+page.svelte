<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import Button from '$lib/components/Button/Button.svelte';
	import PromptForm from '$lib/components/PromptForm/PromptForm.svelte';
	import PromptList from '$lib/components/PromptList/PromptList.svelte';
	import PromptModal from '$lib/components/PromptModal/PromptModal.svelte';
	import SelectionModal from '$lib/components/SelectionModal/SelectionModal.svelte';
	import DetailModal from '$lib/components/DetailModal/DetailModal.svelte';

	import type { Prompt, CreatePromptRequest, UpdatePromptRequest } from '$lib/models/prompt';
	import type { Message } from '$lib/models/chat';
	import {
		fetchPrompts,
		createPrompt,
		updatePrompt,
		deletePrompt
	} from '$lib/services/prompt-service';
	import { testPromptWithCharacter } from '$lib/services/test-service';
	import { fetchCharacters } from '$lib/services/character-service';
	import { isBase64Audio, convertTestAudioToUrl } from '$lib/services/chat-service';
	import { revokeAudioURL } from '$lib/utils/audio-utils';

	// State management
	let prompts = $state<Prompt[]>([]);
	let availableCharacters = $state<string[]>([]);
	let isLoading = $state(false);
	let error = $state<string | null>(null);
	let isSubmitting = $state(false);

	// UI state
	let showForm = $state(false);
	let showModal = $state(false);
	let editingPrompt = $state<Prompt | undefined>(undefined);
	let viewingPrompt = $state<Prompt | undefined>(undefined);

	// Test functionality state
	let showCharacterSelection = $state(false);
	let testingPrompt = $state<Prompt | undefined>(undefined);
	let testResult = $state<Message | null>(null);
	let showTestResult = $state(false);
	let testAudioUrls = $state<string[]>([]); // Object URLs for base64 audio cleanup
	let processedAudioUrls = $state<string[]>([]); // Processed audio URLs for display

	// Load prompts when component mounts
	onMount(() => {
		loadPrompts();
		loadAvailableCharacters();
	});

	// Clean up audio URLs when component is destroyed
	onDestroy(() => {
		cleanupAudioUrls();
	});

	async function loadPrompts() {
		try {
			isLoading = true;
			error = null;
			prompts = await fetchPrompts();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load prompts';
			console.error('Error loading prompts:', err);
		} finally {
			isLoading = false;
		}
	}

	async function loadAvailableCharacters() {
		try {
			const characters = await fetchCharacters();
			availableCharacters = characters.map((c) => c.name);
		} catch (err) {
			console.error('Error loading characters:', err);
			// Don't show error for this as it's not critical
		}
	}

	function handleCreateNew() {
		editingPrompt = undefined;
		showForm = true;
	}

	function handleEdit(prompt: Prompt) {
		editingPrompt = prompt;
		showForm = true;
	}

	function handleView(prompt: Prompt) {
		viewingPrompt = prompt;
		showModal = true;
	}

	async function handleDelete(prompt: Prompt) {
		const confirmed = confirm(
			`Are you sure you want to delete "${prompt.title}"? This action cannot be undone.`
		);
		if (!confirmed) return;

		try {
			isSubmitting = true;
			error = null;
			await deletePrompt(prompt.title);
			// Refresh the prompt list
			await loadPrompts();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete prompt';
			console.error('Error deleting prompt:', err);
		} finally {
			isSubmitting = false;
		}
	}

	function handleTestPrompt(prompt: Prompt) {
		testingPrompt = prompt;
		showCharacterSelection = true;
	}

	async function handleCharacterSelect(character: string) {
		if (!testingPrompt) return;

		try {
			error = null;
			showCharacterSelection = false;

			const result = await testPromptWithCharacter(testingPrompt, character, false); // Don't save to chat history for prompt tests
			testResult = result;
			processTestResultAudio(result);
			showTestResult = true;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to test prompt';
			console.error('Error testing prompt:', err);
		} finally {
			testingPrompt = undefined;
		}
	}

	function handleCharacterSelectCancel() {
		showCharacterSelection = false;
		testingPrompt = undefined;
	}

	// Clean up audio URLs to prevent memory leaks
	function cleanupAudioUrls() {
		testAudioUrls.forEach((url) => revokeAudioURL(url));
		testAudioUrls = [];
		processedAudioUrls = [];
	}

	// Process test result audio data
	function processTestResultAudio(result: Message) {
		// Clean up previous URLs
		testAudioUrls.forEach((url) => revokeAudioURL(url));

		if (result?.audio) {
			const newTestAudioUrls: string[] = [];
			const newProcessedAudioUrls: string[] = [];

			result.audio.forEach((audio) => {
				if (isBase64Audio(audio)) {
					const objectUrl = convertTestAudioToUrl(audio);
					newTestAudioUrls.push(objectUrl); // Track for cleanup
					newProcessedAudioUrls.push(objectUrl);
				} else {
					newProcessedAudioUrls.push(audio); // File path, use as-is
				}
			});

			testAudioUrls = newTestAudioUrls;
			processedAudioUrls = newProcessedAudioUrls;
		} else {
			testAudioUrls = [];
			processedAudioUrls = [];
		}
	}

	async function handleFormTest(promptData: CreatePromptRequest, character: string) {
		try {
			error = null;

			// Convert CreatePromptRequest to Prompt format
			const prompt: Prompt = {
				title: promptData.title,
				description: promptData.description,
				context: promptData.context,
				setup: promptData.setup,
				create_audio: promptData.create_audio,
				create_images: promptData.create_images
			};

			const result = await testPromptWithCharacter(prompt, character, false); // Don't save to chat history for prompt tests
			testResult = result;
			processTestResultAudio(result);
			showTestResult = true;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to test prompt';
			console.error('Error testing prompt:', err);
		}
	}

	async function handleFormSubmit(promptData: CreatePromptRequest) {
		try {
			isSubmitting = true;
			error = null;

			if (editingPrompt) {
				// Update existing prompt
				const updates: UpdatePromptRequest = {
					description: promptData.description,
					context: promptData.context,
					setup: promptData.setup,
					create_audio: promptData.create_audio,
					create_images: promptData.create_images
				};
				await updatePrompt(editingPrompt.title, updates);
			} else {
				// Create new prompt
				await createPrompt(promptData);
			}

			// Refresh the prompt list and close form
			await loadPrompts();
			handleFormCancel();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to save prompt';
			console.error('Error saving prompt:', err);
		} finally {
			isSubmitting = false;
		}
	}

	function handleFormCancel() {
		showForm = false;
		editingPrompt = undefined;
	}

	function handleModalClose() {
		showModal = false;
		viewingPrompt = undefined;
	}

	function handleModalEdit(prompt: Prompt) {
		handleModalClose();
		handleEdit(prompt);
	}

	function handleModalDelete(prompt: Prompt) {
		handleModalClose();
		handleDelete(prompt);
	}
</script>

<div class="container mx-auto max-w-7xl px-4 py-8">
	<!-- Header -->
	<div class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
		<div>
			<h1 class="h1 gradient-heading">Prompts</h1>
			<p class="opacity-75">
				Manage your story prompts with detailed context and setup instructions.
			</p>
		</div>

		{#if !showForm}
			<Button onclick={handleCreateNew} disabled={isLoading}>+ Create Prompt</Button>
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
				<Button size="sm" preset="ghost" color="error" onclick={() => (error = null)}>
					Dismiss
				</Button>
			</div>
		</aside>
	{/if}

	<!-- Prompt Form -->
	{#if showForm}
		<div class="mb-8">
			<PromptForm
				prompt={editingPrompt}
				onSubmit={handleFormSubmit}
				onCancel={handleFormCancel}
				onTest={handleFormTest}
				{isSubmitting}
				submitLabel={editingPrompt ? 'Update Prompt' : 'Create Prompt'}
				{availableCharacters}
			/>
		</div>
	{/if}

	<!-- Prompt List -->
	{#if !showForm}
		<PromptList
			{prompts}
			onEdit={handleEdit}
			onDelete={handleDelete}
			onView={handleView}
			onTest={handleTestPrompt}
			{isLoading}
		/>
	{/if}

	<!-- Refresh Button -->
	{#if !showForm && !isLoading && prompts.length > 0}
		<div class="mt-8 text-center">
			<Button color="secondary" onclick={loadPrompts}>Refresh Prompts</Button>
		</div>
	{/if}
</div>

<!-- Prompt Details Modal -->
<PromptModal
	prompt={viewingPrompt}
	isOpen={showModal}
	onClose={handleModalClose}
	onEdit={handleModalEdit}
	onDelete={handleModalDelete}
	onTest={handleTestPrompt}
/>

<!-- Character Selection Modal -->
<SelectionModal
	title="Select Character for Testing"
	isOpen={showCharacterSelection}
	onClose={handleCharacterSelectCancel}
	onSelect={handleCharacterSelect}
	options={availableCharacters}
	placeholder="Choose a character to test with"
/>

<!-- Test Result Modal -->
<DetailModal
	isOpen={showTestResult}
	onClose={() => {
		cleanupAudioUrls();
		showTestResult = false;
	}}
	title="Prompt Test Result"
>
	{#snippet children()}
		{#if testResult}
			<div class="space-y-4">
				<div>
					<h4 class="h4 mb-2">Generated Text:</h4>
					{#each testResult.text as textItem}
						<p class="bg-surface-200-800 mb-2 rounded p-3 text-sm">{textItem}</p>
					{/each}
				</div>

				{#if processedAudioUrls && processedAudioUrls.length > 0}
					<div>
						<h4 class="h4 mb-2">Generated Audio:</h4>

						<div class="space-y-3">
							{#each processedAudioUrls as audioUrl, index}
								{@const isBase64Source = testResult && isBase64Audio(testResult.audio[index])}
								<div class="audio-player">
									{#if isBase64Source}
										<div class="text-surface-600 mb-1 text-xs font-medium uppercase tracking-wide">
											Test Generated Audio
										</div>
									{:else}
										<div class="text-surface-600 mb-1 text-xs font-medium uppercase tracking-wide">
											Audio File: {testResult.audio[index]}
										</div>
									{/if}

									<audio controls class="w-full max-w-md">
										<source src={audioUrl} type={isBase64Source ? 'audio/mp3' : ''} />
										{#if isBase64Source}
											<div class="text-error-500 p-2 text-sm">
												Audio format not supported by your browser
											</div>
										{:else}
											<div class="text-surface-500 p-2 text-sm">
												Your browser does not support the audio element
											</div>
										{/if}
									</audio>
								</div>
							{/each}
						</div>
					</div>
				{/if}

				{#if testResult.images && testResult.images.length > 0}
					<div>
						<h4 class="h4 mb-2">Image Files:</h4>
						<ul class="list-inside list-disc text-sm">
							{#each testResult.images as imageFile}
								<li>{imageFile}</li>
							{/each}
						</ul>
					</div>
				{/if}
			</div>
		{/if}
	{/snippet}
</DetailModal>

<style>
	.audio-player {
		padding: 0.75rem;
		background-color: var(--color-surface-100);
		border: 1px solid var(--color-surface-300);
		border-radius: 0.5rem;
	}

	.audio-player audio {
		width: 100%;
		max-width: 400px;
	}

	:global(.dark) .audio-player {
		background-color: var(--color-surface-800);
		border-color: var(--color-surface-600);
	}
</style>

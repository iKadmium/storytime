<script lang="ts">
	import { onMount } from 'svelte';
	import Button from '$lib/components/Button/Button.svelte';
	import CharacterForm from '$lib/components/CharacterForm/CharacterForm.svelte';
	import CharacterList from '$lib/components/CharacterList/CharacterList.svelte';
	import CharacterModal from '$lib/components/CharacterModal/CharacterModal.svelte';
	import SelectionModal from '$lib/components/SelectionModal/SelectionModal.svelte';
	import DetailModal from '$lib/components/DetailModal/DetailModal.svelte';

	import type {
		Character,
		CreateCharacterRequest,
		UpdateCharacterRequest
	} from '$lib/models/character';
	import type { Message } from '$lib/models/chat';
	import {
		fetchCharacters,
		createCharacter,
		updateCharacter,
		deleteCharacter
	} from '$lib/services/character-service';
	import { getReferenceFiles } from '$lib/services/chatterbox-service';
	import { testCharacterWithPrompt } from '$lib/services/test-service';
	import { fetchPrompts } from '$lib/services/prompt-service';
	import { base64ToAudioURL, revokeAudioURL } from '$lib/utils/audio-utils';

	// State management
	let characters = $state<Character[]>([]);
	let referenceVoiceFiles = $state<string[]>([]);
	let availablePrompts = $state<string[]>([]);
	let isLoading = $state(false);
	let error = $state<string | null>(null);
	let isSubmitting = $state(false);

	// UI state
	let showForm = $state(false);
	let showModal = $state(false);
	let editingCharacter = $state<Character | undefined>(undefined);
	let viewingCharacter = $state<Character | undefined>(undefined);

	// Test functionality state
	let showPromptSelection = $state(false);
	let testingCharacter = $state<Character | undefined>(undefined);
	let testResult = $state<Message | null>(null);
	let showTestResult = $state(false);
	let audioURLs = $state<string[]>([]);

	// Load characters and voice files when component mounts
	onMount(() => {
		loadCharacters();
		loadReferenceVoiceFiles();
		loadAvailablePrompts();
	});

	async function loadCharacters() {
		try {
			isLoading = true;
			error = null;
			characters = await fetchCharacters();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load characters';
			console.error('Error loading characters:', err);
		} finally {
			isLoading = false;
		}
	}

	async function loadReferenceVoiceFiles() {
		try {
			referenceVoiceFiles = await getReferenceFiles();
		} catch (err) {
			console.warn('Failed to load reference voice files:', err);
			// Don't set error state for voice files as this is not critical
		}
	}

	async function loadAvailablePrompts() {
		try {
			const prompts = await fetchPrompts();
			availablePrompts = prompts.map((p) => p.title);
		} catch (err) {
			console.error('Error loading prompts:', err);
			// Don't show error for this as it's not critical
		}
	}

	function handleCreateNew() {
		editingCharacter = undefined;
		showForm = true;
	}

	function handleEdit(character: Character) {
		editingCharacter = character;
		showForm = true;
	}

	function handleView(character: Character) {
		viewingCharacter = character;
		showModal = true;
	}

	async function handleDelete(character: Character) {
		const confirmed = confirm(
			`Are you sure you want to delete "${character.name}"? This action cannot be undone.`
		);
		if (!confirmed) return;

		try {
			isSubmitting = true;
			error = null;
			await deleteCharacter(character.name);
			// Refresh the character list
			await loadCharacters();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete character';
			console.error('Error deleting character:', err);
		} finally {
			isSubmitting = false;
		}
	}

	async function handleFormSubmit(characterData: CreateCharacterRequest) {
		try {
			isSubmitting = true;
			error = null;

			if (editingCharacter) {
				// Update existing character
				const updates: UpdateCharacterRequest = {
					description: characterData.description,
					personality: characterData.personality,
					background: characterData.background,
					voice: characterData.voice
				};
				await updateCharacter(editingCharacter.name, updates);
			} else {
				// Create new character
				await createCharacter(characterData);
			}

			// Refresh the character list and close form
			await loadCharacters();
			handleFormCancel();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to save character';
			console.error('Error saving character:', err);
		} finally {
			isSubmitting = false;
		}
	}

	function handleFormCancel() {
		showForm = false;
		editingCharacter = undefined;
	}

	function handleModalClose() {
		showModal = false;
		viewingCharacter = undefined;
	}

	function handleModalEdit(character: Character) {
		handleModalClose();
		handleEdit(character);
	}

	function handleModalDelete(character: Character) {
		handleModalClose();
		handleDelete(character);
	}

	function handleTestCharacter(character: Character) {
		testingCharacter = character;
		showPromptSelection = true;
	}

	async function handlePromptSelect(prompt: string) {
		if (!testingCharacter) return;

		try {
			error = null;
			showPromptSelection = false;

			const result = await testCharacterWithPrompt(testingCharacter, prompt, false); // Don't save to chat history for character tests
			testResult = result;

			// Create audio URLs for base64 MP3 data
			if (result.audio && result.audio.length > 0) {
				audioURLs = result.audio.map((base64Data) => base64ToAudioURL(base64Data));
			}

			showTestResult = true;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to test character';
			console.error('Error testing character:', err);
		} finally {
			testingCharacter = undefined;
		}
	}

	function handlePromptSelectCancel() {
		showPromptSelection = false;
		testingCharacter = undefined;
	}

	function handleTestResultClose() {
		// Clean up audio URLs to prevent memory leaks
		audioURLs.forEach((url) => revokeAudioURL(url));
		audioURLs = [];
		showTestResult = false;
		testResult = null;
	}
</script>

<div class="container mx-auto max-w-7xl px-4 py-8">
	<!-- Header -->
	<div class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
		<div>
			<h1 class="h1 gradient-heading">Characters</h1>
			<p class="opacity-75">
				Manage your story characters with detailed descriptions, personalities, and backgrounds.
			</p>
		</div>

		{#if !showForm}
			<Button onclick={handleCreateNew} disabled={isLoading}>+ Create Character</Button>
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

	<!-- Character Form -->
	{#if showForm}
		<div class="mb-8">
			<CharacterForm
				character={editingCharacter}
				onSubmit={handleFormSubmit}
				onCancel={handleFormCancel}
				{isSubmitting}
				{referenceVoiceFiles}
				submitLabel={editingCharacter ? 'Update Character' : 'Create Character'}
			/>
		</div>
	{/if}

	<!-- Character List -->
	{#if !showForm}
		<CharacterList
			{characters}
			onEdit={handleEdit}
			onDelete={handleDelete}
			onView={handleView}
			onTest={handleTestCharacter}
			{isLoading}
		/>
	{/if}

	<!-- Refresh Button -->
	{#if !showForm && !isLoading && characters.length > 0}
		<div class="mt-8 text-center">
			<Button color="secondary" onclick={loadCharacters}>Refresh Characters</Button>
		</div>
	{/if}
</div>

<!-- Character Details Modal -->
<CharacterModal
	character={viewingCharacter}
	isOpen={showModal}
	onClose={handleModalClose}
	onEdit={handleModalEdit}
	onDelete={handleModalDelete}
	onTest={handleTestCharacter}
/>

<!-- Prompt Selection Modal -->
<SelectionModal
	title="Select Prompt for Testing"
	isOpen={showPromptSelection}
	onClose={handlePromptSelectCancel}
	onSelect={handlePromptSelect}
	options={availablePrompts}
	placeholder="Choose a prompt to test with"
/>

<!-- Test Result Modal -->
<DetailModal isOpen={showTestResult} onClose={handleTestResultClose} title="Character Test Result">
	{#snippet children()}
		{#if testResult}
			<div class="space-y-4">
				<div>
					<h4 class="h4 mb-2">Generated Text:</h4>
					{#each testResult.text as textItem}
						<p class="bg-surface-200-800 mb-2 rounded p-3 text-sm">{textItem}</p>
					{/each}
				</div>

				{#if testResult.audio && testResult.audio.length > 0}
					<div>
						<h4 class="h4 mb-2">Generated Audio:</h4>
						<div class="space-y-3">
							{#each audioURLs as audioURL, index}
								<div class="bg-surface-200-800 rounded p-3">
									<p class="mb-2 text-sm opacity-75">Audio {index + 1}:</p>
									<audio controls class="w-full">
										<source src={audioURL} type="audio/mp3" />
										Your browser does not support the audio element.
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

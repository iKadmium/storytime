<script lang="ts">
	import type { Character } from '$lib/models/character.js';
	import ItemCard from '$lib/components/ItemCard/ItemCard.svelte';
	import ItemListCard from '$lib/components/ItemListCard/ItemListCard.svelte';

	interface Props {
		characters: Character[];
		onEdit: (character: Character) => void;
		onDelete: (character: Character) => void;
		onView: (character: Character) => void;
		onTest?: (character: Character) => void;
		isLoading?: boolean;
	}

	let { characters, onEdit, onDelete, onView, onTest, isLoading = false }: Props = $props();

	function handleEdit(character: Character) {
		onEdit(character);
	}

	function handleDelete(character: Character) {
		onDelete(character);
	}

	function handleView(character: Character) {
		onView(character);
	}

	function handleTest(character: Character) {
		if (onTest) {
			onTest(character);
		}
	}
</script>

<div class="space-y-4">
	{#if isLoading}
		<div class="py-8 text-center">
			<div class="placeholder animate-pulse"></div>
		</div>
	{:else if characters.length === 0}
		<div class="py-8 text-center">
			<div class="text-lg opacity-75">No characters found.</div>
			<p class="mt-2 opacity-50">Create your first character to get started!</p>
		</div>
	{:else}
		<!-- Desktop Grid View -->
		<div class="hidden gap-6 md:grid md:grid-cols-2 lg:grid-cols-3">
			{#each characters as character (character.name)}
				<ItemCard
					title={character.name}
					description={character.description}
					badge={character.voice ? `🎤 ${character.voice.voiceName}` : undefined}
					badgeColor="secondary"
					executeLabel="Test"
					fields={[
						{ label: 'Personality', value: character.personality },
						{ label: 'Background', value: character.background },
						{ label: 'Voice', value: character.voice?.voiceName || 'Not set' }
					]}
					onView={() => handleView(character)}
					onEdit={() => handleEdit(character)}
					onDelete={() => handleDelete(character)}
					onExecute={onTest ? () => handleTest(character) : undefined}
				/>
			{/each}
		</div>

		<!-- Mobile List View -->
		<div class="space-y-4 md:hidden">
			{#each characters as character (character.name)}
				<ItemListCard
					title={character.name}
					description={character.description}
					badge={character.voice ? `🎤 ${character.voice.voiceName}` : undefined}
					badgeColor="secondary"
					executeLabel="Test"
					onView={() => handleView(character)}
					onEdit={() => handleEdit(character)}
					onDelete={() => handleDelete(character)}
					onExecute={onTest ? () => handleTest(character) : undefined}
				/>
			{/each}
		</div>

		<!-- Character Count -->
		<div class="mt-6 text-center">
			<p class="text-sm opacity-50">
				{characters.length} character{characters.length === 1 ? '' : 's'}
			</p>
		</div>
	{/if}
</div>

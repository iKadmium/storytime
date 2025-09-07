<script lang="ts">
	import type { Character } from '$lib/models/character.js';
	import Button from '$lib/components/Button/Button.svelte';

	interface Props {
		character?: Character;
		isOpen: boolean;
		onClose: () => void;
		onEdit?: (character: Character) => void;
		onDelete?: (character: Character) => void;
		onTest?: (character: Character) => void;
	}

	let { character, isOpen, onClose, onEdit, onDelete, onTest }: Props = $props();

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			onClose();
		}
	}

	function handleBackdropClick(event: MouseEvent) {
		if (event.target === event.currentTarget) {
			onClose();
		}
	}
</script>

{#if isOpen && character}
	<!-- Modal Backdrop -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
		onclick={handleBackdropClick}
		onkeydown={handleKeydown}
		role="dialog"
		aria-modal="true"
		aria-labelledby="character-modal-title"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="card preset-filled-surface-100-900 flex max-h-full w-full max-w-2xl flex-col shadow-2xl"
			onclick={(e) => e.stopPropagation()}
		>
			<!-- Header -->
			<header
				class="border-surface-500/20 flex flex-shrink-0 items-center justify-between border-b p-6"
			>
				<h2 id="character-modal-title" class="h2">
					{character.name}
				</h2>
				<Button onclick={onClose} preset="ghost-icon" size="sm" aria-label="Close modal">
					<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M6 18L18 6M6 6l12 12"
						></path>
					</svg>
				</Button>
			</header>

			<!-- Content -->
			<section class="min-h-0 flex-1 space-y-6 overflow-y-scroll p-6">
				<!-- Voice Info -->
				{#if character.voice}
					<div class="card preset-soft-secondary p-4">
						<div class="space-y-3">
							<div class="flex items-center gap-2">
								<span>🎤</span>
								<span class="font-semibold">Voice Configuration</span>
							</div>
							<div class="grid grid-cols-2 gap-3 text-sm">
								<div>
									<span class="font-medium">Name:</span>
									<span class="chip preset-filled-secondary ml-2">{character.voice.voiceName}</span>
								</div>
								<div>
									<span class="font-medium">Temperature:</span>
									<span class="ml-2">{character.voice.temperature}</span>
								</div>
								<div>
									<span class="font-medium">Exaggeration:</span>
									<span class="ml-2">{character.voice.exaggeration}</span>
								</div>
								<div>
									<span class="font-medium">CFG Weight:</span>
									<span class="ml-2">{character.voice.cfgWeight}</span>
								</div>
								<div>
									<span class="font-medium">Speed Factor:</span>
									<span class="ml-2">{character.voice.speedFactor}</span>
								</div>
							</div>
						</div>
					</div>
				{/if}

				<!-- Description -->
				<div>
					<h3 class="h3 mb-3 flex items-center gap-2">
						<span>📝</span>
						Description
					</h3>
					<div class="card preset-glass-surface p-4">
						<p class="whitespace-pre-wrap leading-relaxed">
							{character.description}
						</p>
					</div>
				</div>

				<!-- Personality -->
				<div>
					<h3 class="h3 mb-3 flex items-center gap-2">
						<span>🎭</span>
						Personality
					</h3>
					<div class="card preset-glass-surface p-4">
						<p class="whitespace-pre-wrap leading-relaxed">
							{character.personality}
						</p>
					</div>
				</div>

				<!-- Background -->
				<div>
					<h3 class="h3 mb-3 flex items-center gap-2">
						<span>📚</span>
						Background
					</h3>
					<div class="card preset-glass-surface p-4">
						<p class="whitespace-pre-wrap leading-relaxed">
							{character.background}
						</p>
					</div>
				</div>
			</section>

			<!-- Footer Actions -->
			<footer class="border-surface-500/20 flex flex-shrink-0 items-center gap-3 border-t p-6">
				{#if onTest}
					<Button onclick={() => onTest?.(character)} preset="outlined" color="secondary"
						>Test Character</Button
					>
				{/if}
				{#if onEdit}
					<Button onclick={() => onEdit?.(character)} preset="outlined">Edit Character</Button>
				{/if}
				{#if onDelete}
					<Button onclick={() => onDelete?.(character)} preset="outlined" color="error"
						>Delete Character</Button
					>
				{/if}
				<div class="flex-1"></div>
				<Button onclick={onClose} preset="filled" color="primary">Close</Button>
			</footer>
		</div>
	</div>
{/if}

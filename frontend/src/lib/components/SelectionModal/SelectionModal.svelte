<script lang="ts">
	import Button from '$lib/components/Button/Button.svelte';

	interface Props {
		title: string;
		isOpen: boolean;
		onClose: () => void;
		onSelect: (selection: string) => void;
		options: string[];
		placeholder?: string;
	}

	let { title, isOpen, onClose, onSelect, options, placeholder = 'Select an option' }: Props = $props();

	let selectedOption = $state('');

	function handleSubmit(event: Event) {
		event.preventDefault();
		if (selectedOption.trim()) {
			onSelect(selectedOption.trim());
		}
	}

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

{#if isOpen}
	<!-- Modal Backdrop -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
		onclick={handleBackdropClick}
		onkeydown={handleKeydown}
		role="dialog"
		aria-modal="true"
		aria-labelledby="modal-title"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div class="w-full max-w-md overflow-hidden card preset-filled-surface-50-950" onclick={(e) => e.stopPropagation()}>
			<!-- Header -->
			<header class="border-b border-surface-500/20 p-4">
				<h2 id="modal-title" class="h2">{title}</h2>
			</header>

			<!-- Content -->
			<form class="p-6" onsubmit={handleSubmit}>
				<div class="mb-4">
					<label for="selection" class="mb-2 block text-sm font-medium">
						{placeholder}
					</label>
					<select id="selection" bind:value={selectedOption} class="select" required>
						<option value="">-- Select --</option>
						{#each options as option, index (index)}
							<option value={option}>{option}</option>
						{/each}
					</select>
				</div>

				<!-- Actions -->
				<div class="flex justify-end space-x-2">
					<Button type="button" preset="ghost" onclick={onClose}>Cancel</Button>
					<Button type="submit" preset="filled" disabled={!selectedOption.trim()}>Select</Button>
				</div>
			</form>
		</div>
	</div>
{/if}

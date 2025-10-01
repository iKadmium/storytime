<script lang="ts">
	import Button from '$lib/components/Button/Button.svelte';
	import type { Snippet } from 'svelte';

	interface Props {
		title: string;
		isOpen: boolean;
		onClose: () => void;
		onEdit?: () => void;
		onDelete?: () => void;
		onTest?: () => void;
		editLabel?: string;
		deleteLabel?: string;
		testLabel?: string;
		children: Snippet;
	}

	let { title, isOpen, onClose, onEdit, onDelete, onTest, editLabel = 'Edit', deleteLabel = 'Delete', testLabel = 'Test', children }: Props = $props();

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
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="flex max-h-full w-full max-w-2xl flex-col card preset-filled-surface-100-900 shadow-2xl" onclick={(e) => e.stopPropagation()}>
			<!-- Header -->
			<header class="flex flex-shrink-0 items-center justify-between border-b border-surface-500/20 p-6">
				<h2 id="modal-title" class="h2">
					{title}
				</h2>
				<Button onclick={onClose} preset="ghost-icon" size="sm" aria-label="Close modal">
					<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
					</svg>
				</Button>
			</header>

			<!-- Content -->
			<section class="min-h-0 flex-1 overflow-y-scroll p-6">
				{@render children()}
			</section>

			<!-- Footer Actions -->
			<footer class="flex flex-shrink-0 items-center gap-3 border-t border-surface-500/20 p-6">
				{#if onTest}
					<Button onclick={onTest} preset="outlined" color="secondary">{testLabel}</Button>
				{/if}
				{#if onEdit}
					<Button onclick={onEdit} preset="outlined">{editLabel}</Button>
				{/if}
				{#if onDelete}
					<Button onclick={onDelete} preset="outlined" color="error">{deleteLabel}</Button>
				{/if}
				<div class="flex-1"></div>
				<Button onclick={onClose} preset="filled" color="primary">Close</Button>
			</footer>
		</div>
	</div>
{/if}

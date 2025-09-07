<script lang="ts">
	import Button from '$lib/components/Button/Button.svelte';

	interface Props {
		title: string;
		description: string;
		badge?: string;
		badgeColor?: string;
		executeLabel?: string;
		onView: () => void;
		onEdit: () => void;
		onDelete: () => void;
		onExecute?: () => void;
	}

	let {
		title,
		description,
		badge,
		badgeColor = 'secondary',
		executeLabel = 'Execute',
		onView,
		onEdit,
		onDelete,
		onExecute
	}: Props = $props();

	function truncateText(text: string, maxLength: number = 100): string {
		if (text.length <= maxLength) return text;
		return text.substring(0, maxLength) + '...';
	}
</script>

<div class="card preset-glass-surface flex flex-col">
	<div class="flex-1 p-4">
		<div class="mb-3 flex items-start justify-between">
			<div>
				<h3 class="h3">{title}</h3>
				{#if badge}
					<span class="chip preset-soft-{badgeColor} text-xs">{badge}</span>
				{/if}
			</div>
			<Button
				onclick={onView}
				preset="ghost-icon"
				color="primary"
				size="sm"
				type="button"
				aria-label={`View ${title} details`}
			>
				<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
					></path>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
					></path>
				</svg>
			</Button>
		</div>

		<p class="text-sm opacity-75">
			{truncateText(description)}
		</p>
	</div>

	<footer class="border-surface-500/20 border-t p-3">
		<div class="flex justify-end gap-2">
			{#if onExecute}
				<Button size="sm" preset="outlined" color="primary" onclick={onExecute}
					>{executeLabel}</Button
				>
			{/if}
			<Button size="sm" preset="outlined" onclick={onEdit}>Edit</Button>
			<Button size="sm" preset="outlined" color="error" onclick={onDelete}>Delete</Button>
		</div>
	</footer>
</div>

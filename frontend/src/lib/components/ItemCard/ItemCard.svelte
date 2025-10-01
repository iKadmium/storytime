<script lang="ts">
	import Button from '$lib/components/Button/Button.svelte';

	interface Props {
		title: string;
		description: string;
		fields: Array<{ label: string; value: string }>;
		badge?: string;
		badgeColor?: string;
		executeLabel?: string;
		onView: () => void;
		onEdit: () => void;
		onDelete: () => void;
		onExecute?: () => void;
	}

	let { title, description, fields, badge, badgeColor = 'secondary', executeLabel = 'Execute', onView, onEdit, onDelete, onExecute }: Props = $props();

	function truncateText(text: string, maxLength: number = 150): string {
		if (text.length <= maxLength) return text;
		return text.substring(0, maxLength) + '...';
	}
</script>

<div class="hover:preset-glass-primary flex flex-col card preset-filled-surface-100-900 transition-all">
	<div class="flex-1 p-6">
		<div class="mb-4">
			<h3 class="mb-2 h3">{title}</h3>
			{#if badge}
				<span class="chip preset-soft-{badgeColor}">{badge}</span>
			{/if}
		</div>

		<div class="space-y-3">
			<div>
				<h4 class="mb-1 h4">Description:</h4>
				<p class="text-sm opacity-75">{truncateText(description)}</p>
			</div>

			{#each fields as field, index (index)}
				<div>
					<h4 class="mb-1 h4">{field.label}:</h4>
					<p class="text-sm opacity-75">{truncateText(field.value)}</p>
				</div>
			{/each}
		</div>
	</div>

	<footer class="border-t border-surface-500/20 p-4">
		<div class="flex flex-wrap justify-end gap-2">
			{#if onExecute}
				<Button size="sm" preset="outlined" color="primary" onclick={onExecute}>{executeLabel}</Button>
			{/if}
			<Button size="sm" preset="outlined" onclick={onView}>View</Button>
			<Button size="sm" preset="outlined" onclick={onEdit}>Edit</Button>
			<Button size="sm" preset="outlined" color="error" onclick={onDelete}>Delete</Button>
		</div>
	</footer>
</div>

<script lang="ts">
	interface Field {
		icon: string;
		label: string;
		value: string;
	}

	interface Props {
		fields: Field[];
	}

	let { fields }: Props = $props();
</script>

<div class="space-y-6">
	{#each fields as field, index (index)}
		<div>
			<h3 class="mb-3 flex items-center gap-2 h3">
				<span>{field.icon}</span>
				{field.label}
			</h3>
			<div class="preset-glass-surface card p-4">
				<p class="leading-relaxed whitespace-pre-wrap">
					{field.value}
				</p>
			</div>
		</div>
	{/each}
</div>

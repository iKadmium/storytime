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
	{#each fields as field}
		<div>
			<h3 class="h3 mb-3 flex items-center gap-2">
				<span>{field.icon}</span>
				{field.label}
			</h3>
			<div class="card preset-glass-surface p-4">
				<p class="whitespace-pre-wrap leading-relaxed">
					{field.value}
				</p>
			</div>
		</div>
	{/each}
</div>

<script lang="ts">
	interface Option {
		value: string;
		label: string;
	}

	interface Props {
		label: string;
		value: string;
		onChange: (value: string) => void;
		options: Option[];
		placeholder?: string;
		required?: boolean;
		disabled?: boolean;
		error?: string;
	}

	let { label, value = $bindable(), onChange, options, placeholder = '', required = false, disabled = false, error = '' }: Props = $props();

	function handleChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		onChange(target.value);
	}
</script>

<label class="label">
	<span>
		{label}
		{#if required}<span class="text-error-500">*</span>{/if}
	</span>
	<select class="select" bind:value onchange={handleChange} {disabled} {required}>
		{#if placeholder}
			<option value="">{placeholder}</option>
		{/if}
		{#each options as option, index (index)}
			<option value={option.value}>{option.label}</option>
		{/each}
	</select>
	{#if error}
		<small class="text-error-500">{error}</small>
	{/if}
</label>

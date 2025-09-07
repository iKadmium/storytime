<script lang="ts">
	interface Props {
		label: string;
		value: string;
		onInput: (value: string) => void;
		placeholder?: string;
		required?: boolean;
		disabled?: boolean;
		error?: string;
		type?: 'text' | 'email' | 'password' | 'url';
		maxlength?: number;
	}

	let {
		label,
		value = $bindable(),
		onInput,
		placeholder = '',
		required = false,
		disabled = false,
		error = '',
		type = 'text',
		maxlength
	}: Props = $props();

	function handleInput(event: Event) {
		const target = event.target as HTMLInputElement;
		onInput(target.value);
	}
</script>

<label class="label">
	<span>
		{label}
		{#if required}<span class="text-error-500">*</span>{/if}
	</span>
	<input
		{type}
		class="input"
		bind:value
		oninput={handleInput}
		{disabled}
		{placeholder}
		{required}
		{maxlength}
	/>
	{#if error}
		<small class="text-error-500">{error}</small>
	{/if}
</label>

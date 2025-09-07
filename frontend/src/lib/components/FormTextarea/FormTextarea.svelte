<script lang="ts">
	interface Props {
		label: string;
		value: string;
		onInput: (value: string) => void;
		placeholder?: string;
		required?: boolean;
		disabled?: boolean;
		error?: string;
		rows?: number;
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
		rows = 3,
		maxlength
	}: Props = $props();

	function handleInput(event: Event) {
		const target = event.target as HTMLTextAreaElement;
		onInput(target.value);
	}
</script>

<label class="label">
	<span>
		{label}
		{#if required}<span class="text-error-500">*</span>{/if}
	</span>
	<textarea
		class="textarea"
		bind:value
		oninput={handleInput}
		{disabled}
		{placeholder}
		{required}
		{rows}
		{maxlength}
	></textarea>
	{#if error}
		<small class="text-error-500">{error}</small>
	{/if}
</label>

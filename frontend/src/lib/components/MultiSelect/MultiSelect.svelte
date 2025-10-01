<script lang="ts">
	interface Props {
		label: string;
		selected: string[];
		options: string[];
		placeholder?: string;
		required?: boolean;
		disabled?: boolean;
		error?: string;
		minItems?: number;
	}

	let { label, selected = $bindable([]), options, placeholder = '', required = false, disabled = false, error = '', minItems = 1 }: Props = $props();

	let isOpen = $state(false);
	let searchTerm = $state('');
	let containerElement = $state<HTMLDivElement>();

	let filteredOptions = $derived(options.filter((option) => option.toLowerCase().includes(searchTerm.toLowerCase()) && !selected.includes(option)));

	function toggleOption(option: string) {
		if (selected.includes(option)) {
			selected = selected.filter((item) => item !== option);
		} else {
			selected = [...selected, option];
		}
	}

	function removeOption(option: string) {
		selected = selected.filter((item) => item !== option);
	}

	function toggleDropdown() {
		console.log('toggleDropdown called, current isOpen:', isOpen, 'disabled:', disabled);
		if (!disabled) {
			isOpen = !isOpen;
			console.log('toggleDropdown after toggle, isOpen:', isOpen);
		}
	}

	function handleClickOutside(event: Event) {
		const target = event.target as HTMLElement;
		console.log('handleClickOutside called, containerElement:', containerElement, 'target:', target);
		if (containerElement && !containerElement.contains(target)) {
			console.log('Closing dropdown due to outside click');
			isOpen = false;
		}
	}

	$effect(() => {
		console.log('Effect running, isOpen:', isOpen);
		if (isOpen) {
			// Add a small delay to prevent immediate closing
			setTimeout(() => {
				document.addEventListener('click', handleClickOutside);
			}, 0);
			return () => {
				document.removeEventListener('click', handleClickOutside);
			};
		}
	});
</script>

<div class="form-group">
	<div class="label">
		<span>
			{label}
			{#if required}<span class="text-red-500">*</span>{/if}
		</span>
	</div>

	<div class="multiselect-container relative" bind:this={containerElement}>
		<!-- Selected items display -->
		<div
			class="input flex min-h-[2.5rem] cursor-pointer flex-wrap gap-1 p-2"
			class:input-error={error}
			role="button"
			tabindex="0"
			onclick={toggleDropdown}
			onkeydown={(e) => {
				if (e.key === 'Enter' || e.key === ' ') {
					e.preventDefault();
					toggleDropdown();
				}
			}}
		>
			{#each selected as item, index (index)}
				<span class="inline-flex items-center gap-1 rounded bg-primary-500 px-2 py-1 text-sm text-white">
					{item}
					{#if !disabled}
						<button
							type="button"
							class="ml-1 text-white hover:text-gray-200"
							onclick={(e) => {
								e.stopPropagation();
								removeOption(item);
							}}
						>
							×
						</button>
					{/if}
				</span>
			{/each}

			{#if selected.length === 0}
				<span class="py-1 text-sm text-gray-500">{placeholder}</span>
			{/if}

			<!-- Dropdown toggle button -->
			{#if !disabled}
				<button
					type="button"
					class="ml-auto text-gray-500 hover:text-gray-700"
					onclick={(e) => {
						e.stopPropagation();
						toggleDropdown();
					}}
				>
					{isOpen ? '▲' : '▼'}
				</button>
			{/if}
		</div>

		<!-- Dropdown -->
		{#if isOpen && !disabled}
			<div
				class="absolute z-10 mt-1 max-h-48 w-full overflow-y-auto rounded-lg border border-gray-300 bg-white shadow-lg dark:border-gray-600 dark:bg-gray-800"
			>
				<!-- Search input -->
				<div class="border-b border-gray-200 p-2 dark:border-gray-600">
					<input
						type="text"
						class="w-full rounded border border-gray-300 bg-white px-2 py-1 text-sm dark:border-gray-600 dark:bg-gray-700 dark:text-white"
						placeholder="Search options..."
						bind:value={searchTerm}
					/>
				</div>

				<!-- Options list -->
				<div class="max-h-32 overflow-y-auto">
					{#each filteredOptions as option, index (index)}
						<button
							type="button"
							class="w-full cursor-pointer border-none bg-transparent px-3 py-2 text-left text-sm text-gray-900 hover:bg-gray-100 dark:text-gray-100 dark:hover:bg-gray-700"
							onclick={() => toggleOption(option)}
						>
							{option}
						</button>
					{:else}
						<div class="px-3 py-2 text-sm text-gray-500 dark:text-gray-400">
							{searchTerm ? 'No matches found' : 'No more options available'}
						</div>
					{/each}
				</div>
			</div>
		{/if}
	</div>

	{#if error}
		<div class="mt-1 text-sm text-error-500">{error}</div>
	{:else if minItems > 1}
		<div class="mt-1 text-sm text-surface-500">
			Select at least {minItems} option{minItems > 1 ? 's' : ''}
		</div>
	{/if}
</div>

<style>
	.multiselect-container {
		position: relative;
	}
</style>

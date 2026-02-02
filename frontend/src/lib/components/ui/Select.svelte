<script lang="ts">
	interface Props {
		value?: any;
		options?: { value: any; label: string }[];
		placeholder?: string;
		label?: string;
		error?: string;
		disabled?: boolean;
		required?: boolean;
		id?: string;
		onchange?: (e: Event) => void;
	}

	let props: Props = $props();
	let value = $state(props.value ?? '');
	let options = $derived(props.options ?? []);
	let placeholder = $derived(props.placeholder ?? 'Select...');
	let label = $derived(props.label ?? '');
	let error = $derived(props.error ?? '');
	let disabled = $derived(props.disabled ?? false);
	let required = $derived(props.required ?? false);
	let id = $derived(props.id ?? '');
	let onchange = $derived(props.onchange);

	// Sync with external value changes
	$effect(() => {
		if (props.value !== undefined && props.value !== value) {
			value = props.value;
		}
	});
</script>

<div class="w-full">
	{#if label}
		<label for={id} class="block text-sm font-medium text-gray-700 mb-1">
			{label}
			{#if required}
				<span class="text-red-500">*</span>
			{/if}
		</label>
	{/if}

	<select
		{id}
		bind:value
		{disabled}
		{required}
		class="w-full px-3 py-2 border rounded-md shadow-sm
      {error ? 'border-red-500' : 'border-gray-300'}
      focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500
      disabled:bg-gray-100 disabled:cursor-not-allowed"
		{onchange}
	>
		{#if placeholder}
			<option value="" disabled selected>{placeholder}</option>
		{/if}
		{#each options as option}
			<option value={option.value}>{option.label}</option>
		{/each}
	</select>

	{#if error}
		<p class="mt-1 text-sm text-red-600">{error}</p>
	{/if}
</div>

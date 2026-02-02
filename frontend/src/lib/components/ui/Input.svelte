<script lang="ts">
	interface Props {
		type?: string;
		value?: any;
		placeholder?: string;
		label?: string;
		error?: string;
		disabled?: boolean;
		required?: boolean;
		id?: string;
		min?: string | number;
		max?: string | number;
		step?: string | number;
		oninput?: (e: Event) => void;
		onchange?: (e: Event) => void;
		onblur?: (e: FocusEvent) => void;
	}

	let props: Props = $props();
	let value = $state(props.value ?? '');
	let type = $derived(props.type ?? 'text');
	let placeholder = $derived(props.placeholder ?? '');
	let label = $derived(props.label ?? '');
	let error = $derived(props.error ?? '');
	let disabled = $derived(props.disabled ?? false);
	let required = $derived(props.required ?? false);
	let id = $derived(props.id ?? '');
	let min = $derived(props.min);
	let max = $derived(props.max);
	let step = $derived(props.step);
	let oninput = $derived(props.oninput);
	let onchange = $derived(props.onchange);
	let onblur = $derived(props.onblur);

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

	<input
		{id}
		{type}
		bind:value
		{placeholder}
		{disabled}
		{required}
		{min}
		{max}
		{step}
		class="w-full px-4 py-3 min-h-[44px] text-base border rounded-md shadow-sm touch-manipulation
      {error ? 'border-red-500' : 'border-gray-300'}
      focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500
      disabled:bg-gray-100 disabled:cursor-not-allowed"
		{oninput}
		{onchange}
		{onblur}
	/>

	{#if error}
		<p class="mt-1 text-sm text-red-600">{error}</p>
	{/if}
</div>

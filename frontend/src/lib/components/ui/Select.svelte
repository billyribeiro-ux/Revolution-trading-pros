<script lang="ts">
	export let value: any = '';
	export let options: { value: any; label: string }[] = [];
	export let placeholder: string = 'Select...';
	export let label: string = '';
	export let error: string = '';
	export let disabled: boolean = false;
	export let required: boolean = false;
	export let id: string = '';
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
		on:change
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

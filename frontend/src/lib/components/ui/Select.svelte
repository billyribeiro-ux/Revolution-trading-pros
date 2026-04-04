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

	let {
		value = $bindable(''),
		options = [],
		placeholder = 'Select...',
		label = '',
		error = '',
		disabled = false,
		required = false,
		id = '',
		onchange
	}: Props = $props();
</script>

<div class="standalone-select-wrapper">
	{#if label}
		<label for={id} class="standalone-select-label">
			{label}
			{#if required}
				<span class="standalone-select-required">*</span>
			{/if}
		</label>
	{/if}

	<select
		{id}
		bind:value
		{disabled}
		{required}
		class="standalone-select"
		data-error={error ? '' : undefined}
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
		<p class="standalone-select-error">{error}</p>
	{/if}
</div>

<style>
	.standalone-select-wrapper {
		inline-size: 100%;
	}

	.standalone-select-label {
		display: block;
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(0.4 0.01 265);
		margin-block-end: var(--space-1);
	}

	.standalone-select-required {
		color: oklch(0.58 0.24 27);
	}

	.standalone-select {
		inline-size: 100%;
		padding-inline: var(--space-3);
		padding-block: var(--space-2);
		border: 1px solid var(--border);
		border-radius: var(--radius-md);
		box-shadow: var(--shadow-sm);

		&:focus {
			outline: none;
			box-shadow: 0 0 0 2px oklch(0.55 0.2 260);
			border-color: oklch(0.55 0.2 260);
		}

		&:disabled {
			background-color: oklch(0.97 0.002 265);
			cursor: not-allowed;
		}

		&[data-error] {
			border-color: oklch(0.58 0.24 27);
		}
	}

	.standalone-select-error {
		margin-block-start: var(--space-1);
		font-size: var(--text-sm);
		color: oklch(0.55 0.24 27);
	}
</style>

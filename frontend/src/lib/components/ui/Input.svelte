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

	let {
		value = $bindable(''),
		type = 'text',
		placeholder = '',
		label = '',
		error = '',
		disabled = false,
		required = false,
		id = '',
		min,
		max,
		step,
		oninput,
		onchange,
		onblur
	}: Props = $props();
</script>

<div class="standalone-input-wrapper">
	{#if label}
		<label for={id} class="standalone-input-label">
			{label}
			{#if required}
				<span class="standalone-input-required">*</span>
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
		class="standalone-input"
		data-error={error ? '' : undefined}
		oninput={(e) => {
			if (oninput) oninput(e);
		}}
		onchange={(e) => {
			if (onchange) onchange(e);
		}}
		onblur={(e) => {
			if (onblur) onblur(e);
		}}
	/>

	{#if error}
		<p class="standalone-input-error">{error}</p>
	{/if}
</div>

<style>
	.standalone-input-wrapper {
		inline-size: 100%;
	}

	.standalone-input-label {
		display: block;
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(0.4 0.01 265);
		margin-block-end: var(--space-1);
	}

	.standalone-input-required {
		color: oklch(0.58 0.24 27);
	}

	.standalone-input {
		inline-size: 100%;
		padding-inline: var(--space-4);
		padding-block: var(--space-3);
		min-block-size: 44px;
		font-size: var(--text-base);
		border: 1px solid var(--border);
		border-radius: var(--radius-md);
		box-shadow: var(--shadow-sm);
		touch-action: manipulation;

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

	.standalone-input-error {
		margin-block-start: var(--space-1);
		font-size: var(--text-sm);
		color: oklch(0.55 0.24 27);
	}
</style>

<script lang="ts">
	import type { Attachment } from 'svelte/attachments';
	import type { HTMLInputAttributes, HTMLInputTypeAttribute } from 'svelte/elements';
	import { type WithElementRef } from '$lib/utils.js';

	type InputType = Exclude<HTMLInputTypeAttribute, 'file'>;

	type Props = WithElementRef<
		Omit<HTMLInputAttributes, 'type'> &
			({ type: 'file'; files?: FileList } | { type?: InputType; files?: undefined })
	>;

	let {
		ref = $bindable(null),
		value = $bindable(),
		type,
		files = $bindable(),
		class: className,
		'data-slot': dataSlot = 'input',
		...restProps
	}: Props = $props();

	const captureRef: Attachment<HTMLInputElement> = (node) => {
		ref = node;
		return () => {
			if (ref === node) {
				ref = null;
			}
		};
	};
</script>

{#if type === 'file'}
	<input
		{@attach captureRef}
		data-slot={dataSlot}
		class={['ui-input', 'ui-input--file', className]}
		type="file"
		bind:files
		bind:value
		{...restProps}
	/>
{:else}
	<input
		{@attach captureRef}
		data-slot={dataSlot}
		class={['ui-input', className]}
		{type}
		bind:value
		{...restProps}
	/>
{/if}

<style>
	.ui-input {
		display: flex;
		width: 100%;
		min-width: 0;
		min-height: 44px;
		border: 1px solid var(--input-border, #cbd5e1);
		border-radius: 0.375rem;
		background: var(--input-background, #ffffff);
		color: var(--input-foreground, #0f172a);
		box-shadow: 0 1px 2px rgba(15, 23, 42, 0.06);
		padding: 0.75rem 1rem;
		font-size: 1rem;
		line-height: 1.5rem;
		outline: none;
		touch-action: manipulation;
		transition:
			border-color 0.18s ease,
			box-shadow 0.18s ease,
			color 0.18s ease,
			background-color 0.18s ease;
	}

	.ui-input--file {
		padding-top: 0.75rem;
		padding-bottom: 0;
		font-weight: 500;
	}

	.ui-input::placeholder {
		color: var(--input-placeholder, #64748b);
	}

	.ui-input::selection {
		background: var(--input-selection-background, #2563eb);
		color: var(--input-selection-foreground, #ffffff);
	}

	.ui-input:focus-visible {
		border-color: var(--input-ring, #3b82f6);
		box-shadow: 0 0 0 3px var(--input-ring-shadow, rgba(59, 130, 246, 0.28));
	}

	.ui-input[aria-invalid='true'] {
		border-color: var(--input-invalid, #dc2626);
		box-shadow: 0 0 0 3px var(--input-invalid-shadow, rgba(220, 38, 38, 0.2));
	}

	.ui-input:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	@media (prefers-color-scheme: dark) {
		.ui-input {
			--input-border: #475569;
			--input-background: rgba(30, 41, 59, 0.3);
			--input-foreground: #f8fafc;
			--input-placeholder: #94a3b8;
			--input-ring-shadow: rgba(59, 130, 246, 0.5);
			--input-invalid-shadow: rgba(220, 38, 38, 0.4);
		}
	}
</style>

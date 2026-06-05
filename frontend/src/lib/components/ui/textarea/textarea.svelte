<script lang="ts">
	import type { Attachment } from 'svelte/attachments';
	import { type WithElementRef, type WithoutChildren } from '$lib/utils.js';
	import type { HTMLTextareaAttributes } from 'svelte/elements';

	let {
		ref = $bindable(null),
		value = $bindable(),
		class: className,
		'data-slot': dataSlot = 'textarea',
		...restProps
	}: WithoutChildren<WithElementRef<HTMLTextareaAttributes>> = $props();

	const captureRef: Attachment<HTMLTextAreaElement> = (node) => {
		ref = node;
		return () => {
			if (ref === node) {
				ref = null;
			}
		};
	};
</script>

<textarea
	{@attach captureRef}
	data-slot={dataSlot}
	class={['ui-textarea', className]}
	bind:value
	{...restProps}
></textarea>

<style>
	.ui-textarea {
		display: flex;
		width: 100%;
		min-height: 88px;
		field-sizing: content;
		border: 1px solid var(--textarea-border, #cbd5e1);
		border-radius: 0.375rem;
		background: var(--textarea-background, transparent);
		color: var(--textarea-foreground, #0f172a);
		box-shadow: 0 1px 2px rgba(15, 23, 42, 0.06);
		padding: 0.75rem 1rem;
		font-size: 1rem;
		line-height: 1.5rem;
		outline: none;
		resize: vertical;
		touch-action: manipulation;
		transition:
			border-color 0.18s ease,
			box-shadow 0.18s ease,
			color 0.18s ease,
			background-color 0.18s ease;
	}

	.ui-textarea::placeholder {
		color: var(--textarea-placeholder, #64748b);
	}

	.ui-textarea:focus-visible {
		border-color: var(--textarea-ring, #3b82f6);
		box-shadow: 0 0 0 3px var(--textarea-ring-shadow, rgba(59, 130, 246, 0.28));
	}

	.ui-textarea[aria-invalid='true'] {
		border-color: var(--textarea-invalid, #dc2626);
		box-shadow: 0 0 0 3px var(--textarea-invalid-shadow, rgba(220, 38, 38, 0.2));
	}

	.ui-textarea:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	@media (prefers-color-scheme: dark) {
		.ui-textarea {
			--textarea-border: #475569;
			--textarea-background: rgba(30, 41, 59, 0.3);
			--textarea-foreground: #f8fafc;
			--textarea-placeholder: #94a3b8;
			--textarea-ring-shadow: rgba(59, 130, 246, 0.5);
			--textarea-invalid-shadow: rgba(220, 38, 38, 0.4);
		}
	}
</style>

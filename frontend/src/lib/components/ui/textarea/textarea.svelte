<script lang="ts">
	import type { WithElementRef, WithoutChildren } from '$lib/utils.js';
	import type { HTMLTextareaAttributes } from 'svelte/elements';

	let {
		ref = $bindable(null),
		value = $bindable(),
		class: className,
		'data-slot': dataSlot = 'textarea',
		...restProps
	}: WithoutChildren<WithElementRef<HTMLTextareaAttributes>> = $props();
</script>

<textarea
	bind:this={ref}
	data-slot={dataSlot}
	class={className}
	bind:value
	{...restProps}
></textarea>

<style>
	:global([data-slot='textarea']) {
		display: flex;
		field-sizing: content;
		min-block-size: 88px;
		inline-size: 100%;
		border-radius: var(--radius-md);
		border: 1px solid var(--input);
		background-color: transparent;
		padding-inline: var(--space-4);
		padding-block: var(--space-3);
		font-size: var(--text-base);
		box-shadow: var(--shadow-xs);
		outline: none;
		touch-action: manipulation;
		transition: color var(--duration-fast) var(--ease-default),
			box-shadow var(--duration-fast) var(--ease-default);

		&::placeholder {
			color: var(--muted-foreground);
		}

		&:focus-visible {
			border-color: var(--ring);
			box-shadow: 0 0 0 3px oklch(from var(--ring) l c h / 50%);
		}

		&[aria-invalid='true'] {
			border-color: var(--destructive);
			box-shadow: 0 0 0 3px oklch(from var(--destructive) l c h / 20%);
		}

		&:disabled {
			cursor: not-allowed;
			opacity: 0.5;
		}
	}

	:global(.dark [data-slot='textarea']) {
		background-color: oklch(from var(--input) l c h / 30%);
	}

	:global(.dark [data-slot='textarea'][aria-invalid='true']) {
		box-shadow: 0 0 0 3px oklch(from var(--destructive) l c h / 40%);
	}
</style>

<script lang="ts">
	import { Select as SelectPrimitive } from 'bits-ui';
	import { CaretDown as ChevronDownIcon } from 'phosphor-svelte';
	import type { WithoutChild } from '$lib/utils.js';

	type TriggerProps = WithoutChild<SelectPrimitive.TriggerProps> & {
		size?: 'sm' | 'default';
	};

	let {
		ref = $bindable(null),
		class: className,
		children,
		size = 'default',
		...restProps
	}: TriggerProps = $props();
</script>

<SelectPrimitive.Trigger
	bind:ref
	data-slot="select-trigger"
	data-size={size}
	class={className}
	{...restProps}
>
	{@render children?.()}
	<ChevronDownIcon class="size-4 opacity-50" />
</SelectPrimitive.Trigger>

<style>
	:global([data-slot='select-trigger']) {
		display: flex;
		inline-size: 100%;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-2);
		border-radius: var(--radius-md);
		border: 1px solid var(--input);
		background-color: transparent;
		padding-inline: var(--space-4);
		padding-block: var(--space-3);
		font-size: var(--text-base);
		white-space: nowrap;
		box-shadow: var(--shadow-xs);
		outline: none;
		user-select: none;
		min-block-size: 44px;
		touch-action: manipulation;
		transition:
			color var(--duration-fast) var(--ease-default),
			box-shadow var(--duration-fast) var(--ease-default);

		@media (min-width: 640px) {
			inline-size: fit-content;
		}

		&[data-placeholder] {
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

		& :global(svg) {
			pointer-events: none;
			flex-shrink: 0;
		}

		& :global(svg:not([class*='size-'])) {
			inline-size: 1rem;
			block-size: 1rem;
		}

		& :global(svg:not([class*='text-'])) {
			color: var(--muted-foreground);
		}

		& :global([data-slot='select-value']) {
			display: flex;
			align-items: center;
			gap: var(--space-2);
			overflow: hidden;
			text-overflow: ellipsis;
			display: -webkit-box;
			-webkit-line-clamp: 1;
			line-clamp: 1;
			-webkit-box-orient: vertical;
		}
	}

	:global(.dark [data-slot='select-trigger']) {
		background-color: oklch(from var(--input) l c h / 30%);

		&:hover {
			background-color: oklch(from var(--input) l c h / 50%);
		}
	}

	:global(.dark [data-slot='select-trigger'][aria-invalid='true']) {
		box-shadow: 0 0 0 3px oklch(from var(--destructive) l c h / 40%);
	}

	:global(.select-chevron) {
		inline-size: 1rem;
		block-size: 1rem;
		opacity: 0.5;
	}
</style>

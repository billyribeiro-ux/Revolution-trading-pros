<script lang="ts" module>
	// eslint-disable-next-line no-import-assign -- Svelte module context re-export
	export { type BadgeVariant } from './badge.types';
</script>

<script lang="ts">
	import type { HTMLAnchorAttributes } from 'svelte/elements';
	import type { WithElementRef } from '$lib/utils.js';
	import type { BadgeVariant } from './badge.types';

	let {
		ref = $bindable(null),
		href,
		class: className,
		variant = 'default',
		children,
		...restProps
	}: WithElementRef<HTMLAnchorAttributes> & {
		variant?: BadgeVariant;
	} = $props();
</script>

<svelte:element
	this={href ? 'a' : 'span'}
	bind:this={ref}
	data-slot="badge"
	data-variant={variant}
	{href}
	class={className}
	{...restProps}
>
	{@render children?.()}
</svelte:element>

<style>
	:global([data-slot='badge']) {
		display: inline-flex;
		inline-size: fit-content;
		flex-shrink: 0;
		align-items: center;
		justify-content: center;
		gap: var(--space-1);
		overflow: hidden;
		border-radius: var(--radius-full);
		border: 1px solid transparent;
		padding-inline: var(--space-2);
		padding-block: 2px;
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
		white-space: nowrap;
		transition: color var(--duration-fast) var(--ease-default),
			box-shadow var(--duration-fast) var(--ease-default);

		&:focus-visible {
			border-color: var(--ring);
			box-shadow: 0 0 0 3px oklch(from var(--ring) l c h / 50%);
		}

		& > :global(svg) {
			pointer-events: none;
			inline-size: 0.75rem;
			block-size: 0.75rem;
		}
	}

	:global([data-slot='badge'][data-variant='default']) {
		background-color: var(--primary);
		color: var(--primary-foreground);
		border-color: transparent;
	}

	:global(a[data-slot='badge'][data-variant='default']:hover) {
		background-color: oklch(from var(--primary) l c h / 90%);
	}

	:global([data-slot='badge'][data-variant='secondary']) {
		background-color: var(--secondary);
		color: var(--secondary-foreground);
		border-color: transparent;
	}

	:global(a[data-slot='badge'][data-variant='secondary']:hover) {
		background-color: oklch(from var(--secondary) l c h / 90%);
	}

	:global([data-slot='badge'][data-variant='destructive']) {
		background-color: var(--destructive);
		color: oklch(1 0 0);
		border-color: transparent;
	}

	:global(.dark [data-slot='badge'][data-variant='destructive']) {
		background-color: oklch(from var(--destructive) l c h / 70%);
	}

	:global(a[data-slot='badge'][data-variant='destructive']:hover) {
		background-color: oklch(from var(--destructive) l c h / 90%);
	}

	:global([data-slot='badge'][data-variant='outline']) {
		color: var(--foreground);
	}

	:global(a[data-slot='badge'][data-variant='outline']:hover) {
		background-color: var(--accent);
		color: var(--accent-foreground);
	}
</style>

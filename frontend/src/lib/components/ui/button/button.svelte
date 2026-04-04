<script lang="ts" module>
	/* eslint-disable no-import-assign -- Svelte module context re-export */
	export { type ButtonVariant, type ButtonSize, type ButtonProps } from './button.types';
	/* eslint-enable no-import-assign */
</script>

<script lang="ts">
	import type { ButtonProps } from './button.types';

	let {
		class: className,
		variant = 'default',
		size = 'default',
		ref = $bindable(null),
		href = undefined,
		type = 'button',
		disabled,
		children,
		...restProps
	}: ButtonProps = $props();
</script>

{#if href}
	<a
		bind:this={ref}
		data-slot="button"
		data-variant={variant}
		data-size={size}
		class={className}
		href={disabled ? undefined : href}
		aria-disabled={disabled}
		role={disabled ? 'link' : undefined}
		tabindex={disabled ? -1 : undefined}
		{...restProps}
	>
		{@render children?.()}
	</a>
{:else}
	<button
		bind:this={ref}
		data-slot="button"
		data-variant={variant}
		data-size={size}
		class={className}
		{type}
		{disabled}
		{...restProps}
	>
		{@render children?.()}
	</button>
{/if}

<style>
	:global([data-slot='button']) {
		display: inline-flex;
		flex-shrink: 0;
		align-items: center;
		justify-content: center;
		gap: var(--space-2);
		border-radius: var(--radius-md);
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		white-space: nowrap;
		outline: none;
		transition: all var(--duration-fast) var(--ease-default);

		&:focus-visible {
			border-color: var(--ring);
			box-shadow: 0 0 0 3px oklch(from var(--ring) l c h / 50%);
		}

		&:disabled,
		&[aria-disabled='true'] {
			pointer-events: none;
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
	}

	/* ── Variants ── */
	:global([data-slot='button'][data-variant='default']) {
		background-color: var(--primary);
		color: var(--primary-foreground);
		box-shadow: var(--shadow-xs);

		&:hover { background-color: oklch(from var(--primary) l c h / 90%); }
	}

	:global([data-slot='button'][data-variant='destructive']) {
		background-color: var(--destructive);
		color: oklch(1 0 0);
		box-shadow: var(--shadow-xs);

		&:hover { background-color: oklch(from var(--destructive) l c h / 90%); }
		&:focus-visible { box-shadow: 0 0 0 3px oklch(from var(--destructive) l c h / 20%); }
	}

	:global(.dark [data-slot='button'][data-variant='destructive']) {
		background-color: oklch(from var(--destructive) l c h / 60%);

		&:focus-visible { box-shadow: 0 0 0 3px oklch(from var(--destructive) l c h / 40%); }
	}

	:global([data-slot='button'][data-variant='outline']) {
		background-color: var(--background);
		border: 1px solid var(--border);
		box-shadow: var(--shadow-xs);

		&:hover {
			background-color: var(--accent);
			color: var(--accent-foreground);
		}
	}

	:global(.dark [data-slot='button'][data-variant='outline']) {
		background-color: oklch(from var(--input) l c h / 30%);
		border-color: var(--input);

		&:hover { background-color: oklch(from var(--input) l c h / 50%); }
	}

	:global([data-slot='button'][data-variant='secondary']) {
		background-color: var(--secondary);
		color: var(--secondary-foreground);
		box-shadow: var(--shadow-xs);

		&:hover { background-color: oklch(from var(--secondary) l c h / 80%); }
	}

	:global([data-slot='button'][data-variant='ghost']) {
		&:hover {
			background-color: var(--accent);
			color: var(--accent-foreground);
		}
	}

	:global(.dark [data-slot='button'][data-variant='ghost']) {
		&:hover { background-color: oklch(from var(--accent) l c h / 50%); }
	}

	:global([data-slot='button'][data-variant='link']) {
		color: var(--primary);
		text-underline-offset: 4px;

		&:hover { text-decoration: underline; }
	}

	/* ── Sizes ── */
	:global([data-slot='button'][data-size='default']) {
		block-size: 2.25rem;
		padding-inline: var(--space-4);
		padding-block: var(--space-2);

		&:has(> :global(svg)) { padding-inline: var(--space-3); }
	}

	:global([data-slot='button'][data-size='sm']) {
		block-size: 2rem;
		gap: var(--space-1-5);
		border-radius: var(--radius-md);
		padding-inline: var(--space-3);

		&:has(> :global(svg)) { padding-inline: var(--space-2-5); }
	}

	:global([data-slot='button'][data-size='lg']) {
		block-size: 2.5rem;
		border-radius: var(--radius-md);
		padding-inline: var(--space-6);

		&:has(> :global(svg)) { padding-inline: var(--space-4); }
	}

	:global([data-slot='button'][data-size='icon']) {
		inline-size: 2.75rem;
		block-size: 2.75rem;
	}

	:global([data-slot='button'][data-size='icon-sm']) {
		inline-size: 2.25rem;
		block-size: 2.25rem;
	}

	:global([data-slot='button'][data-size='icon-lg']) {
		inline-size: 3rem;
		block-size: 3rem;
	}
</style>

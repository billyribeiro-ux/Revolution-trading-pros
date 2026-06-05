<script lang="ts">
	import type { Attachment } from 'svelte/attachments';
	import { buttonVariants, type ButtonProps } from './button.types';
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

	const captureRef: Attachment<HTMLElement> = (node) => {
		ref = node;
		return () => {
			if (ref === node) {
				ref = null;
			}
		};
	};
</script>

{#if href}
	<a
		{@attach captureRef}
		data-slot="button"
		class={[buttonVariants({ variant, size }), className]}
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
		{@attach captureRef}
		data-slot="button"
		class={[buttonVariants({ variant, size }), className]}
		{type}
		{disabled}
		{...restProps}
	>
		{@render children?.()}
	</button>
{/if}

<style>
	.ui-button {
		--button-bg: #2563eb;
		--button-bg-hover: #1d4ed8;
		--button-color: #ffffff;
		--button-border: transparent;
		--button-ring: rgba(37, 99, 235, 0.35);

		display: inline-flex;
		flex-shrink: 0;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		min-width: 0;
		border: 1px solid var(--button-border);
		border-radius: 0.375rem;
		background: var(--button-bg);
		color: var(--button-color);
		box-shadow: 0 1px 2px rgba(15, 23, 42, 0.08);
		font-size: 0.875rem;
		font-weight: 500;
		line-height: 1.25rem;
		text-decoration: none;
		white-space: nowrap;
		cursor: pointer;
		outline: none;
		transition:
			background-color 0.18s ease,
			border-color 0.18s ease,
			box-shadow 0.18s ease,
			color 0.18s ease,
			opacity 0.18s ease;
	}

	.ui-button:hover {
		background: var(--button-bg-hover);
	}

	.ui-button:focus-visible {
		border-color: var(--button-ring);
		box-shadow: 0 0 0 3px var(--button-ring);
	}

	.ui-button[disabled],
	.ui-button[aria-disabled='true'] {
		pointer-events: none;
		opacity: 0.5;
	}

	.ui-button :global(svg) {
		pointer-events: none;
		width: 1rem;
		height: 1rem;
		flex-shrink: 0;
	}

	.ui-button--destructive {
		--button-bg: #dc2626;
		--button-bg-hover: #b91c1c;
		--button-color: #ffffff;
		--button-ring: rgba(220, 38, 38, 0.28);
	}

	.ui-button--outline {
		--button-bg: #ffffff;
		--button-bg-hover: #f1f5f9;
		--button-color: #0f172a;
		--button-border: #cbd5e1;
		--button-ring: rgba(37, 99, 235, 0.25);
	}

	.ui-button--secondary {
		--button-bg: #e2e8f0;
		--button-bg-hover: #cbd5e1;
		--button-color: #0f172a;
		--button-ring: rgba(100, 116, 139, 0.28);
	}

	.ui-button--ghost {
		--button-bg: transparent;
		--button-bg-hover: #f1f5f9;
		--button-color: #0f172a;
		--button-border: transparent;
		--button-ring: rgba(37, 99, 235, 0.24);
		box-shadow: none;
	}

	.ui-button--link {
		--button-bg: transparent;
		--button-bg-hover: transparent;
		--button-color: #2563eb;
		--button-border: transparent;
		box-shadow: none;
		text-underline-offset: 4px;
	}

	.ui-button--link:hover {
		text-decoration: underline;
	}

	.ui-button--default {
		min-height: 2.25rem;
		padding: 0.5rem 1rem;
	}

	.ui-button--default:has(> :global(svg)) {
		padding-inline: 0.75rem;
	}

	.ui-button--sm {
		min-height: 2rem;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
	}

	.ui-button--sm:has(> :global(svg)) {
		padding-inline: 0.625rem;
	}

	.ui-button--lg {
		min-height: 2.5rem;
		padding: 0.5rem 1.5rem;
	}

	.ui-button--lg:has(> :global(svg)) {
		padding-inline: 1rem;
	}

	.ui-button--icon,
	.ui-button--icon-sm,
	.ui-button--icon-lg {
		padding: 0;
		aspect-ratio: 1;
	}

	.ui-button--icon {
		width: 2.75rem;
		min-height: 2.75rem;
	}

	.ui-button--icon-sm {
		width: 2.25rem;
		min-height: 2.25rem;
	}

	.ui-button--icon-lg {
		width: 3rem;
		min-height: 3rem;
	}

	@media (prefers-color-scheme: dark) {
		.ui-button--outline {
			--button-bg: rgba(15, 23, 42, 0.3);
			--button-bg-hover: rgba(51, 65, 85, 0.5);
			--button-color: #f8fafc;
			--button-border: #475569;
		}

		.ui-button--secondary,
		.ui-button--ghost {
			--button-bg-hover: rgba(51, 65, 85, 0.5);
			--button-color: #f8fafc;
		}
	}
</style>

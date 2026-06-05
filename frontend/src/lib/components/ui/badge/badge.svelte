<script lang="ts">
	import type { Attachment } from 'svelte/attachments';
	import type { HTMLAnchorAttributes } from 'svelte/elements';
	import { type WithElementRef } from '$lib/utils.js';
	import { badgeVariants, type BadgeVariant } from './badge.types';

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

	const captureRef: Attachment<HTMLElement> = (node) => {
		ref = node;
		return () => {
			if (ref === node) {
				ref = null;
			}
		};
	};
</script>

<svelte:element
	this={href ? 'a' : 'span'}
	{@attach captureRef}
	data-slot="badge"
	{href}
	class={[badgeVariants({ variant }), className]}
	{...restProps}
>
	{@render children?.()}
</svelte:element>

<style>
	.ui-badge {
		--badge-bg: #2563eb;
		--badge-bg-hover: #1d4ed8;
		--badge-color: #ffffff;
		--badge-border: transparent;
		--badge-ring: rgba(37, 99, 235, 0.35);

		display: inline-flex;
		width: fit-content;
		flex-shrink: 0;
		align-items: center;
		justify-content: center;
		gap: 0.25rem;
		overflow: hidden;
		border: 1px solid var(--badge-border);
		border-radius: 999px;
		background: var(--badge-bg);
		color: var(--badge-color);
		padding: 0.125rem 0.5rem;
		font-size: 0.75rem;
		font-weight: 500;
		line-height: 1rem;
		text-decoration: none;
		white-space: nowrap;
		transition:
			background-color 0.18s ease,
			border-color 0.18s ease,
			box-shadow 0.18s ease,
			color 0.18s ease;
	}

	.ui-badge:focus-visible {
		border-color: var(--badge-ring);
		box-shadow: 0 0 0 3px var(--badge-ring);
		outline: none;
	}

	.ui-badge :global(svg) {
		pointer-events: none;
		width: 0.75rem;
		height: 0.75rem;
	}

	a.ui-badge:hover {
		background: var(--badge-bg-hover);
	}

	.ui-badge--secondary {
		--badge-bg: #e2e8f0;
		--badge-bg-hover: #cbd5e1;
		--badge-color: #0f172a;
		--badge-ring: rgba(100, 116, 139, 0.28);
	}

	.ui-badge--destructive {
		--badge-bg: #dc2626;
		--badge-bg-hover: #b91c1c;
		--badge-color: #ffffff;
		--badge-ring: rgba(220, 38, 38, 0.28);
	}

	.ui-badge--outline {
		--badge-bg: transparent;
		--badge-bg-hover: #f1f5f9;
		--badge-color: #0f172a;
		--badge-border: #cbd5e1;
		--badge-ring: rgba(37, 99, 235, 0.25);
	}

	@media (prefers-color-scheme: dark) {
		.ui-badge--secondary {
			--badge-bg: #334155;
			--badge-bg-hover: #475569;
			--badge-color: #f8fafc;
		}

		.ui-badge--outline {
			--badge-bg-hover: rgba(51, 65, 85, 0.5);
			--badge-color: #f8fafc;
			--badge-border: #475569;
		}
	}
</style>

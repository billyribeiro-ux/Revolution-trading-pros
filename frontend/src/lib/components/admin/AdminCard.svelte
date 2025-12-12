<script lang="ts">
	/**
	 * AdminCard - Reusable card component for admin dashboard
	 * Netflix L11+ Principal Engineer Grade
	 *
	 * Uses CSS custom properties for bulletproof light/dark theme support
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */
	import type { Snippet } from 'svelte';

	interface Props {
		/** Card title */
		title?: string;
		/** Optional subtitle */
		subtitle?: string;
		/** Card variant */
		variant?: 'default' | 'elevated' | 'bordered' | 'ghost';
		/** Card padding size */
		padding?: 'none' | 'sm' | 'md' | 'lg';
		/** Enable hover effect */
		hover?: boolean;
		/** Optional header action slot */
		headerAction?: Snippet;
		/** Main content slot */
		children: Snippet;
		/** Optional footer slot */
		footer?: Snippet;
		/** Optional CSS class */
		class?: string;
	}

	let {
		title = '',
		subtitle = '',
		variant = 'default',
		padding = 'md',
		hover = false,
		headerAction,
		children,
		footer,
		class: className = ''
	}: Props = $props();

	const paddingClasses = {
		none: '',
		sm: 'p-3',
		md: 'p-5',
		lg: 'p-6'
	};
</script>

<div
	class="admin-card variant-{variant} {paddingClasses[padding]} {hover ? 'hoverable' : ''} {className}"
	role="region"
	aria-label={title || 'Card'}
>
	{#if title || headerAction}
		<header class="card-header">
			<div class="card-header-text">
				{#if title}
					<h3 class="card-title">{title}</h3>
				{/if}
				{#if subtitle}
					<p class="card-subtitle">{subtitle}</p>
				{/if}
			</div>
			{#if headerAction}
				<div class="card-header-action">
					{@render headerAction()}
				</div>
			{/if}
		</header>
	{/if}

	<div class="card-content">
		{@render children()}
	</div>

	{#if footer}
		<footer class="card-footer">
			{@render footer()}
		</footer>
	{/if}
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * ADMIN CARD - Netflix L11+ Principal Engineer Grade
	 * Theme-aware card component with multiple variants
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.admin-card {
		background: var(--admin-card-bg);
		border: 1px solid var(--admin-card-border);
		border-radius: var(--radius-xl, 1rem);
		box-shadow: var(--admin-card-shadow);
		transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
	}

	/* Variants */
	.variant-elevated {
		background: var(--admin-surface-elevated);
		box-shadow: var(--admin-card-shadow-hover);
	}

	.variant-bordered {
		background: transparent;
		border: 2px solid var(--admin-border);
		box-shadow: none;
	}

	.variant-ghost {
		background: transparent;
		border: none;
		box-shadow: none;
	}

	/* Hover Effect */
	.hoverable {
		cursor: pointer;
	}

	.hoverable:hover {
		transform: translateY(-2px);
		box-shadow: var(--admin-card-shadow-hover);
		border-color: var(--admin-border-interactive);
	}

	.hoverable:focus-visible {
		box-shadow: var(--admin-focus-ring);
		outline: none;
	}

	/* Card Header */
	.card-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 1rem;
		padding-bottom: 1rem;
		margin-bottom: 1rem;
		border-bottom: 1px solid var(--admin-border-light);
	}

	.card-header-text {
		flex: 1;
		min-width: 0;
	}

	.card-title {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--admin-text-primary);
		margin: 0;
		line-height: 1.4;
	}

	.card-subtitle {
		font-size: 0.875rem;
		color: var(--admin-text-muted);
		margin: 0.25rem 0 0;
		line-height: 1.5;
	}

	.card-header-action {
		flex-shrink: 0;
	}

	/* Card Content */
	.card-content {
		color: var(--admin-text-primary);
	}

	/* Card Footer */
	.card-footer {
		padding-top: 1rem;
		margin-top: 1rem;
		border-top: 1px solid var(--admin-border-light);
	}

	/* Remove padding adjustments for header when padding is none */
	.p-none .card-header,
	.p-none .card-footer {
		padding: 1rem;
		margin: 0;
	}

	/* Small padding variant adjustments */
	.p-3 .card-header {
		padding-bottom: 0.75rem;
		margin-bottom: 0.75rem;
	}

	.p-3 .card-footer {
		padding-top: 0.75rem;
		margin-top: 0.75rem;
	}
</style>

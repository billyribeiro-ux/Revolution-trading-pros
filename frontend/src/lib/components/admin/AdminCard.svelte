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
		none: 'p-none',
		sm: 'p-sm',
		md: 'p-md',
		lg: 'p-lg'
	};
</script>

<div
	class="admin-card variant-{variant} {paddingClasses[padding]} {hover
		? 'hoverable'
		: ''} {className}"
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
		border-radius: var(--radius-xl);
		box-shadow: var(--admin-card-shadow);
		transition: var(--transition-all);
	}

	/* Padding Variants */
	.p-none {
		padding: 0;
	}

	.p-sm {
		padding: var(--space-3);
	}

	.p-md {
		padding: var(--space-5);
	}

	.p-lg {
		padding: var(--space-6);
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
		gap: var(--space-4);
		padding-bottom: var(--space-4);
		margin-bottom: var(--space-4);
		border-bottom: 1px solid var(--admin-border-light);
	}

	.card-header-text {
		flex: 1;
		min-width: 0;
	}

	.card-title {
		font-size: var(--text-lg);
		font-weight: var(--font-semibold);
		color: var(--admin-text-primary);
		margin: 0;
		line-height: var(--leading-snug);
	}

	.card-subtitle {
		font-size: var(--text-sm);
		color: var(--admin-text-muted);
		margin: var(--space-1) 0 0;
		line-height: var(--leading-normal);
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
		padding-top: var(--space-4);
		margin-top: var(--space-4);
		border-top: 1px solid var(--admin-border-light);
	}

	/* Padding adjustments for header/footer when card padding is none */
	.p-none .card-header,
	.p-none .card-footer {
		padding: var(--space-4);
		margin: 0;
	}

	/* Small padding variant adjustments */
	.p-sm .card-header {
		padding-bottom: var(--space-3);
		margin-bottom: var(--space-3);
	}

	.p-sm .card-footer {
		padding-top: var(--space-3);
		margin-top: var(--space-3);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - Mobile (< sm: 640px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: calc(var(--breakpoint-sm) - 1px)) {
		.admin-card {
			border-radius: var(--radius-lg);
		}

		.p-md {
			padding: var(--space-4);
		}

		.p-lg {
			padding: var(--space-5);
		}

		.card-header {
			gap: var(--space-3);
			flex-direction: column;
			align-items: stretch;
		}

		.card-title {
			font-size: var(--text-base);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.admin-card {
			transition: none;
		}

		.hoverable:hover {
			transform: none;
		}
	}
</style>

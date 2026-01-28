<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * MetricCard Component - Key Performance Indicator Display
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @version 1.0.0 - Phase 4: Analytics Dashboard
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA | Svelte 5 January 2026
	 */

	interface Props {
		title: string;
		value: string | number;
		subtitle?: string;
		change?: number;
		changeLabel?: string;
		changeType?: 'positive' | 'negative' | 'neutral';
		icon?: string;
		variant?: 'default' | 'primary' | 'success' | 'danger' | 'warning';
		size?: 'sm' | 'md' | 'lg';
		isLoading?: boolean;
	}

	const {
		title,
		value,
		subtitle,
		change,
		changeLabel,
		changeType = 'neutral',
		icon,
		variant = 'default',
		size = 'md',
		isLoading = false
	}: Props = $props();

	// Format change value for display
	const formattedChange = $derived(
		change !== undefined
			? `${change >= 0 ? '+' : ''}${change.toFixed(1)}%`
			: null
	);

	// Determine change color
	const changeColor = $derived(
		changeType === 'positive' ? 'positive' :
		changeType === 'negative' ? 'negative' : 'neutral'
	);
</script>

<div class="metric-card variant-{variant} size-{size}" class:loading={isLoading}>
	{#if isLoading}
		<div class="skeleton-title"></div>
		<div class="skeleton-value"></div>
		{#if subtitle}
			<div class="skeleton-subtitle"></div>
		{/if}
	{:else}
		<div class="header">
			{#if icon}
				<span class="icon">{icon}</span>
			{/if}
			<span class="title">{title}</span>
		</div>

		<div class="value">{value}</div>

		{#if subtitle || formattedChange}
			<div class="footer">
				{#if subtitle}
					<span class="subtitle">{subtitle}</span>
				{/if}
				{#if formattedChange}
					<span class="change {changeColor}">
						{formattedChange}
						{#if changeLabel}
							<span class="change-label">{changeLabel}</span>
						{/if}
					</span>
				{/if}
			</div>
		{/if}
	{/if}
</div>

<style>
	.metric-card {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: var(--radius-lg);
		padding: var(--space-4);
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		transition: var(--transition-all);
	}

	.metric-card:hover {
		border-color: var(--color-border-strong);
		box-shadow: var(--shadow-md);
	}

	/* Variants */
	.metric-card.variant-primary {
		border-color: var(--color-brand-primary);
		background: linear-gradient(135deg, var(--color-bg-card) 0%, rgba(var(--color-brand-primary-rgb), 0.05) 100%);
	}

	.metric-card.variant-success {
		border-color: var(--color-profit-border);
		background: linear-gradient(135deg, var(--color-bg-card) 0%, var(--color-profit-bg) 100%);
	}

	.metric-card.variant-danger {
		border-color: var(--color-loss-border);
		background: linear-gradient(135deg, var(--color-bg-card) 0%, var(--color-loss-bg) 100%);
	}

	.metric-card.variant-warning {
		border-color: var(--color-warning-border, orange);
		background: linear-gradient(135deg, var(--color-bg-card) 0%, rgba(255, 165, 0, 0.05) 100%);
	}

	/* Sizes */
	.metric-card.size-sm {
		padding: var(--space-3);
	}

	.metric-card.size-lg {
		padding: var(--space-6);
	}

	/* Header */
	.header {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.icon {
		font-size: var(--text-lg);
	}

	.title {
		font-size: var(--text-xs);
		font-weight: var(--font-semibold);
		color: var(--color-text-tertiary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.size-lg .title {
		font-size: var(--text-sm);
	}

	/* Value */
	.value {
		font-size: var(--text-2xl);
		font-weight: var(--font-extrabold);
		color: var(--color-text-primary);
		font-variant-numeric: tabular-nums;
		line-height: 1.1;
	}

	.size-sm .value {
		font-size: var(--text-xl);
	}

	.size-lg .value {
		font-size: var(--text-3xl);
	}

	.variant-success .value {
		color: var(--color-profit);
	}

	.variant-danger .value {
		color: var(--color-loss);
	}

	/* Footer */
	.footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-2);
		margin-top: var(--space-1);
	}

	.subtitle {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
	}

	.change {
		font-size: var(--text-sm);
		font-weight: var(--font-semibold);
		display: flex;
		align-items: center;
		gap: var(--space-1);
	}

	.change.positive {
		color: var(--color-profit);
	}

	.change.negative {
		color: var(--color-loss);
	}

	.change.neutral {
		color: var(--color-text-secondary);
	}

	.change-label {
		font-weight: var(--font-normal);
		color: var(--color-text-tertiary);
	}

	/* Loading skeleton */
	.metric-card.loading {
		pointer-events: none;
	}

	.skeleton-title,
	.skeleton-value,
	.skeleton-subtitle {
		background: linear-gradient(90deg, var(--color-bg-subtle) 25%, var(--color-bg-muted) 50%, var(--color-bg-subtle) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: var(--radius-sm);
	}

	.skeleton-title {
		width: 60%;
		height: 12px;
	}

	.skeleton-value {
		width: 80%;
		height: 28px;
		margin-top: var(--space-2);
	}

	.skeleton-subtitle {
		width: 40%;
		height: 14px;
		margin-top: var(--space-2);
	}

	@keyframes shimmer {
		0% { background-position: 200% 0; }
		100% { background-position: -200% 0; }
	}
</style>

<script lang="ts">
	/**
	 * StatCard - Statistic card component for admin dashboard
	 * Netflix L11+ Principal Engineer Grade
	 *
	 * Uses CSS custom properties for bulletproof light/dark theme support
	 *
	 * @version 2.0.0
	 * @author Revolution Trading Pros
	 */
	import type { ComponentType } from 'svelte';

	interface Props {
		title: string;
		value: string | number;
		icon: ComponentType;
		trend?: string;
		trendUp?: boolean;
		color?: 'blue' | 'green' | 'purple' | 'amber';
	}

	let { title, value, icon: Icon, trend = '', trendUp = true, color = 'blue' }: Props = $props();
</script>

<div class="stat-card">
	<div class="stat-card-inner">
		<div class="flex-1">
			<p class="stat-title">{title}</p>
			<p class="stat-value">{value}</p>
			{#if trend}
				<p class="stat-trend" class:up={trendUp} class:down={!trendUp}>
					<span class="trend-arrow">{trendUp ? '↑' : '↓'}</span>
					{trend}
				</p>
			{/if}
		</div>
		<div class="stat-icon {color}">
			<Icon size={24} />
		</div>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * STAT CARD - Netflix L11+ Principal Engineer Grade
	 * Theme-aware statistic card with color variants
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.stat-card {
		background: var(--admin-card-bg);
		border: 1px solid var(--admin-card-border);
		border-radius: var(--radius-xl, 1rem);
		padding: 1.25rem;
		box-shadow: var(--admin-card-shadow);
		transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.stat-card:hover {
		transform: translateY(-2px);
		box-shadow: var(--admin-card-shadow-hover);
		border-color: var(--admin-border-interactive);
	}

	.stat-card-inner {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 1rem;
	}

	.stat-title {
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--admin-text-muted);
		margin: 0;
	}

	.stat-value {
		font-size: 1.875rem;
		font-weight: 700;
		color: var(--admin-text-primary);
		margin: 0.5rem 0 0;
		letter-spacing: -0.02em;
	}

	.stat-trend {
		font-size: 0.875rem;
		font-weight: 500;
		margin: 0.5rem 0 0;
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.stat-trend.up {
		color: var(--admin-success);
	}

	.stat-trend.down {
		color: var(--admin-error);
	}

	.trend-arrow {
		font-weight: 600;
	}

	/* Icon Container */
	.stat-icon {
		width: 3rem;
		height: 3rem;
		border-radius: 0.75rem;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	/* Color Variants */
	.stat-icon.blue {
		background: var(--admin-widget-blue-bg);
		color: var(--admin-widget-blue-icon);
	}

	.stat-icon.green {
		background: var(--admin-widget-emerald-bg);
		color: var(--admin-widget-emerald-icon);
	}

	.stat-icon.purple {
		background: var(--admin-widget-purple-bg);
		color: var(--admin-widget-purple-icon);
	}

	.stat-icon.amber {
		background: var(--admin-widget-amber-bg);
		color: var(--admin-widget-amber-icon);
	}
</style>

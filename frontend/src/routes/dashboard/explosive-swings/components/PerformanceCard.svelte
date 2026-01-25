<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * PerformanceCard Component - 30-Day Track Record
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays 30-day performance metrics in a compact sidebar card
	 * @version 5.0.0 - Nuclear Refactor: Design Tokens
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */
	import type { ThirtyDayPerformance } from '../types';
	import { formatPercent, formatRiskReward } from '../utils/formatters';
	import { calculateRiskRewardFromPercentages } from '../utils/calculations';

	interface Props {
		performance: ThirtyDayPerformance;
		isLoading?: boolean;
	}

	const { performance, isLoading = false }: Props = $props();

	const riskReward = $derived(
		calculateRiskRewardFromPercentages(performance.avgWinPercent, performance.avgLossPercent)
	);
</script>

<div class="performance-card" role="region" aria-labelledby="performance-card-heading">
	<h3 id="performance-card-heading" class="card-title">30-Day Track Record</h3>

	{#if isLoading}
		<div class="skeleton-content">
			<div class="skeleton-line large"></div>
			<div class="skeleton-line medium"></div>
			<div class="skeleton-line"></div>
			<div class="skeleton-line"></div>
			<div class="skeleton-line"></div>
		</div>
	{:else}
		<div class="card-content">
			<!-- Win Rate - Primary Metric -->
			<div class="win-rate-display">
				<span class="win-rate-value">{performance.winRate}%</span>
				<span class="win-rate-label">Win Rate</span>
			</div>

			<p class="win-count">
				{performance.profitableAlerts} of {performance.totalAlerts} alerts profitable
			</p>

			<div class="stats-divider"></div>

			<!-- Supporting Stats -->
			<div class="stats-grid">
				<div class="stat-row">
					<span class="stat-label">Avg Win:</span>
					<span class="stat-value positive">{formatPercent(performance.avgWinPercent)}</span>
				</div>
				<div class="stat-row">
					<span class="stat-label">Avg Loss:</span>
					<span class="stat-value negative">{formatPercent(-Math.abs(performance.avgLossPercent))}</span>
				</div>
				<div class="stat-row">
					<span class="stat-label">R/R:</span>
					<span class="stat-value">{formatRiskReward(riskReward)}</span>
				</div>
			</div>
		</div>

		<a href="/dashboard/explosive-swings/performance" class="view-history-link">
			View Full History →
		</a>
	{/if}
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════
	   PERFORMANCE CARD - Design Token Implementation
	   ═══════════════════════════════════════════════════════════════════════ */
	.performance-card {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: var(--radius-lg);
		padding: var(--space-5);
		box-shadow: var(--shadow-sm);
		transition: var(--transition-shadow);
		contain: layout style;
	}

	.performance-card:hover {
		box-shadow: var(--shadow-md);
	}

	.card-title {
		font-size: var(--text-sm);
		font-weight: var(--font-bold);
		color: var(--color-text-secondary);
		text-transform: uppercase;
		letter-spacing: var(--tracking-wider);
		margin: 0 0 var(--space-4) 0;
	}

	.card-content {
		margin-bottom: var(--space-4);
	}

	.win-rate-display {
		display: flex;
		align-items: baseline;
		gap: var(--space-2);
		margin-bottom: var(--space-1);
	}

	.win-rate-value {
		font-size: var(--text-4xl);
		font-weight: var(--font-extrabold);
		color: var(--color-profit);
		font-variant-numeric: tabular-nums;
		line-height: var(--leading-none);
	}

	.win-rate-label {
		font-size: var(--text-base);
		font-weight: var(--font-medium);
		color: var(--color-text-tertiary);
	}

	.win-count {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		margin: 0 0 var(--space-4) 0;
	}

	.stats-divider {
		height: 1px;
		background: linear-gradient(90deg, var(--color-border-default) 0%, transparent 100%);
		margin-bottom: var(--space-4);
	}

	.stats-grid {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.stat-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.stat-label {
		font-size: var(--text-base);
		color: var(--color-text-tertiary);
	}

	.stat-value {
		font-size: var(--text-base);
		font-weight: var(--font-semibold);
		color: var(--color-text-primary);
		font-variant-numeric: tabular-nums;
	}

	.stat-value.positive {
		color: var(--color-profit);
	}

	.stat-value.negative {
		color: var(--color-loss);
	}

	.view-history-link {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		font-size: var(--text-base);
		font-weight: var(--font-semibold);
		color: var(--color-brand-primary);
		text-decoration: none;
		padding: var(--space-2) var(--space-3);
		margin: 0 calc(-1 * var(--space-3));
		border-radius: var(--radius-md);
		transition: var(--transition-colors);
	}

	.view-history-link:hover {
		background: var(--color-bg-subtle);
		color: var(--color-text-primary);
	}

	.view-history-link:focus-visible {
		outline: 2px solid var(--color-brand-primary);
		outline-offset: 2px;
	}

	/* Skeleton Loading States */
	.skeleton-content {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	.skeleton-line {
		height: 16px;
		background: linear-gradient(90deg, var(--color-bg-subtle) 25%, var(--color-bg-muted) 50%, var(--color-bg-subtle) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: var(--radius-sm);
	}

	.skeleton-line.large {
		height: 32px;
		width: 60%;
	}

	.skeleton-line.medium {
		width: 80%;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════ */
	@media (max-width: 640px) {
		.performance-card {
			padding: var(--space-4);
			border-radius: var(--radius-lg);
		}

		.win-rate-value {
			font-size: var(--text-2xl);
		}

		.stat-row {
			flex-direction: column;
			align-items: flex-start;
			gap: var(--space-1);
		}
	}
</style>

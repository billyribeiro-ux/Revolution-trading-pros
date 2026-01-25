<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * PerformanceCard Component - COMPACT 30-Day Track Record
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @version 6.0.0 - High Density Refactor: 100px height
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

<div class="card" role="region" aria-label="30-day performance">
	<h3 class="title">30-Day Track Record</h3>

	{#if isLoading}
		<div class="skeleton">
			<div class="skel-line lg"></div>
			<div class="skel-line"></div>
			<div class="skel-line"></div>
		</div>
	{:else}
		<!-- Win Rate + Count -->
		<div class="row-main">
			<span class="win-rate">{performance.winRate}%</span>
			<span class="win-label">Win Rate</span>
			<span class="count">{performance.profitableAlerts}/{performance.totalAlerts}</span>
		</div>

		<!-- Stats Row -->
		<div class="row-stats">
			<span>Avg Win: <strong class="profit">{formatPercent(performance.avgWinPercent)}</strong></span>
			<span>Avg Loss: <strong class="loss">{formatPercent(-Math.abs(performance.avgLossPercent))}</strong></span>
			<span>R/R: <strong>{formatRiskReward(riskReward)}</strong></span>
		</div>

		<a href="/dashboard/explosive-swings/performance" class="link">View Full History →</a>
	{/if}
</div>

<style>
	/* COMPACT PERFORMANCE CARD */
	.card {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: var(--radius-md);
		padding: 10px 12px;
		box-shadow: var(--shadow-sm);
		transition: var(--transition-shadow);
	}

	.card:hover {
		box-shadow: var(--shadow-md);
	}

	.title {
		font-size: 10px;
		font-weight: var(--font-bold);
		color: var(--color-text-tertiary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin: 0 0 8px 0;
	}

	/* Main Row: Win Rate + Count */
	.row-main {
		display: flex;
		align-items: baseline;
		gap: 6px;
		margin-bottom: 6px;
	}

	.win-rate {
		font-size: 24px;
		font-weight: var(--font-extrabold);
		color: var(--color-profit);
		font-variant-numeric: tabular-nums;
		line-height: 1;
	}

	.win-label {
		font-size: 11px;
		font-weight: var(--font-medium);
		color: var(--color-text-tertiary);
	}

	.count {
		margin-left: auto;
		font-size: 11px;
		color: var(--color-text-secondary);
		font-variant-numeric: tabular-nums;
	}

	/* Stats Row */
	.row-stats {
		display: flex;
		gap: 10px;
		font-size: 11px;
		color: var(--color-text-tertiary);
		margin-bottom: 8px;
		flex-wrap: wrap;
	}

	.row-stats strong {
		font-weight: var(--font-semibold);
		color: var(--color-text-primary);
		font-variant-numeric: tabular-nums;
	}

	.row-stats .profit {
		color: var(--color-profit);
	}

	.row-stats .loss {
		color: var(--color-loss);
	}

	/* Link */
	.link {
		display: inline-flex;
		align-items: center;
		font-size: 11px;
		font-weight: var(--font-semibold);
		color: var(--color-brand-primary);
		text-decoration: none;
		padding: 4px 6px;
		margin: 0 -6px;
		border-radius: var(--radius-sm);
		transition: var(--transition-colors);
	}

	.link:hover {
		background: var(--color-bg-subtle);
	}

	.link:focus-visible {
		outline: 2px solid var(--color-brand-primary);
		outline-offset: 2px;
	}

	/* Skeleton */
	.skeleton {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.skel-line {
		height: 12px;
		background: linear-gradient(90deg, var(--color-bg-subtle) 25%, var(--color-bg-muted) 50%, var(--color-bg-subtle) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: var(--radius-sm);
	}

	.skel-line.lg {
		height: 24px;
		width: 50%;
	}

	@keyframes shimmer {
		0% { background-position: 200% 0; }
		100% { background-position: -200% 0; }
	}

	/* Responsive */
	@media (max-width: 640px) {
		.card {
			padding: 8px 10px;
		}

		.win-rate {
			font-size: 20px;
		}

		.row-stats {
			flex-direction: column;
			gap: 4px;
		}
	}
</style>

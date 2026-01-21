<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * PerformanceCard Component - 30-Day Track Record
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays 30-day performance metrics in a compact sidebar card
	 * @version 4.0.0 - January 2026 - Nuclear Build Specification
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 */
	import type { ThirtyDayPerformance } from '../types';
	import { formatPercent, formatRiskReward } from '../utils/formatters';
	import { calculateRiskReward } from '../utils/calculations';

	interface Props {
		performance: ThirtyDayPerformance;
		isLoading?: boolean;
	}

	const { performance, isLoading = false }: Props = $props();

	const riskReward = $derived(
		calculateRiskReward(performance.avgWinPercent, performance.avgLossPercent)
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
	.performance-card {
		background: #fff;
		border: 1px solid #e2e8f0;
		border-radius: 12px;
		padding: 20px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
	}

	.card-title {
		font-size: 12px;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin: 0 0 16px 0;
	}

	.card-content {
		margin-bottom: 16px;
	}

	.win-rate-display {
		display: flex;
		align-items: baseline;
		gap: 8px;
		margin-bottom: 4px;
	}

	.win-rate-value {
		font-size: 32px;
		font-weight: 700;
		color: #059669;
		font-variant-numeric: tabular-nums;
		line-height: 1;
	}

	.win-rate-label {
		font-size: 14px;
		font-weight: 500;
		color: #64748b;
	}

	.win-count {
		font-size: 13px;
		color: #64748b;
		margin: 0 0 16px 0;
	}

	.stats-divider {
		height: 1px;
		background: #e2e8f0;
		margin-bottom: 16px;
	}

	.stats-grid {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.stat-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.stat-label {
		font-size: 14px;
		color: #64748b;
	}

	.stat-value {
		font-size: 14px;
		font-weight: 600;
		color: #0f172a;
		font-variant-numeric: tabular-nums;
	}

	.stat-value.positive {
		color: #059669;
	}

	.stat-value.negative {
		color: #dc2626;
	}

	.view-history-link {
		display: block;
		font-size: 14px;
		font-weight: 600;
		color: #143e59;
		text-decoration: none;
		transition: color 0.2s;
	}

	.view-history-link:hover {
		color: #0984ae;
	}

	.skeleton-content {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.skeleton-line {
		height: 16px;
		background: linear-gradient(90deg, #f1f5f9 25%, #e2e8f0 50%, #f1f5f9 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
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
</style>

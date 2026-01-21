<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * PerformanceSummary Component - Weekly Performance Display
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays this week's performance with ticker pills and active positions
	 * @version 4.0.0 - January 2026 - Nuclear Build Specification
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 */
	import type { WeeklyPerformance, ClosedTrade, ActivePosition } from '../types';
	import { formatPercent, formatWinLossRatio } from '../utils/formatters';
	import TickerPill from './TickerPill.svelte';
	import ActivePositionCard from './ActivePositionCard.svelte';

	interface Props {
		performance: WeeklyPerformance;
		closedTrades: ClosedTrade[];
		activePositions: ActivePosition[];
		isLoading?: boolean;
	}

	const { performance, closedTrades, activePositions, isLoading = false }: Props = $props();
</script>

<section class="performance-summary" aria-labelledby="performance-heading">
	<!-- Header Row -->
	<div class="summary-header">
		<h2 id="performance-heading" class="summary-title">This Week's Performance</h2>
		<div class="win-rate-badge">
			<span class="win-rate-value">{performance.winRate}%</span>
			<span class="win-rate-label">Win Rate ({formatWinLossRatio(performance.winningTrades, performance.totalTrades)})</span>
		</div>
	</div>

	<div class="summary-divider"></div>

	<!-- Closed Trades Section -->
	{#if closedTrades.length > 0}
		<div class="closed-trades-section">
			<div class="section-header">
				<h3 class="section-title">Closed This Week</h3>
				<div class="avg-stats">
					<span class="avg-stat">
						<span class="avg-label">Avg Winner:</span>
						<span class="avg-value positive">{formatPercent(performance.avgWinPercent)}</span>
					</span>
					<span class="stat-separator">|</span>
					<span class="avg-stat">
						<span class="avg-label">Avg Loser:</span>
						<span class="avg-value negative">{formatPercent(-Math.abs(performance.avgLossPercent))}</span>
					</span>
				</div>
			</div>

			<div class="ticker-pills-container" role="list" aria-label="Closed trades this week">
				{#if isLoading}
					{#each Array(4) as _}
						<div class="ticker-pill-skeleton"></div>
					{/each}
				{:else}
					{#each closedTrades as trade (trade.id)}
						<TickerPill {trade} />
					{/each}
				{/if}
			</div>
		</div>
	{/if}

	<!-- Active Positions Section -->
	{#if activePositions.length > 0}
		<div class="active-positions-section">
			<h3 class="section-title">Active Positions ({activePositions.length})</h3>

			<div class="positions-grid">
				{#if isLoading}
					{#each Array(2) as _}
						<div class="position-card-skeleton"></div>
					{/each}
				{:else}
					{#each activePositions as position (position.id)}
						<ActivePositionCard {position} />
					{/each}
				{/if}
			</div>
		</div>
	{/if}

	<!-- Empty State -->
	{#if closedTrades.length === 0 && activePositions.length === 0 && !isLoading}
		<div class="empty-state">
			<p>No trades this week yet. Check back after the market opens!</p>
		</div>
	{/if}
</section>

<style>
	.performance-summary {
		background: #fff;
		border: 1px solid #e2e8f0;
		border-radius: 16px;
		padding: 24px;
		margin-bottom: 24px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
	}

	.summary-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		flex-wrap: wrap;
		gap: 16px;
		margin-bottom: 16px;
	}

	.summary-title {
		font-size: 18px;
		font-weight: 600;
		color: #0f172a;
		margin: 0;
	}

	.win-rate-badge {
		display: flex;
		align-items: baseline;
		gap: 8px;
	}

	.win-rate-value {
		font-size: 28px;
		font-weight: 700;
		color: #059669;
		font-variant-numeric: tabular-nums;
	}

	.win-rate-label {
		font-size: 14px;
		color: #64748b;
	}

	.summary-divider {
		height: 1px;
		background: #e2e8f0;
		margin-bottom: 20px;
	}

	.closed-trades-section {
		margin-bottom: 28px;
	}

	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		flex-wrap: wrap;
		gap: 12px;
		margin-bottom: 16px;
	}

	.section-title {
		font-size: 14px;
		font-weight: 600;
		color: #475569;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin: 0;
	}

	.avg-stats {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.avg-stat {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.avg-label {
		font-size: 13px;
		color: #64748b;
	}

	.avg-value {
		font-size: 14px;
		font-weight: 600;
		font-variant-numeric: tabular-nums;
	}

	.avg-value.positive {
		color: #059669;
	}

	.avg-value.negative {
		color: #dc2626;
	}

	.stat-separator {
		color: #cbd5e1;
	}

	.ticker-pills-container {
		display: flex;
		gap: 12px;
		overflow-x: auto;
		padding-bottom: 8px;
		scroll-snap-type: x mandatory;
		-webkit-overflow-scrolling: touch;
	}

	.ticker-pills-container::-webkit-scrollbar {
		height: 6px;
	}

	.ticker-pills-container::-webkit-scrollbar-track {
		background: #f1f5f9;
		border-radius: 3px;
	}

	.ticker-pills-container::-webkit-scrollbar-thumb {
		background: #cbd5e1;
		border-radius: 3px;
	}

	.ticker-pill-skeleton {
		min-width: 80px;
		height: 80px;
		background: linear-gradient(90deg, #f1f5f9 25%, #e2e8f0 50%, #f1f5f9 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 8px;
		flex-shrink: 0;
	}

	.active-positions-section {
		margin-bottom: 8px;
	}

	.positions-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
		gap: 16px;
		margin-top: 16px;
	}

	.position-card-skeleton {
		height: 240px;
		background: linear-gradient(90deg, #f1f5f9 25%, #e2e8f0 50%, #f1f5f9 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 12px;
	}

	.empty-state {
		text-align: center;
		padding: 32px;
		color: #64748b;
		font-size: 14px;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	@media (max-width: 768px) {
		.performance-summary {
			padding: 16px;
		}

		.summary-header {
			flex-direction: column;
			align-items: flex-start;
		}

		.section-header {
			flex-direction: column;
			align-items: flex-start;
		}

		.positions-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

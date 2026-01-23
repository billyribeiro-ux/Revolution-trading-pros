<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * PerformanceSummary Component - Weekly Performance Display
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays this week's performance with ticker pills and active positions
	 * @version 4.1.0 - Visual Polish Pass
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
		isAdmin?: boolean;
		onClosePosition?: (position: ActivePosition) => void;
		onAddTrade?: () => void;
	}

	const { performance, closedTrades, activePositions, isLoading = false, isAdmin = false, onClosePosition, onAddTrade }: Props = $props();

	// Calculate risk/reward display
	const rrDisplay = $derived(
		performance.riskRewardRatio > 0 
			? `${performance.riskRewardRatio.toFixed(1)}:1` 
			: '—'
	);
</script>

<section class="performance-summary" aria-labelledby="performance-heading">
	<!-- ═══════════════════════════════════════════════════════════════════════
	     HEADER ROW - Title + Win Rate Badge
	     ═══════════════════════════════════════════════════════════════════════ -->
	<header class="summary-header">
		<div class="header-left">
			<h2 id="performance-heading" class="summary-title">This Week's Performance</h2>
			<span class="summary-subtitle">Trade results and active positions</span>
		</div>
		
		<div class="win-rate-container">
			<div class="win-rate-badge">
				<span class="win-rate-value">{performance.winRate}%</span>
				<span class="win-rate-label">Win Rate</span>
			</div>
			<span class="win-rate-detail">
				{formatWinLossRatio(performance.winningTrades, performance.totalTrades)} trades
			</span>
		</div>
	</header>

	<!-- ═══════════════════════════════════════════════════════════════════════
	     CLOSED TRADES SECTION - Ticker Pills
	     ═══════════════════════════════════════════════════════════════════════ -->
	{#if closedTrades.length > 0 || isLoading}
		<div class="closed-section">
			<div class="section-header">
				<h3 class="section-label">Closed This Week</h3>
				<div class="avg-metrics">
					<div class="metric">
						<span class="metric-label">Avg Winner:</span>
						<span class="metric-value positive">{formatPercent(performance.avgWinPercent)}</span>
					</div>
					<span class="metric-divider">|</span>
					<div class="metric">
						<span class="metric-label">Avg Loser:</span>
						<span class="metric-value negative">{formatPercent(-Math.abs(performance.avgLossPercent))}</span>
					</div>
					{#if performance.riskRewardRatio > 0}
						<span class="metric-divider">|</span>
						<div class="metric">
							<span class="metric-label">R/R:</span>
							<span class="metric-value neutral">{rrDisplay}</span>
						</div>
					{/if}
				</div>
			</div>

			<div class="ticker-pills-scroll" role="list" aria-label="Closed trades this week">
				{#if isLoading}
					{#each Array(5) as _, i}
						<div class="ticker-pill-skeleton" style="animation-delay: {i * 0.1}s"></div>
					{/each}
				{:else}
					{#each closedTrades as trade (trade.id)}
						<TickerPill {trade} />
					{/each}
				{/if}
			</div>
		</div>
	{/if}

	<!-- ═══════════════════════════════════════════════════════════════════════
	     ACTIVE POSITIONS SECTION - Position Cards
	     ═══════════════════════════════════════════════════════════════════════ -->
	{#if activePositions.length > 0 || isLoading || isAdmin}
		<div class="active-section">
			<div class="section-header-row">
				<h3 class="section-label">
					Active Positions 
					{#if !isLoading}
						<span class="position-count">({activePositions.length})</span>
					{/if}
				</h3>
				{#if isAdmin && onAddTrade}
					<button class="add-trade-btn" onclick={onAddTrade} aria-label="Add new trade">
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16">
							<path d="M12 4v16m8-8H4" />
						</svg>
						Add Trade
					</button>
				{/if}
			</div>

			<div class="positions-grid">
				{#if isLoading}
					{#each Array(2) as _, i}
						<div class="position-skeleton" style="animation-delay: {i * 0.15}s"></div>
					{/each}
				{:else if activePositions.length === 0 && isAdmin}
					<div class="empty-positions">
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="48" height="48">
							<path d="M12 20V10M18 20V4M6 20v-4" />
						</svg>
						<p>No active positions yet</p>
						<button class="add-first-trade-btn" onclick={onAddTrade}>
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18">
								<path d="M12 4v16m8-8H4" />
							</svg>
							Add Your First Trade
						</button>
					</div>
				{:else}
					{#each activePositions as position (position.id)}
						<ActivePositionCard {position} {isAdmin} onClose={onClosePosition} />
					{/each}
				{/if}
			</div>
		</div>
	{/if}

	<!-- ═══════════════════════════════════════════════════════════════════════
	     EMPTY STATE
	     ═══════════════════════════════════════════════════════════════════════ -->
	{#if closedTrades.length === 0 && activePositions.length === 0 && !isLoading}
		<div class="empty-state">
			<div class="empty-icon">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
					<path stroke-linecap="round" stroke-linejoin="round" d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z" />
				</svg>
			</div>
			<h4 class="empty-title">No trades this week yet</h4>
			<p class="empty-text">Check back after the market opens for live alerts and positions.</p>
		</div>
	{/if}
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════
	   CONTAINER
	   ═══════════════════════════════════════════════════════════════════════ */
	.performance-summary {
		background: #ffffff;
		border: 1px solid #e2e8f0;
		border-radius: 16px;
		padding: 28px;
		margin: 24px 30px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.04), 0 4px 12px rgba(0, 0, 0, 0.03);
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   HEADER
	   ═══════════════════════════════════════════════════════════════════════ */
	.summary-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		flex-wrap: wrap;
		gap: 20px;
		margin-bottom: 28px;
		padding-bottom: 20px;
		border-bottom: 1px solid #e2e8f0;
	}

	.header-left {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.summary-title {
		font-size: 22px;
		font-weight: 700;
		color: #0f172a;
		margin: 0;
		letter-spacing: -0.01em;
	}

	.summary-subtitle {
		font-size: 14px;
		color: #64748b;
	}

	/* Win Rate Badge */
	.win-rate-container {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		gap: 4px;
	}

	.win-rate-badge {
		display: flex;
		align-items: baseline;
		gap: 8px;
		background: linear-gradient(135deg, #ecfdf5 0%, #d1fae5 100%);
		border: 1px solid #a7f3d0;
		padding: 10px 18px;
		border-radius: 10px;
	}

	.win-rate-value {
		font-size: 32px;
		font-weight: 800;
		color: #059669;
		font-variant-numeric: tabular-nums;
		line-height: 1;
	}

	.win-rate-label {
		font-size: 13px;
		font-weight: 600;
		color: #10b981;
		text-transform: uppercase;
		letter-spacing: 0.03em;
	}

	.win-rate-detail {
		font-size: 13px;
		color: #64748b;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   CLOSED TRADES SECTION
	   ═══════════════════════════════════════════════════════════════════════ */
	.closed-section {
		margin-bottom: 32px;
	}

	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		flex-wrap: wrap;
		gap: 16px;
		margin-bottom: 18px;
	}

	.section-label {
		font-size: 13px;
		font-weight: 700;
		color: #475569;
		text-transform: uppercase;
		letter-spacing: 0.06em;
		margin: 0;
	}

	.position-count {
		color: #94a3b8;
		font-weight: 600;
	}

	/* Average Metrics */
	.avg-metrics {
		display: flex;
		align-items: center;
		gap: 14px;
		flex-wrap: wrap;
	}

	.metric {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.metric-label {
		font-size: 13px;
		color: #64748b;
		font-weight: 500;
	}

	.metric-value {
		font-size: 14px;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
	}

	.metric-value.positive {
		color: #059669;
	}

	.metric-value.negative {
		color: #dc2626;
	}

	.metric-value.neutral {
		color: #0f172a;
	}

	.metric-divider {
		color: #cbd5e1;
		font-weight: 300;
	}

	/* Ticker Pills Container */
	.ticker-pills-scroll {
		display: flex;
		gap: 14px;
		overflow-x: auto;
		padding: 8px 4px;
		margin: -8px -4px;
		scroll-snap-type: x mandatory;
		-webkit-overflow-scrolling: touch;
		scrollbar-width: thin;
		scrollbar-color: #cbd5e1 #f1f5f9;
	}

	.ticker-pills-scroll::-webkit-scrollbar {
		height: 8px;
	}

	.ticker-pills-scroll::-webkit-scrollbar-track {
		background: #f1f5f9;
		border-radius: 4px;
	}

	.ticker-pills-scroll::-webkit-scrollbar-thumb {
		background: #cbd5e1;
		border-radius: 4px;
	}

	.ticker-pills-scroll::-webkit-scrollbar-thumb:hover {
		background: #94a3b8;
	}

	/* Skeleton for Ticker Pills */
	.ticker-pill-skeleton {
		min-width: 88px;
		height: 82px;
		background: linear-gradient(90deg, #f1f5f9 25%, #e2e8f0 50%, #f1f5f9 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 10px;
		flex-shrink: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   ACTIVE POSITIONS SECTION
	   ═══════════════════════════════════════════════════════════════════════ */
	.active-section {
		margin-bottom: 8px;
	}

	.section-header-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 18px;
	}

	.active-section .section-label {
		margin-bottom: 0;
	}

	.add-trade-btn {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 18px;
		background: linear-gradient(135deg, #143E59 0%, #0f2d42 100%);
		color: #ffffff;
		border: none;
		border-radius: 10px;
		font-size: 13px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
		box-shadow: 0 2px 8px rgba(20, 62, 89, 0.2);
	}

	.add-trade-btn:hover {
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.3);
		transform: translateY(-2px);
	}

	.add-trade-btn svg {
		flex-shrink: 0;
	}

	.positions-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(340px, 1fr));
		gap: 20px;
	}

	.empty-positions {
		grid-column: 1 / -1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
		border-radius: 16px;
		border: 2px dashed #cbd5e1;
	}

	.empty-positions svg {
		color: #94a3b8;
		margin-bottom: 16px;
	}

	.empty-positions p {
		margin: 0 0 20px 0;
		font-size: 15px;
		font-weight: 600;
		color: #64748b;
	}

	.add-first-trade-btn {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 14px 28px;
		background: linear-gradient(135deg, #143E59 0%, #0f2d42 100%);
		color: #ffffff;
		border: none;
		border-radius: 12px;
		font-size: 14px;
		font-weight: 700;
		cursor: pointer;
		transition: all 0.2s ease;
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.25);
	}

	.add-first-trade-btn:hover {
		box-shadow: 0 6px 16px rgba(20, 62, 89, 0.35);
		transform: translateY(-2px);
	}

	/* Skeleton for Position Cards */
	.position-skeleton {
		height: 260px;
		background: linear-gradient(90deg, #f1f5f9 25%, #e2e8f0 50%, #f1f5f9 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 12px;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   EMPTY STATE
	   ═══════════════════════════════════════════════════════════════════════ */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 48px 24px;
		text-align: center;
	}

	.empty-icon {
		width: 56px;
		height: 56px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #f1f5f9;
		border-radius: 12px;
		margin-bottom: 16px;
	}

	.empty-icon svg {
		width: 28px;
		height: 28px;
		color: #94a3b8;
	}

	.empty-title {
		font-size: 16px;
		font-weight: 600;
		color: #334155;
		margin: 0 0 6px 0;
	}

	.empty-text {
		font-size: 14px;
		color: #64748b;
		margin: 0;
		max-width: 280px;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   ANIMATIONS
	   ═══════════════════════════════════════════════════════════════════════ */
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
	@media (max-width: 768px) {
		.performance-summary {
			padding: 20px;
			margin: 16px;
			border-radius: 12px;
		}

		.summary-header {
			flex-direction: column;
			align-items: flex-start;
		}

		.win-rate-container {
			align-items: flex-start;
		}

		.section-header {
			flex-direction: column;
			align-items: flex-start;
		}

		.avg-metrics {
			flex-direction: column;
			align-items: flex-start;
			gap: 8px;
		}

		.metric-divider {
			display: none;
		}

		.positions-grid {
			grid-template-columns: 1fr;
		}

		.summary-title {
			font-size: 18px;
		}

		.win-rate-value {
			font-size: 26px;
		}
	}
</style>
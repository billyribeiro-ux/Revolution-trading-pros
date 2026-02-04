<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * PerformanceSummary Component - COMPACT Weekly Performance Display
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @version 6.0.0 - High Density Refactor: Reduced spacing and padding
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
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
		onUpdatePosition?: (position: ActivePosition) => void;
		onInvalidatePosition?: (position: ActivePosition) => void;
		onDeletePosition?: (position: ActivePosition) => void;
		onAddTrade?: () => void;
	}

	const {
		performance,
		closedTrades,
		activePositions,
		isLoading = false,
		isAdmin = false,
		onClosePosition,
		onUpdatePosition,
		onInvalidatePosition,
		onDeletePosition,
		onAddTrade
	}: Props = $props();

	// Calculate risk/reward display
	const rrDisplay = $derived(
		performance.riskRewardRatio > 0 ? `${performance.riskRewardRatio.toFixed(1)}:1` : '—'
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
						<span class="metric-value negative"
							>{formatPercent(-Math.abs(performance.avgLossPercent))}</span
						>
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
						<svg
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							width="16"
							height="16"
						>
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
						<svg
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="1.5"
							width="48"
							height="48"
						>
							<path d="M12 20V10M18 20V4M6 20v-4" />
						</svg>
						<p>No active positions yet</p>
						<button class="add-first-trade-btn" onclick={onAddTrade}>
							<svg
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								width="18"
								height="18"
							>
								<path d="M12 4v16m8-8H4" />
							</svg>
							Add Your First Trade
						</button>
					</div>
				{:else}
					{#each activePositions as position (position.id)}
						<ActivePositionCard
							{position}
							{isAdmin}
							onUpdate={onUpdatePosition}
							onInvalidate={onInvalidatePosition}
							onClose={onClosePosition}
							onDelete={onDeletePosition}
						/>
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
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z"
					/>
				</svg>
			</div>
			<h4 class="empty-title">No trades this week yet</h4>
			<p class="empty-text">Check back after the market opens for live alerts and positions.</p>
		</div>
	{/if}
</section>

<style>
	/* COMPACT CONTAINER - Responsive padding with design tokens */
	.performance-summary {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: var(--radius-lg);
		padding: 20px;
		margin: var(--space-2);
		box-shadow: var(--shadow-sm);
	}

	/* COMPACT HEADER */
	.summary-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		flex-wrap: wrap;
		gap: 16px;
		margin-bottom: 20px;
		padding-bottom: 16px;
		border-bottom: 1px solid var(--color-border-default);
	}

	.header-left {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.summary-title {
		font-size: 1.5rem;
		font-weight: var(--font-bold);
		color: var(--color-text-primary);
		margin: 0;
		letter-spacing: var(--tracking-tight);
		font-family: var(--font-display);
		line-height: 1.2;
	}

	.summary-subtitle {
		font-size: 0.875rem;
		color: var(--color-text-tertiary);
		line-height: 1.4;
	}

	/* Win Rate Badge - Compact */
	.win-rate-container {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		gap: 6px;
	}

	.win-rate-badge {
		display: flex;
		align-items: baseline;
		gap: 8px;
		background: var(--color-profit-bg);
		border: 1px solid var(--color-profit-border);
		padding: 10px 16px;
		border-radius: var(--radius-md);
	}

	.win-rate-value {
		font-size: 28px;
		font-weight: var(--font-extrabold);
		color: var(--color-profit);
		font-variant-numeric: tabular-nums;
		line-height: 1;
	}

	.win-rate-label {
		font-size: 11px;
		font-weight: var(--font-semibold);
		color: var(--color-profit-light);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.win-rate-detail {
		font-size: 12px;
		color: var(--color-text-tertiary);
		font-weight: var(--font-medium);
	}

	/* COMPACT SECTIONS */
	.closed-section {
		margin-bottom: 24px;
	}

	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		flex-wrap: wrap;
		gap: 16px;
		margin-bottom: 14px;
	}

	.section-label {
		font-size: 11px;
		font-weight: var(--font-bold);
		color: var(--color-text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.08em;
		margin: 0;
	}

	.position-count {
		color: var(--color-text-muted);
		font-weight: var(--font-semibold);
	}

	/* Average Metrics - Compact */
	.avg-metrics {
		display: flex;
		align-items: center;
		gap: 12px;
		flex-wrap: wrap;
	}

	.metric {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.metric-label {
		font-size: 12px;
		color: var(--color-text-tertiary);
		font-weight: var(--font-medium);
	}

	.metric-value {
		font-size: 13px;
		font-weight: var(--font-semibold);
		font-variant-numeric: tabular-nums;
	}

	.metric-value.positive {
		color: var(--color-profit);
	}

	.metric-value.negative {
		color: var(--color-loss);
	}

	.metric-value.neutral {
		color: var(--color-text-primary);
	}

	.metric-divider {
		color: var(--color-border-default);
		font-size: 12px;
		opacity: 0.5;
	}

	/* Ticker Pills Container - with scroll shadow affordance */
	.ticker-pills-scroll {
		display: flex;
		gap: var(--space-2);
		overflow-x: auto;
		padding: var(--space-1) var(--space-1);
		margin: calc(-1 * var(--space-1)) calc(-1 * var(--space-1));
		scroll-snap-type: x mandatory;
		-webkit-overflow-scrolling: touch;
		scrollbar-width: thin;
		scrollbar-color: var(--color-border-strong) var(--color-bg-subtle);
		mask-image: linear-gradient(
			to right,
			transparent 0,
			black var(--space-1),
			black calc(100% - var(--space-4)),
			transparent 100%
		);
	}

	.ticker-pills-scroll::-webkit-scrollbar {
		height: 8px;
	}

	.ticker-pills-scroll::-webkit-scrollbar-track {
		background: var(--color-bg-subtle);
		border-radius: var(--radius-sm);
	}

	.ticker-pills-scroll::-webkit-scrollbar-thumb {
		background: var(--color-border-strong);
		border-radius: var(--radius-sm);
	}

	.ticker-pills-scroll::-webkit-scrollbar-thumb:hover {
		background: var(--color-text-muted);
	}

	/* Skeleton for Ticker Pills */
	.ticker-pill-skeleton {
		min-width: 88px;
		height: 82px;
		background: linear-gradient(
			90deg,
			var(--color-bg-subtle) 25%,
			var(--color-bg-muted) 50%,
			var(--color-bg-subtle) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: var(--radius-md);
		flex-shrink: 0;
	}

	/* ACTIVE POSITIONS - Responsive Grid */
	.active-section {
		margin-bottom: var(--space-1);
	}

	.section-header-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-1);
		margin-bottom: var(--space-2);
	}

	/* Mobile: 1 column, Tablet: 2 columns, Desktop: 3 columns */
	.positions-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: var(--space-2);
	}

	.add-trade-btn {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-4);
		background: linear-gradient(
			135deg,
			var(--color-brand-primary) 0%,
			var(--color-brand-primary-hover) 100%
		);
		color: white;
		border: none;
		border-radius: var(--radius-md);
		font-size: var(--text-sm);
		font-weight: var(--font-semibold);
		cursor: pointer;
		transition: var(--transition-all);
		box-shadow: var(--shadow-brand);
	}

	.add-trade-btn:hover {
		box-shadow: var(--shadow-brand-lg);
		transform: translateY(-2px);
	}

	.add-trade-btn:focus-visible {
		outline: 2px solid var(--color-brand-primary);
		outline-offset: 2px;
	}

	.add-trade-btn svg {
		flex-shrink: 0;
	}

	.empty-positions {
		grid-column: 1 / -1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: var(--space-8) var(--space-4);
		background: linear-gradient(135deg, var(--color-bg-card-hover) 0%, var(--color-bg-subtle) 100%);
		border-radius: var(--radius-xl);
		border: 2px dashed var(--color-border-strong);
	}

	.empty-positions svg {
		color: var(--color-text-muted);
		margin-bottom: var(--space-2);
	}

	.empty-positions p {
		margin: 0 0 var(--space-2) 0;
		font-size: var(--text-base);
		font-weight: var(--font-semibold);
		color: var(--color-text-tertiary);
	}

	.add-first-trade-btn {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-4);
		background: linear-gradient(
			135deg,
			var(--color-brand-primary) 0%,
			var(--color-brand-primary-hover) 100%
		);
		color: white;
		border: none;
		border-radius: var(--radius-lg);
		font-size: var(--text-base);
		font-weight: var(--font-bold);
		cursor: pointer;
		transition: var(--transition-all);
		box-shadow: var(--shadow-brand);
	}

	.add-first-trade-btn:hover {
		box-shadow: var(--shadow-brand-lg);
		transform: translateY(-2px);
	}

	.add-first-trade-btn:focus-visible {
		outline: 2px solid var(--color-brand-primary);
		outline-offset: 2px;
	}

	/* Skeleton for Position Cards */
	.position-skeleton {
		height: 220px;
		background: linear-gradient(
			90deg,
			var(--color-bg-subtle) 25%,
			var(--color-bg-muted) 50%,
			var(--color-bg-subtle) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: var(--radius-lg);
	}

	/* EMPTY STATE */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: var(--space-8) var(--space-4);
		text-align: center;
	}

	.empty-icon {
		width: 48px;
		height: 48px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--color-bg-subtle);
		border-radius: var(--radius-lg);
		margin-bottom: var(--space-2);
	}

	.empty-icon svg {
		width: 24px;
		height: 24px;
		color: var(--color-text-muted);
	}

	.empty-title {
		font-size: var(--text-lg);
		font-weight: var(--font-semibold);
		color: var(--color-text-secondary);
		margin: 0 0 var(--space-1) 0;
	}

	.empty-text {
		font-size: var(--text-base);
		color: var(--color-text-tertiary);
		margin: 0;
		max-width: 240px;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS - Mobile-first (min-width)
	   ═══════════════════════════════════════════════════════════════════════ */

	/* Tablet (640px+) - 2 column positions grid */
	@media (min-width: 640px) {
		.positions-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	/* Tablet (768px+) */
	@media (min-width: 768px) {
		.performance-summary {
			margin: var(--space-3);
			padding: 24px;
		}

		.summary-title {
			font-size: 1.625rem;
		}

		.win-rate-value {
			font-size: 32px;
		}

		.summary-header {
			gap: 20px;
			margin-bottom: 24px;
			padding-bottom: 20px;
		}
	}

	/* Desktop (1024px+) - 3 column positions grid */
	@media (min-width: 1024px) {
		.performance-summary {
			margin: var(--space-3);
			padding: 28px;
		}

		.positions-grid {
			grid-template-columns: repeat(3, 1fr);
			gap: var(--space-3);
		}

		.summary-title {
			font-size: 1.75rem;
		}

		.win-rate-value {
			font-size: 36px;
		}

		.win-rate-badge {
			padding: 12px 20px;
		}
	}

	/* Large Desktop (1440px+) */
	@media (min-width: 1440px) {
		.performance-summary {
			margin: var(--space-4);
			padding: 32px;
		}

		.summary-title {
			font-size: 2rem;
		}
	}
</style>

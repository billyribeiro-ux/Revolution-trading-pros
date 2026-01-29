<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * TickerHeatmap Component - Performance Heatmap by Ticker
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays trading performance by ticker with color-coded heatmap
	 * @version 1.0.0
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA | Svelte 5
	 */
	import type { TickerPerformance } from '../analytics.state.svelte';

	interface Props {
		data: TickerPerformance[];
		topPerformers: TickerPerformance[];
		worstPerformers: TickerPerformance[];
		isLoading?: boolean;
	}

	const { data, topPerformers, worstPerformers, isLoading = false }: Props = $props();

	// Format functions
	function formatPercent(value: number): string {
		return `${value >= 0 ? '+' : ''}${value.toFixed(1)}%`;
	}

	function formatWinRate(value: number): string {
		return `${value.toFixed(0)}%`;
	}

	// Get color intensity based on P&L
	function getHeatmapColor(pnl: number): string {
		if (pnl >= 20) return 'var(--color-profit)';
		if (pnl >= 10) return 'color-mix(in srgb, var(--color-profit) 80%, transparent)';
		if (pnl >= 5) return 'color-mix(in srgb, var(--color-profit) 60%, transparent)';
		if (pnl >= 0) return 'color-mix(in srgb, var(--color-profit) 40%, transparent)';
		if (pnl >= -5) return 'color-mix(in srgb, var(--color-loss) 40%, transparent)';
		if (pnl >= -10) return 'color-mix(in srgb, var(--color-loss) 60%, transparent)';
		if (pnl >= -20) return 'color-mix(in srgb, var(--color-loss) 80%, transparent)';
		return 'var(--color-loss)';
	}

	function getBackgroundColor(pnl: number): string {
		if (pnl >= 0) return 'var(--color-profit-bg)';
		return 'var(--color-loss-bg)';
	}

	// Sort data for heatmap display
	const sortedData = $derived([...data].sort((a, b) => b.total_pnl_percent - a.total_pnl_percent));

	// Tab state
	let activeTab: 'all' | 'top' | 'worst' = $state('all');
</script>

<div class="ticker-heatmap" role="region" aria-label="Ticker performance heatmap">
	<div class="heatmap-header">
		<h3 class="section-title">Performance by Ticker</h3>
		<div class="tab-group" role="tablist">
			<button
				class="tab-btn"
				class:active={activeTab === 'all'}
				onclick={() => (activeTab = 'all')}
				role="tab"
				aria-selected={activeTab === 'all'}
			>
				All ({data.length})
			</button>
			<button
				class="tab-btn"
				class:active={activeTab === 'top'}
				onclick={() => (activeTab = 'top')}
				role="tab"
				aria-selected={activeTab === 'top'}
			>
				Top 5
			</button>
			<button
				class="tab-btn"
				class:active={activeTab === 'worst'}
				onclick={() => (activeTab = 'worst')}
				role="tab"
				aria-selected={activeTab === 'worst'}
			>
				Bottom 5
			</button>
		</div>
	</div>

	{#if isLoading}
		<div class="skeleton-grid">
			{#each Array(8) as _}
				<div class="skel-cell"></div>
			{/each}
		</div>
	{:else if data.length === 0}
		<div class="empty-state">
			<p>No ticker data available</p>
		</div>
	{:else}
		<!-- Heatmap Grid for All -->
		{#if activeTab === 'all'}
			<div class="heatmap-grid">
				{#each sortedData as ticker}
					<div
						class="heatmap-cell"
						style="background-color: {getBackgroundColor(ticker.total_pnl_percent)}"
					>
						<div class="cell-ticker">{ticker.ticker}</div>
						<div class="cell-pnl" style="color: {getHeatmapColor(ticker.total_pnl_percent)}">
							{formatPercent(ticker.total_pnl_percent)}
						</div>
						<div class="cell-stats">
							{ticker.total_trades} trades | {formatWinRate(ticker.win_rate)} WR
						</div>
					</div>
				{/each}
			</div>
		{/if}

		<!-- Top Performers Table -->
		{#if activeTab === 'top'}
			<table class="performance-table">
				<thead>
					<tr>
						<th>Ticker</th>
						<th>Total P&L</th>
						<th>Avg P&L</th>
						<th>Win Rate</th>
						<th>Trades</th>
					</tr>
				</thead>
				<tbody>
					{#each topPerformers as ticker, index}
						<tr>
							<td class="ticker-cell">
								<span class="rank profit">{index + 1}</span>
								<span class="ticker-name">{ticker.ticker}</span>
							</td>
							<td class="pnl-cell profit">{formatPercent(ticker.total_pnl_percent)}</td>
							<td class="pnl-cell profit">{formatPercent(ticker.avg_pnl_percent)}</td>
							<td>{formatWinRate(ticker.win_rate)}</td>
							<td>{ticker.total_trades}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		{/if}

		<!-- Worst Performers Table -->
		{#if activeTab === 'worst'}
			<table class="performance-table">
				<thead>
					<tr>
						<th>Ticker</th>
						<th>Total P&L</th>
						<th>Avg P&L</th>
						<th>Win Rate</th>
						<th>Trades</th>
					</tr>
				</thead>
				<tbody>
					{#each worstPerformers as ticker, index}
						<tr>
							<td class="ticker-cell">
								<span class="rank loss">{index + 1}</span>
								<span class="ticker-name">{ticker.ticker}</span>
							</td>
							<td class="pnl-cell loss">{formatPercent(ticker.total_pnl_percent)}</td>
							<td class="pnl-cell loss">{formatPercent(ticker.avg_pnl_percent)}</td>
							<td>{formatWinRate(ticker.win_rate)}</td>
							<td>{ticker.total_trades}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		{/if}

		<!-- Legend -->
		<div class="legend">
			<span class="legend-label">Performance:</span>
			<div class="legend-scale">
				<span class="legend-item loss">-20%</span>
				<div class="legend-gradient"></div>
				<span class="legend-item profit">+20%</span>
			</div>
		</div>
	{/if}
</div>

<style>
	.ticker-heatmap {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 12px;
		padding: 20px;
		box-shadow: var(--shadow-sm);
	}

	.heatmap-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 16px;
		flex-wrap: wrap;
		gap: 12px;
	}

	.section-title {
		font-size: 16px;
		font-weight: 700;
		color: var(--color-text-primary);
		margin: 0;
	}

	.tab-group {
		display: flex;
		gap: 4px;
		background: var(--color-bg-subtle);
		padding: 3px;
		border-radius: 6px;
	}

	.tab-btn {
		padding: 6px 12px;
		font-size: 12px;
		font-weight: 600;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		background: transparent;
		color: var(--color-text-secondary);
		transition:
			background 0.15s,
			color 0.15s;
	}

	.tab-btn:hover {
		background: var(--color-bg-muted);
	}

	.tab-btn.active {
		background: var(--color-bg-card);
		color: var(--color-text-primary);
		box-shadow: var(--shadow-sm);
	}

	.heatmap-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(110px, 1fr));
		gap: 8px;
		margin-bottom: 16px;
	}

	.heatmap-cell {
		padding: 12px;
		border-radius: 8px;
		text-align: center;
		transition: transform 0.15s;
	}

	.heatmap-cell:hover {
		transform: scale(1.02);
	}

	.cell-ticker {
		font-size: 14px;
		font-weight: 700;
		color: var(--color-text-primary);
		margin-bottom: 4px;
	}

	.cell-pnl {
		font-size: 16px;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
		margin-bottom: 4px;
	}

	.cell-stats {
		font-size: 10px;
		color: var(--color-text-tertiary);
	}

	/* Performance Table */
	.performance-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 13px;
	}

	.performance-table th {
		text-align: left;
		padding: 10px 12px;
		font-weight: 600;
		color: var(--color-text-tertiary);
		text-transform: uppercase;
		font-size: 11px;
		letter-spacing: 0.05em;
		border-bottom: 1px solid var(--color-border-default);
	}

	.performance-table td {
		padding: 12px;
		border-bottom: 1px solid var(--color-border-subtle);
	}

	.performance-table tr:last-child td {
		border-bottom: none;
	}

	.ticker-cell {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.rank {
		width: 22px;
		height: 22px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 50%;
		font-size: 11px;
		font-weight: 700;
	}

	.rank.profit {
		background: var(--color-profit-bg);
		color: var(--color-profit);
	}

	.rank.loss {
		background: var(--color-loss-bg);
		color: var(--color-loss);
	}

	.ticker-name {
		font-weight: 600;
		color: var(--color-text-primary);
	}

	.pnl-cell {
		font-weight: 600;
		font-variant-numeric: tabular-nums;
	}

	.pnl-cell.profit {
		color: var(--color-profit);
	}

	.pnl-cell.loss {
		color: var(--color-loss);
	}

	/* Legend */
	.legend {
		display: flex;
		align-items: center;
		gap: 12px;
		margin-top: 16px;
		padding-top: 16px;
		border-top: 1px solid var(--color-border-subtle);
	}

	.legend-label {
		font-size: 11px;
		color: var(--color-text-tertiary);
	}

	.legend-scale {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.legend-item {
		font-size: 10px;
		font-weight: 600;
		font-variant-numeric: tabular-nums;
	}

	.legend-item.profit {
		color: var(--color-profit);
	}

	.legend-item.loss {
		color: var(--color-loss);
	}

	.legend-gradient {
		width: 100px;
		height: 8px;
		border-radius: 4px;
		background: linear-gradient(to right, var(--color-loss), transparent 50%, var(--color-profit));
	}

	/* Skeleton */
	.skeleton-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(110px, 1fr));
		gap: 8px;
	}

	.skel-cell {
		height: 80px;
		background: linear-gradient(
			90deg,
			var(--color-bg-subtle) 25%,
			var(--color-bg-muted) 50%,
			var(--color-bg-subtle) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 8px;
	}

	.empty-state {
		height: 150px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-text-tertiary);
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

	@media (max-width: 640px) {
		.ticker-heatmap {
			padding: 16px;
		}

		.heatmap-header {
			flex-direction: column;
			align-items: flex-start;
		}

		.heatmap-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.performance-table {
			font-size: 12px;
		}

		.performance-table th,
		.performance-table td {
			padding: 8px;
		}
	}
</style>

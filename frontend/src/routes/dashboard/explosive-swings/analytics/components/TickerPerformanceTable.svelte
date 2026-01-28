<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * TickerPerformanceTable Component - Detailed Ticker Performance Table
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @version 1.0.0 - Phase 4: Analytics Dashboard
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA | Svelte 5 January 2026
	 */
	import type { TickerPerformance } from '../analytics.state.svelte';

	interface Props {
		data: TickerPerformance[];
		isLoading?: boolean;
	}

	const { data, isLoading = false }: Props = $props();

	// Sorting state
	type SortKey = 'ticker' | 'total_trades' | 'win_rate' | 'total_pnl_percent' | 'avg_pnl_percent' | 'avg_holding_days';
	type SortDirection = 'asc' | 'desc';

	let sortKey = $state<SortKey>('total_pnl_percent');
	let sortDirection = $state<SortDirection>('desc');

	// Sorted data
	const sortedData = $derived(
		[...data].sort((a, b) => {
			const aVal = a[sortKey];
			const bVal = b[sortKey];

			if (typeof aVal === 'string' && typeof bVal === 'string') {
				return sortDirection === 'asc'
					? aVal.localeCompare(bVal)
					: bVal.localeCompare(aVal);
			}

			const aNum = Number(aVal) || 0;
			const bNum = Number(bVal) || 0;

			return sortDirection === 'asc' ? aNum - bNum : bNum - aNum;
		})
	);

	// Handle sort
	function handleSort(key: SortKey) {
		if (sortKey === key) {
			sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
		} else {
			sortKey = key;
			sortDirection = 'desc';
		}
	}

	// Format helpers
	function formatPercent(value: number): string {
		return `${value >= 0 ? '+' : ''}${value.toFixed(1)}%`;
	}

	// Column definitions
	const columns: { key: SortKey; label: string; align: 'left' | 'right' }[] = [
		{ key: 'ticker', label: 'Ticker', align: 'left' },
		{ key: 'total_trades', label: 'Trades', align: 'right' },
		{ key: 'win_rate', label: 'Win Rate', align: 'right' },
		{ key: 'total_pnl_percent', label: 'Total P&L', align: 'right' },
		{ key: 'avg_pnl_percent', label: 'Avg P&L', align: 'right' },
		{ key: 'avg_holding_days', label: 'Avg Hold', align: 'right' }
	];
</script>

<div class="ticker-performance-table" role="region" aria-label="Ticker performance details">
	<div class="table-header">
		<h3 class="section-title">Ticker Performance Details</h3>
		<span class="ticker-count">{data.length} tickers</span>
	</div>

	{#if isLoading}
		<div class="skeleton-table">
			{#each Array(5) as _}
				<div class="skel-row"></div>
			{/each}
		</div>
	{:else if data.length === 0}
		<div class="empty-state">
			<p>No ticker performance data available</p>
		</div>
	{:else}
		<div class="table-wrapper">
			<table class="data-table">
				<thead>
					<tr>
						{#each columns as col}
							<th
								class="sortable"
								class:sorted={sortKey === col.key}
								onclick={() => handleSort(col.key)}
								style="text-align: {col.align}"
							>
								<span class="th-content">
									{col.label}
									{#if sortKey === col.key}
										<span class="sort-indicator">
											{sortDirection === 'asc' ? '↑' : '↓'}
										</span>
									{/if}
								</span>
							</th>
						{/each}
						<th style="text-align: right">Best</th>
						<th style="text-align: right">Worst</th>
					</tr>
				</thead>
				<tbody>
					{#each sortedData as ticker, index}
						<tr class:even={index % 2 === 0}>
							<td class="ticker-cell">
								<span class="ticker-rank">{index + 1}</span>
								<span class="ticker-symbol">{ticker.ticker}</span>
							</td>
							<td class="numeric-cell">{ticker.total_trades}</td>
							<td class="numeric-cell" class:profit={ticker.win_rate >= 50} class:loss={ticker.win_rate < 50}>
								{ticker.win_rate.toFixed(1)}%
							</td>
							<td class="numeric-cell pnl" class:profit={ticker.total_pnl_percent >= 0} class:loss={ticker.total_pnl_percent < 0}>
								{formatPercent(ticker.total_pnl_percent)}
							</td>
							<td class="numeric-cell" class:profit={ticker.avg_pnl_percent >= 0} class:loss={ticker.avg_pnl_percent < 0}>
								{formatPercent(ticker.avg_pnl_percent)}
							</td>
							<td class="numeric-cell">
								{ticker.avg_holding_days.toFixed(1)}d
							</td>
							<td class="numeric-cell profit">
								{formatPercent(ticker.largest_win_percent)}
							</td>
							<td class="numeric-cell loss">
								{formatPercent(-ticker.largest_loss_percent)}
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>

<style>
	.ticker-performance-table {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 12px;
		padding: 20px;
		box-shadow: var(--shadow-sm);
	}

	.table-header {
		display: flex;
		justify-content: space-between;
		align-items: baseline;
		margin-bottom: 16px;
	}

	.section-title {
		font-size: 16px;
		font-weight: 700;
		color: var(--color-text-primary);
		margin: 0;
	}

	.ticker-count {
		font-size: 12px;
		color: var(--color-text-tertiary);
	}

	.table-wrapper {
		overflow-x: auto;
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 13px;
	}

	.data-table th {
		padding: 12px 8px;
		font-weight: 600;
		color: var(--color-text-tertiary);
		text-transform: uppercase;
		font-size: 11px;
		letter-spacing: 0.05em;
		border-bottom: 1px solid var(--color-border-default);
		white-space: nowrap;
	}

	.data-table th.sortable {
		cursor: pointer;
		user-select: none;
	}

	.data-table th.sortable:hover {
		color: var(--color-text-primary);
	}

	.data-table th.sorted {
		color: var(--color-brand-primary);
	}

	.th-content {
		display: inline-flex;
		align-items: center;
		gap: 4px;
	}

	.sort-indicator {
		font-size: 10px;
	}

	.data-table td {
		padding: 12px 8px;
		border-bottom: 1px solid var(--color-border-subtle);
	}

	.data-table tr.even {
		background: var(--color-bg-subtle);
	}

	.data-table tr:last-child td {
		border-bottom: none;
	}

	.ticker-cell {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.ticker-rank {
		width: 22px;
		height: 22px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--color-bg-muted);
		border-radius: 50%;
		font-size: 10px;
		font-weight: 600;
		color: var(--color-text-tertiary);
	}

	.ticker-symbol {
		font-weight: 600;
		color: var(--color-text-primary);
	}

	.numeric-cell {
		text-align: right;
		font-variant-numeric: tabular-nums;
		color: var(--color-text-secondary);
	}

	.numeric-cell.pnl {
		font-weight: 600;
	}

	.numeric-cell.profit {
		color: var(--color-profit);
	}

	.numeric-cell.loss {
		color: var(--color-loss);
	}

	/* Skeleton */
	.skeleton-table {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.skel-row {
		height: 44px;
		background: linear-gradient(90deg, var(--color-bg-subtle) 25%, var(--color-bg-muted) 50%, var(--color-bg-subtle) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
	}

	.empty-state {
		height: 200px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-text-tertiary);
		font-size: 14px;
	}

	@keyframes shimmer {
		0% { background-position: 200% 0; }
		100% { background-position: -200% 0; }
	}

	@media (max-width: 768px) {
		.ticker-performance-table {
			padding: 16px;
		}

		.data-table th,
		.data-table td {
			padding: 8px 4px;
		}

		.data-table {
			font-size: 12px;
		}
	}
</style>

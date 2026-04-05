<!--
  StockListViewer.svelte
  Apple Principal Engineer ICT 7 Grade - February 2026

  Display stock/ETF lists with:
  - Symbol table with details
  - Filtering and sorting
  - Price targets visualization
  - Export functionality
  - Responsive design
-->
<script lang="ts">
	import type { StockList, StockSymbol } from '$lib/api/room-resources';

	interface Props {
		stockList: StockList;
		showHeader?: boolean;
		showExport?: boolean;
		compact?: boolean;
		onSymbolClick?: (symbol: StockSymbol) => void;
		onExport?: (format: 'csv' | 'json') => void;
	}

	let {
		stockList,
		showHeader = true,
		showExport = true,
		compact = false,
		onSymbolClick,
		onExport
	}: Props = $props();

	let sortColumn = $state<keyof StockSymbol>('symbol');
	let sortDirection = $state<'asc' | 'desc'>('asc');
	let searchQuery = $state('');

	// Parse symbols if needed
	let symbols = $derived(
		Array.isArray(stockList.symbols)
			? stockList.symbols
			: typeof stockList.symbols === 'string'
				? JSON.parse(stockList.symbols)
				: []
	);

	// Filter and sort symbols
	let filteredSymbols = $derived(
		symbols
			.filter((s: StockSymbol) => {
				if (!searchQuery) return true;
				const query = searchQuery.toLowerCase();
				return (
					s.symbol.toLowerCase().includes(query) ||
					s.name?.toLowerCase().includes(query) ||
					s.sector?.toLowerCase().includes(query)
				);
			})
			.sort((a: StockSymbol, b: StockSymbol) => {
				const aVal = a[sortColumn] ?? '';
				const bVal = b[sortColumn] ?? '';
				const comparison = String(aVal).localeCompare(String(bVal), undefined, { numeric: true });
				return sortDirection === 'asc' ? comparison : -comparison;
			})
	);

	function handleSort(column: keyof StockSymbol) {
		if (sortColumn === column) {
			sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
		} else {
			sortColumn = column;
			sortDirection = 'asc';
		}
	}

	function handleExport(format: 'csv' | 'json') {
		onExport?.(format);

		if (format === 'csv') {
			const headers = [
				'Symbol',
				'Name',
				'Sector',
				'Entry Price',
				'Price Target',
				'Stop Loss',
				'Notes'
			];
			const rows = symbols.map((s: StockSymbol) => [
				s.symbol,
				s.name ?? '',
				s.sector ?? '',
				s.entry_price ?? '',
				s.price_target ?? '',
				s.stop_loss ?? '',
				s.notes ?? ''
			]);
			const csv = [headers.join(','), ...rows.map((r: string[]) => r.join(','))].join('\n');
			downloadFile(csv, `${stockList.slug}.csv`, 'text/csv');
		} else {
			const json = JSON.stringify(symbols, null, 2);
			downloadFile(json, `${stockList.slug}.json`, 'application/json');
		}
	}

	function downloadFile(content: string, filename: string, type: string) {
		const blob = new Blob([content], { type });
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = filename;
		a.click();
		URL.revokeObjectURL(url);
	}

	function handleSymbolClick(symbol: StockSymbol) {
		onSymbolClick?.(symbol);
	}

	function formatDate(dateStr?: string): string {
		if (!dateStr) return '';
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	}

	function getListType(type: string): string {
		switch (type) {
			case 'etf':
				return 'etf';
			case 'stock':
				return 'stock';
			case 'watchlist':
				return 'watchlist';
			case 'sector':
				return 'sector';
			default:
				return 'default';
		}
	}
</script>

<div class="sl-root" data-compact={compact || undefined}>
	{#if showHeader}
		<div class="sl-header">
			<div class="sl-header-top">
				<div>
					<div class="sl-title-row">
						<h2 class="sl-title">{stockList.name}</h2>
						<span class="sl-type-badge" data-type={getListType(stockList.list_type)}
							>{stockList.list_type}</span
						>
						{#if stockList.is_featured}
							<span class="sl-featured-badge">Featured</span>
						{/if}
					</div>
					{#if stockList.description}
						<p class="sl-desc">{stockList.description}</p>
					{/if}
					<div class="sl-meta">
						<span>{symbols.length} symbols</span>
						{#if stockList.week_of}
							<span>Week of {formatDate(stockList.week_of)}</span>
						{/if}
						<span>Updated {formatDate(stockList.updated_at)}</span>
					</div>
				</div>

				{#if showExport}
					<div class="sl-export-group">
						<button class="sl-export-btn" onclick={() => handleExport('csv')}>Export CSV</button>
						<button class="sl-export-btn" onclick={() => handleExport('json')}>Export JSON</button>
					</div>
				{/if}
			</div>

			{#if symbols.length > 5}
				<div class="sl-search-wrap">
					<input
						type="search"
						bind:value={searchQuery}
						placeholder="Search symbols, names, or sectors..."
						class="sl-search-input"
					/>
					<svg class="sl-search-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
						/>
					</svg>
				</div>
			{/if}
		</div>
	{/if}

	<!-- Table -->
	<div class="sl-table-wrap">
		<table class="sl-table">
			<thead class="sl-thead">
				<tr>
					<th class="sl-th">
						<button class="sl-sort-btn" onclick={() => handleSort('symbol')}>
							Symbol
							{#if sortColumn === 'symbol'}
								<svg
									class="sl-sort-icon"
									data-desc={sortDirection === 'desc' || undefined}
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M5 15l7-7 7 7"
									/>
								</svg>
							{/if}
						</button>
					</th>
					<th class="sl-th">
						<button class="sl-sort-btn" onclick={() => handleSort('name')}>
							Name
							{#if sortColumn === 'name'}
								<svg
									class="sl-sort-icon"
									data-desc={sortDirection === 'desc' || undefined}
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M5 15l7-7 7 7"
									/>
								</svg>
							{/if}
						</button>
					</th>
					{#if !compact}
						<th class="sl-th sl-th-lg">
							<button class="sl-sort-btn" onclick={() => handleSort('sector')}>Sector</button>
						</th>
					{/if}
					<th class="sl-th sl-th-right">Entry</th>
					<th class="sl-th sl-th-right">Target</th>
					<th class="sl-th sl-th-right">Stop</th>
					{#if !compact}
						<th class="sl-th sl-th-md">Notes</th>
					{/if}
				</tr>
			</thead>
			<tbody class="sl-tbody">
				{#each filteredSymbols as symbol (symbol.symbol)}
					<tr class="sl-row" onclick={() => handleSymbolClick(symbol)}>
						<td class="sl-td"><span class="sl-symbol">{symbol.symbol}</span></td>
						<td class="sl-td sl-td-name">{symbol.name || '-'}</td>
						{#if !compact}
							<td class="sl-td sl-td-sector sl-td-lg">{symbol.sector || '-'}</td>
						{/if}
						<td class="sl-td sl-td-mono sl-td-right"
							>{symbol.entry_price ? `$${symbol.entry_price.toFixed(2)}` : '-'}</td
						>
						<td class="sl-td sl-td-mono sl-td-right sl-td-green"
							>{symbol.price_target ? `$${symbol.price_target.toFixed(2)}` : '-'}</td
						>
						<td class="sl-td sl-td-mono sl-td-right sl-td-red"
							>{symbol.stop_loss ? `$${symbol.stop_loss.toFixed(2)}` : '-'}</td
						>
						{#if !compact}
							<td class="sl-td sl-td-notes sl-td-md">{symbol.notes || '-'}</td>
						{/if}
					</tr>
				{:else}
					<tr>
						<td colspan={compact ? 5 : 7} class="sl-td sl-empty">
							{#if searchQuery}
								No symbols match your search
							{:else}
								No symbols in this list
							{/if}
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>

	{#if filteredSymbols.length !== symbols.length}
		<div class="sl-footer">
			Showing {filteredSymbols.length} of {symbols.length} symbols
		</div>
	{/if}
</div>

<style>
	.sl-root {
		border-radius: var(--radius-xl);
		border: 1px solid oklch(0.9 0.005 265);
		background-color: oklch(1 0 0);
	}

	/* ─── Header ─── */
	.sl-header {
		border-block-end: 1px solid oklch(0.9 0.005 265);
		padding: var(--space-4);

		@media (min-width: 1024px) {
			padding: var(--space-6);
		}
	}

	.sl-root[data-compact] .sl-header {
		padding: var(--space-3);
	}

	.sl-header-top {
		display: flex;
		flex-wrap: wrap;
		align-items: flex-start;
		justify-content: space-between;
		gap: var(--space-4);
	}

	.sl-title-row {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.sl-title {
		font-size: var(--text-xl);
		font-weight: var(--weight-bold);
		color: oklch(0.15 0.01 265);
	}

	.sl-root[data-compact] .sl-title {
		font-size: var(--text-lg);
	}

	.sl-type-badge {
		border-radius: 9999px;
		padding-inline: 0.625rem;
		padding-block: 0.125rem;
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
		text-transform: capitalize;

		&[data-type='etf'] {
			background-color: oklch(0.92 0.06 300);
			color: oklch(0.4 0.2 300);
		}
		&[data-type='stock'] {
			background-color: oklch(0.92 0.06 260);
			color: oklch(0.4 0.2 260);
		}
		&[data-type='watchlist'] {
			background-color: oklch(0.92 0.08 80);
			color: oklch(0.45 0.15 80);
		}
		&[data-type='sector'] {
			background-color: oklch(0.92 0.06 160);
			color: oklch(0.4 0.15 160);
		}
		&[data-type='default'] {
			background-color: oklch(0.95 0.002 265);
			color: oklch(0.4 0.01 265);
		}
	}

	.sl-featured-badge {
		border-radius: 9999px;
		background-color: oklch(0.92 0.1 90);
		padding-inline: 0.625rem;
		padding-block: 0.125rem;
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
		color: oklch(0.45 0.15 90);
	}

	.sl-desc {
		margin-block-start: 0.25rem;
		font-size: var(--text-sm);
		color: oklch(0.45 0.01 265);
	}

	.sl-meta {
		margin-block-start: var(--space-2);
		display: flex;
		align-items: center;
		gap: var(--space-4);
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 265);
	}

	/* ─── Export ─── */
	.sl-export-group {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.sl-export-btn {
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.82 0.005 265);
		padding-inline: var(--space-3);
		padding-block: 0.375rem;
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(0.35 0.01 265);
		background: none;
		cursor: pointer;
		transition: background-color 200ms var(--ease-default);

		&:hover {
			background-color: oklch(0.97 0.002 265);
		}
	}

	/* ─── Search ─── */
	.sl-search-wrap {
		position: relative;
		margin-block-start: var(--space-4);
	}

	.sl-search-input {
		inline-size: 100%;
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.82 0.005 265);
		background-color: oklch(1 0 0);
		padding-block: var(--space-2);
		padding-inline-start: 2.5rem;
		padding-inline-end: var(--space-4);
		font-size: var(--text-sm);
		color: oklch(0.15 0.01 265);

		&:focus {
			outline: none;
			border-color: oklch(0.6 0.2 260);
			box-shadow: 0 0 0 1px oklch(0.6 0.2 260);
		}

		@media (min-width: 1024px) {
			max-inline-size: 28rem;
		}
	}

	.sl-search-icon {
		position: absolute;
		inset-inline-start: 0.75rem;
		inset-block-start: 50%;
		transform: translateY(-50%);
		inline-size: 1rem;
		block-size: 1rem;
		color: oklch(0.65 0.01 265);
	}

	/* ─── Table ─── */
	.sl-table-wrap {
		overflow-x: auto;
	}

	.sl-table {
		inline-size: 100%;
		text-align: start;
		font-size: var(--text-sm);
		border-collapse: collapse;
	}

	.sl-thead {
		border-block-end: 1px solid oklch(0.9 0.005 265);
		background-color: oklch(0.97 0.002 265);
	}

	.sl-th {
		padding-inline: var(--space-4);
		padding-block: var(--space-3);
	}

	.sl-th-right {
		text-align: end;
	}
	.sl-th-lg {
		display: none;
		@media (min-width: 1024px) {
			display: table-cell;
		}
	}
	.sl-th-md {
		display: none;
		@media (min-width: 768px) {
			display: table-cell;
		}
	}

	.sl-sort-btn {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-weight: var(--weight-semibold);
		color: oklch(0.35 0.01 265);
		background: none;
		border: none;
		cursor: pointer;

		&:hover {
			color: oklch(0.15 0.01 265);
		}
	}

	.sl-sort-icon {
		inline-size: 1rem;
		block-size: 1rem;
		transition: transform 200ms var(--ease-default);

		&[data-desc] {
			transform: rotate(180deg);
		}
	}

	.sl-tbody {
		& > tr + tr {
			border-block-start: 1px solid oklch(0.9 0.005 265);
		}
	}

	.sl-row {
		cursor: pointer;
		transition: background-color 200ms var(--ease-default);

		&:hover {
			background-color: oklch(0.97 0.002 265);
		}
	}

	.sl-td {
		padding-inline: var(--space-4);
		padding-block: var(--space-3);
	}

	.sl-td-right {
		text-align: end;
	}
	.sl-td-mono {
		font-family: var(--font-mono, monospace);
	}
	.sl-td-name {
		color: oklch(0.35 0.01 265);
	}
	.sl-td-sector {
		color: oklch(0.55 0.01 265);
	}
	.sl-td-green {
		color: oklch(0.5 0.18 160);
	}
	.sl-td-red {
		color: oklch(0.5 0.2 25);
	}
	.sl-td-lg {
		display: none;
		@media (min-width: 1024px) {
			display: table-cell;
		}
	}
	.sl-td-md {
		display: none;
		@media (min-width: 768px) {
			display: table-cell;
		}
	}

	.sl-td-notes {
		max-inline-size: 20rem;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		color: oklch(0.55 0.01 265);
	}

	.sl-symbol {
		font-weight: var(--weight-semibold);
		color: oklch(0.5 0.2 260);
	}

	.sl-empty {
		padding-block: var(--space-8);
		text-align: center;
		color: oklch(0.55 0.01 265);
	}

	/* ─── Footer ─── */
	.sl-footer {
		border-block-start: 1px solid oklch(0.9 0.005 265);
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		font-size: var(--text-sm);
		color: oklch(0.55 0.01 265);
	}
</style>

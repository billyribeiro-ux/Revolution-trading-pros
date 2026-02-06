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

	function getListTypeColor(type: string): string {
		switch (type) {
			case 'etf':
				return 'bg-purple-100 text-purple-800 dark:bg-purple-900/30 dark:text-purple-300';
			case 'stock':
				return 'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300';
			case 'watchlist':
				return 'bg-amber-100 text-amber-800 dark:bg-amber-900/30 dark:text-amber-300';
			case 'sector':
				return 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-300';
			default:
				return 'bg-gray-100 text-gray-800 dark:bg-gray-900/30 dark:text-gray-300';
		}
	}
</script>

<div
	class="stock-list-viewer rounded-xl border border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-900"
>
	{#if showHeader}
		<!-- Header -->
		<div class="border-b border-gray-200 p-4 dark:border-gray-700 {compact ? 'p-3' : 'p-4 lg:p-6'}">
			<div class="flex flex-wrap items-start justify-between gap-4">
				<div>
					<div class="flex items-center gap-2">
						<h2
							class="text-xl font-bold text-gray-900 dark:text-white {compact
								? 'text-lg'
								: 'text-xl'}"
						>
							{stockList.name}
						</h2>
						<span
							class="rounded-full px-2.5 py-0.5 text-xs font-medium capitalize {getListTypeColor(
								stockList.list_type
							)}"
						>
							{stockList.list_type}
						</span>
						{#if stockList.is_featured}
							<span
								class="rounded-full bg-yellow-100 px-2.5 py-0.5 text-xs font-medium text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-300"
							>
								Featured
							</span>
						{/if}
					</div>
					{#if stockList.description}
						<p class="mt-1 text-sm text-gray-600 dark:text-gray-400">{stockList.description}</p>
					{/if}
					<div class="mt-2 flex items-center gap-4 text-xs text-gray-500 dark:text-gray-400">
						<span>{symbols.length} symbols</span>
						{#if stockList.week_of}
							<span>Week of {formatDate(stockList.week_of)}</span>
						{/if}
						<span>Updated {formatDate(stockList.updated_at)}</span>
					</div>
				</div>

				{#if showExport}
					<div class="flex items-center gap-2">
						<button
							class="rounded-lg border border-gray-300 px-3 py-1.5 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-800"
							onclick={() => handleExport('csv')}
						>
							Export CSV
						</button>
						<button
							class="rounded-lg border border-gray-300 px-3 py-1.5 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-800"
							onclick={() => handleExport('json')}
						>
							Export JSON
						</button>
					</div>
				{/if}
			</div>

			<!-- Search -->
			{#if symbols.length > 5}
				<div class="relative mt-4">
					<input
						type="search"
						bind:value={searchQuery}
						placeholder="Search symbols, names, or sectors..."
						class="w-full rounded-lg border border-gray-300 bg-white py-2 pl-10 pr-4 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-800 dark:text-white lg:max-w-md"
					/>
					<svg
						class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
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
	<div class="overflow-x-auto">
		<table class="w-full text-left text-sm">
			<thead class="border-b border-gray-200 bg-gray-50 dark:border-gray-700 dark:bg-gray-800">
				<tr>
					<th class="px-4 py-3">
						<button
							class="flex items-center gap-1 font-semibold text-gray-700 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white"
							onclick={() => handleSort('symbol')}
						>
							Symbol
							{#if sortColumn === 'symbol'}
								<svg
									class="h-4 w-4 {sortDirection === 'desc' ? 'rotate-180' : ''}"
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
					<th class="px-4 py-3">
						<button
							class="flex items-center gap-1 font-semibold text-gray-700 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white"
							onclick={() => handleSort('name')}
						>
							Name
							{#if sortColumn === 'name'}
								<svg
									class="h-4 w-4 {sortDirection === 'desc' ? 'rotate-180' : ''}"
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
						<th class="hidden px-4 py-3 lg:table-cell">
							<button
								class="flex items-center gap-1 font-semibold text-gray-700 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white"
								onclick={() => handleSort('sector')}
							>
								Sector
							</button>
						</th>
					{/if}
					<th class="px-4 py-3 text-right">Entry</th>
					<th class="px-4 py-3 text-right">Target</th>
					<th class="px-4 py-3 text-right">Stop</th>
					{#if !compact}
						<th class="hidden px-4 py-3 md:table-cell">Notes</th>
					{/if}
				</tr>
			</thead>
			<tbody class="divide-y divide-gray-200 dark:divide-gray-700">
				{#each filteredSymbols as symbol (symbol.symbol)}
					<tr
						class="cursor-pointer transition-colors hover:bg-gray-50 dark:hover:bg-gray-800"
						onclick={() => handleSymbolClick(symbol)}
					>
						<td class="px-4 py-3">
							<span class="font-semibold text-blue-600 dark:text-blue-400">{symbol.symbol}</span>
						</td>
						<td class="px-4 py-3 text-gray-700 dark:text-gray-300">
							{symbol.name || '-'}
						</td>
						{#if !compact}
							<td class="hidden px-4 py-3 text-gray-500 dark:text-gray-400 lg:table-cell">
								{symbol.sector || '-'}
							</td>
						{/if}
						<td class="px-4 py-3 text-right font-mono text-gray-700 dark:text-gray-300">
							{symbol.entry_price ? `$${symbol.entry_price.toFixed(2)}` : '-'}
						</td>
						<td class="px-4 py-3 text-right font-mono text-green-600 dark:text-green-400">
							{symbol.price_target ? `$${symbol.price_target.toFixed(2)}` : '-'}
						</td>
						<td class="px-4 py-3 text-right font-mono text-red-600 dark:text-red-400">
							{symbol.stop_loss ? `$${symbol.stop_loss.toFixed(2)}` : '-'}
						</td>
						{#if !compact}
							<td
								class="hidden max-w-xs truncate px-4 py-3 text-gray-500 dark:text-gray-400 md:table-cell"
							>
								{symbol.notes || '-'}
							</td>
						{/if}
					</tr>
				{:else}
					<tr>
						<td
							colspan={compact ? 5 : 7}
							class="px-4 py-8 text-center text-gray-500 dark:text-gray-400"
						>
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

	<!-- Footer with count -->
	{#if filteredSymbols.length !== symbols.length}
		<div
			class="border-t border-gray-200 px-4 py-2 text-sm text-gray-500 dark:border-gray-700 dark:text-gray-400"
		>
			Showing {filteredSymbols.length} of {symbols.length} symbols
		</div>
	{/if}
</div>

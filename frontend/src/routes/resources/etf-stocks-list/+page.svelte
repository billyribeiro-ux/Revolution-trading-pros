<script lang="ts">
	import { onMount } from 'svelte';
	import { fade, fly, scale } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import {
		IconSearch,
		IconTrendingUp,
		IconTrendingDown,
		IconChartLine,
		IconFilter,
		IconDownload,
		IconStar,
		IconStarFilled
	} from '@tabler/icons-svelte';

	// ETF Data - Institutional Grade
	interface ETF {
		symbol: string;
		name: string;
		price: number;
		change: number;
		changePercent: number;
		volume: string;
		marketCap: string;
		sector: string;
		favorite: boolean;
	}

	let etfs: ETF[] = [
		{
			symbol: 'SPY',
			name: 'SPDR S&P 500 ETF Trust',
			price: 478.52,
			change: 5.23,
			changePercent: 1.1,
			volume: '82.5M',
			marketCap: '$445.2B',
			sector: 'Broad Market',
			favorite: false
		},
		{
			symbol: 'QQQ',
			name: 'Invesco QQQ Trust',
			price: 412.89,
			change: 8.45,
			changePercent: 2.09,
			volume: '45.3M',
			marketCap: '$198.7B',
			sector: 'Technology',
			favorite: false
		},
		{
			symbol: 'IWM',
			name: 'iShares Russell 2000 ETF',
			price: 198.34,
			change: -2.15,
			changePercent: -1.07,
			volume: '28.9M',
			marketCap: '$67.4B',
			sector: 'Small Cap',
			favorite: false
		},
		{
			symbol: 'DIA',
			name: 'SPDR Dow Jones Industrial Average ETF',
			price: 392.45,
			change: 3.12,
			changePercent: 0.8,
			volume: '4.2M',
			marketCap: '$31.8B',
			sector: 'Broad Market',
			favorite: false
		},
		{
			symbol: 'VTI',
			name: 'Vanguard Total Stock Market ETF',
			price: 256.78,
			change: 4.67,
			changePercent: 1.85,
			volume: '3.8M',
			marketCap: '$1.5T',
			sector: 'Broad Market',
			favorite: false
		},
		{
			symbol: 'XLF',
			name: 'Financial Select Sector SPDR Fund',
			price: 41.23,
			change: -0.45,
			changePercent: -1.08,
			volume: '52.1M',
			marketCap: '$38.9B',
			sector: 'Financial',
			favorite: false
		},
		{
			symbol: 'XLE',
			name: 'Energy Select Sector SPDR Fund',
			price: 89.67,
			change: 2.34,
			changePercent: 2.68,
			volume: '18.7M',
			marketCap: '$12.3B',
			sector: 'Energy',
			favorite: false
		},
		{
			symbol: 'XLK',
			name: 'Technology Select Sector SPDR Fund',
			price: 198.45,
			change: 6.78,
			changePercent: 3.54,
			volume: '8.9M',
			marketCap: '$52.1B',
			sector: 'Technology',
			favorite: false
		}
	];

	let searchQuery = '';
	let selectedSector = 'all';
	let sortBy: 'symbol' | 'price' | 'change' | 'volume' = 'symbol';
	let sortDirection: 'asc' | 'desc' = 'asc';

	$: sectors = ['all', ...new Set(etfs.map((e) => e.sector))];

	$: filteredETFs = etfs
		.filter((etf) => {
			const matchesSearch =
				etf.symbol.toLowerCase().includes(searchQuery.toLowerCase()) ||
				etf.name.toLowerCase().includes(searchQuery.toLowerCase());
			const matchesSector = selectedSector === 'all' || etf.sector === selectedSector;
			return matchesSearch && matchesSector;
		})
		.sort((a, b) => {
			let comparison = 0;
			switch (sortBy) {
				case 'symbol':
					comparison = a.symbol.localeCompare(b.symbol);
					break;
				case 'price':
					comparison = a.price - b.price;
					break;
				case 'change':
					comparison = a.changePercent - b.changePercent;
					break;
				case 'volume':
					comparison = parseFloat(a.volume) - parseFloat(b.volume);
					break;
			}
			return sortDirection === 'asc' ? comparison : -comparison;
		});

	function toggleFavorite(symbol: string) {
		etfs = etfs.map((etf) => (etf.symbol === symbol ? { ...etf, favorite: !etf.favorite } : etf));
	}

	function toggleSort(column: typeof sortBy) {
		if (sortBy === column) {
			sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
		} else {
			sortBy = column;
			sortDirection = 'asc';
		}
	}
</script>

<svelte:head>
	<title>ETF Stocks List | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Comprehensive list of ETF stocks for professional traders. Real-time data, advanced filtering, and institutional-grade analytics."
	/>
</svelte:head>

<main class="min-h-screen bg-rtp-bg text-rtp-text">
	<!-- Hero Section -->
	<section class="relative overflow-hidden py-24 bg-gradient-to-br from-rtp-bg via-rtp-surface to-rtp-bg">
		<div class="absolute inset-0 bg-[url('/grid.svg')] opacity-10"></div>
		<div
			class="absolute top-0 right-0 w-[600px] h-[600px] bg-rtp-primary/10 rounded-full blur-[150px]"
		></div>

		<div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-center" in:fly={{ y: 20, duration: 600, easing: cubicOut }}>
				<div class="inline-flex items-center gap-2 px-4 py-2 bg-rtp-primary/10 border border-rtp-primary/20 rounded-full mb-6">
					<IconChartLine size={20} class="text-rtp-primary" />
					<span class="text-sm font-semibold text-rtp-primary">Live Market Data</span>
				</div>
				
				<h1 class="text-6xl md:text-7xl font-extrabold mb-6 bg-gradient-to-r from-white via-rtp-primary to-white bg-clip-text text-transparent">
					ETF Stocks List
				</h1>
				
				<p class="text-xl text-rtp-muted max-w-3xl mx-auto leading-relaxed">
					Track and analyze the most actively traded Exchange-Traded Funds with institutional-grade data and real-time market insights.
				</p>
			</div>
		</div>
	</section>

	<!-- Filters & Search -->
	<section class="sticky top-0 z-40 bg-rtp-surface/95 backdrop-blur-xl border-b border-rtp-border shadow-lg">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
			<div class="flex flex-col md:flex-row gap-4 items-center justify-between">
				<!-- Search -->
				<div class="relative flex-1 max-w-md w-full">
					<IconSearch size={20} class="absolute left-4 top-1/2 -translate-y-1/2 text-rtp-muted" />
					<input
						type="text"
						bind:value={searchQuery}
						placeholder="Search ETFs by symbol or name..."
						class="w-full pl-12 pr-4 py-3 bg-rtp-bg border border-rtp-border rounded-xl text-rtp-text placeholder-rtp-muted focus:outline-none focus:ring-2 focus:ring-rtp-primary focus:border-transparent transition-all"
					/>
				</div>

				<!-- Sector Filter -->
				<div class="flex items-center gap-4">
					<div class="flex items-center gap-2">
						<IconFilter size={20} class="text-rtp-muted" />
						<select
							bind:value={selectedSector}
							class="px-4 py-3 bg-rtp-bg border border-rtp-border rounded-xl text-rtp-text focus:outline-none focus:ring-2 focus:ring-rtp-primary transition-all cursor-pointer"
						>
							{#each sectors as sector}
								<option value={sector}>{sector === 'all' ? 'All Sectors' : sector}</option>
							{/each}
						</select>
					</div>

					<button
						class="flex items-center gap-2 px-4 py-3 bg-rtp-primary/10 border border-rtp-primary/20 rounded-xl text-rtp-primary hover:bg-rtp-primary/20 transition-all"
					>
						<IconDownload size={20} />
						<span class="font-semibold">Export</span>
					</button>
				</div>
			</div>
		</div>
	</section>

	<!-- ETF Table -->
	<section class="py-12">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="bg-rtp-surface border border-rtp-border rounded-2xl overflow-hidden shadow-2xl">
				<!-- Table Header -->
				<div class="overflow-x-auto">
					<table class="w-full">
						<thead class="bg-rtp-bg border-b border-rtp-border">
							<tr>
								<th class="px-6 py-4 text-left">
									<span class="text-xs font-bold text-rtp-muted uppercase tracking-wider">Favorite</span>
								</th>
								<th class="px-6 py-4 text-left cursor-pointer hover:bg-rtp-surface/50 transition-colors" on:click={() => toggleSort('symbol')}>
									<span class="text-xs font-bold text-rtp-muted uppercase tracking-wider">Symbol</span>
								</th>
								<th class="px-6 py-4 text-left">
									<span class="text-xs font-bold text-rtp-muted uppercase tracking-wider">Name</span>
								</th>
								<th class="px-6 py-4 text-right cursor-pointer hover:bg-rtp-surface/50 transition-colors" on:click={() => toggleSort('price')}>
									<span class="text-xs font-bold text-rtp-muted uppercase tracking-wider">Price</span>
								</th>
								<th class="px-6 py-4 text-right cursor-pointer hover:bg-rtp-surface/50 transition-colors" on:click={() => toggleSort('change')}>
									<span class="text-xs font-bold text-rtp-muted uppercase tracking-wider">Change</span>
								</th>
								<th class="px-6 py-4 text-right cursor-pointer hover:bg-rtp-surface/50 transition-colors" on:click={() => toggleSort('volume')}>
									<span class="text-xs font-bold text-rtp-muted uppercase tracking-wider">Volume</span>
								</th>
								<th class="px-6 py-4 text-right">
									<span class="text-xs font-bold text-rtp-muted uppercase tracking-wider">Market Cap</span>
								</th>
								<th class="px-6 py-4 text-left">
									<span class="text-xs font-bold text-rtp-muted uppercase tracking-wider">Sector</span>
								</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-rtp-border">
							{#each filteredETFs as etf (etf.symbol)}
								<tr
									class="hover:bg-rtp-bg/50 transition-colors group"
									in:fade={{ duration: 200 }}
								>
									<td class="px-6 py-4">
										<button
											on:click={() => toggleFavorite(etf.symbol)}
											class="text-rtp-muted hover:text-yellow-400 transition-colors"
										>
											{#if etf.favorite}
												<IconStarFilled size={20} class="text-yellow-400" />
											{:else}
												<IconStar size={20} />
											{/if}
										</button>
									</td>
									<td class="px-6 py-4">
										<span class="text-lg font-bold text-rtp-primary">{etf.symbol}</span>
									</td>
									<td class="px-6 py-4">
										<span class="text-sm text-rtp-text">{etf.name}</span>
									</td>
									<td class="px-6 py-4 text-right">
										<span class="text-lg font-semibold text-rtp-text">${etf.price.toFixed(2)}</span>
									</td>
									<td class="px-6 py-4 text-right">
										<div class="flex items-center justify-end gap-2">
											{#if etf.change >= 0}
												<IconTrendingUp size={16} class="text-green-400" />
												<span class="text-sm font-semibold text-green-400">
													+${etf.change.toFixed(2)} (+{etf.changePercent.toFixed(2)}%)
												</span>
											{:else}
												<IconTrendingDown size={16} class="text-red-400" />
												<span class="text-sm font-semibold text-red-400">
													${etf.change.toFixed(2)} ({etf.changePercent.toFixed(2)}%)
												</span>
											{/if}
										</div>
									</td>
									<td class="px-6 py-4 text-right">
										<span class="text-sm text-rtp-muted">{etf.volume}</span>
									</td>
									<td class="px-6 py-4 text-right">
										<span class="text-sm text-rtp-muted">{etf.marketCap}</span>
									</td>
									<td class="px-6 py-4">
										<span class="inline-flex px-3 py-1 text-xs font-semibold rounded-full bg-rtp-primary/10 text-rtp-primary border border-rtp-primary/20">
											{etf.sector}
										</span>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>

				{#if filteredETFs.length === 0}
					<div class="py-16 text-center">
						<p class="text-rtp-muted text-lg">No ETFs found matching your criteria.</p>
					</div>
				{/if}
			</div>
		</div>
	</section>
</main>

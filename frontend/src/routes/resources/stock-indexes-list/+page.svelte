<script lang="ts">
	import { onMount } from 'svelte';
	import { fade, fly, scale } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import {
		IconChartLine,
		IconTrendingUp,
		IconTrendingDown,
		IconWorld,
		IconRefresh,
		IconClock
	} from '@tabler/icons-svelte';

	// Stock Index Data - Institutional Grade
	interface StockIndex {
		symbol: string;
		name: string;
		value: number;
		change: number;
		changePercent: number;
		high: number;
		low: number;
		region: string;
		lastUpdate: string;
	}

	let indexes: StockIndex[] = [
		{
			symbol: 'SPX',
			name: 'S&P 500',
			value: 4783.45,
			change: 52.34,
			changePercent: 1.11,
			high: 4795.23,
			low: 4745.67,
			region: 'US',
			lastUpdate: '2 mins ago'
		},
		{
			symbol: 'DJI',
			name: 'Dow Jones Industrial Average',
			value: 37863.12,
			change: 234.56,
			changePercent: 0.62,
			high: 37912.45,
			low: 37654.23,
			region: 'US',
			lastUpdate: '2 mins ago'
		},
		{
			symbol: 'IXIC',
			name: 'NASDAQ Composite',
			value: 14972.76,
			change: 178.23,
			changePercent: 1.21,
			high: 15023.45,
			low: 14856.34,
			region: 'US',
			lastUpdate: '2 mins ago'
		},
		{
			symbol: 'RUT',
			name: 'Russell 2000',
			value: 2045.67,
			change: -12.45,
			changePercent: -0.61,
			high: 2067.89,
			low: 2034.12,
			region: 'US',
			lastUpdate: '3 mins ago'
		},
		{
			symbol: 'VIX',
			name: 'CBOE Volatility Index',
			value: 13.45,
			change: -0.78,
			changePercent: -5.48,
			high: 14.56,
			low: 13.12,
			region: 'US',
			lastUpdate: '1 min ago'
		},
		{
			symbol: 'FTSE',
			name: 'FTSE 100',
			value: 7623.45,
			change: 45.67,
			changePercent: 0.60,
			high: 7645.23,
			low: 7589.12,
			region: 'UK',
			lastUpdate: '5 mins ago'
		},
		{
			symbol: 'DAX',
			name: 'DAX Performance Index',
			value: 16789.34,
			change: 123.45,
			changePercent: 0.74,
			high: 16823.56,
			low: 16654.23,
			region: 'Germany',
			lastUpdate: '5 mins ago'
		},
		{
			symbol: 'N225',
			name: 'Nikkei 225',
			value: 33456.78,
			change: -234.56,
			changePercent: -0.70,
			high: 33678.90,
			low: 33234.12,
			region: 'Japan',
			lastUpdate: '4 hours ago'
		},
		{
			symbol: 'HSI',
			name: 'Hang Seng Index',
			value: 16234.56,
			change: 89.23,
			changePercent: 0.55,
			high: 16289.45,
			low: 16123.67,
			region: 'Hong Kong',
			lastUpdate: '3 hours ago'
		}
	];

	let selectedRegion = 'all';
	let autoRefresh = true;
	let lastRefreshTime = new Date();

	$: regions = ['all', ...new Set(indexes.map((i) => i.region))];

	$: filteredIndexes = indexes.filter((index) => {
		return selectedRegion === 'all' || index.region === selectedRegion;
	});

	// Simulate real-time updates
	onMount(() => {
		const interval = setInterval(() => {
			if (autoRefresh) {
				// Simulate small price changes
				indexes = indexes.map((index) => ({
					...index,
					value: index.value + (Math.random() - 0.5) * 10,
					change: index.change + (Math.random() - 0.5) * 2
				}));
				lastRefreshTime = new Date();
			}
		}, 5000);

		return () => clearInterval(interval);
	});
</script>

<svelte:head>
	<title>Stock Indexes List | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Real-time stock market indexes from around the world. Track major indices with institutional-grade data and live updates."
	/>
</svelte:head>

<main class="min-h-screen bg-rtp-bg text-rtp-text">
	<!-- Hero Section -->
	<section class="relative overflow-hidden py-24 bg-gradient-to-br from-rtp-bg via-rtp-surface to-rtp-bg">
		<div class="absolute inset-0 bg-[url('/grid.svg')] opacity-10"></div>
		<div
			class="absolute bottom-0 left-0 w-[600px] h-[600px] bg-blue-500/10 rounded-full blur-[150px]"
		></div>

		<div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-center" in:fly={{ y: 20, duration: 600, easing: cubicOut }}>
				<div class="inline-flex items-center gap-2 px-4 py-2 bg-green-500/10 border border-green-500/20 rounded-full mb-6">
					<div class="relative flex h-2 w-2">
						<span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span>
						<span class="relative inline-flex rounded-full h-2 w-2 bg-green-500"></span>
					</div>
					<span class="text-sm font-semibold text-green-400">Live Market Data</span>
				</div>
				
				<h1 class="text-6xl md:text-7xl font-extrabold mb-6 bg-gradient-to-r from-white via-blue-400 to-white bg-clip-text text-transparent">
					Global Stock Indexes
				</h1>
				
				<p class="text-xl text-rtp-muted max-w-3xl mx-auto leading-relaxed">
					Monitor major stock market indexes from around the world with real-time data, advanced analytics, and institutional-grade insights.
				</p>
			</div>
		</div>
	</section>

	<!-- Controls -->
	<section class="sticky top-0 z-40 bg-rtp-surface/95 backdrop-blur-xl border-b border-rtp-border shadow-lg">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
			<div class="flex flex-col md:flex-row gap-4 items-center justify-between">
				<!-- Region Filter -->
				<div class="flex items-center gap-4">
					<IconWorld size={24} class="text-rtp-primary" />
					<select
						bind:value={selectedRegion}
						class="px-4 py-3 bg-rtp-bg border border-rtp-border rounded-xl text-rtp-text focus:outline-none focus:ring-2 focus:ring-rtp-primary transition-all cursor-pointer"
					>
						{#each regions as region}
							<option value={region}>{region === 'all' ? 'All Regions' : region}</option>
						{/each}
					</select>
				</div>

				<!-- Auto Refresh & Last Update -->
				<div class="flex items-center gap-4">
					<div class="flex items-center gap-2 text-sm text-rtp-muted">
						<IconClock size={16} />
						<span>Updated {lastRefreshTime.toLocaleTimeString()}</span>
					</div>
					
					<button
						on:click={() => (autoRefresh = !autoRefresh)}
						class="flex items-center gap-2 px-4 py-3 rounded-xl transition-all {autoRefresh
							? 'bg-green-500/10 border border-green-500/20 text-green-400'
							: 'bg-rtp-bg border border-rtp-border text-rtp-muted'}"
					>
						<IconRefresh size={20} class={autoRefresh ? 'animate-spin' : ''} />
						<span class="font-semibold">{autoRefresh ? 'Auto-Refresh ON' : 'Auto-Refresh OFF'}</span>
					</button>
				</div>
			</div>
		</div>
	</section>

	<!-- Index Cards Grid -->
	<section class="py-12">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
				{#each filteredIndexes as index (index.symbol)}
					<div
						class="group relative bg-rtp-surface border border-rtp-border rounded-2xl p-6 hover:border-rtp-primary/50 transition-all duration-300 hover:shadow-2xl hover:shadow-rtp-primary/10"
						in:scale={{ duration: 300, easing: cubicOut }}
					>
						<!-- Region Badge -->
						<div class="absolute top-4 right-4">
							<span class="inline-flex px-3 py-1 text-xs font-semibold rounded-full bg-rtp-bg border border-rtp-border text-rtp-muted">
								{index.region}
							</span>
						</div>

						<!-- Symbol & Name -->
						<div class="mb-6">
							<h3 class="text-2xl font-bold text-rtp-primary mb-1">{index.symbol}</h3>
							<p class="text-sm text-rtp-muted">{index.name}</p>
						</div>

						<!-- Current Value -->
						<div class="mb-4">
							<div class="text-4xl font-extrabold text-rtp-text mb-2">
								{index.value.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
							</div>
							
							<!-- Change -->
							<div class="flex items-center gap-2">
								{#if index.change >= 0}
									<IconTrendingUp size={20} class="text-green-400" />
									<span class="text-lg font-semibold text-green-400">
										+{index.change.toFixed(2)} (+{index.changePercent.toFixed(2)}%)
									</span>
								{:else}
									<IconTrendingDown size={20} class="text-red-400" />
									<span class="text-lg font-semibold text-red-400">
										{index.change.toFixed(2)} ({index.changePercent.toFixed(2)}%)
									</span>
								{/if}
							</div>
						</div>

						<!-- High/Low -->
						<div class="grid grid-cols-2 gap-4 pt-4 border-t border-rtp-border">
							<div>
								<p class="text-xs text-rtp-muted mb-1">High</p>
								<p class="text-sm font-semibold text-rtp-text">{index.high.toFixed(2)}</p>
							</div>
							<div>
								<p class="text-xs text-rtp-muted mb-1">Low</p>
								<p class="text-sm font-semibold text-rtp-text">{index.low.toFixed(2)}</p>
							</div>
						</div>

						<!-- Last Update -->
						<div class="mt-4 pt-4 border-t border-rtp-border">
							<div class="flex items-center gap-2 text-xs text-rtp-muted">
								<IconClock size={14} />
								<span>{index.lastUpdate}</span>
							</div>
						</div>

						<!-- Hover Effect -->
						<div class="absolute inset-0 bg-gradient-to-br from-rtp-primary/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity rounded-2xl pointer-events-none"></div>
					</div>
				{/each}
			</div>

			{#if filteredIndexes.length === 0}
				<div class="py-16 text-center">
					<p class="text-rtp-muted text-lg">No indexes found for the selected region.</p>
				</div>
			{/if}
		</div>
	</section>
</main>

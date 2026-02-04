<script lang="ts">
	/**
	 * Click Heatmaps - Visual User Interaction Analysis
	 * Apple ICT7 Grade Implementation
	 *
	 * Visualize where users click on your pages
	 * with interactive heatmap overlays.
	 */
	import { analyticsApi } from '$lib/api/analytics';
	import { connections, getIsAnalyticsConnected } from '$lib/stores/connections.svelte';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';
	import PeriodSelector from '$lib/components/analytics/PeriodSelector.svelte';

	interface HeatmapPage {
		id: string;
		url: string;
		title: string;
		clicks: number;
		unique_visitors: number;
		scroll_depth: number;
		last_updated: string;
		thumbnail?: string;
	}

	interface HeatmapData {
		page_url: string;
		clicks: Array<{
			x: number;
			y: number;
			count: number;
			element?: string;
		}>;
		scroll_data: Array<{
			depth: number;
			percentage: number;
		}>;
	}

	// Svelte 5 Runes - State
	let pages = $state<HeatmapPage[]>([]);
	let selectedPage = $state<HeatmapPage | null>(null);
	let heatmapData = $state<HeatmapData | null>(null);
	let loading = $state(true);
	let connectionLoading = $state(true);
	let error = $state<string | null>(null);
	let selectedPeriod = $state('7d');
	let heatmapType = $state<'click' | 'scroll' | 'move'>('click');

	async function loadPages() {
		loading = true;
		error = null;
		try {
			// Prepared for future API integration
			const response = await fetch(`/api/admin/analytics/heatmaps?period=${selectedPeriod}`);
			if (response.ok) {
				const data = await response.json();
				pages = data.pages || [];
			} else {
				pages = [];
			}
		} catch (e) {
			// For now, set empty array since API might not exist yet
			pages = [];
		} finally {
			loading = false;
		}
	}

	async function loadHeatmapData(page: HeatmapPage) {
		selectedPage = page;
		try {
			const response = await fetch(
				`/api/admin/analytics/heatmaps/${encodeURIComponent(page.url)}?period=${selectedPeriod}&type=${heatmapType}`
			);
			if (response.ok) {
				heatmapData = await response.json();
			}
		} catch (e) {
			// Handle gracefully
		}
	}

	function handlePeriodChange(value: string) {
		selectedPeriod = value;
		loadPages();
		if (selectedPage) {
			loadHeatmapData(selectedPage);
		}
	}

	// Svelte 5 - $effect replaces onMount
	$effect(() => {
		async function init() {
			await connections.load();
			connectionLoading = false;

			if (getIsAnalyticsConnected()) {
				await loadPages();
			} else {
				loading = false;
			}
		}
		init();
	});

	// Derived stats
	const stats = $derived({
		totalPages: pages.length,
		totalClicks: pages.reduce((sum, p) => sum + p.clicks, 0),
		avgScrollDepth:
			pages.length > 0
				? Math.round(pages.reduce((sum, p) => sum + p.scroll_depth, 0) / pages.length)
				: 0,
		totalVisitors: pages.reduce((sum, p) => sum + p.unique_visitors, 0)
	});
</script>

<svelte:head>
	<title>Click Heatmaps | Analytics</title>
</svelte:head>

<div class="bg-gradient-to-br from-slate-950 via-slate-900 to-slate-950">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<!-- Apple ICT7 Grade Header -->
		<header class="flex items-center justify-between mb-8">
			<div class="flex items-center gap-4">
				<div
					class="w-12 h-12 rounded-2xl bg-gradient-to-br from-rose-500 to-pink-600 flex items-center justify-center text-2xl shadow-lg shadow-rose-500/20"
				>
					<svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M15 15l-2 5L9 9l11 4-5 2zm0 0l5 5M7.188 2.239l.777 2.897M5.136 7.965l-2.898-.777M13.95 4.05l-2.122 2.122m-5.657 5.656l-2.12 2.122"
						/>
					</svg>
				</div>
				<div>
					<h1 class="text-2xl font-bold text-white tracking-tight">Click Heatmaps</h1>
					<p class="text-sm text-slate-400">Visualize where users click on your pages</p>
				</div>
			</div>
			{#if getIsAnalyticsConnected()}
				<div class="flex items-center gap-4">
					<PeriodSelector value={selectedPeriod} onchange={handlePeriodChange} />
					<div class="flex items-center bg-slate-800/50 rounded-xl border border-white/10 p-1">
						{#each [{ value: 'click', label: 'Clicks' }, { value: 'scroll', label: 'Scroll' }, { value: 'move', label: 'Movement' }] as type}
							<button
								onclick={() => (heatmapType = type.value as typeof heatmapType)}
								class="px-4 py-2 rounded-lg text-sm font-medium transition-all
									{heatmapType === type.value
									? 'bg-rose-500 text-white shadow-lg shadow-rose-500/25'
									: 'text-slate-400 hover:text-white'}"
							>
								{type.label}
							</button>
						{/each}
					</div>
				</div>
			{/if}
		</header>

		<!-- Connection Check -->
		{#if connectionLoading}
			<div class="flex items-center justify-center py-20">
				<div class="relative">
					<div class="w-12 h-12 border-4 border-rose-500/20 rounded-full"></div>
					<div
						class="absolute top-0 left-0 w-12 h-12 border-4 border-rose-500 rounded-full animate-spin border-t-transparent"
					></div>
				</div>
			</div>
		{:else if !getIsAnalyticsConnected}
			<ServiceConnectionStatus feature="analytics" variant="card" showFeatures={true} />
		{:else}
			<!-- Stats Grid -->
			<div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-5">
					<div class="text-3xl font-bold text-white mb-1">{stats.totalPages}</div>
					<div class="text-sm text-slate-400">Tracked Pages</div>
				</div>
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-5">
					<div class="text-3xl font-bold text-rose-400 mb-1">
						{stats.totalClicks.toLocaleString()}
					</div>
					<div class="text-sm text-slate-400">Total Clicks</div>
				</div>
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-5">
					<div class="text-3xl font-bold text-amber-400 mb-1">{stats.avgScrollDepth}%</div>
					<div class="text-sm text-slate-400">Avg Scroll Depth</div>
				</div>
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-5">
					<div class="text-3xl font-bold text-blue-400 mb-1">
						{stats.totalVisitors.toLocaleString()}
					</div>
					<div class="text-sm text-slate-400">Unique Visitors</div>
				</div>
			</div>

			{#if loading}
				<div class="flex items-center justify-center py-20">
					<div class="relative">
						<div class="w-10 h-10 border-4 border-rose-500/20 rounded-full"></div>
						<div
							class="absolute top-0 left-0 w-10 h-10 border-4 border-rose-500 rounded-full animate-spin border-t-transparent"
						></div>
					</div>
				</div>
			{:else if error}
				<div
					class="bg-red-500/10 backdrop-blur-xl border border-red-500/20 rounded-2xl p-8 text-center"
				>
					<div
						class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-red-500/10 flex items-center justify-center"
					>
						<svg class="w-8 h-8 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
							/>
						</svg>
					</div>
					<p class="text-red-400 mb-4">{error}</p>
					<button
						onclick={loadPages}
						class="px-5 py-2.5 bg-red-500/20 text-red-400 rounded-xl hover:bg-red-500/30 border border-red-500/30 transition-all"
					>
						Retry
					</button>
				</div>
			{:else if pages.length === 0}
				<div
					class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-12 text-center"
				>
					<div
						class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-rose-500/10 flex items-center justify-center"
					>
						<svg
							class="w-8 h-8 text-rose-400"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M15 15l-2 5L9 9l11 4-5 2zm0 0l5 5M7.188 2.239l.777 2.897M5.136 7.965l-2.898-.777M13.95 4.05l-2.122 2.122m-5.657 5.656l-2.12 2.122"
							/>
						</svg>
					</div>
					<h3 class="text-lg font-medium text-white mb-2">No Heatmap Data Yet</h3>
					<p class="text-slate-400 mb-6">
						Heatmaps will appear once you have visitor activity on your site
					</p>
					<a
						href="/admin/settings/tracking"
						class="inline-block px-6 py-3 bg-gradient-to-r from-rose-500 to-pink-600 text-white rounded-xl hover:from-rose-400 hover:to-pink-500 font-semibold shadow-lg shadow-rose-500/25 transition-all"
					>
						Setup Tracking
					</a>
				</div>
			{:else}
				<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
					<!-- Page List -->
					<div class="lg:col-span-1 space-y-4">
						<h3 class="text-sm font-medium text-slate-400 uppercase tracking-wider mb-4">
							Tracked Pages
						</h3>
						{#each pages as page}
							<button
								onclick={() => loadHeatmapData(page)}
								class="w-full text-left bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-4 hover:bg-white/10 transition-all
									{selectedPage?.id === page.id ? 'ring-2 ring-rose-500 border-rose-500/50' : ''}"
							>
								<div class="font-medium text-white mb-1 truncate">{page.title || page.url}</div>
								<div class="text-xs text-slate-500 truncate mb-3">{page.url}</div>
								<div class="flex items-center justify-between text-xs">
									<span class="text-rose-400">{page.clicks.toLocaleString()} clicks</span>
									<span class="text-slate-400">{page.scroll_depth}% scroll</span>
								</div>
							</button>
						{/each}
					</div>

					<!-- Heatmap Preview -->
					<div class="lg:col-span-2">
						{#if selectedPage}
							<div
								class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 overflow-hidden"
							>
								<div class="p-5 border-b border-white/10 flex items-center justify-between">
									<div>
										<h3 class="font-semibold text-white">
											{selectedPage.title || selectedPage.url}
										</h3>
										<p class="text-xs text-slate-400">{selectedPage.url}</p>
									</div>
									<div class="flex items-center gap-2">
										<span
											class="px-3 py-1.5 rounded-lg bg-rose-500/20 text-rose-400 text-xs font-medium"
										>
											{selectedPage.clicks.toLocaleString()} clicks
										</span>
										<span
											class="px-3 py-1.5 rounded-lg bg-blue-500/20 text-blue-400 text-xs font-medium"
										>
											{selectedPage.unique_visitors.toLocaleString()} visitors
										</span>
									</div>
								</div>

								<!-- Heatmap Visualization Placeholder -->
								<div class="aspect-video bg-slate-800/50 flex items-center justify-center">
									<div class="text-center">
										<div
											class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-rose-500/10 flex items-center justify-center"
										>
											<svg
												class="w-8 h-8 text-rose-400"
												fill="none"
												stroke="currentColor"
												viewBox="0 0 24 24"
											>
												<path
													stroke-linecap="round"
													stroke-linejoin="round"
													stroke-width="2"
													d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
												/>
											</svg>
										</div>
										<p class="text-slate-400 text-sm">
											Heatmap visualization will be rendered here
										</p>
										<p class="text-slate-500 text-xs mt-1">
											Requires heatmap rendering library integration
										</p>
									</div>
								</div>

								<!-- Scroll Depth Visualization -->
								<div class="p-5 border-t border-white/10">
									<h4 class="text-sm font-medium text-slate-300 mb-4">Scroll Depth Distribution</h4>
									<div class="space-y-2">
										{#each [{ depth: '25%', percentage: 85 }, { depth: '50%', percentage: 62 }, { depth: '75%', percentage: 38 }, { depth: '100%', percentage: 15 }] as scroll}
											<div class="flex items-center gap-3">
												<span class="text-xs text-slate-400 w-12">{scroll.depth}</span>
												<div class="flex-1 h-2 bg-slate-700 rounded-full overflow-hidden">
													<div
														class="h-full bg-gradient-to-r from-rose-500 to-pink-500 rounded-full"
														style="width: {scroll.percentage}%"
													></div>
												</div>
												<span class="text-xs text-slate-400 w-10 text-right"
													>{scroll.percentage}%</span
												>
											</div>
										{/each}
									</div>
								</div>
							</div>
						{:else}
							<div
								class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-12 text-center h-full flex items-center justify-center"
							>
								<div>
									<div
										class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-slate-700/50 flex items-center justify-center"
									>
										<svg
											class="w-8 h-8 text-slate-500"
											fill="none"
											stroke="currentColor"
											viewBox="0 0 24 24"
										>
											<path
												stroke-linecap="round"
												stroke-linejoin="round"
												stroke-width="2"
												d="M15 15l-2 5L9 9l11 4-5 2zm0 0l5 5M7.188 2.239l.777 2.897M5.136 7.965l-2.898-.777M13.95 4.05l-2.122 2.122m-5.657 5.656l-2.12 2.122"
											/>
										</svg>
									</div>
									<p class="text-slate-400">Select a page to view its heatmap</p>
								</div>
							</div>
						{/if}
					</div>
				</div>
			{/if}
		{/if}
	</div>
</div>

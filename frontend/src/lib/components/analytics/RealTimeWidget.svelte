<script lang="ts">
	/**
	 * RealTimeWidget - Live Analytics Dashboard Widget
	 *
	 * Displays real-time metrics with auto-refresh and
	 * animated counters.
	 */
	import { onMount, onDestroy } from 'svelte';
	import { analyticsApi, type RealTimeMetrics } from '$lib/api/analytics';

	export let refreshInterval: number = 30000; // 30 seconds
	export let compact: boolean = false;

	let metrics: RealTimeMetrics | null = null;
	let loading = true;
	let error: string | null = null;
	let lastUpdated: Date | null = null;
	let interval: ReturnType<typeof setInterval>;

	async function fetchMetrics() {
		try {
			const response = await analyticsApi.getRealTimeMetrics();
			metrics = response.metrics;
			lastUpdated = new Date();
			error = null;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load metrics';
		} finally {
			loading = false;
		}
	}

	onMount(() => {
		fetchMetrics();
		interval = setInterval(fetchMetrics, refreshInterval);
	});

	onDestroy(() => {
		if (interval) clearInterval(interval);
	});

	function formatNumber(num: number): string {
		if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
		if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
		return num.toString();
	}

	function formatCurrency(num: number): string {
		return '$' + formatNumber(num);
	}

	function formatTime(date: Date): string {
		return date.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' });
	}
</script>

<div class="bg-gradient-to-br from-gray-900 to-gray-800 rounded-xl p-4 text-white">
	<!-- Header -->
	<div class="flex items-center justify-between mb-4">
		<div class="flex items-center gap-2">
			<div class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
			<span class="text-sm font-medium">Real-Time</span>
		</div>
		{#if lastUpdated}
			<span class="text-xs text-gray-400">
				Updated {formatTime(lastUpdated)}
			</span>
		{/if}
	</div>

	{#if loading}
		<div class="flex items-center justify-center py-8">
			<div class="w-6 h-6 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
		</div>
	{:else if error}
		<div class="text-center py-4 text-red-400">
			<p>{error}</p>
		</div>
	{:else if metrics}
		<!-- Main Metrics -->
		<div class="grid {compact ? 'grid-cols-2' : 'grid-cols-2 md:grid-cols-4'} gap-4 mb-4">
			<div class="text-center">
				<div class="text-2xl font-bold text-green-400">
					{formatNumber(metrics.active_users)}
				</div>
				<div class="text-xs text-gray-400">Active Users</div>
			</div>
			<div class="text-center">
				<div class="text-2xl font-bold text-blue-400">
					{formatNumber(metrics.page_views)}
				</div>
				<div class="text-xs text-gray-400">Page Views</div>
			</div>
			<div class="text-center">
				<div class="text-2xl font-bold text-purple-400">
					{formatNumber(metrics.conversions)}
				</div>
				<div class="text-xs text-gray-400">Conversions</div>
			</div>
			<div class="text-center">
				<div class="text-2xl font-bold text-yellow-400">
					{formatCurrency(metrics.revenue)}
				</div>
				<div class="text-xs text-gray-400">Revenue</div>
			</div>
		</div>

		{#if !compact}
			<!-- Top Pages -->
			{#if metrics.top_pages && metrics.top_pages.length > 0}
				<div class="border-t border-gray-700 pt-4">
					<h4 class="text-xs font-medium text-gray-400 mb-2">Top Pages (30m)</h4>
					<div class="space-y-2">
						{#each metrics.top_pages.slice(0, 5) as page}
							<div class="flex items-center justify-between text-sm">
								<span class="text-gray-300 truncate max-w-[200px]">{page.page_path}</span>
								<span class="text-gray-400">{page.views}</span>
							</div>
						{/each}
					</div>
				</div>
			{/if}

			<!-- Top Events -->
			{#if metrics.top_events && metrics.top_events.length > 0}
				<div class="border-t border-gray-700 pt-4 mt-4">
					<h4 class="text-xs font-medium text-gray-400 mb-2">Top Events (30m)</h4>
					<div class="flex flex-wrap gap-2">
						{#each metrics.top_events.slice(0, 8) as event}
							<span class="px-2 py-1 bg-gray-700 rounded text-xs">
								{event.event_name}
								<span class="text-gray-400 ml-1">{event.count}</span>
							</span>
						{/each}
					</div>
				</div>
			{/if}
		{/if}
	{/if}
</div>

<script lang="ts">
	/**
	 * RealTimeWidget - Live Analytics Dashboard Widget
	 *
	 * Displays real-time metrics with auto-refresh and
	 * animated counters.
	 */
	import { onMount } from 'svelte';
	import { analyticsApi, type RealTimeMetrics } from '$lib/api/analytics';

	interface Props {
		refreshInterval?: number;
		compact?: boolean;
	}

	let { refreshInterval = 30000, compact = false }: Props = $props();

	let metrics: RealTimeMetrics | null = $state(null);
	let loading = $state(true);
	let error: string | null = $state(null);
	let lastUpdated: Date | null = $state(null);
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

		// Cleanup function (Svelte 5 pattern)
		return () => {
			if (interval) clearInterval(interval);
		};
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

<div class="rt-widget">
	<!-- Header -->
	<div class="rt-header">
		<div class="rt-status">
			<div class="rt-dot"></div>
			<span class="rt-label">Real-Time</span>
		</div>
		{#if lastUpdated}
			<span class="rt-updated">
				Updated {formatTime(lastUpdated)}
			</span>
		{/if}
	</div>

	{#if loading}
		<div class="rt-loading">
			<div class="rt-spinner"></div>
		</div>
	{:else if error}
		<div class="rt-error">
			<p>{error}</p>
		</div>
	{:else if metrics}
		<!-- Main Metrics -->
		<div class="rt-metrics" data-compact={compact || undefined}>
			<div class="metric-item">
				<div class="metric-value" data-color="green">
					{formatNumber(metrics.active_users)}
				</div>
				<div class="metric-label">Active Users</div>
			</div>
			<div class="metric-item">
				<div class="metric-value" data-color="blue">
					{formatNumber(metrics.page_views)}
				</div>
				<div class="metric-label">Page Views</div>
			</div>
			<div class="metric-item">
				<div class="metric-value" data-color="purple">
					{formatNumber(metrics.conversions)}
				</div>
				<div class="metric-label">Conversions</div>
			</div>
			<div class="metric-item">
				<div class="metric-value" data-color="yellow">
					{formatCurrency(metrics.revenue)}
				</div>
				<div class="metric-label">Revenue</div>
			</div>
		</div>

		{#if !compact}
			<!-- Top Pages -->
			{#if metrics.top_pages && metrics.top_pages.length > 0}
				<div class="rt-section">
					<h4 class="rt-section-title">Top Pages (30m)</h4>
					<div class="rt-list">
						{#each metrics.top_pages.slice(0, 5) as page (page.page_path)}
							<div class="rt-list-item">
								<span class="rt-page-path">{page.page_path}</span>
								<span class="rt-page-views">{page.views}</span>
							</div>
						{/each}
					</div>
				</div>
			{/if}

			<!-- Top Events -->
			{#if metrics.top_events && metrics.top_events.length > 0}
				<div class="rt-section rt-events-section">
					<h4 class="rt-section-title">Top Events (30m)</h4>
					<div class="rt-tags">
						{#each metrics.top_events.slice(0, 8) as event (event.event_name)}
							<span class="rt-tag">
								{event.event_name}
								<span class="rt-tag-count">{event.count}</span>
							</span>
						{/each}
					</div>
				</div>
			{/if}
		{/if}
	{/if}
</div>

<style>
	.rt-widget {
		background: linear-gradient(to bottom right, oklch(0.17 0.02 260), oklch(0.22 0.02 250));
		border-radius: var(--radius-xl);
		padding: var(--space-4);
		color: oklch(1 0 0);
	}

	.rt-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: var(--space-4);
	}

	.rt-status { display: flex; align-items: center; gap: var(--space-2); }

	.rt-dot {
		inline-size: 0.5rem;
		block-size: 0.5rem;
		background-color: oklch(0.7 0.18 160);
		border-radius: 9999px;
		animation: pulse 2s ease-in-out infinite;
	}

	@keyframes pulse {
		0%, 100% { opacity: 1; }
		50% { opacity: 0.5; }
	}

	.rt-label { font-size: var(--text-sm); font-weight: var(--weight-medium); }
	.rt-updated { font-size: var(--text-xs); color: oklch(0.65 0.01 250); }

	.rt-loading {
		display: flex;
		align-items: center;
		justify-content: center;
		padding-block: var(--space-8);
	}

	.rt-spinner {
		inline-size: 1.5rem;
		block-size: 1.5rem;
		border: 2px solid oklch(1 0 0 / 20%);
		border-block-start-color: oklch(1 0 0);
		border-radius: 9999px;
		animation: spin 1s linear infinite;
	}

	@keyframes spin { to { transform: rotate(360deg); } }

	.rt-error {
		text-align: center;
		padding-block: var(--space-4);
		color: oklch(0.7 0.2 25);
	}

	.rt-metrics {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: var(--space-4);
		margin-block-end: var(--space-4);

		&:not([data-compact]) {
			@media (min-width: 768px) { grid-template-columns: repeat(4, 1fr); }
		}
	}

	.metric-item { text-align: center; }

	.metric-value {
		font-size: var(--text-2xl);
		font-weight: var(--weight-bold);

		&[data-color='green'] { color: oklch(0.7 0.18 160); }
		&[data-color='blue'] { color: oklch(0.7 0.18 260); }
		&[data-color='purple'] { color: oklch(0.7 0.18 300); }
		&[data-color='yellow'] { color: oklch(0.8 0.18 90); }
	}

	.metric-label { font-size: var(--text-xs); color: oklch(0.65 0.01 250); }

	.rt-section {
		border-block-start: 1px solid oklch(0.38 0.01 250);
		padding-block-start: var(--space-4);
	}

	.rt-events-section { margin-block-start: var(--space-4); }

	.rt-section-title {
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
		color: oklch(0.65 0.01 250);
		margin-block-end: var(--space-2);
	}

	.rt-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.rt-list-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		font-size: var(--text-sm);
	}

	.rt-page-path {
		color: oklch(0.75 0.01 250);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		max-inline-size: 200px;
	}

	.rt-page-views { color: oklch(0.65 0.01 250); }

	.rt-tags { display: flex; flex-wrap: wrap; gap: var(--space-2); }

	.rt-tag {
		padding-inline: var(--space-2);
		padding-block: var(--space-1);
		background-color: oklch(0.38 0.01 250);
		border-radius: var(--radius-sm);
		font-size: var(--text-xs);
	}

	.rt-tag-count { color: oklch(0.65 0.01 250); margin-inline-start: var(--space-1); }
</style>

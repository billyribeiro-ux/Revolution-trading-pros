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

<div class="real-time-widget">
	<!-- Header -->
	<div class="real-time-widget__header">
		<div class="real-time-widget__title">
			<div class="real-time-widget__status-dot"></div>
			<span>Real-Time</span>
		</div>
		{#if lastUpdated}
			<span class="real-time-widget__timestamp">
				Updated {formatTime(lastUpdated)}
			</span>
		{/if}
	</div>

	{#if loading}
		<div class="real-time-widget__loading">
			<div class="real-time-widget__spinner"></div>
		</div>
	{:else if error}
		<div class="real-time-widget__error">
			<p>{error}</p>
		</div>
	{:else if metrics}
		<!-- Main Metrics -->
		<div
			class={{
				'real-time-widget__metrics': true,
				'real-time-widget__metrics--compact': compact
			}}
		>
			<div class="real-time-widget__metric">
				<div class="real-time-widget__metric-value real-time-widget__metric-value--active">
					{formatNumber(metrics.active_users)}
				</div>
				<div class="real-time-widget__metric-label">Active Users</div>
			</div>
			<div class="real-time-widget__metric">
				<div class="real-time-widget__metric-value real-time-widget__metric-value--views">
					{formatNumber(metrics.page_views)}
				</div>
				<div class="real-time-widget__metric-label">Page Views</div>
			</div>
			<div class="real-time-widget__metric">
				<div class="real-time-widget__metric-value real-time-widget__metric-value--conversions">
					{formatNumber(metrics.conversions)}
				</div>
				<div class="real-time-widget__metric-label">Conversions</div>
			</div>
			<div class="real-time-widget__metric">
				<div class="real-time-widget__metric-value real-time-widget__metric-value--revenue">
					{formatCurrency(metrics.revenue)}
				</div>
				<div class="real-time-widget__metric-label">Revenue</div>
			</div>
		</div>

		{#if !compact}
			<!-- Top Pages -->
			{#if metrics.top_pages && metrics.top_pages.length > 0}
				<div class="real-time-widget__section">
					<h4>Top Pages (30m)</h4>
					<div class="real-time-widget__rows">
						{#each metrics.top_pages.slice(0, 5) as page (page.page_path)}
							<div class="real-time-widget__row">
								<span class="real-time-widget__path">{page.page_path}</span>
								<span class="real-time-widget__row-count">{page.views}</span>
							</div>
						{/each}
					</div>
				</div>
			{/if}

			<!-- Top Events -->
			{#if metrics.top_events && metrics.top_events.length > 0}
				<div class="real-time-widget__section">
					<h4>Top Events (30m)</h4>
					<div class="real-time-widget__events">
						{#each metrics.top_events.slice(0, 8) as event (event.event_name)}
							<span class="real-time-widget__event">
								{event.event_name}
								<span>{event.count}</span>
							</span>
						{/each}
					</div>
				</div>
			{/if}
		{/if}
	{/if}
</div>

<style>
	.real-time-widget {
		border-radius: 0.75rem;
		background: linear-gradient(135deg, #0f172a, #1e293b);
		color: #ffffff;
		padding: 1rem;
	}

	.real-time-widget__header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.real-time-widget__title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		line-height: 1.25rem;
	}

	.real-time-widget__status-dot {
		width: 0.5rem;
		height: 0.5rem;
		border-radius: 999px;
		background: #4ade80;
		animation: pulse-dot 1.6s ease-in-out infinite;
	}

	.real-time-widget__timestamp,
	.real-time-widget__metric-label,
	.real-time-widget__row-count,
	.real-time-widget__event span {
		color: #9ca3af;
	}

	.real-time-widget__timestamp,
	.real-time-widget__metric-label,
	.real-time-widget__event {
		font-size: 0.75rem;
		line-height: 1rem;
	}

	.real-time-widget__loading {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2rem 0;
	}

	.real-time-widget__spinner {
		width: 1.5rem;
		height: 1.5rem;
		border: 2px solid rgba(255, 255, 255, 0.2);
		border-top-color: #ffffff;
		border-radius: 999px;
		animation: spin 900ms linear infinite;
	}

	.real-time-widget__error {
		color: #f87171;
		padding: 1rem 0;
		text-align: center;
	}

	.real-time-widget__error p {
		margin: 0;
	}

	.real-time-widget__metrics {
		display: grid;
		grid-template-columns: repeat(4, minmax(0, 1fr));
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.real-time-widget__metrics--compact {
		grid-template-columns: repeat(2, minmax(0, 1fr));
	}

	.real-time-widget__metric {
		text-align: center;
	}

	.real-time-widget__metric-value {
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 2rem;
	}

	.real-time-widget__metric-value--active {
		color: #4ade80;
	}

	.real-time-widget__metric-value--views {
		color: #60a5fa;
	}

	.real-time-widget__metric-value--conversions {
		color: #c084fc;
	}

	.real-time-widget__metric-value--revenue {
		color: #facc15;
	}

	.real-time-widget__section {
		border-top: 1px solid #374151;
		padding-top: 1rem;
	}

	.real-time-widget__section + .real-time-widget__section {
		margin-top: 1rem;
	}

	.real-time-widget__section h4 {
		margin: 0 0 0.5rem;
		color: #9ca3af;
		font-size: 0.75rem;
		font-weight: 500;
		line-height: 1rem;
	}

	.real-time-widget__rows {
		display: grid;
		gap: 0.5rem;
	}

	.real-time-widget__row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.real-time-widget__path {
		overflow: hidden;
		max-width: 12.5rem;
		color: #d1d5db;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.real-time-widget__events {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.real-time-widget__event {
		border-radius: 0.25rem;
		background: #374151;
		padding: 0.25rem 0.5rem;
	}

	.real-time-widget__event span {
		margin-left: 0.25rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@keyframes pulse-dot {
		0%,
		100% {
			opacity: 1;
			transform: scale(1);
		}

		50% {
			opacity: 0.55;
			transform: scale(0.86);
		}
	}

	@media (max-width: 768px) {
		.real-time-widget__metrics {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	@media (max-width: 420px) {
		.real-time-widget__header,
		.real-time-widget__row {
			align-items: flex-start;
			flex-direction: column;
			gap: 0.25rem;
		}

		.real-time-widget__path {
			max-width: 100%;
		}
	}
</style>

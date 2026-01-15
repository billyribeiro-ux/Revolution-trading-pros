<script lang="ts">
	/**
	 * Video Analytics Dashboard - Revolution Trading Pros
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 */

	import { onMount } from 'svelte';
	import { analyticsApi, type AnalyticsDashboard, type VideoAnalytics } from '$lib/api/video-advanced';
	import IconChartBar from '@tabler/icons-svelte/icons/chart-bar';
	import IconEye from '@tabler/icons-svelte/icons/eye';
	import IconClock from '@tabler/icons-svelte/icons/clock';
	import IconUsers from '@tabler/icons-svelte/icons/users';
	import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
	import IconDeviceDesktop from '@tabler/icons-svelte/icons/device-desktop';
	import IconDeviceMobile from '@tabler/icons-svelte/icons/device-mobile';
	import IconDeviceTablet from '@tabler/icons-svelte/icons/device-tablet';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconX from '@tabler/icons-svelte/icons/x';

	interface Props {
		videoId?: number;
		onClose?: () => void;
	}

	let { videoId, onClose }: Props = $props();

	let dashboard = $state<AnalyticsDashboard | null>(null);
	let videoStats = $state<VideoAnalytics | null>(null);
	let isLoading = $state(true);
	let error = $state('');
	let selectedPeriod = $state<'7d' | '30d' | '90d'>('30d');

	onMount(() => {
		loadData();
	});

	async function loadData() {
		isLoading = true;
		error = '';

		if (videoId) {
			const result = await analyticsApi.getVideoAnalytics(videoId, selectedPeriod);
			if (result.success && result.data) {
				videoStats = result.data;
			} else {
				error = result.error || 'Failed to load analytics';
			}
		} else {
			const result = await analyticsApi.getDashboard({ period: selectedPeriod });
			if (result.success && result.data) {
				dashboard = result.data;
			} else {
				error = result.error || 'Failed to load dashboard';
			}
		}

		isLoading = false;
	}

	function changePeriod(period: '7d' | '30d' | '90d') {
		selectedPeriod = period;
		loadData();
	}

	function formatNumber(num: number): string {
		if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`;
		if (num >= 1000) return `${(num / 1000).toFixed(1)}K`;
		return num.toString();
	}

	function formatHours(hours: number): string {
		if (hours >= 24) return `${(hours / 24).toFixed(1)} days`;
		if (hours >= 1) return `${hours.toFixed(1)} hrs`;
		return `${Math.round(hours * 60)} min`;
	}

	function getDeviceIcon(device: string) {
		switch (device.toLowerCase()) {
			case 'mobile':
				return IconDeviceMobile;
			case 'tablet':
				return IconDeviceTablet;
			default:
				return IconDeviceDesktop;
		}
	}

	function getMaxViewCount(): number {
		if (videoStats?.daily_views) {
			return Math.max(...videoStats.daily_views.map((d) => d.views), 1);
		}
		if (dashboard?.views_by_day) {
			return Math.max(...dashboard.views_by_day.map((d) => d.views), 1);
		}
		return 1;
	}
</script>

<div class="analytics-dashboard">
	<div class="dashboard-header">
		<div class="header-left">
			<IconChartBar size={24} />
			<h3>{videoId ? 'Video Analytics' : 'Analytics Dashboard'}</h3>
		</div>
		<div class="header-right">
			<div class="period-selector">
				<button
					type="button"
					class="period-btn"
					class:active={selectedPeriod === '7d'}
					onclick={() => changePeriod('7d')}
				>
					7 Days
				</button>
				<button
					type="button"
					class="period-btn"
					class:active={selectedPeriod === '30d'}
					onclick={() => changePeriod('30d')}
				>
					30 Days
				</button>
				<button
					type="button"
					class="period-btn"
					class:active={selectedPeriod === '90d'}
					onclick={() => changePeriod('90d')}
				>
					90 Days
				</button>
			</div>
			<button type="button" class="btn-refresh" onclick={loadData} title="Refresh">
				<IconRefresh size={18} />
			</button>
			{#if onClose}
				<button type="button" class="btn-close" onclick={onClose}>
					<IconX size={20} />
				</button>
			{/if}
		</div>
	</div>

	{#if error}
		<div class="error-message">{error}</div>
	{/if}

	{#if isLoading}
		<div class="loading">Loading analytics...</div>
	{:else if videoStats}
		<!-- Single Video Analytics -->
		<div class="video-info">
			<h4>{videoStats.title}</h4>
			{#if videoStats.thumbnail_url}
				<img src={videoStats.thumbnail_url} alt={videoStats.title} class="video-thumb" />
			{/if}
		</div>

		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon"><IconEye size={24} /></div>
				<div class="stat-value">{formatNumber(videoStats.total_views)}</div>
				<div class="stat-label">Total Views</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon"><IconUsers size={24} /></div>
				<div class="stat-value">{formatNumber(videoStats.unique_viewers)}</div>
				<div class="stat-label">Unique Viewers</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon"><IconClock size={24} /></div>
				<div class="stat-value">{formatHours(videoStats.total_watch_time_hours)}</div>
				<div class="stat-label">Watch Time</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon"><IconTrendingUp size={24} /></div>
				<div class="stat-value">{Math.round(videoStats.avg_completion_percent)}%</div>
				<div class="stat-label">Avg Completion</div>
			</div>
		</div>

		<!-- Views Chart -->
		<div class="chart-section">
			<h4>Daily Views</h4>
			<div class="chart-container">
				<div class="bar-chart">
					{#each videoStats.daily_views as day (day.date)}
						<div
							class="bar-wrapper"
							title="{day.date}: {day.views} views"
						>
							<div
								class="bar"
								style="height: {(day.views / getMaxViewCount()) * 100}%"
							></div>
							<div class="bar-label">{day.date.slice(5)}</div>
						</div>
					{/each}
				</div>
			</div>
		</div>
	{:else if dashboard}
		<!-- Dashboard Overview -->
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon"><IconEye size={24} /></div>
				<div class="stat-value">{formatNumber(dashboard.total_views)}</div>
				<div class="stat-label">Total Views</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon"><IconUsers size={24} /></div>
				<div class="stat-value">{formatNumber(dashboard.unique_viewers)}</div>
				<div class="stat-label">Unique Viewers</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon"><IconClock size={24} /></div>
				<div class="stat-value">{formatHours(dashboard.total_watch_time_hours)}</div>
				<div class="stat-label">Total Watch Time</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon"><IconTrendingUp size={24} /></div>
				<div class="stat-value">{Math.round(dashboard.avg_completion_rate)}%</div>
				<div class="stat-label">Avg Completion</div>
			</div>
		</div>

		<!-- Views Chart -->
		<div class="chart-section">
			<h4>Views Over Time</h4>
			<div class="chart-container">
				<div class="bar-chart">
					{#each dashboard.views_by_day as day (day.date)}
						<div
							class="bar-wrapper"
							title="{day.date}: {day.views} views"
						>
							<div
								class="bar"
								style="height: {(day.views / getMaxViewCount()) * 100}%"
							></div>
							<div class="bar-label">{day.date.slice(5)}</div>
						</div>
					{/each}
				</div>
			</div>
		</div>

		<!-- Top Videos -->
		{#if dashboard.top_videos && dashboard.top_videos.length > 0}
			<div class="top-videos-section">
				<h4>Top Videos</h4>
				<div class="top-videos-list">
					{#each dashboard.top_videos as video, index (video.video_id)}
						<div class="top-video-item">
							<div class="rank">#{index + 1}</div>
							{#if video.thumbnail_url}
								<img src={video.thumbnail_url} alt={video.title} class="top-video-thumb" />
							{/if}
							<div class="top-video-info">
								<div class="top-video-title">{video.title}</div>
								<div class="top-video-views">{formatNumber(video.views)} views</div>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Device Breakdown -->
		{#if dashboard.device_breakdown && Object.keys(dashboard.device_breakdown).length > 0}
			<div class="device-section">
				<h4>Devices</h4>
				<div class="device-grid">
					{#each Object.entries(dashboard.device_breakdown) as [device, count]}
						{@const DeviceIcon = getDeviceIcon(device)}
						{@const total = Object.values(dashboard.device_breakdown).reduce((a, b) => a + b, 0)}
						<div class="device-item">
							<DeviceIcon size={24} />
							<div class="device-info">
								<div class="device-name">{device}</div>
								<div class="device-percent">{Math.round((count / total) * 100)}%</div>
							</div>
							<div class="device-bar">
								<div
									class="device-bar-fill"
									style="width: {(count / total) * 100}%"
								></div>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}
	{/if}
</div>

<style>
	.analytics-dashboard {
		background: var(--bg-secondary, #1a1a2e);
		border-radius: 12px;
		padding: 1.5rem;
		max-width: 800px;
		width: 100%;
	}

	.dashboard-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.header-left h3 {
		margin: 0;
		font-size: 1.25rem;
	}

	.header-right {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.period-selector {
		display: flex;
		background: var(--bg-tertiary, #252542);
		border-radius: 8px;
		padding: 0.25rem;
	}

	.period-btn {
		padding: 0.5rem 0.75rem;
		background: none;
		border: none;
		color: var(--text-secondary);
		font-size: 0.875rem;
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.period-btn:hover {
		color: var(--text-primary, white);
	}

	.period-btn.active {
		background: var(--primary, #6366f1);
		color: white;
	}

	.btn-refresh,
	.btn-close {
		background: none;
		border: none;
		color: var(--text-secondary);
		cursor: pointer;
		padding: 0.375rem;
		border-radius: 6px;
	}

	.btn-refresh:hover,
	.btn-close:hover {
		background: var(--bg-hover, #ffffff1a);
		color: var(--text-primary, white);
	}

	.error-message {
		background: #ef44441a;
		border: 1px solid #ef4444;
		color: #ef4444;
		padding: 0.75rem;
		border-radius: 8px;
		margin-bottom: 1rem;
	}

	.loading {
		text-align: center;
		padding: 3rem;
		color: var(--text-secondary);
	}

	.video-info {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-bottom: 1.5rem;
		padding: 1rem;
		background: var(--bg-tertiary, #252542);
		border-radius: 8px;
	}

	.video-info h4 {
		margin: 0;
		flex: 1;
	}

	.video-thumb {
		width: 120px;
		height: 68px;
		object-fit: cover;
		border-radius: 6px;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.stat-card {
		background: var(--bg-tertiary, #252542);
		padding: 1.25rem;
		border-radius: 12px;
		text-align: center;
	}

	.stat-icon {
		color: var(--primary, #6366f1);
		margin-bottom: 0.5rem;
	}

	.stat-value {
		font-size: 1.75rem;
		font-weight: 700;
		margin-bottom: 0.25rem;
	}

	.stat-label {
		font-size: 0.75rem;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.chart-section,
	.top-videos-section,
	.device-section {
		background: var(--bg-tertiary, #252542);
		padding: 1.25rem;
		border-radius: 12px;
		margin-bottom: 1rem;
	}

	.chart-section h4,
	.top-videos-section h4,
	.device-section h4 {
		margin: 0 0 1rem;
		font-size: 0.875rem;
		color: var(--text-secondary);
	}

	.chart-container {
		height: 200px;
		overflow-x: auto;
	}

	.bar-chart {
		display: flex;
		align-items: flex-end;
		gap: 4px;
		height: 100%;
		min-width: max-content;
	}

	.bar-wrapper {
		display: flex;
		flex-direction: column;
		align-items: center;
		width: 30px;
		height: 100%;
	}

	.bar {
		width: 20px;
		background: var(--primary, #6366f1);
		border-radius: 4px 4px 0 0;
		min-height: 2px;
		transition: height 0.3s ease;
	}

	.bar-label {
		font-size: 0.625rem;
		color: var(--text-secondary);
		margin-top: 0.25rem;
		white-space: nowrap;
	}

	.top-videos-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.top-video-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.rank {
		min-width: 24px;
		font-weight: 600;
		color: var(--text-secondary);
	}

	.top-video-thumb {
		width: 60px;
		height: 34px;
		object-fit: cover;
		border-radius: 4px;
	}

	.top-video-info {
		flex: 1;
		min-width: 0;
	}

	.top-video-title {
		font-size: 0.875rem;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.top-video-views {
		font-size: 0.75rem;
		color: var(--text-secondary);
	}

	.device-grid {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.device-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.device-info {
		display: flex;
		justify-content: space-between;
		width: 100px;
	}

	.device-name {
		font-size: 0.875rem;
		text-transform: capitalize;
	}

	.device-percent {
		font-size: 0.875rem;
		font-weight: 600;
	}

	.device-bar {
		flex: 1;
		height: 8px;
		background: var(--bg-primary, #0f0f1a);
		border-radius: 4px;
		overflow: hidden;
	}

	.device-bar-fill {
		height: 100%;
		background: var(--primary, #6366f1);
		border-radius: 4px;
	}

	@media (max-width: 640px) {
		.dashboard-header {
			flex-direction: column;
			align-items: flex-start;
		}

		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>

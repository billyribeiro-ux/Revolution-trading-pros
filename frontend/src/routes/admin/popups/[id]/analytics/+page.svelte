<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { Card, Button, Badge } from '#lib/components/ui/index.js';
	import { addToast } from '#lib/utils/toast.js';
	import { popupsApi, type Popup } from '#lib/api/popups.js';
	import {
		IconArrowLeft,
		IconTrendingUp,
		IconEye,
		IconClick,
		IconDevices,
		IconCalendar
	} from '#lib/icons/index.js';
	import IconTrendingDown from '@tabler/icons-svelte-runes/icons/trending-down';
	import IconArrowRight from '@tabler/icons-svelte-runes/icons/arrow-right';
	import IconDeviceDesktop from '@tabler/icons-svelte-runes/icons/device-desktop';
	import IconDeviceTablet from '@tabler/icons-svelte-runes/icons/device-tablet';
	import IconDeviceMobile from '@tabler/icons-svelte-runes/icons/device-mobile';
	import IconMedal from '@tabler/icons-svelte-runes/icons/medal';
	import IconBulb from '@tabler/icons-svelte-runes/icons/bulb';

	interface MetricTrend {
		total: number;
		today: number;
		this_week: number;
		this_month: number;
		trend: 'up' | 'down' | 'stable';
		trend_percentage: number;
	}

	interface ConversionRate {
		overall: number;
		today: number;
		this_week: number;
		this_month: number;
	}

	interface DeviceBreakdown {
		desktop: number;
		tablet: number;
		mobile: number;
	}

	interface TopPageData {
		url: string;
		views: number;
		conversions: number;
		conversion_rate: number;
	}

	interface TimelinePoint {
		date: string;
		count: number;
	}

	interface PopupAnalyticsView {
		views: MetricTrend;
		conversions: MetricTrend;
		conversion_rate: ConversionRate;
		device_breakdown: DeviceBreakdown;
		top_pages?: TopPageData[];
		timeline: {
			views?: TimelinePoint[];
			conversions?: TimelinePoint[];
		};
	}

	const popupId = parseInt(page.params.id ?? '');

	let popup = $state<Popup | null>(null);
	let analytics = $state<PopupAnalyticsView | null>(null);
	let loading = $state(true);

	onMount(async () => {
		await loadAnalytics();
	});

	async function loadAnalytics() {
		try {
			loading = true;

			// Load popup details
			const popupResponse = await popupsApi.get(popupId);
			popup = popupResponse.popup ?? null;

			// Load analytics data - getAnalytics returns the analytics view shape at runtime
			analytics = (await popupsApi.getAnalytics(popupId)) as unknown as PopupAnalyticsView;
		} catch (error) {
			console.error('Failed to load analytics:', error);
			addToast({ type: 'error', message: 'Failed to load analytics' });
		} finally {
			loading = false;
		}
	}

	function getStatusColor(status: string): 'success' | 'warning' | 'info' | 'default' {
		switch (status) {
			case 'published':
				return 'success';
			case 'draft':
				return 'warning';
			case 'paused':
				return 'info';
			default:
				return 'default';
		}
	}

	function getTrendIcon(trend: 'up' | 'down' | 'stable') {
		if (trend === 'up') return IconTrendingUp;
		if (trend === 'down') return IconTrendingDown;
		return IconArrowRight;
	}

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	function formatPercent(num: number): string {
		return num.toFixed(2) + '%';
	}

	function truncateUrl(url: string, maxLength: number = 50): string {
		if (url.length <= maxLength) return url;
		return url.substring(0, maxLength) + '...';
	}

	function getTrendClass(trend: 'up' | 'down' | 'stable'): Record<string, boolean> {
		return {
			'trend-value': true,
			'trend-value--up': trend === 'up',
			'trend-value--down': trend === 'down',
			'trend-value--stable': trend === 'stable'
		};
	}

	function getPerformanceClass(status: string | undefined): Record<string, boolean> {
		return {
			'performance-value': true,
			'performance-value--excellent': status === 'excellent',
			'performance-value--good': status === 'good',
			'performance-value--average': status === 'average',
			'performance-value--poor': status !== 'excellent' && status !== 'good' && status !== 'average'
		};
	}

	function getSharePercent(count: number, total: number): number {
		return total > 0 ? (count / total) * 100 : 0;
	}

	function getTimelineWidth(count: number, points: TimelinePoint[] | undefined): number {
		const max = Math.max(...(points?.map((point) => point.count) ?? []), 0);
		return max > 0 && count > 0 ? Math.max((count / max) * 100, 5) : 0;
	}
</script>

<svelte:head>
	<title>Analytics - {popup?.name || 'Popup'} | Revolution Admin</title>
</svelte:head>

{#if loading}
	<div class="loading-state">
		<div class="loading-panel">
			<div class="loading-spinner"></div>
			<p class="loading-copy">Loading analytics...</p>
		</div>
	</div>
{:else if popup && analytics}
	{@const ViewsTrendIcon = getTrendIcon(analytics.views.trend)}
	{@const ConversionsTrendIcon = getTrendIcon(analytics.conversions.trend)}
	<div class="analytics-page">
		<!-- Header -->
		<div class="page-header">
			<Button variant="ghost" onclick={() => goto('/admin/popups')} class="back-button">
				<span class="button-icon"><IconArrowLeft size={20} aria-hidden="true" /></span>
				Back to Popups
			</Button>

			<div class="header-row">
				<div>
					<h1 class="page-title">{popup.name}</h1>
					<div class="popup-meta">
						<Badge variant={getStatusColor(popup.status ?? 'draft')}>
							{popup.status}
						</Badge>
						<span class="popup-type">
							{(popup.type ?? 'popup').replace('_', ' ').replace(/\b\w/g, (l) => l.toUpperCase())}
						</span>
					</div>
				</div>

				<div class="header-actions">
					<Button variant="outline" onclick={() => goto(`/admin/popups/${popupId}/edit`)}>
						Edit Popup
					</Button>
				</div>
			</div>
		</div>

		<!-- Key Metrics -->
		<div class="metric-grid">
			<!-- Total Views -->
			<Card>
				<div class="metric-summary">
					<div>
						<p class="metric-label">Total Views</p>
						<p class="metric-value">{formatNumber(analytics.views.total)}</p>
						<div class="trend-row">
							<span class="trend-icon"><ViewsTrendIcon size={16} aria-hidden="true" /></span>
							<span class={getTrendClass(analytics.views.trend)}>
								{formatPercent(analytics.views.trend_percentage)}
							</span>
							<span class="trend-context">vs last period</span>
						</div>
					</div>
					<div class="metric-icon metric-icon--blue">
						<IconEye size={24} aria-hidden="true" />
					</div>
				</div>
			</Card>

			<!-- Total Conversions -->
			<Card>
				<div class="metric-summary">
					<div>
						<p class="metric-label">Conversions</p>
						<p class="metric-value">{formatNumber(analytics.conversions.total)}</p>
						<div class="trend-row">
							<span class="trend-icon"><ConversionsTrendIcon size={16} aria-hidden="true" /></span>
							<span class={getTrendClass(analytics.conversions.trend)}>
								{formatPercent(analytics.conversions.trend_percentage)}
							</span>
							<span class="trend-context">vs last period</span>
						</div>
					</div>
					<div class="metric-icon metric-icon--green">
						<IconClick size={24} aria-hidden="true" />
					</div>
				</div>
			</Card>

			<!-- Conversion Rate -->
			<Card>
				<div class="metric-summary">
					<div>
						<p class="metric-label">Conversion Rate</p>
						<p class="metric-value">{formatPercent(analytics.conversion_rate.overall)}</p>
						<div class="metric-context">
							<span>Industry avg: 2-5%</span>
						</div>
					</div>
					<div class="metric-icon metric-icon--purple">
						<IconTrendingUp size={24} aria-hidden="true" />
					</div>
				</div>
			</Card>

			<!-- Performance Status -->
			<Card>
				<div class="metric-summary">
					<div>
						<p class="metric-label">Performance</p>
						<p class={getPerformanceClass(popup.performance_status)}>
							{popup.performance_status || 'N/A'}
						</p>
						<div class="metric-context">
							<span>Based on conversion rate</span>
						</div>
					</div>
					<div class="metric-icon metric-icon--yellow">
						<IconCalendar size={24} aria-hidden="true" />
					</div>
				</div>
			</Card>
		</div>

		<!-- Period Breakdown -->
		<Card class="section-card">
			<h2 class="section-heading">Performance Over Time</h2>

			<div class="period-grid">
				<!-- Today -->
				<div class="period-card period-card--blue">
					<h3>Today</h3>
					<div class="period-stack">
						<p>
							<span class="inline-strong">{formatNumber(analytics.views.today)}</span> views
						</p>
						<p>
							<span class="inline-strong inline-strong--green"
								>{formatNumber(analytics.conversions.today)}</span
							>
							conversions
						</p>
						<p>
							<span class="inline-strong inline-strong--purple"
								>{formatPercent(analytics.conversion_rate.today)}</span
							>
							rate
						</p>
					</div>
				</div>

				<!-- This Week -->
				<div class="period-card period-card--green">
					<h3>This Week</h3>
					<div class="period-stack">
						<p>
							<span class="inline-strong">{formatNumber(analytics.views.this_week)}</span> views
						</p>
						<p>
							<span class="inline-strong inline-strong--green"
								>{formatNumber(analytics.conversions.this_week)}</span
							>
							conversions
						</p>
						<p>
							<span class="inline-strong inline-strong--purple"
								>{formatPercent(analytics.conversion_rate.this_week)}</span
							>
							rate
						</p>
					</div>
				</div>

				<!-- This Month -->
				<div class="period-card period-card--purple">
					<h3>This Month</h3>
					<div class="period-stack">
						<p>
							<span class="inline-strong">{formatNumber(analytics.views.this_month)}</span> views
						</p>
						<p>
							<span class="inline-strong inline-strong--green"
								>{formatNumber(analytics.conversions.this_month)}</span
							>
							conversions
						</p>
						<p>
							<span class="inline-strong inline-strong--purple"
								>{formatPercent(analytics.conversion_rate.this_month)}</span
							>
							rate
						</p>
					</div>
				</div>
			</div>
		</Card>

		<div class="content-grid section-spacing">
			<!-- Device Breakdown -->
			<Card>
				<h2 class="section-heading section-heading--inline">
					<IconDevices size={24} aria-hidden="true" />
					Device Breakdown
				</h2>

				<div class="bar-stack">
					<!-- Desktop -->
					<div>
						<div class="bar-header">
							<span class="bar-label"><IconDeviceDesktop size={16} aria-hidden="true" />Desktop</span>
							<span class="bar-value">{formatNumber(analytics.device_breakdown.desktop)}</span>
						</div>
						<div class="progress-track progress-track--small">
							<div
								class="progress-bar progress-bar--blue"
								style:width="{getSharePercent(
									analytics.device_breakdown.desktop,
									analytics.views.total
								).toFixed(1)}%"
							></div>
						</div>
						<p class="bar-note">
							{getSharePercent(analytics.device_breakdown.desktop, analytics.views.total).toFixed(
								1
							)}% of total views
						</p>
					</div>

					<!-- Tablet -->
					<div>
						<div class="bar-header">
							<span class="bar-label"><IconDeviceTablet size={16} aria-hidden="true" />Tablet</span>
							<span class="bar-value">{formatNumber(analytics.device_breakdown.tablet)}</span>
						</div>
						<div class="progress-track progress-track--small">
							<div
								class="progress-bar progress-bar--green"
								style:width="{getSharePercent(
									analytics.device_breakdown.tablet,
									analytics.views.total
								).toFixed(1)}%"
							></div>
						</div>
						<p class="bar-note">
							{getSharePercent(analytics.device_breakdown.tablet, analytics.views.total).toFixed(
								1
							)}% of total views
						</p>
					</div>

					<!-- Mobile -->
					<div>
						<div class="bar-header">
							<span class="bar-label"><IconDeviceMobile size={16} aria-hidden="true" />Mobile</span>
							<span class="bar-value">{formatNumber(analytics.device_breakdown.mobile)}</span>
						</div>
						<div class="progress-track progress-track--small">
							<div
								class="progress-bar progress-bar--purple"
								style:width="{getSharePercent(
									analytics.device_breakdown.mobile,
									analytics.views.total
								).toFixed(1)}%"
							></div>
						</div>
						<p class="bar-note">
							{getSharePercent(analytics.device_breakdown.mobile, analytics.views.total).toFixed(
								1
							)}% of total views
						</p>
					</div>
				</div>
			</Card>

			<!-- Top Performing Pages -->
			<Card>
				<h2 class="section-heading">Top Performing Pages</h2>

				{#if analytics.top_pages && analytics.top_pages.length > 0}
					<div class="rank-stack">
						{#each analytics.top_pages as pageData, index (index)}
							<div
								class={{
									'rank-card': true,
									'rank-card--gold': index === 0,
									'rank-card--silver': index === 1,
									'rank-card--bronze': index === 2,
									'rank-card--default': index > 2
								}}
							>
								<div class="rank-header">
									<p class="rank-url">
										{#if index < 3}
											<span class="rank-medal"><IconMedal size={16} aria-hidden="true" /></span>
										{/if}
										{truncateUrl(pageData.url)}
									</p>
									<Badge variant="success" size="sm">
										{formatPercent(pageData.conversion_rate)}
									</Badge>
								</div>
								<div class="rank-meta">
									<span>{formatNumber(pageData.views)} views</span>
									<span class="rank-conversions"
										>{formatNumber(pageData.conversions)} conversions</span
									>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<p class="empty-copy">No page data available yet</p>
				{/if}
			</Card>
		</div>

		<!-- Timeline Charts -->
		<div class="content-grid">
			<!-- Views Timeline -->
			<Card>
				<h2 class="section-heading">Views Timeline (Last 30 Days)</h2>

				{#if analytics.timeline.views && analytics.timeline.views.length > 0}
					<div class="timeline-stack">
						{#each analytics.timeline.views.slice(-15) as day (day.date)}
							<div class="timeline-row">
								<span class="timeline-date">{day.date}</span>
								<div class="progress-track">
									<div
										class="progress-bar progress-bar--blue progress-bar--labeled"
										style:width="{getTimelineWidth(day.count, analytics.timeline.views)}%"
									>
										{#if day.count > 0}
											<span>{day.count}</span>
										{/if}
									</div>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<p class="empty-copy">No timeline data available yet</p>
				{/if}
			</Card>

			<!-- Conversions Timeline -->
			<Card>
				<h2 class="section-heading">Conversions Timeline (Last 30 Days)</h2>

				{#if analytics.timeline.conversions && analytics.timeline.conversions.length > 0}
					<div class="timeline-stack">
						{#each analytics.timeline.conversions.slice(-15) as day (day.date)}
							<div class="timeline-row">
								<span class="timeline-date">{day.date}</span>
								<div class="progress-track">
									<div
										class="progress-bar progress-bar--green progress-bar--labeled"
										style:width="{getTimelineWidth(day.count, analytics.timeline.conversions)}%"
									>
										{#if day.count > 0}
											<span>{day.count}</span>
										{/if}
									</div>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<p class="empty-copy">No conversion data available yet</p>
				{/if}
			</Card>
		</div>

		<!-- Insights & Recommendations -->
		<Card class="insights-card">
			<h2 class="section-heading"><IconBulb size={20} aria-hidden="true" /> Insights & Recommendations</h2>

			<div class="insight-stack">
				{#if analytics.conversion_rate.overall < 1}
					<div class="insight insight--danger">
						<p>
							<strong>Low Conversion Rate:</strong> Your conversion rate is below 1%. Consider:
						</p>
						<ul>
							<li>Testing a more compelling headline</li>
							<li>Simplifying your call-to-action</li>
							<li>Adjusting trigger timing</li>
						</ul>
					</div>
				{:else if analytics.conversion_rate.overall > 5}
					<div class="insight insight--success">
						<p>
							<strong>Excellent Performance!</strong> Your conversion rate of {formatPercent(
								analytics.conversion_rate.overall
							)} is above industry average.
						</p>
					</div>
				{/if}

				{#if analytics.device_breakdown.mobile > analytics.views.total * 0.5}
					<div class="insight insight--info">
						<p>
							<strong>Mobile-First Audience:</strong> Over 50% of your views come from mobile devices.
							Ensure your popup is mobile-optimized.
						</p>
					</div>
				{/if}

				{#if analytics.views.total < 100}
					<div class="insight insight--warning">
						<p>
							<strong>Limited Data:</strong> With fewer than 100 views, continue monitoring before making
							major changes.
						</p>
					</div>
				{/if}
			</div>
		</Card>
	</div>
{:else}
	<div class="error-state">
		<p>Failed to load analytics data</p>
		<Button class="error-action" onclick={() => goto('/admin/popups')}>Back to Popups</Button>
	</div>
{/if}

<style>
	.loading-state,
	.error-state {
		display: grid;
		min-height: 24rem;
		place-items: center;
		text-align: center;
	}

	.loading-spinner {
		width: 3rem;
		height: 3rem;
		margin: 0 auto;
		border: 2px solid rgba(88, 166, 255, 0.18);
		border-bottom-color: var(--primary-500, #e6b800);
		border-radius: 999px;
		animation: spin 0.8s linear infinite;
	}

	.loading-copy,
	.error-state p {
		margin: 1rem 0 0;
		color: var(--text-tertiary, #6e7681);
	}

	.analytics-page {
		max-width: 80rem;
		margin: 0 auto;
	}

	.page-header {
		margin-bottom: 1.5rem;
	}

	.back-button {
		margin-bottom: 1rem;
	}

	.button-icon {
		display: inline-grid;
		margin-right: 0.5rem;
		place-items: center;
	}

	.header-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
	}

	.header-actions {
		display: flex;
		gap: 0.5rem;
	}

	.page-title {
		margin: 0;
		color: var(--text-primary, #f0f6fc);
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 1.2;
	}

	.popup-meta {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 0.75rem;
		margin-top: 0.5rem;
	}

	.popup-type {
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.metric-grid {
		display: grid;
		grid-template-columns: minmax(0, 1fr);
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.metric-summary {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 1rem;
	}

	.metric-label {
		margin: 0 0 0.25rem;
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.4;
	}

	.metric-value {
		margin: 0;
		color: var(--text-primary, #f0f6fc);
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 1.2;
	}

	.performance-value {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 1.25;
		text-transform: capitalize;
	}

	.performance-value--excellent {
		color: var(--success-emphasis, #3fb950);
	}

	.performance-value--good {
		color: var(--info-emphasis, #58a6ff);
	}

	.performance-value--average {
		color: var(--warning-emphasis, #d29922);
	}

	.performance-value--poor {
		color: var(--error-emphasis, #f85149);
	}

	.trend-row {
		display: flex;
		align-items: center;
		margin-top: 0.5rem;
	}

	.trend-icon {
		display: inline-flex;
		margin-right: 0.25rem;
		line-height: 1;
	}

	.trend-value {
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.4;
	}

	.trend-value--up {
		color: var(--success-emphasis, #3fb950);
	}

	.trend-value--down {
		color: var(--error-emphasis, #f85149);
	}

	.trend-value--stable {
		color: var(--text-tertiary, #6e7681);
	}

	.trend-context,
	.metric-context {
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
		line-height: 1.4;
	}

	.trend-context {
		margin-left: 0.25rem;
	}

	.metric-context {
		margin-top: 0.5rem;
	}

	.metric-icon {
		display: grid;
		width: 3rem;
		height: 3rem;
		flex: 0 0 auto;
		border-radius: 8px;
		place-items: center;
	}

	.metric-icon--blue {
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
		color: var(--info-emphasis, #58a6ff);
	}

	.metric-icon--green {
		background: var(--success-soft, rgba(46, 160, 67, 0.15));
		color: var(--success-emphasis, #3fb950);
	}

	.metric-icon--purple {
		background: var(--bg-elevated, #161b22);
		color: #9333ea;
	}

	.metric-icon--yellow {
		background: var(--warning-soft, rgba(187, 128, 9, 0.15));
		color: var(--warning-emphasis, #d29922);
	}

	.section-card,
	.section-spacing {
		margin-bottom: 2rem;
	}

	.section-heading {
		margin: 0 0 1rem;
		color: var(--text-primary, #f0f6fc);
		font-size: 1.25rem;
		font-weight: 700;
		line-height: 1.3;
	}

	.section-heading--inline {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.period-grid,
	.content-grid {
		display: grid;
		grid-template-columns: minmax(0, 1fr);
		gap: 1.5rem;
	}

	.period-card {
		border-left: 4px solid var(--primary-500, #e6b800);
		padding-left: 1rem;
	}

	.period-card--green {
		border-left-color: var(--success-base, #2ea043);
	}

	.period-card--purple {
		border-left-color: var(--primary-500, #e6b800);
	}

	.period-card h3 {
		margin: 0 0 0.5rem;
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.4;
	}

	.period-stack {
		display: grid;
		gap: 0.25rem;
	}

	.period-stack p {
		margin: 0;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.inline-strong {
		font-weight: 700;
	}

	.inline-strong--green,
	.rank-conversions {
		color: var(--success-emphasis, #3fb950);
	}

	.inline-strong--purple {
		color: #9333ea;
	}

	.bar-stack,
	.rank-stack,
	.insight-stack {
		display: grid;
		gap: 1rem;
	}

	.rank-stack,
	.insight-stack {
		gap: 0.75rem;
	}

	.bar-header,
	.rank-header,
	.timeline-row {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.bar-header,
	.rank-header {
		justify-content: space-between;
		margin-bottom: 0.25rem;
	}

	.bar-label {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.4;
	}

	.bar-value {
		color: var(--text-primary, #f0f6fc);
		font-size: 0.875rem;
		font-weight: 700;
		line-height: 1.4;
	}

	.bar-note,
	.empty-copy {
		margin: 0.25rem 0 0;
		color: var(--text-tertiary, #6e7681);
		font-size: 0.75rem;
		line-height: 1.5;
	}

	.empty-copy {
		font-size: 0.875rem;
	}

	.progress-track {
		flex: 1 1 auto;
		height: 1rem;
		min-width: 0;
		overflow: hidden;
		border-radius: 999px;
		background: var(--bg-surface, #1c2128);
	}

	.progress-track--small {
		height: 0.5rem;
	}

	.progress-bar {
		height: 100%;
		border-radius: inherit;
		transition: width 0.2s ease;
	}

	.progress-bar--blue {
		background: var(--primary-500, #e6b800);
	}

	.progress-bar--green {
		background: var(--success-base, #2ea043);
	}

	.progress-bar--purple {
		background: #9333ea;
	}

	.progress-bar--labeled {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		padding-right: 0.5rem;
	}

	.progress-bar--labeled span {
		color: #ffffff;
		font-size: 0.75rem;
		font-weight: 700;
		line-height: 1;
	}

	.rank-card {
		border-left: 4px solid var(--border-default, #30363d);
		padding-left: 0.75rem;
	}

	.rank-card--gold {
		border-left-color: var(--warning-emphasis, #d29922);
	}

	.rank-card--silver {
		border-left-color: #9ca3af;
	}

	.rank-card--bronze {
		border-left-color: var(--primary-500, #e6b800);
	}

	.rank-url {
		flex: 1 1 auto;
		min-width: 0;
		margin: 0;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.5;
	}

	.rank-medal {
		display: inline-flex;
		vertical-align: middle;
		margin-right: 0.25rem;
	}

	.rank-meta {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem 1rem;
		color: var(--text-tertiary, #6e7681);
		font-size: 0.75rem;
		line-height: 1.5;
	}

	.timeline-stack {
		display: grid;
		gap: 0.5rem;
	}

	.timeline-date {
		width: 5rem;
		flex: 0 0 auto;
		color: var(--text-tertiary, #6e7681);
		font-size: 0.75rem;
		line-height: 1.4;
	}

	.insights-card {
		margin-top: 2rem;
	}

	.insight {
		border: 1px solid var(--border-default, #30363d);
		border-radius: 6px;
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
		padding: 0.75rem;
	}

	.insight p {
		margin: 0;
		color: var(--primary-400, #ffd11a);
		font-size: 0.875rem;
		line-height: 1.6;
	}

	.insight ul {
		margin: 0.25rem 0 0 1.25rem;
		padding: 0;
		color: var(--primary-400, #ffd11a);
		font-size: 0.875rem;
		line-height: 1.6;
	}

	.insight--danger {
		border-color: var(--border-default, #30363d);
		background: var(--bg-elevated, #161b22);
	}

	.insight--danger p,
	.insight--danger ul {
		color: var(--error-emphasis, #f85149);
	}

	.insight--success {
		border-color: var(--border-default, #30363d);
		background: var(--success-soft, rgba(46, 160, 67, 0.15));
	}

	.insight--success p {
		color: var(--success-emphasis, #3fb950);
	}

	.insight--warning {
		border-color: var(--border-default, #30363d);
		background: var(--warning-soft, rgba(187, 128, 9, 0.15));
	}

	.insight--warning p {
		color: var(--warning-emphasis, #d29922);
	}

	.error-action {
		margin-top: 1rem;
	}

	@media (min-width: 768px) {
		.metric-grid {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}

		.period-grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}

	@media (min-width: 1024px) {
		.content-grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	@media (max-width: 767.98px) {
		.header-row {
			align-items: flex-start;
			flex-direction: column;
		}

		.header-actions {
			width: 100%;
		}

		.metric-summary {
			align-items: center;
		}
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>

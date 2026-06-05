<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { Card, Button, Badge } from '$lib/components/ui';
	import { addToast } from '$lib/utils/toast';
	import { popupsApi, type Popup } from '$lib/api/popups';
	import {
		IconArrowLeft,
		IconTrendingUp,
		IconEye,
		IconClick,
		IconDevices,
		IconCalendar
	} from '$lib/icons';

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
		if (trend === 'up') return '📈';
		if (trend === 'down') return '📉';
		return '➡️';
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
							<span class="trend-icon">{getTrendIcon(analytics.views.trend)}</span>
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
							<span class="trend-icon">{getTrendIcon(analytics.conversions.trend)}</span>
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
							<span class="bar-label">🖥️ Desktop</span>
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
							<span class="bar-label">📱 Tablet</span>
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
							<span class="bar-label">📱 Mobile</span>
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
											<span class="rank-medal"
												>{index === 0 ? '🥇' : index === 1 ? '🥈' : '🥉'}</span
											>
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
			<h2 class="section-heading">💡 Insights & Recommendations</h2>

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
		border: 2px solid rgba(37, 99, 235, 0.18);
		border-bottom-color: #2563eb;
		border-radius: 999px;
		animation: spin 0.8s linear infinite;
	}

	.loading-copy,
	.error-state p {
		margin: 1rem 0 0;
		color: #4b5563;
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
		color: #111827;
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
		color: #4b5563;
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
		color: #4b5563;
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.4;
	}

	.metric-value {
		margin: 0;
		color: #111827;
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
		color: #16a34a;
	}

	.performance-value--good {
		color: #2563eb;
	}

	.performance-value--average {
		color: #ca8a04;
	}

	.performance-value--poor {
		color: #dc2626;
	}

	.trend-row {
		display: flex;
		align-items: center;
		margin-top: 0.5rem;
	}

	.trend-icon {
		margin-right: 0.25rem;
		font-size: 0.875rem;
		line-height: 1;
	}

	.trend-value {
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.4;
	}

	.trend-value--up {
		color: #16a34a;
	}

	.trend-value--down {
		color: #dc2626;
	}

	.trend-value--stable {
		color: #4b5563;
	}

	.trend-context,
	.metric-context {
		color: #6b7280;
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
		background: #eff6ff;
		color: #2563eb;
	}

	.metric-icon--green {
		background: #f0fdf4;
		color: #16a34a;
	}

	.metric-icon--purple {
		background: #faf5ff;
		color: #9333ea;
	}

	.metric-icon--yellow {
		background: #fefce8;
		color: #ca8a04;
	}

	.section-card,
	.section-spacing {
		margin-bottom: 2rem;
	}

	.section-heading {
		margin: 0 0 1rem;
		color: #111827;
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
		border-left: 4px solid #2563eb;
		padding-left: 1rem;
	}

	.period-card--green {
		border-left-color: #22c55e;
	}

	.period-card--purple {
		border-left-color: #a855f7;
	}

	.period-card h3 {
		margin: 0 0 0.5rem;
		color: #4b5563;
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
		color: #111827;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.inline-strong {
		font-weight: 700;
	}

	.inline-strong--green,
	.rank-conversions {
		color: #16a34a;
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
		color: #374151;
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.4;
	}

	.bar-value {
		color: #111827;
		font-size: 0.875rem;
		font-weight: 700;
		line-height: 1.4;
	}

	.bar-note,
	.empty-copy {
		margin: 0.25rem 0 0;
		color: #6b7280;
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
		background: #e5e7eb;
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
		background: #2563eb;
	}

	.progress-bar--green {
		background: #16a34a;
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
		border-left: 4px solid #d1d5db;
		padding-left: 0.75rem;
	}

	.rank-card--gold {
		border-left-color: #eab308;
	}

	.rank-card--silver {
		border-left-color: #9ca3af;
	}

	.rank-card--bronze {
		border-left-color: #ea580c;
	}

	.rank-url {
		flex: 1 1 auto;
		min-width: 0;
		margin: 0;
		color: #111827;
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.5;
	}

	.rank-medal {
		margin-right: 0.25rem;
	}

	.rank-meta {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem 1rem;
		color: #4b5563;
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
		color: #4b5563;
		font-size: 0.75rem;
		line-height: 1.4;
	}

	.insights-card {
		margin-top: 2rem;
	}

	.insight {
		border: 1px solid #bfdbfe;
		border-radius: 6px;
		background: #eff6ff;
		padding: 0.75rem;
	}

	.insight p {
		margin: 0;
		color: #1e40af;
		font-size: 0.875rem;
		line-height: 1.6;
	}

	.insight ul {
		margin: 0.25rem 0 0 1.25rem;
		padding: 0;
		color: #1d4ed8;
		font-size: 0.875rem;
		line-height: 1.6;
	}

	.insight--danger {
		border-color: #fecaca;
		background: #fef2f2;
	}

	.insight--danger p,
	.insight--danger ul {
		color: #991b1b;
	}

	.insight--success {
		border-color: #bbf7d0;
		background: #f0fdf4;
	}

	.insight--success p {
		color: #166534;
	}

	.insight--warning {
		border-color: #fde68a;
		background: #fefce8;
	}

	.insight--warning p {
		color: #854d0e;
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

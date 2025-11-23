<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
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
	} from '@tabler/icons-svelte';

	const popupId = parseInt($page.params.id);

	let popup: Popup | null = null;
	let analytics: any = null;
	let loading = true;

	interface AnalyticsData {
		views: {
			total: number;
			today: number;
			this_week: number;
			this_month: number;
			trend: 'up' | 'down' | 'stable';
			trend_percentage: number;
		};
		conversions: {
			total: number;
			today: number;
			this_week: number;
			this_month: number;
			trend: 'up' | 'down' | 'stable';
			trend_percentage: number;
		};
		conversion_rate: {
			overall: number;
			today: number;
			this_week: number;
			this_month: number;
		};
		device_breakdown: {
			desktop: number;
			tablet: number;
			mobile: number;
		};
		top_pages: Array<{
			url: string;
			views: number;
			conversions: number;
			conversion_rate: number;
		}>;
		timeline: {
			views: Array<{ date: string; count: number }>;
			conversions: Array<{ date: string; count: number }>;
		};
	}

	onMount(async () => {
		await loadAnalytics();
	});

	async function loadAnalytics() {
		try {
			loading = true;

			// Load popup details
			const popupResponse = await popupsApi.get(popupId);
			popup = popupResponse.popup;

			// Load analytics data
			const analyticsResponse = await popupsApi.getAnalytics(popupId);
			analytics = analyticsResponse.analytics;
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
</script>

<svelte:head>
	<title>Analytics - {popup?.name || 'Popup'} | Revolution Admin</title>
</svelte:head>

{#if loading}
	<div class="flex items-center justify-center h-96">
		<div class="text-center">
			<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
			<p class="mt-4 text-gray-600">Loading analytics...</p>
		</div>
	</div>
{:else if popup && analytics}
	<div class="max-w-7xl mx-auto">
		<!-- Header -->
		<div class="mb-6">
			<Button variant="ghost" on:click={() => goto('/admin/popups')} class="mb-4">
				<IconArrowLeft size={20} class="mr-2" />
				Back to Popups
			</Button>

			<div class="flex items-center justify-between">
				<div>
					<h1 class="text-3xl font-bold text-gray-900">{popup.name}</h1>
					<div class="flex items-center gap-3 mt-2">
						<Badge variant={getStatusColor(popup.status)}>
							{popup.status}
						</Badge>
						<span class="text-sm text-gray-600">
							{popup.type.replace('_', ' ').replace(/\b\w/g, (l) => l.toUpperCase())}
						</span>
					</div>
				</div>

				<div class="flex gap-2">
					<Button variant="outline" on:click={() => goto(`/admin/popups/${popupId}/edit`)}>
						Edit Popup
					</Button>
				</div>
			</div>
		</div>

		<!-- Key Metrics -->
		<div class="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
			<!-- Total Views -->
			<Card>
				<div class="flex items-start justify-between">
					<div>
						<p class="text-sm font-medium text-gray-600 mb-1">Total Views</p>
						<p class="text-3xl font-bold text-gray-900">{formatNumber(analytics.views.total)}</p>
						<div class="flex items-center mt-2">
							<span class="text-sm mr-1">{getTrendIcon(analytics.views.trend)}</span>
							<span
								class="text-sm font-medium {analytics.views.trend === 'up'
									? 'text-green-600'
									: analytics.views.trend === 'down'
										? 'text-red-600'
										: 'text-gray-600'}"
							>
								{formatPercent(analytics.views.trend_percentage)}
							</span>
							<span class="text-sm text-gray-500 ml-1">vs last period</span>
						</div>
					</div>
					<div class="p-3 bg-blue-50 rounded-lg">
						<IconEye size={24} class="text-blue-600" />
					</div>
				</div>
			</Card>

			<!-- Total Conversions -->
			<Card>
				<div class="flex items-start justify-between">
					<div>
						<p class="text-sm font-medium text-gray-600 mb-1">Conversions</p>
						<p class="text-3xl font-bold text-gray-900">
							{formatNumber(analytics.conversions.total)}
						</p>
						<div class="flex items-center mt-2">
							<span class="text-sm mr-1">{getTrendIcon(analytics.conversions.trend)}</span>
							<span
								class="text-sm font-medium {analytics.conversions.trend === 'up'
									? 'text-green-600'
									: analytics.conversions.trend === 'down'
										? 'text-red-600'
										: 'text-gray-600'}"
							>
								{formatPercent(analytics.conversions.trend_percentage)}
							</span>
							<span class="text-sm text-gray-500 ml-1">vs last period</span>
						</div>
					</div>
					<div class="p-3 bg-green-50 rounded-lg">
						<IconClick size={24} class="text-green-600" />
					</div>
				</div>
			</Card>

			<!-- Conversion Rate -->
			<Card>
				<div class="flex items-start justify-between">
					<div>
						<p class="text-sm font-medium text-gray-600 mb-1">Conversion Rate</p>
						<p class="text-3xl font-bold text-gray-900">
							{formatPercent(analytics.conversion_rate.overall)}
						</p>
						<div class="mt-2">
							<span class="text-sm text-gray-500"> Industry avg: 2-5% </span>
						</div>
					</div>
					<div class="p-3 bg-purple-50 rounded-lg">
						<IconTrendingUp size={24} class="text-purple-600" />
					</div>
				</div>
			</Card>

			<!-- Performance Status -->
			<Card>
				<div class="flex items-start justify-between">
					<div>
						<p class="text-sm font-medium text-gray-600 mb-1">Performance</p>
						<p
							class="text-2xl font-bold capitalize {popup.performance_status === 'excellent'
								? 'text-green-600'
								: popup.performance_status === 'good'
									? 'text-blue-600'
									: popup.performance_status === 'average'
										? 'text-yellow-600'
										: 'text-red-600'}"
						>
							{popup.performance_status || 'N/A'}
						</p>
						<div class="mt-2">
							<span class="text-sm text-gray-500"> Based on conversion rate </span>
						</div>
					</div>
					<div class="p-3 bg-yellow-50 rounded-lg">
						<IconCalendar size={24} class="text-yellow-600" />
					</div>
				</div>
			</Card>
		</div>

		<!-- Period Breakdown -->
		<Card class="mb-8">
			<h2 class="text-xl font-semibold mb-4">Performance Over Time</h2>

			<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
				<!-- Today -->
				<div class="border-l-4 border-blue-500 pl-4">
					<h3 class="text-sm font-medium text-gray-600 mb-2">Today</h3>
					<div class="space-y-1">
						<p class="text-sm">
							<span class="font-semibold">{formatNumber(analytics.views.today)}</span> views
						</p>
						<p class="text-sm">
							<span class="font-semibold text-green-600"
								>{formatNumber(analytics.conversions.today)}</span
							> conversions
						</p>
						<p class="text-sm">
							<span class="font-semibold text-purple-600"
								>{formatPercent(analytics.conversion_rate.today)}</span
							> rate
						</p>
					</div>
				</div>

				<!-- This Week -->
				<div class="border-l-4 border-green-500 pl-4">
					<h3 class="text-sm font-medium text-gray-600 mb-2">This Week</h3>
					<div class="space-y-1">
						<p class="text-sm">
							<span class="font-semibold">{formatNumber(analytics.views.this_week)}</span> views
						</p>
						<p class="text-sm">
							<span class="font-semibold text-green-600"
								>{formatNumber(analytics.conversions.this_week)}</span
							> conversions
						</p>
						<p class="text-sm">
							<span class="font-semibold text-purple-600"
								>{formatPercent(analytics.conversion_rate.this_week)}</span
							> rate
						</p>
					</div>
				</div>

				<!-- This Month -->
				<div class="border-l-4 border-purple-500 pl-4">
					<h3 class="text-sm font-medium text-gray-600 mb-2">This Month</h3>
					<div class="space-y-1">
						<p class="text-sm">
							<span class="font-semibold">{formatNumber(analytics.views.this_month)}</span> views
						</p>
						<p class="text-sm">
							<span class="font-semibold text-green-600"
								>{formatNumber(analytics.conversions.this_month)}</span
							> conversions
						</p>
						<p class="text-sm">
							<span class="font-semibold text-purple-600"
								>{formatPercent(analytics.conversion_rate.this_month)}</span
							> rate
						</p>
					</div>
				</div>
			</div>
		</Card>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
			<!-- Device Breakdown -->
			<Card>
				<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
					<IconDevices size={24} />
					Device Breakdown
				</h2>

				<div class="space-y-4">
					<!-- Desktop -->
					<div>
						<div class="flex justify-between items-center mb-1">
							<span class="text-sm font-medium text-gray-700">🖥️ Desktop</span>
							<span class="text-sm font-semibold text-gray-900"
								>{formatNumber(analytics.device_breakdown.desktop)}</span
							>
						</div>
						<div class="w-full bg-gray-200 rounded-full h-2">
							<div
								class="bg-blue-600 h-2 rounded-full"
								style="width: {(
									(analytics.device_breakdown.desktop / analytics.views.total) *
									100
								).toFixed(1)}%"
							></div>
						</div>
						<p class="text-xs text-gray-500 mt-1">
							{((analytics.device_breakdown.desktop / analytics.views.total) * 100).toFixed(1)}% of
							total views
						</p>
					</div>

					<!-- Tablet -->
					<div>
						<div class="flex justify-between items-center mb-1">
							<span class="text-sm font-medium text-gray-700">📱 Tablet</span>
							<span class="text-sm font-semibold text-gray-900"
								>{formatNumber(analytics.device_breakdown.tablet)}</span
							>
						</div>
						<div class="w-full bg-gray-200 rounded-full h-2">
							<div
								class="bg-green-600 h-2 rounded-full"
								style="width: {(
									(analytics.device_breakdown.tablet / analytics.views.total) *
									100
								).toFixed(1)}%"
							></div>
						</div>
						<p class="text-xs text-gray-500 mt-1">
							{((analytics.device_breakdown.tablet / analytics.views.total) * 100).toFixed(1)}% of
							total views
						</p>
					</div>

					<!-- Mobile -->
					<div>
						<div class="flex justify-between items-center mb-1">
							<span class="text-sm font-medium text-gray-700">📱 Mobile</span>
							<span class="text-sm font-semibold text-gray-900"
								>{formatNumber(analytics.device_breakdown.mobile)}</span
							>
						</div>
						<div class="w-full bg-gray-200 rounded-full h-2">
							<div
								class="bg-purple-600 h-2 rounded-full"
								style="width: {(
									(analytics.device_breakdown.mobile / analytics.views.total) *
									100
								).toFixed(1)}%"
							></div>
						</div>
						<p class="text-xs text-gray-500 mt-1">
							{((analytics.device_breakdown.mobile / analytics.views.total) * 100).toFixed(1)}% of
							total views
						</p>
					</div>
				</div>
			</Card>

			<!-- Top Performing Pages -->
			<Card>
				<h2 class="text-xl font-semibold mb-4">Top Performing Pages</h2>

				{#if analytics.top_pages && analytics.top_pages.length > 0}
					<div class="space-y-3">
						{#each analytics.top_pages as pageData, index}
							<div
								class="border-l-4 {index === 0
									? 'border-yellow-500'
									: index === 1
										? 'border-gray-400'
										: index === 2
											? 'border-orange-600'
											: 'border-gray-300'} pl-3"
							>
								<div class="flex items-start justify-between mb-1">
									<p class="text-sm font-medium text-gray-900 flex-1">
										{#if index < 3}
											<span class="mr-1">{index === 0 ? '🥇' : index === 1 ? '🥈' : '🥉'}</span>
										{/if}
										{truncateUrl(pageData.url)}
									</p>
									<Badge variant="success" size="sm">
										{formatPercent(pageData.conversion_rate)}
									</Badge>
								</div>
								<div class="flex gap-4 text-xs text-gray-600">
									<span>{formatNumber(pageData.views)} views</span>
									<span class="text-green-600"
										>{formatNumber(pageData.conversions)} conversions</span
									>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<p class="text-sm text-gray-500">No page data available yet</p>
				{/if}
			</Card>
		</div>

		<!-- Timeline Charts -->
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			<!-- Views Timeline -->
			<Card>
				<h2 class="text-xl font-semibold mb-4">Views Timeline (Last 30 Days)</h2>

				{#if analytics.timeline.views && analytics.timeline.views.length > 0}
					<div class="space-y-2">
						{#each analytics.timeline.views.slice(-15) as day}
							<div class="flex items-center gap-2">
								<span class="text-xs text-gray-600 w-20">{day.date}</span>
								<div class="flex-1 bg-gray-200 rounded-full h-4">
									<div
										class="bg-blue-600 h-4 rounded-full flex items-center justify-end pr-2"
										style="width: {Math.max(
											(day.count / Math.max(...analytics.timeline.views.map((d: any) => d.count))) *
												100,
											5
										)}%"
									>
										<span class="text-xs text-white font-semibold">{day.count}</span>
									</div>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<p class="text-sm text-gray-500">No timeline data available yet</p>
				{/if}
			</Card>

			<!-- Conversions Timeline -->
			<Card>
				<h2 class="text-xl font-semibold mb-4">Conversions Timeline (Last 30 Days)</h2>

				{#if analytics.timeline.conversions && analytics.timeline.conversions.length > 0}
					<div class="space-y-2">
						{#each analytics.timeline.conversions.slice(-15) as day}
							<div class="flex items-center gap-2">
								<span class="text-xs text-gray-600 w-20">{day.date}</span>
								<div class="flex-1 bg-gray-200 rounded-full h-4">
									<div
										class="bg-green-600 h-4 rounded-full flex items-center justify-end pr-2"
										style="width: {Math.max(
											(day.count /
												Math.max(...analytics.timeline.conversions.map((d: any) => d.count), 1)) *
												100,
											5
										)}%"
									>
										{#if day.count > 0}
											<span class="text-xs text-white font-semibold">{day.count}</span>
										{/if}
									</div>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<p class="text-sm text-gray-500">No conversion data available yet</p>
				{/if}
			</Card>
		</div>

		<!-- Insights & Recommendations -->
		<Card class="mt-8">
			<h2 class="text-xl font-semibold mb-4">💡 Insights & Recommendations</h2>

			<div class="space-y-3">
				{#if analytics.conversion_rate.overall < 1}
					<div class="bg-red-50 border border-red-200 rounded-md p-3">
						<p class="text-sm text-red-800">
							<strong>Low Conversion Rate:</strong> Your conversion rate is below 1%. Consider:
						</p>
						<ul class="text-sm text-red-700 ml-5 mt-1 list-disc">
							<li>Testing a more compelling headline</li>
							<li>Simplifying your call-to-action</li>
							<li>Adjusting trigger timing</li>
						</ul>
					</div>
				{:else if analytics.conversion_rate.overall > 5}
					<div class="bg-green-50 border border-green-200 rounded-md p-3">
						<p class="text-sm text-green-800">
							<strong>Excellent Performance!</strong> Your conversion rate of {formatPercent(
								analytics.conversion_rate.overall
							)} is above industry average.
						</p>
					</div>
				{/if}

				{#if analytics.device_breakdown.mobile > analytics.views.total * 0.5}
					<div class="bg-blue-50 border border-blue-200 rounded-md p-3">
						<p class="text-sm text-blue-800">
							<strong>Mobile-First Audience:</strong> Over 50% of your views come from mobile devices.
							Ensure your popup is mobile-optimized.
						</p>
					</div>
				{/if}

				{#if analytics.views.total < 100}
					<div class="bg-yellow-50 border border-yellow-200 rounded-md p-3">
						<p class="text-sm text-yellow-800">
							<strong>Limited Data:</strong> With fewer than 100 views, continue monitoring before making
							major changes.
						</p>
					</div>
				{/if}
			</div>
		</Card>
	</div>
{:else}
	<div class="text-center py-12">
		<p class="text-gray-600">Failed to load analytics data</p>
		<Button class="mt-4" on:click={() => goto('/admin/popups')}>Back to Popups</Button>
	</div>
{/if}

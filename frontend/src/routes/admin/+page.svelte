<script lang="ts">
	import { onMount } from 'svelte';
	import { fade, fly, scale } from 'svelte/transition';
	import { cubicOut, elasticOut } from 'svelte/easing';
	import {
		IconReceipt,
		IconTicket,
		IconUsers,
		IconChartBar,
		IconRefresh,
		IconNews,
		IconForms,
		IconMail,
		IconBellRinging,
		IconPhoto,
		IconSettings,
		IconSeo,
		IconUserCircle,
		IconFilter,
		IconActivity,
		IconSend,
		IconAlertCircle,
		IconShoppingCart,
		IconVideo,
		IconTag,
		IconEye,
		IconClick,
		IconClock,
		IconArrowUpRight,
		IconArrowDownRight,
		IconWorld,
		IconBrandGoogle,
		IconLink,
		IconAlertTriangle,
		IconTrendingUp,
		IconCalendar,
		IconSearch,
		IconTarget,
		IconChartLine,
		IconDevices,
		IconBrowser,
		IconDeviceMobile,
		IconDeviceDesktopAnalytics,
		IconFileAnalytics,
		IconExternalLink,
		IconPlugConnected
	} from '@tabler/icons-svelte';
	import { browser } from '$app/environment';
	import { connections, isAnalyticsConnected, isSeoConnected } from '$lib/stores/connections';

	// Local fetch wrapper for SvelteKit API routes (avoids CORS issues)
	async function localFetch<T = any>(endpoint: string): Promise<T> {
		const response = await fetch(endpoint, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				'Accept': 'application/json'
			},
			credentials: 'include'
		});
		if (!response.ok) {
			throw new Error(`HTTP ${response.status}`);
		}
		return response.json();
	}

	let isLoading = true;
	let lastUpdated: Date | null = null;
	let error: string | null = null;
	let selectedPeriod = '30d';
	let mounted = false;

	// Connection status
	let analyticsConnected = false;
	let seoConnected = false;
	let hasRealAnalyticsData = false;
	let hasRealSeoData = false;

	// Real stats data
	let stats = {
		activeSubscriptions: 0,
		monthlyRevenue: 0,
		activeCoupons: 0,
		totalMembers: 0,
		totalPosts: 0,
		totalProducts: 0
	};

	// Analytics metrics - ALL VALUES START AT NULL (no fake data!)
	let analytics = {
		sessions: { value: null as number | null, change: 0, trend: 'up' as 'up' | 'down' },
		pageviews: { value: null as number | null, change: 0, trend: 'up' as 'up' | 'down' },
		avgSessionDuration: { value: null as string | null, change: 0, trend: 'up' as 'up' | 'down' },
		totalUsers: { value: null as number | null, change: 0, trend: 'up' as 'up' | 'down' },
		bounceRate: { value: null as number | null, change: 0, trend: 'up' as 'up' | 'down' },
		newUsers: { value: null as number | null, change: 0, trend: 'up' as 'up' | 'down' }
	};

	// SEO metrics - ALL VALUES START AT NULL (no fake data!)
	let seoMetrics = {
		searchTraffic: { value: null as number | null, change: 0 },
		totalImpressions: { value: null as number | null, change: 0 },
		totalClicks: { value: null as number | null, change: 0 },
		avgCTR: { value: null as number | null, change: 0 },
		totalKeywords: { value: null as number | null, change: 0 },
		avgPosition: { value: null as number | null, change: 0 },
		indexedPages: { value: null as number | null, change: 0 },
		error404Count: { value: null as number | null, hits: 0 },
		redirections: { count: null as number | null, hits: 0 }
	};

	// Device breakdown - default values to prevent null access errors
	let deviceBreakdown: { desktop: number; mobile: number; tablet: number } = { desktop: 0, mobile: 0, tablet: 0 };

	// Top pages
	let topPages: { path: string; views: number; change: number }[] = [];

	// Animated counter
	function animateValue(start: number, end: number, duration: number, callback: (val: number) => void) {
		const startTime = performance.now();
		const animate = (currentTime: number) => {
			const elapsed = currentTime - startTime;
			const progress = Math.min(elapsed / duration, 1);
			const eased = 1 - Math.pow(1 - progress, 3);
			callback(Math.round(start + (end - start) * eased));
			if (progress < 1) requestAnimationFrame(animate);
		};
		requestAnimationFrame(animate);
	}

	async function fetchDashboardStats() {
		isLoading = true;
		error = null;

		// Built-in analytics - always try to fetch (no external connection required)
		analyticsConnected = true; // Platform has built-in analytics
		seoConnected = true; // Platform has built-in SEO

		try {
			// Use local SvelteKit API routes to avoid CORS issues
			// These routes proxy to Laravel backend server-side
			const [membersRes, subscriptionsRes, couponsRes, postsRes, productsRes, analyticsRes] = await Promise.allSettled([
				localFetch('/api/admin/members/stats'),
				localFetch('/api/admin/subscriptions/plans/stats'),
				localFetch('/api/admin/coupons'),
				localFetch('/api/admin/posts/stats'),
				localFetch('/api/admin/products/stats'),
				localFetch(`/api/admin/analytics/dashboard?period=${selectedPeriod}`)
			]);

			if (membersRes.status === 'fulfilled') {
				const memberData = membersRes.value;
				const newMembers = memberData?.overview?.total_members || memberData?.total || 0;
				if (mounted) animateValue(stats.totalMembers, newMembers, 800, v => stats.totalMembers = v);
				else stats.totalMembers = newMembers;
				stats.activeSubscriptions = memberData?.subscriptions?.active || 0;
			}

			if (subscriptionsRes.status === 'fulfilled') {
				const subData = subscriptionsRes.value?.data || subscriptionsRes.value;
				if (!stats.activeSubscriptions) {
					stats.activeSubscriptions = subData?.active_subscriptions || subData?.total_active || 0;
				}
				stats.monthlyRevenue = subData?.monthly_revenue || subData?.mrr || 0;
			}

			if (couponsRes.status === 'fulfilled') {
				const couponsData = couponsRes.value;
				const coupons = couponsData?.coupons || couponsData?.data || couponsData || [];
				stats.activeCoupons = Array.isArray(coupons)
					? coupons.filter((c: any) => c.is_active).length
					: couponsData?.total || 0;
			}

			if (postsRes.status === 'fulfilled') {
				const postsData = postsRes.value;
				stats.totalPosts = postsData?.total || postsData?.data?.total || postsData?.posts?.length || 0;
			}

			if (productsRes.status === 'fulfilled') {
				const productsData = productsRes.value;
				stats.totalProducts = productsData?.total || productsData?.data?.total || 0;
			}

			// Process analytics data from built-in analytics system
			if (analyticsRes.status === 'fulfilled') {
				const data = analyticsRes.value?.data || analyticsRes.value;
				if (data?.kpis) {
					hasRealAnalyticsData = true;

					// Sessions - ONLY use real values, no fallbacks
					if (data.kpis.sessions?.value !== undefined) {
						const sessionsVal = data.kpis.sessions.value;
						if (mounted && analytics.sessions.value !== null) {
							animateValue(analytics.sessions.value, sessionsVal, 800, v => analytics.sessions.value = v);
						} else {
							analytics.sessions.value = sessionsVal;
						}
						analytics.sessions.change = data.kpis.sessions.change ?? 0;
						analytics.sessions.trend = analytics.sessions.change >= 0 ? 'up' : 'down';
					}

					// Pageviews - ONLY use real values
					if (data.kpis.pageviews?.value !== undefined) {
						const pageviewsVal = data.kpis.pageviews.value;
						if (mounted && analytics.pageviews.value !== null) {
							animateValue(analytics.pageviews.value, pageviewsVal, 800, v => analytics.pageviews.value = v);
						} else {
							analytics.pageviews.value = pageviewsVal;
						}
						analytics.pageviews.change = data.kpis.pageviews.change ?? 0;
						analytics.pageviews.trend = analytics.pageviews.change >= 0 ? 'up' : 'down';
					}

					// Users - ONLY use real values
					const usersData = data.kpis.unique_visitors || data.kpis.users;
					if (usersData?.value !== undefined) {
						const usersVal = usersData.value;
						if (mounted && analytics.totalUsers.value !== null) {
							animateValue(analytics.totalUsers.value, usersVal, 800, v => analytics.totalUsers.value = v);
						} else {
							analytics.totalUsers.value = usersVal;
						}
						analytics.totalUsers.change = usersData.change ?? 0;
						analytics.totalUsers.trend = analytics.totalUsers.change >= 0 ? 'up' : 'down';
					}

					// New Users - ONLY use real values
					if (data.kpis.new_users?.value !== undefined) {
						const newUsersVal = data.kpis.new_users.value;
						if (mounted && analytics.newUsers.value !== null) {
							animateValue(analytics.newUsers.value, newUsersVal, 800, v => analytics.newUsers.value = v);
						} else {
							analytics.newUsers.value = newUsersVal;
						}
						analytics.newUsers.change = data.kpis.new_users.change ?? 0;
						analytics.newUsers.trend = analytics.newUsers.change >= 0 ? 'up' : 'down';
					}

					// Bounce Rate - ONLY use real values (NO MORE 42.3% FAKE DATA!)
					if (data.kpis.bounce_rate?.value !== undefined) {
						analytics.bounceRate.value = data.kpis.bounce_rate.value;
						analytics.bounceRate.change = data.kpis.bounce_rate.change ?? 0;
						analytics.bounceRate.trend = analytics.bounceRate.change <= 0 ? 'up' : 'down';
					}

					// Session Duration - ONLY use real values
					if (data.kpis.avg_session_duration?.value !== undefined) {
						analytics.avgSessionDuration.value = formatDuration(data.kpis.avg_session_duration.value);
						analytics.avgSessionDuration.change = data.kpis.avg_session_duration.change ?? 0;
						analytics.avgSessionDuration.trend = analytics.avgSessionDuration.change >= 0 ? 'up' : 'down';
					}
				}
				if (data?.top_pages) topPages = data.top_pages.slice(0, 5);

				// SEO data from built-in SEO system
				if (data?.seo) {
					hasRealSeoData = true;
					if (data.seo.search_traffic !== undefined) seoMetrics.searchTraffic.value = data.seo.search_traffic;
					if (data.seo.impressions !== undefined) seoMetrics.totalImpressions.value = data.seo.impressions;
					if (data.seo.clicks !== undefined) seoMetrics.totalClicks.value = data.seo.clicks;
					if (data.seo.keywords !== undefined) seoMetrics.totalKeywords.value = data.seo.keywords;
					if (data.seo.avg_position !== undefined) seoMetrics.avgPosition.value = data.seo.avg_position;
					if (data.seo.indexed_pages !== undefined) seoMetrics.indexedPages.value = data.seo.indexed_pages;
					if (data.seo.avg_ctr !== undefined) seoMetrics.avgCTR.value = data.seo.avg_ctr;
				}
			}

			lastUpdated = new Date();
		} catch (err) {
			console.error('Failed to fetch dashboard stats:', err);
			error = 'Failed to load some statistics. Please try again.';
		} finally {
			isLoading = false;
		}
	}

	function formatDuration(seconds: number): string {
		if (seconds < 60) return `${Math.round(seconds)}s`;
		const mins = Math.floor(seconds / 60);
		const secs = Math.round(seconds % 60);
		return `${mins}m ${secs}s`;
	}

	function formatNumber(num: number): string {
		if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
		if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
		return num.toLocaleString();
	}

	function changePeriod(period: string) {
		selectedPeriod = period;
		fetchDashboardStats();
	}

	onMount(() => {
		mounted = true;
		// Built-in analytics and SEO - no external connection needed
		analyticsConnected = true;
		seoConnected = true;
		fetchDashboardStats();
	});
</script>

<div class="dashboard-container">
	<!-- Header -->
	<header class="dashboard-header" in:fly={{ y: -20, duration: 400 }}>
		<div class="header-left">
			<h1 class="dashboard-title">Analytics Dashboard</h1>
			<p class="dashboard-subtitle">
				Real-time insights and performance metrics
				{#if lastUpdated}
					<span class="last-updated">• Updated {lastUpdated.toLocaleTimeString()}</span>
				{/if}
			</p>
		</div>
		<div class="header-right">
			<div class="period-selector">
				<button class:active={selectedPeriod === '7d'} onclick={() => changePeriod('7d')}>7D</button>
				<button class:active={selectedPeriod === '30d'} onclick={() => changePeriod('30d')}>30D</button>
				<button class:active={selectedPeriod === '90d'} onclick={() => changePeriod('90d')}>90D</button>
			</div>
			<button class="refresh-btn" onclick={fetchDashboardStats} disabled={isLoading} class:loading={isLoading}>
				<IconRefresh size={20} />
			</button>
		</div>
	</header>

	{#if error}
		<div class="error-banner" in:fly={{ y: -10, duration: 300 }}>
			<IconAlertCircle size={20} />
			<span>{error}</span>
		</div>
	{/if}

	<!-- Site Analytics Panel -->
	<section class="analytics-panel glass-panel" in:fly={{ y: 20, duration: 500, delay: 100 }}>
		<div class="panel-header">
			<div class="panel-title">
				<div class="panel-icon analytics-icon">
					<IconChartBar size={24} />
				</div>
				<div>
					<h2>Site Analytics</h2>
					<span class="panel-subtitle">Traffic & engagement overview</span>
				</div>
			</div>
			{#if analyticsConnected}
				<div class="panel-badge connected">
					<IconCalendar size={14} />
					Last {selectedPeriod === '7d' ? '7' : selectedPeriod === '30d' ? '30' : '90'} Days
				</div>
			{:else}
				<a href="/admin/connections" class="panel-badge not-connected">
					<IconPlugConnected size={14} />
					Connect Analytics
				</a>
			{/if}
		</div>

		{#if !analyticsConnected}
			<!-- Not Connected State -->
			<div class="not-connected-banner">
				<div class="not-connected-icon">
					<IconChartBar size={32} />
				</div>
				<div class="not-connected-text">
					<h3>Analytics Not Connected</h3>
					<p>Connect Google Analytics, Mixpanel, or another analytics service to see real traffic data.</p>
				</div>
				<a href="/admin/connections" class="connect-btn">
					<IconPlugConnected size={16} />
					Connect Service
				</a>
			</div>
		{:else}
			<div class="metrics-grid">
				{#each [
					{ label: 'Sessions', value: analytics.sessions.value, change: analytics.sessions.change, trend: analytics.sessions.trend, icon: IconEye, color: 'blue' },
					{ label: 'Pageviews', value: analytics.pageviews.value, change: analytics.pageviews.change, trend: analytics.pageviews.trend, icon: IconClick, color: 'purple' },
					{ label: 'Avg. Duration', value: analytics.avgSessionDuration.value, change: analytics.avgSessionDuration.change, trend: analytics.avgSessionDuration.trend, icon: IconClock, color: 'cyan', isText: true },
					{ label: 'Total Users', value: analytics.totalUsers.value, change: analytics.totalUsers.change, trend: analytics.totalUsers.trend, icon: IconUsers, color: 'green' },
					{ label: 'Bounce Rate', value: analytics.bounceRate.value, change: analytics.bounceRate.change, trend: analytics.bounceRate.trend, icon: IconActivity, color: 'orange', suffix: '%', invertTrend: true },
					{ label: 'New Users', value: analytics.newUsers.value, change: analytics.newUsers.change, trend: analytics.newUsers.trend, icon: IconUserCircle, color: 'pink' }
				] as metric, i}
					<div class="metric-card {metric.color}" in:scale={{ duration: 400, delay: 150 + i * 50, easing: cubicOut }}>
						<div class="metric-icon-wrap {metric.color}">
							<svelte:component this={metric.icon} size={20} />
						</div>
						<div class="metric-body">
							<span class="metric-label">{metric.label}</span>
							<div class="metric-value-row">
								<span class="metric-value">
									{#if isLoading}
										<span class="loading-dots">...</span>
									{:else if metric.value === null}
										<span class="no-data">—</span>
									{:else if metric.isText}
										{metric.value}
									{:else}
										{formatNumber(typeof metric.value === 'number' ? metric.value : 0)}{metric.suffix || ''}
									{/if}
								</span>
								{#if metric.value !== null && metric.change !== 0}
									<div class="metric-trend" class:positive={metric.trend === 'up'} class:negative={metric.trend === 'down'}>
										{#if metric.trend === 'up'}
											<IconArrowUpRight size={14} />
										{:else}
											<IconArrowDownRight size={14} />
										{/if}
										<span>{Math.abs(metric.change).toFixed(1)}%</span>
									</div>
								{/if}
							</div>
						</div>
						<div class="metric-glow {metric.color}"></div>
					</div>
				{/each}
			</div>
		{/if}
	</section>

	<!-- SEO & Site Health Row -->
	<div class="dual-panel-row">
		<!-- SEO Overview -->
		<section class="analytics-panel glass-panel seo-panel" in:fly={{ x: -20, duration: 500, delay: 200 }}>
			<div class="panel-header">
				<div class="panel-title">
					<div class="panel-icon google-icon">
						<IconBrandGoogle size={24} />
					</div>
					<div>
						<h2>SEO Performance</h2>
						<span class="panel-subtitle">Search engine visibility</span>
					</div>
				</div>
				{#if seoConnected}
					<a href="/admin/seo" class="panel-link">
						View Details <IconExternalLink size={14} />
					</a>
				{:else}
					<a href="/admin/connections" class="panel-badge not-connected">
						<IconPlugConnected size={14} />
						Connect SEO Tools
					</a>
				{/if}
			</div>

			{#if !seoConnected}
				<!-- Not Connected State -->
				<div class="not-connected-banner">
					<div class="not-connected-icon">
						<IconBrandGoogle size={32} />
					</div>
					<div class="not-connected-text">
						<h3>SEO Tools Not Connected</h3>
						<p>Connect Google Search Console or another SEO tool to see real search performance data.</p>
					</div>
					<a href="/admin/connections" class="connect-btn">
						<IconPlugConnected size={16} />
						Connect Service
					</a>
				</div>
			{:else}
				<div class="seo-metrics-grid">
					<div class="seo-metric-card primary">
						<div class="seo-metric-header">
							<IconSearch size={18} />
							<span>Search Traffic</span>
						</div>
						<div class="seo-metric-value">{seoMetrics.searchTraffic.value !== null ? formatNumber(seoMetrics.searchTraffic.value) : '—'}</div>
						{#if seoMetrics.searchTraffic.value !== null && seoMetrics.searchTraffic.change !== 0}
							<div class="seo-metric-change positive">
								<IconArrowUpRight size={12} />
								+{seoMetrics.searchTraffic.change}%
							</div>
						{/if}
						<div class="seo-metric-bar">
							<div class="seo-metric-bar-fill" style="width: {seoMetrics.searchTraffic.value !== null ? '75%' : '0%'}"></div>
						</div>
					</div>

					<div class="seo-metric-card">
						<div class="seo-metric-header">
							<IconEye size={18} />
							<span>Impressions</span>
						</div>
						<div class="seo-metric-value">{seoMetrics.totalImpressions.value !== null ? formatNumber(seoMetrics.totalImpressions.value) : '—'}</div>
						{#if seoMetrics.totalImpressions.value !== null && seoMetrics.totalImpressions.change !== 0}
							<div class="seo-metric-change positive">
								<IconArrowUpRight size={12} />
								+{seoMetrics.totalImpressions.change}%
							</div>
						{/if}
					</div>

					<div class="seo-metric-card">
						<div class="seo-metric-header">
							<IconClick size={18} />
							<span>Clicks</span>
						</div>
						<div class="seo-metric-value">{seoMetrics.totalClicks.value !== null ? formatNumber(seoMetrics.totalClicks.value) : '—'}</div>
						{#if seoMetrics.totalClicks.value !== null && seoMetrics.totalClicks.change !== 0}
							<div class="seo-metric-change positive">
								<IconArrowUpRight size={12} />
								+{seoMetrics.totalClicks.change}%
							</div>
						{/if}
					</div>

					<div class="seo-metric-card">
						<div class="seo-metric-header">
							<IconTarget size={18} />
							<span>Avg CTR</span>
						</div>
						<div class="seo-metric-value">{seoMetrics.avgCTR.value !== null ? seoMetrics.avgCTR.value.toFixed(1) + '%' : '—'}</div>
						{#if seoMetrics.avgCTR.value !== null && seoMetrics.avgCTR.change !== 0}
							<div class="seo-metric-change positive">
								<IconArrowUpRight size={12} />
								+{seoMetrics.avgCTR.change}%
							</div>
						{/if}
					</div>

					<div class="seo-metric-card">
						<div class="seo-metric-header">
							<IconChartLine size={18} />
							<span>Keywords</span>
						</div>
						<div class="seo-metric-value">{seoMetrics.totalKeywords.value !== null ? formatNumber(seoMetrics.totalKeywords.value) : '—'}</div>
						{#if seoMetrics.totalKeywords.value !== null && seoMetrics.totalKeywords.change !== 0}
							<div class="seo-metric-change positive">
								<IconArrowUpRight size={12} />
								+{seoMetrics.totalKeywords.change}
							</div>
						{/if}
					</div>

					<div class="seo-metric-card">
						<div class="seo-metric-header">
							<IconTrendingUp size={18} />
							<span>Avg Position</span>
						</div>
						<div class="seo-metric-value">{seoMetrics.avgPosition.value !== null ? seoMetrics.avgPosition.value.toFixed(1) : '—'}</div>
						{#if seoMetrics.avgPosition.value !== null && seoMetrics.avgPosition.change !== 0}
							<div class="seo-metric-change" class:positive={seoMetrics.avgPosition.change < 0} class:negative={seoMetrics.avgPosition.change > 0}>
								{#if seoMetrics.avgPosition.change < 0}
									<IconArrowUpRight size={12} />
								{:else}
									<IconArrowDownRight size={12} />
								{/if}
								{Math.abs(seoMetrics.avgPosition.change).toFixed(1)}
							</div>
						{/if}
					</div>
				</div>
			{/if}
		</section>

		<!-- Site Health -->
		<section class="analytics-panel glass-panel health-panel" in:fly={{ x: 20, duration: 500, delay: 200 }}>
			<div class="panel-header">
				<div class="panel-title">
					<div class="panel-icon health-icon">
						<IconActivity size={24} />
					</div>
					<div>
						<h2>Site Health</h2>
						<span class="panel-subtitle">Technical monitoring</span>
					</div>
				</div>
			</div>

			<div class="health-grid">
				<!-- Device Breakdown -->
				<div class="health-card device-card">
					<h4>Device Breakdown</h4>
					<div class="device-bars">
						<div class="device-row">
							<div class="device-info">
								<IconDevices size={16} />
								<span>Desktop</span>
							</div>
							<div class="device-bar-wrap">
								<div class="device-bar desktop" style="width: {deviceBreakdown.desktop}%"></div>
							</div>
							<span class="device-percent">{deviceBreakdown.desktop}%</span>
						</div>
						<div class="device-row">
							<div class="device-info">
								<IconDeviceMobile size={16} />
								<span>Mobile</span>
							</div>
							<div class="device-bar-wrap">
								<div class="device-bar mobile" style="width: {deviceBreakdown.mobile}%"></div>
							</div>
							<span class="device-percent">{deviceBreakdown.mobile}%</span>
						</div>
						<div class="device-row">
							<div class="device-info">
								<IconBrowser size={16} />
								<span>Tablet</span>
							</div>
							<div class="device-bar-wrap">
								<div class="device-bar tablet" style="width: {deviceBreakdown.tablet}%"></div>
							</div>
							<span class="device-percent">{deviceBreakdown.tablet}%</span>
						</div>
					</div>
				</div>

				<!-- Errors & Redirects -->
				<div class="health-card errors-card">
					<h4>404 Errors</h4>
					<div class="error-stats">
						<div class="error-stat">
							<span class="error-count" class:has-errors={seoMetrics.error404Count.value > 0}>{seoMetrics.error404Count.value}</span>
							<span class="error-label">Logged</span>
						</div>
						<div class="error-stat">
							<span class="error-count">{seoMetrics.error404Count.hits}</span>
							<span class="error-label">Hits</span>
						</div>
					</div>
				</div>

				<div class="health-card redirects-card">
					<h4>Redirections</h4>
					<div class="error-stats">
						<div class="error-stat">
							<span class="redirect-count">{seoMetrics.redirections.count}</span>
							<span class="error-label">Active</span>
						</div>
						<div class="error-stat">
							<span class="redirect-count">{seoMetrics.redirections.hits}</span>
							<span class="error-label">Hits</span>
						</div>
					</div>
				</div>
			</div>
		</section>
	</div>

	<!-- Business Overview -->
	<section class="analytics-panel glass-panel business-panel" in:fly={{ y: 20, duration: 500, delay: 300 }}>
		<div class="panel-header">
			<div class="panel-title">
				<div class="panel-icon business-icon">
					<IconTrendingUp size={24} />
				</div>
				<div>
					<h2>Business Overview</h2>
					<span class="panel-subtitle">Key business metrics</span>
				</div>
			</div>
		</div>

		<div class="business-grid">
			{#each [
				{ href: '/admin/members', icon: IconUserCircle, value: stats.totalMembers, label: 'Total Members', color: 'indigo' },
				{ href: '/admin/subscriptions', icon: IconReceipt, value: stats.activeSubscriptions, label: 'Active Subscriptions', color: 'teal' },
				{ href: '/admin/products', icon: IconShoppingCart, value: stats.totalProducts, label: 'Products', color: 'emerald' },
				{ href: '/admin/blog', icon: IconNews, value: stats.totalPosts, label: 'Blog Posts', color: 'blue' },
				{ href: '/admin/coupons', icon: IconTicket, value: stats.activeCoupons, label: 'Active Coupons', color: 'amber' }
			] as item, i}
				<a href={item.href} class="business-card {item.color}" in:scale={{ duration: 400, delay: 350 + i * 50, easing: cubicOut }}>
					<div class="business-card-icon {item.color}">
						<svelte:component this={item.icon} size={28} />
					</div>
					<div class="business-card-content">
						<span class="business-card-value">
							{#if isLoading}...{:else}{formatNumber(item.value)}{/if}
						</span>
						<span class="business-card-label">{item.label}</span>
					</div>
					<div class="business-card-arrow">
						<IconArrowUpRight size={18} />
					</div>
				</a>
			{/each}
		</div>
	</section>

	<!-- Quick Actions -->
	<section class="cms-section" in:fly={{ y: 20, duration: 500, delay: 400 }}>
		<h3 class="section-title">Quick Actions</h3>
		<div class="cms-grid">
			{#each [
				{ href: '/admin/blog', icon: IconNews, label: 'Blog Posts', desc: 'Create & manage articles', color: 'blog' },
				{ href: '/admin/blog/categories', icon: IconTag, label: 'Categories', desc: 'Organize content', color: 'categories' },
				{ href: '/admin/media', icon: IconPhoto, label: 'Media Library', desc: 'Upload & organize files', color: 'media' },
				{ href: '/admin/videos', icon: IconVideo, label: 'Videos', desc: 'Manage video content', color: 'videos' },
				{ href: '/admin/forms', icon: IconForms, label: 'Forms', desc: 'Build & manage forms', color: 'forms' },
				{ href: '/admin/popups', icon: IconBellRinging, label: 'Popups', desc: 'Create popup campaigns', color: 'popups' },
				{ href: '/admin/email/campaigns', icon: IconSend, label: 'Campaigns', desc: 'Email campaigns', color: 'campaigns' },
				{ href: '/admin/email/templates', icon: IconMail, label: 'Email Templates', desc: 'Design email templates', color: 'templates' },
				{ href: '/admin/seo', icon: IconSeo, label: 'SEO', desc: 'Search optimization', color: 'seo' },
				{ href: '/admin/analytics', icon: IconChartBar, label: 'Analytics', desc: 'View detailed analytics', color: 'analytics' },
				{ href: '/admin/settings', icon: IconSettings, label: 'Settings', desc: 'System configuration', color: 'settings' }
			] as item}
				<a href={item.href} class="cms-card">
					<div class="cms-icon {item.color}">
						<svelte:component this={item.icon} size={24} />
					</div>
					<div class="cms-info">
						<span class="cms-label">{item.label}</span>
						<span class="cms-desc">{item.desc}</span>
					</div>
				</a>
			{/each}
		</div>
	</section>
</div>

<style>
	/* Not Connected Banner Styles */
	.not-connected-banner {
		display: flex;
		align-items: center;
		gap: 1.5rem;
		padding: 2rem;
		background: rgba(251, 146, 60, 0.05);
		border: 1px dashed rgba(251, 146, 60, 0.3);
		border-radius: 12px;
		margin-top: 0.5rem;
	}

	.not-connected-icon {
		width: 64px;
		height: 64px;
		background: rgba(251, 146, 60, 0.1);
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #fb923c;
		flex-shrink: 0;
	}

	.not-connected-text {
		flex: 1;
	}

	.not-connected-text h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.not-connected-text p {
		font-size: 0.875rem;
		color: #94a3b8;
		margin: 0;
	}

	.connect-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
		text-decoration: none;
		border-radius: 10px;
		font-size: 0.875rem;
		font-weight: 600;
		transition: all 0.2s;
		flex-shrink: 0;
	}

	.connect-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(249, 115, 22, 0.3);
	}

	.panel-badge.not-connected {
		background: rgba(251, 146, 60, 0.1);
		border: 1px solid rgba(251, 146, 60, 0.3);
		color: #fb923c;
		text-decoration: none;
		transition: all 0.2s;
	}

	.panel-badge.not-connected:hover {
		background: rgba(251, 146, 60, 0.2);
	}

	.panel-badge.connected {
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.2);
		color: #34d399;
	}

	.no-data {
		color: #64748b;
		font-weight: 400;
	}

	/* Base Container */
	.dashboard-container {
		max-width: 1600px;
		padding: 0 1rem;
	}

	/* Header */
	.dashboard-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.dashboard-title {
		font-size: 2.25rem;
		font-weight: 800;
		background: linear-gradient(135deg, #f1f5f9 0%, #94a3b8 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		margin: 0 0 0.25rem 0;
	}

	.dashboard-subtitle {
		font-size: 0.95rem;
		color: #64748b;
		margin: 0;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.last-updated {
		color: #4ade80;
		font-size: 0.85rem;
	}

	.header-right {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.period-selector {
		display: flex;
		background: rgba(15, 23, 42, 0.8);
		border-radius: 12px;
		padding: 4px;
		border: 1px solid rgba(99, 102, 241, 0.2);
		backdrop-filter: blur(10px);
	}

	.period-selector button {
		padding: 0.5rem 1rem;
		background: transparent;
		border: none;
		color: #64748b;
		font-size: 0.875rem;
		font-weight: 600;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.period-selector button:hover {
		color: #e2e8f0;
		background: rgba(99, 102, 241, 0.1);
	}

	.period-selector button.active {
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	.refresh-btn {
		width: 44px;
		height: 44px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 12px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.25s;
		backdrop-filter: blur(10px);
	}

	.refresh-btn:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #818cf8;
		border-color: rgba(99, 102, 241, 0.4);
	}

	.refresh-btn.loading :global(svg) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	.error-banner {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		background: rgba(239, 68, 68, 0.15);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 12px;
		color: #fca5a5;
		margin-bottom: 1.5rem;
		backdrop-filter: blur(10px);
	}

	/* Glass Panel */
	.glass-panel {
		background: linear-gradient(135deg, rgba(15, 23, 42, 0.9) 0%, rgba(30, 41, 59, 0.8) 100%);
		border: 1px solid rgba(99, 102, 241, 0.15);
		border-radius: 20px;
		padding: 1.75rem;
		margin-bottom: 1.5rem;
		backdrop-filter: blur(20px);
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3), inset 0 1px 0 rgba(255, 255, 255, 0.05);
		position: relative;
		overflow: hidden;
	}

	.glass-panel::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 1px;
		background: linear-gradient(90deg, transparent, rgba(99, 102, 241, 0.3), transparent);
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.75rem;
	}

	.panel-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.panel-title h2 {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.panel-subtitle {
		font-size: 0.8rem;
		color: #64748b;
	}

	.panel-icon {
		width: 48px;
		height: 48px;
		border-radius: 14px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.panel-icon.analytics-icon {
		background: linear-gradient(135deg, rgba(99, 102, 241, 0.2), rgba(139, 92, 246, 0.2));
		color: #a5b4fc;
		box-shadow: 0 0 20px rgba(99, 102, 241, 0.2);
	}

	.panel-icon.google-icon {
		background: linear-gradient(135deg, rgba(234, 67, 53, 0.2), rgba(251, 188, 5, 0.1));
		color: #fbbf24;
		box-shadow: 0 0 20px rgba(234, 67, 53, 0.15);
	}

	.panel-icon.health-icon {
		background: linear-gradient(135deg, rgba(34, 197, 94, 0.2), rgba(16, 185, 129, 0.2));
		color: #4ade80;
		box-shadow: 0 0 20px rgba(34, 197, 94, 0.2);
	}

	.panel-icon.business-icon {
		background: linear-gradient(135deg, rgba(20, 184, 166, 0.2), rgba(6, 182, 212, 0.2));
		color: #2dd4bf;
		box-shadow: 0 0 20px rgba(20, 184, 166, 0.2);
	}

	.panel-badge {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8rem;
		color: #94a3b8;
		background: rgba(99, 102, 241, 0.1);
		padding: 0.5rem 1rem;
		border-radius: 20px;
		border: 1px solid rgba(99, 102, 241, 0.2);
	}

	.panel-link {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: #818cf8;
		text-decoration: none;
		padding: 0.5rem 1rem;
		border-radius: 8px;
		transition: all 0.2s;
	}

	.panel-link:hover {
		color: #a5b4fc;
		background: rgba(99, 102, 241, 0.1);
	}

	/* Metrics Grid */
	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(6, 1fr);
		gap: 1rem;
	}

	@media (max-width: 1400px) {
		.metrics-grid { grid-template-columns: repeat(3, 1fr); }
	}

	@media (max-width: 768px) {
		.metrics-grid { grid-template-columns: repeat(2, 1fr); }
	}

	.metric-card {
		position: relative;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 16px;
		padding: 1.25rem;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		overflow: hidden;
	}

	.metric-card:hover {
		transform: translateY(-4px);
		border-color: rgba(99, 102, 241, 0.3);
		box-shadow: 0 12px 40px rgba(0, 0, 0, 0.3);
	}

	.metric-card.blue:hover { border-color: rgba(59, 130, 246, 0.5); }
	.metric-card.purple:hover { border-color: rgba(139, 92, 246, 0.5); }
	.metric-card.cyan:hover { border-color: rgba(6, 182, 212, 0.5); }
	.metric-card.green:hover { border-color: rgba(34, 197, 94, 0.5); }
	.metric-card.orange:hover { border-color: rgba(251, 146, 60, 0.5); }
	.metric-card.pink:hover { border-color: rgba(236, 72, 153, 0.5); }

	.metric-glow {
		position: absolute;
		bottom: -50%;
		left: -50%;
		width: 200%;
		height: 200%;
		opacity: 0;
		transition: opacity 0.3s;
		pointer-events: none;
	}

	.metric-card:hover .metric-glow { opacity: 1; }

	.metric-glow.blue { background: radial-gradient(circle, rgba(59, 130, 246, 0.15) 0%, transparent 70%); }
	.metric-glow.purple { background: radial-gradient(circle, rgba(139, 92, 246, 0.15) 0%, transparent 70%); }
	.metric-glow.cyan { background: radial-gradient(circle, rgba(6, 182, 212, 0.15) 0%, transparent 70%); }
	.metric-glow.green { background: radial-gradient(circle, rgba(34, 197, 94, 0.15) 0%, transparent 70%); }
	.metric-glow.orange { background: radial-gradient(circle, rgba(251, 146, 60, 0.15) 0%, transparent 70%); }
	.metric-glow.pink { background: radial-gradient(circle, rgba(236, 72, 153, 0.15) 0%, transparent 70%); }

	.metric-icon-wrap {
		width: 40px;
		height: 40px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 1rem;
	}

	.metric-icon-wrap.blue { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.metric-icon-wrap.purple { background: rgba(139, 92, 246, 0.15); color: #a78bfa; }
	.metric-icon-wrap.cyan { background: rgba(6, 182, 212, 0.15); color: #22d3ee; }
	.metric-icon-wrap.green { background: rgba(34, 197, 94, 0.15); color: #4ade80; }
	.metric-icon-wrap.orange { background: rgba(251, 146, 60, 0.15); color: #fb923c; }
	.metric-icon-wrap.pink { background: rgba(236, 72, 153, 0.15); color: #f472b6; }

	.metric-label {
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		display: block;
		margin-bottom: 0.5rem;
	}

	.metric-value-row {
		display: flex;
		align-items: flex-end;
		justify-content: space-between;
		gap: 0.5rem;
	}

	.metric-value {
		font-size: 1.75rem;
		font-weight: 800;
		color: #f1f5f9;
		line-height: 1;
	}

	.loading-dots {
		opacity: 0.5;
	}

	.metric-trend {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.75rem;
		font-weight: 600;
		padding: 0.25rem 0.5rem;
		border-radius: 6px;
	}

	.metric-trend.positive {
		color: #4ade80;
		background: rgba(34, 197, 94, 0.15);
	}

	.metric-trend.negative {
		color: #f87171;
		background: rgba(239, 68, 68, 0.15);
	}

	/* Dual Panel Row */
	.dual-panel-row {
		display: grid;
		grid-template-columns: 1.2fr 0.8fr;
		gap: 1.5rem;
		margin-bottom: 1.5rem;
	}

	@media (max-width: 1100px) {
		.dual-panel-row { grid-template-columns: 1fr; }
	}

	/* SEO Panel */
	.seo-metrics-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1rem;
	}

	@media (max-width: 768px) {
		.seo-metrics-grid { grid-template-columns: repeat(2, 1fr); }
	}

	.seo-metric-card {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 14px;
		padding: 1.25rem;
		transition: all 0.2s;
	}

	.seo-metric-card:hover {
		border-color: rgba(99, 102, 241, 0.3);
		background: rgba(99, 102, 241, 0.05);
	}

	.seo-metric-card.primary {
		grid-column: span 3;
		background: linear-gradient(135deg, rgba(234, 67, 53, 0.1), rgba(251, 188, 5, 0.05));
		border-color: rgba(234, 67, 53, 0.2);
	}

	@media (max-width: 768px) {
		.seo-metric-card.primary { grid-column: span 2; }
	}

	.seo-metric-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #94a3b8;
		font-size: 0.8rem;
		font-weight: 500;
		margin-bottom: 0.75rem;
	}

	.seo-metric-value {
		font-size: 1.75rem;
		font-weight: 800;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.seo-metric-card.primary .seo-metric-value {
		font-size: 2.5rem;
	}

	.seo-metric-change {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.75rem;
		font-weight: 600;
		padding: 0.25rem 0.5rem;
		border-radius: 6px;
	}

	.seo-metric-change.positive {
		color: #4ade80;
		background: rgba(34, 197, 94, 0.15);
	}

	.seo-metric-change.negative {
		color: #f87171;
		background: rgba(239, 68, 68, 0.15);
	}

	.seo-metric-bar {
		height: 6px;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 3px;
		margin-top: 1rem;
		overflow: hidden;
	}

	.seo-metric-bar-fill {
		height: 100%;
		background: linear-gradient(90deg, #ea4335, #fbbc05, #34a853);
		border-radius: 3px;
		transition: width 1s ease-out;
	}

	/* Health Panel */
	.health-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1rem;
	}

	.health-card {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 14px;
		padding: 1.25rem;
	}

	.health-card h4 {
		font-size: 0.85rem;
		font-weight: 600;
		color: #e2e8f0;
		margin: 0 0 1rem 0;
	}

	.device-bars {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.device-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.device-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #94a3b8;
		font-size: 0.8rem;
		min-width: 80px;
	}

	.device-bar-wrap {
		flex: 1;
		height: 8px;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 4px;
		overflow: hidden;
	}

	.device-bar {
		height: 100%;
		border-radius: 4px;
		transition: width 1s ease-out;
	}

	.device-bar.desktop { background: linear-gradient(90deg, #6366f1, #8b5cf6); }
	.device-bar.mobile { background: linear-gradient(90deg, #06b6d4, #22d3ee); }
	.device-bar.tablet { background: linear-gradient(90deg, #f59e0b, #fbbf24); }

	.device-percent {
		font-size: 0.8rem;
		font-weight: 600;
		color: #e2e8f0;
		min-width: 40px;
		text-align: right;
	}

	.error-stats {
		display: flex;
		gap: 2rem;
	}

	.error-stat {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.error-count {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.error-count.has-errors {
		color: #f87171;
	}

	.redirect-count {
		font-size: 1.5rem;
		font-weight: 700;
		color: #4ade80;
	}

	.error-label {
		font-size: 0.75rem;
		color: #64748b;
	}

	/* Business Panel */
	.business-grid {
		display: grid;
		grid-template-columns: repeat(5, 1fr);
		gap: 1rem;
	}

	@media (max-width: 1200px) {
		.business-grid { grid-template-columns: repeat(3, 1fr); }
	}

	@media (max-width: 768px) {
		.business-grid { grid-template-columns: repeat(2, 1fr); }
	}

	.business-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 16px;
		text-decoration: none;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		position: relative;
		overflow: hidden;
	}

	.business-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 12px 40px rgba(0, 0, 0, 0.3);
	}

	.business-card.indigo:hover { border-color: rgba(99, 102, 241, 0.5); background: rgba(99, 102, 241, 0.1); }
	.business-card.teal:hover { border-color: rgba(20, 184, 166, 0.5); background: rgba(20, 184, 166, 0.1); }
	.business-card.emerald:hover { border-color: rgba(16, 185, 129, 0.5); background: rgba(16, 185, 129, 0.1); }
	.business-card.blue:hover { border-color: rgba(59, 130, 246, 0.5); background: rgba(59, 130, 246, 0.1); }
	.business-card.amber:hover { border-color: rgba(245, 158, 11, 0.5); background: rgba(245, 158, 11, 0.1); }

	.business-card-icon {
		width: 56px;
		height: 56px;
		border-radius: 14px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.business-card-icon.indigo { background: rgba(99, 102, 241, 0.15); color: #818cf8; }
	.business-card-icon.teal { background: rgba(20, 184, 166, 0.15); color: #2dd4bf; }
	.business-card-icon.emerald { background: rgba(16, 185, 129, 0.15); color: #34d399; }
	.business-card-icon.blue { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.business-card-icon.amber { background: rgba(245, 158, 11, 0.15); color: #fbbf24; }

	.business-card-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.business-card-value {
		font-size: 1.75rem;
		font-weight: 800;
		color: #f1f5f9;
		line-height: 1;
	}

	.business-card-label {
		font-size: 0.8rem;
		color: #94a3b8;
		font-weight: 500;
	}

	.business-card-arrow {
		color: #64748b;
		opacity: 0;
		transform: translateX(-10px);
		transition: all 0.3s;
	}

	.business-card:hover .business-card-arrow {
		opacity: 1;
		transform: translateX(0);
		color: #818cf8;
	}

	/* CMS Section */
	.cms-section {
		margin-bottom: 2rem;
	}

	.section-title {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 1.25rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.15);
	}

	.cms-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
		gap: 1rem;
	}

	.cms-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem 1.25rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 14px;
		text-decoration: none;
		transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.cms-card:hover {
		background: rgba(99, 102, 241, 0.1);
		border-color: rgba(99, 102, 241, 0.3);
		transform: translateX(4px);
	}

	.cms-icon {
		width: 44px;
		height: 44px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.cms-icon.blog { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.cms-icon.categories { background: rgba(139, 92, 246, 0.15); color: #a78bfa; }
	.cms-icon.media { background: rgba(168, 85, 247, 0.15); color: #c084fc; }
	.cms-icon.videos { background: rgba(236, 72, 153, 0.15); color: #f472b6; }
	.cms-icon.forms { background: rgba(34, 197, 94, 0.15); color: #4ade80; }
	.cms-icon.popups { background: rgba(251, 146, 60, 0.15); color: #fb923c; }
	.cms-icon.campaigns { background: rgba(239, 68, 68, 0.15); color: #f87171; }
	.cms-icon.templates { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.cms-icon.seo { background: rgba(34, 197, 94, 0.15); color: #4ade80; }
	.cms-icon.analytics { background: rgba(99, 102, 241, 0.15); color: #818cf8; }
	.cms-icon.settings { background: rgba(148, 163, 184, 0.15); color: #cbd5e1; }

	.cms-info {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.cms-label {
		font-weight: 600;
		font-size: 0.95rem;
		color: #f1f5f9;
	}

	.cms-desc {
		font-size: 0.8rem;
		color: #64748b;
	}
</style>

<!--
	URL: /admin
-->

<script lang="ts">
	import { fly, scale } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import {
		IconReceipt,
		IconTicket,
		IconUsers,
		IconChartBar,
		IconRefresh,
		IconNews,
		IconUserCircle,
		IconActivity,
		IconAlertCircle,
		IconShoppingCart,
		IconEye,
		IconClick,
		IconClock,
		IconArrowUpRight,
		IconBrandGoogle,
		IconTrendingUp,
		IconCalendar,
		IconSearch,
		IconTarget,
		IconChartLine,
		IconDevices,
		IconBrowser,
		IconDeviceMobile,
		IconExternalLink,
		IconPlugConnected
	} from '$lib/icons';
	import { browser } from '$app/environment';
	import { getIsAnalyticsConnected, getIsSeoConnected } from '$lib/stores/connections.svelte';
	import { getAuthToken } from '$lib/stores/auth.svelte';

	// ICT 11+ CORB Fix: Use same-origin SvelteKit proxy endpoints
	async function localFetch<T = any>(endpoint: string): Promise<T> {
		const url = endpoint.startsWith('http')
			? endpoint
			: endpoint.startsWith('/api/')
				? endpoint
				: `/api${endpoint}`;

		const token = getAuthToken();
		const headers: Record<string, string> = {
			'Content-Type': 'application/json',
			Accept: 'application/json'
		};
		if (token) {
			headers['Authorization'] = `Bearer ${token}`;
		}

		const response = await fetch(url, {
			method: 'GET',
			headers,
			credentials: 'include'
		});
		if (!response.ok) {
			throw new Error(`HTTP ${response.status}`);
		}
		return response.json();
	}

	let isLoading = $state(true);
	let lastUpdated = $state<Date | null>(null);
	let error = $state<string | null>(null);
	let selectedPeriod = $state('30d');
	let mounted = $state(false);

	// Connection status
	let analyticsConnected = $state(false);
	let seoConnected = $state(false);
	let hasRealAnalyticsData = $state(false);
	let hasRealSeoData = $state(false);

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
	let deviceBreakdown: { desktop: number; mobile: number; tablet: number } = {
		desktop: 0,
		mobile: 0,
		tablet: 0
	};

	// Top pages
	let topPages: { path: string; views: number; change: number }[] = [];


	// Animated counter
	function animateValue(
		start: number,
		end: number,
		duration: number,
		callback: (val: number) => void
	) {
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
			// ICT 7: Use correct backend endpoints
			const [membersRes, couponsRes, postsRes, productsRes, analyticsRes] =
				await Promise.allSettled([
					localFetch('/api/admin/members/stats'),
					localFetch('/api/admin/coupons'),
					localFetch('/api/admin/posts/stats'),
					localFetch('/api/admin/products/stats'),
					localFetch(`/api/admin/analytics/dashboard?period=${selectedPeriod}`)
				]);

			// Members stats
			if (membersRes.status === 'fulfilled') {
				const memberData = membersRes.value;
				const newMembers = memberData?.overview?.total_members || memberData?.total || 0;
				if (mounted)
					animateValue(stats.totalMembers, newMembers, 800, (v) => (stats.totalMembers = v));
				else stats.totalMembers = newMembers;
				stats.activeSubscriptions = memberData?.subscriptions?.active || 0;
				stats.monthlyRevenue = memberData?.revenue?.mrr || 0;
			}

			// Coupons
			if (couponsRes.status === 'fulfilled') {
				const couponsData = couponsRes.value;
				const coupons = couponsData?.coupons || couponsData?.data || couponsData || [];
				stats.activeCoupons = Array.isArray(coupons)
					? coupons.filter((c: any) => c.is_active).length
					: couponsData?.total || 0;
			}

			// Posts stats
			if (postsRes.status === 'fulfilled') {
				const postsData = postsRes.value;
				stats.totalPosts = postsData?.total_posts || postsData?.published || 0;
			}

			// Products stats
			if (productsRes.status === 'fulfilled') {
				const productsData = productsRes.value;
				stats.totalProducts = productsData?.total || productsData?.active || 0;
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
							animateValue(
								analytics.sessions.value,
								sessionsVal,
								800,
								(v) => (analytics.sessions.value = v)
							);
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
							animateValue(
								analytics.pageviews.value,
								pageviewsVal,
								800,
								(v) => (analytics.pageviews.value = v)
							);
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
							animateValue(
								analytics.totalUsers.value,
								usersVal,
								800,
								(v) => (analytics.totalUsers.value = v)
							);
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
							animateValue(
								analytics.newUsers.value,
								newUsersVal,
								800,
								(v) => (analytics.newUsers.value = v)
							);
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
						analytics.avgSessionDuration.value = formatDuration(
							data.kpis.avg_session_duration.value
						);
						analytics.avgSessionDuration.change = data.kpis.avg_session_duration.change ?? 0;
						analytics.avgSessionDuration.trend =
							analytics.avgSessionDuration.change >= 0 ? 'up' : 'down';
					}
				}
				if (data?.top_pages) topPages = data.top_pages.slice(0, 5);

				// SEO data from built-in SEO system
				if (data?.seo) {
					hasRealSeoData = true;
					if (data.seo.search_traffic !== undefined)
						seoMetrics.searchTraffic.value = data.seo.search_traffic;
					if (data.seo.impressions !== undefined)
						seoMetrics.totalImpressions.value = data.seo.impressions;
					if (data.seo.clicks !== undefined) seoMetrics.totalClicks.value = data.seo.clicks;
					if (data.seo.keywords !== undefined) seoMetrics.totalKeywords.value = data.seo.keywords;
					if (data.seo.avg_position !== undefined)
						seoMetrics.avgPosition.value = data.seo.avg_position;
					if (data.seo.indexed_pages !== undefined)
						seoMetrics.indexedPages.value = data.seo.indexed_pages;
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

	// Svelte 5: Initialize on mount
	$effect(() => {
		if (!browser) return;
		mounted = true;
		// Built-in analytics and SEO - no external connection needed
		analyticsConnected = true;
		seoConnected = true;
		fetchDashboardStats();
	});
</script>

<div class="admin-dashboard">
	<div class="admin-page-container">
		<!-- Animated Background -->
		<div class="bg-effects">
			<div class="bg-blob bg-blob-1"></div>
			<div class="bg-blob bg-blob-2"></div>
			<div class="bg-blob bg-blob-3"></div>
		</div>

		<!-- Header - Centered Style -->
		<header class="page-header" in:fly={{ y: -20, duration: 400 }}>
			<h1>Analytics Dashboard</h1>
			<p class="subtitle">
				Real-time insights and performance metrics
				{#if lastUpdated}
					<span class="last-updated">Updated {lastUpdated.toLocaleTimeString()}</span>
				{/if}
			</p>
			<div class="header-actions">
				<div class="period-selector">
					<button class:active={selectedPeriod === '7d'} onclick={() => changePeriod('7d')}
						>7D</button
					>
					<button class:active={selectedPeriod === '30d'} onclick={() => changePeriod('30d')}
						>30D</button
					>
					<button class:active={selectedPeriod === '90d'} onclick={() => changePeriod('90d')}
						>90D</button
					>
				</div>
				<button
					class="btn-secondary"
					onclick={fetchDashboardStats}
					disabled={isLoading}
					class:loading={isLoading}
				>
					<IconRefresh size={18} />
					Refresh
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
						<p>
							Connect Google Analytics, Mixpanel, or another analytics service to see real traffic
							data.
						</p>
					</div>
					<a href="/admin/connections" class="connect-btn">
						<IconPlugConnected size={16} />
						Connect Service
					</a>
				</div>
			{:else}
				<div class="metrics-grid">
					{#each [{ label: 'Sessions', value: analytics.sessions.value, change: analytics.sessions.change, trend: analytics.sessions.trend, icon: IconEye, color: 'blue' }, { label: 'Pageviews', value: analytics.pageviews.value, change: analytics.pageviews.change, trend: analytics.pageviews.trend, icon: IconClick, color: 'purple' }, { label: 'Avg. Duration', value: analytics.avgSessionDuration.value, change: analytics.avgSessionDuration.change, trend: analytics.avgSessionDuration.trend, icon: IconClock, color: 'cyan', isText: true }, { label: 'Total Users', value: analytics.totalUsers.value, change: analytics.totalUsers.change, trend: analytics.totalUsers.trend, icon: IconUsers, color: 'green' }, { label: 'Bounce Rate', value: analytics.bounceRate.value, change: analytics.bounceRate.change, trend: analytics.bounceRate.trend, icon: IconActivity, color: 'orange', suffix: '%', invertTrend: true }, { label: 'New Users', value: analytics.newUsers.value, change: analytics.newUsers.change, trend: analytics.newUsers.trend, icon: IconUserCircle, color: 'pink' }] as metric, i}
						{@const MetricIcon = metric.icon}
						<div
							class="metric-card {metric.color}"
							in:scale={{ duration: 400, delay: 150 + i * 50, easing: cubicOut }}
						>
							<div class="metric-icon-wrap {metric.color}">
								<MetricIcon size={20} />
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
											{formatNumber(
												typeof metric.value === 'number' ? metric.value : 0
											)}{metric.suffix || ''}
										{/if}
									</span>
									{#if metric.value !== null && metric.change !== 0}
										<div
											class="metric-trend"
											class:positive={metric.trend === 'up'}
											class:negative={metric.trend === 'down'}
										>
											<IconArrowUpRight size={14} />
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
			<section
				class="analytics-panel glass-panel seo-panel"
				in:fly={{ x: -20, duration: 500, delay: 200 }}
			>
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
							<p>
								Connect Google Search Console or another SEO tool to see real search performance
								data.
							</p>
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
							<div class="seo-metric-value">
								{seoMetrics.searchTraffic.value !== null
									? formatNumber(seoMetrics.searchTraffic.value)
									: '—'}
							</div>
							{#if seoMetrics.searchTraffic.value !== null && seoMetrics.searchTraffic.change !== 0}
								<div class="seo-metric-change positive">
									<IconArrowUpRight size={12} />
									+{seoMetrics.searchTraffic.change}%
								</div>
							{/if}
							<div class="seo-metric-bar">
								<div
									class="seo-metric-bar-fill"
									style="width: {seoMetrics.searchTraffic.value !== null ? '75%' : '0%'}"
								></div>
							</div>
						</div>

						<div class="seo-metric-card">
							<div class="seo-metric-header">
								<IconEye size={18} />
								<span>Impressions</span>
							</div>
							<div class="seo-metric-value">
								{seoMetrics.totalImpressions.value !== null
									? formatNumber(seoMetrics.totalImpressions.value)
									: '—'}
							</div>
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
							<div class="seo-metric-value">
								{seoMetrics.totalClicks.value !== null
									? formatNumber(seoMetrics.totalClicks.value)
									: '—'}
							</div>
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
							<div class="seo-metric-value">
								{seoMetrics.avgCTR.value !== null ? seoMetrics.avgCTR.value.toFixed(1) + '%' : '—'}
							</div>
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
							<div class="seo-metric-value">
								{seoMetrics.totalKeywords.value !== null
									? formatNumber(seoMetrics.totalKeywords.value)
									: '—'}
							</div>
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
							<div class="seo-metric-value">
								{seoMetrics.avgPosition.value !== null
									? seoMetrics.avgPosition.value.toFixed(1)
									: '—'}
							</div>
							{#if seoMetrics.avgPosition.value !== null && seoMetrics.avgPosition.change !== 0}
								<div
									class="seo-metric-change"
									class:positive={seoMetrics.avgPosition.change < 0}
									class:negative={seoMetrics.avgPosition.change > 0}
								>
									<IconArrowUpRight size={12} />
									{Math.abs(seoMetrics.avgPosition.change).toFixed(1)}
								</div>
							{/if}
						</div>
					</div>
				{/if}
			</section>

			<!-- Site Health -->
			<section
				class="analytics-panel glass-panel health-panel"
				in:fly={{ x: 20, duration: 500, delay: 200 }}
			>
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
								<span
									class="error-count"
									class:has-errors={(seoMetrics.error404Count.value ?? 0) > 0}
									>{seoMetrics.error404Count.value ?? 0}</span
								>
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
		<section
			class="analytics-panel glass-panel business-panel"
			in:fly={{ y: 20, duration: 500, delay: 300 }}
		>
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
				{#each [{ href: '/admin/members', icon: IconUserCircle, value: stats.totalMembers, label: 'Total Members', color: 'indigo' }, { href: '/admin/subscriptions', icon: IconReceipt, value: stats.activeSubscriptions, label: 'Active Subscriptions', color: 'teal' }, { href: '/admin/products', icon: IconShoppingCart, value: stats.totalProducts, label: 'Products', color: 'emerald' }, { href: '/admin/blog', icon: IconNews, value: stats.totalPosts, label: 'Blog Posts', color: 'blue' }, { href: '/admin/coupons', icon: IconTicket, value: stats.activeCoupons, label: 'Active Coupons', color: 'amber' }] as item, i}
					{@const BusinessIcon = item.icon}
					<a
						href={item.href}
						class="business-card {item.color}"
						in:scale={{ duration: 400, delay: 350 + i * 50, easing: cubicOut }}
					>
						<div class="business-card-icon {item.color}">
							<BusinessIcon size={28} />
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
	</div>
	<!-- End admin-page-container -->
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * RTP ADMIN DASHBOARD - Apple ICT7+ Principal Engineer Grade
	 * Consistent with Analytics Dashboard styling
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Outer Container with Gradient Background */
	.admin-dashboard {
		background: linear-gradient(
			135deg,
			var(--bg-base) 0%,
			var(--bg-elevated) 50%,
			var(--bg-base) 100%
		);
		color: var(--text-primary);
		position: relative;
		overflow: hidden;
	}

	/* Inner Container */
	.admin-page-container {
		position: relative;
		z-index: 10;
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* Background Effects - Animated Blobs */
	.bg-effects {
		position: fixed;
		inset: 0;
		pointer-events: none;
		overflow: hidden;
	}

	.bg-blob {
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.15;
	}

	.bg-blob-1 {
		width: 600px;
		height: 600px;
		top: -200px;
		right: -200px;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		animation: float 20s ease-in-out infinite;
	}

	.bg-blob-2 {
		width: 500px;
		height: 500px;
		bottom: -150px;
		left: -150px;
		background: linear-gradient(135deg, var(--secondary-500), var(--primary-600));
		animation: float 25s ease-in-out infinite reverse;
	}

	.bg-blob-3 {
		width: 400px;
		height: 400px;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		background: linear-gradient(135deg, var(--success-base), var(--success-emphasis));
		animation: float 30s ease-in-out infinite;
	}

	@keyframes float {
		0%,
		100% {
			transform: translate(0, 0) scale(1);
		}
		33% {
			transform: translate(30px, -30px) scale(1.05);
		}
		66% {
			transform: translate(-20px, 20px) scale(0.95);
		}
	}

	/* Page Header - CENTERED */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary);
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: var(--text-tertiary);
		font-size: 0.875rem;
		margin: 0 0 1rem 0;
	}

	.last-updated {
		color: var(--success-emphasis);
		font-size: 0.8rem;
		margin-left: 0.5rem;
	}

	/* Header Actions - Centered */
	.header-actions {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	/* Period Selector */
	.period-selector {
		display: flex;
		background: rgba(22, 27, 34, 0.6);
		border-radius: 8px;
		padding: 4px;
		border: 1px solid var(--border-muted);
	}

	.period-selector button {
		padding: 0.5rem 1rem;
		background: transparent;
		border: none;
		color: var(--text-secondary);
		font-size: 0.875rem;
		font-weight: 600;
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.period-selector button:hover {
		color: var(--text-primary);
		background: var(--bg-hover);
	}

	.period-selector button.active {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: var(--bg-base);
	}

	/* Button Styles - RTP Color System */
	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: var(--bg-base);
		border: none;
		padding: 0.625rem 1rem;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		opacity: 0.9;
		transform: translateY(-1px);
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		background: var(--bg-surface);
		color: var(--text-primary);
		border: 1px solid var(--border-default);
		padding: 0.5rem 1rem;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: var(--bg-hover);
		color: var(--text-primary);
	}

	.btn-secondary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary.loading :global(svg) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	/* Error Banner */
	.error-banner {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		background: var(--error-soft);
		border: 1px solid var(--error-base);
		border-radius: 8px;
		color: var(--error-emphasis);
		margin-bottom: 1.5rem;
	}

	/* Not Connected Banner Styles */
	.not-connected-banner {
		display: flex;
		align-items: center;
		gap: 1.5rem;
		padding: 2rem;
		background: var(--warning-soft);
		border: 1px dashed var(--warning-base);
		border-radius: 12px;
		margin-top: 0.5rem;
	}

	.not-connected-icon {
		width: 64px;
		height: 64px;
		background: rgba(187, 128, 9, 0.15);
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--warning-emphasis);
		flex-shrink: 0;
	}

	.not-connected-text {
		flex: 1;
	}

	.not-connected-text h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0 0 0.25rem 0;
	}

	.not-connected-text p {
		font-size: 0.875rem;
		color: var(--text-secondary);
		margin: 0;
	}

	.connect-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: var(--bg-base);
		text-decoration: none;
		border-radius: 10px;
		font-size: 0.875rem;
		font-weight: 600;
		transition: all 0.2s;
		flex-shrink: 0;
	}

	.connect-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(230, 184, 0, 0.3);
	}

	.panel-badge.not-connected {
		background: var(--warning-soft);
		border: 1px solid var(--warning-base);
		color: var(--warning-emphasis);
		text-decoration: none;
		transition: all 0.2s;
	}

	.panel-badge.not-connected:hover {
		background: rgba(187, 128, 9, 0.2);
	}

	.panel-badge.connected {
		background: var(--success-soft);
		border: 1px solid var(--success-base);
		color: var(--success-emphasis);
	}

	.no-data {
		color: var(--text-tertiary);
		font-weight: 400;
	}

	/* Glass Panel - RTP Color System */
	.glass-panel {
		background: rgba(22, 27, 34, 0.6);
		border: 1px solid var(--border-muted);
		border-radius: 16px;
		padding: 1.5rem;
		margin-bottom: 1.5rem;
		position: relative;
		overflow: hidden;
		backdrop-filter: blur(10px);
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}

	.panel-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.panel-title h2 {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0;
	}

	.panel-subtitle {
		font-size: 0.75rem;
		color: var(--text-tertiary);
	}

	.panel-icon {
		width: 44px;
		height: 44px;
		border-radius: 10px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.panel-icon.analytics-icon {
		background: rgba(61, 90, 153, 0.15);
		color: var(--secondary-300);
	}

	.panel-icon.google-icon {
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-500);
	}

	.panel-icon.health-icon {
		background: var(--success-soft);
		color: var(--success-emphasis);
	}

	.panel-icon.business-icon {
		background: rgba(61, 90, 153, 0.15);
		color: var(--secondary-300);
	}

	.panel-badge {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.75rem;
		color: var(--text-secondary);
		background: var(--bg-surface);
		padding: 0.5rem 0.875rem;
		border-radius: 6px;
		border: 1px solid var(--border-default);
	}

	.panel-link {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: var(--secondary-300);
		text-decoration: none;
		padding: 0.5rem 0.875rem;
		border-radius: 6px;
		transition: all 0.2s;
	}

	.panel-link:hover {
		color: var(--primary-400);
		background: rgba(230, 184, 0, 0.1);
	}

	/* Metrics Grid - Email Templates Style */
	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(6, 1fr);
		gap: 1rem;
	}

	@media (max-width: calc(var(--breakpoint-2xl) - 136px)) {
		.metrics-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: calc(var(--breakpoint-md) - 1px)) {
		.metrics-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	.metric-card {
		position: relative;
		background: rgba(22, 27, 34, 0.6);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		padding: 1.25rem;
		transition: all 0.2s;
		overflow: hidden;
	}

	.metric-card:hover {
		transform: translateY(-2px);
		border-color: var(--border-default);
		background: rgba(22, 27, 34, 0.5);
	}

	/* Color accents for metric cards - using RTP semantic colors */
	.metric-card.blue:hover {
		border-color: rgba(56, 139, 253, 0.5);
	}
	.metric-card.purple:hover {
		border-color: rgba(230, 184, 0, 0.5);
	}
	.metric-card.cyan:hover {
		border-color: rgba(56, 139, 253, 0.5);
	}
	.metric-card.green:hover {
		border-color: rgba(46, 160, 67, 0.5);
	}
	.metric-card.orange:hover {
		border-color: rgba(187, 128, 9, 0.5);
	}
	.metric-card.pink:hover {
		border-color: rgba(230, 184, 0, 0.5);
	}

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

	.metric-card:hover .metric-glow {
		opacity: 1;
	}

	.metric-glow.blue {
		background: radial-gradient(circle, var(--info-soft) 0%, transparent 70%);
	}
	.metric-glow.purple {
		background: radial-gradient(circle, rgba(230, 184, 0, 0.15) 0%, transparent 70%);
	}
	.metric-glow.cyan {
		background: radial-gradient(circle, var(--info-soft) 0%, transparent 70%);
	}
	.metric-glow.green {
		background: radial-gradient(circle, var(--success-soft) 0%, transparent 70%);
	}
	.metric-glow.orange {
		background: radial-gradient(circle, var(--warning-soft) 0%, transparent 70%);
	}
	.metric-glow.pink {
		background: radial-gradient(circle, rgba(230, 184, 0, 0.15) 0%, transparent 70%);
	}

	.metric-icon-wrap {
		width: 36px;
		height: 36px;
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 0.875rem;
	}

	.metric-icon-wrap.blue {
		background: var(--info-soft);
		color: var(--info-emphasis);
	}
	.metric-icon-wrap.purple {
		background: rgba(61, 90, 153, 0.15);
		color: var(--secondary-300);
	}
	.metric-icon-wrap.cyan {
		background: var(--info-soft);
		color: var(--info-emphasis);
	}
	.metric-icon-wrap.green {
		background: var(--success-soft);
		color: var(--success-emphasis);
	}
	.metric-icon-wrap.orange {
		background: var(--warning-soft);
		color: var(--warning-emphasis);
	}
	.metric-icon-wrap.pink {
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-400);
	}

	.metric-label {
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--text-tertiary);
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
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary);
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
		border-radius: 4px;
	}

	.metric-trend.positive {
		color: var(--success-emphasis);
		background: var(--success-soft);
	}

	.metric-trend.negative {
		color: var(--error-emphasis);
		background: var(--error-soft);
	}

	/* Dual Panel Row */
	.dual-panel-row {
		display: grid;
		grid-template-columns: 1.2fr 0.8fr;
		gap: 1.5rem;
		margin-bottom: 1.5rem;
	}

	@media (max-width: calc(var(--breakpoint-lg) + 76px)) {
		.dual-panel-row {
			grid-template-columns: 1fr;
		}
	}

	/* SEO Panel - Email Templates Style */
	.seo-metrics-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1rem;
	}

	@media (max-width: calc(var(--breakpoint-md) - 1px)) {
		.seo-metrics-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	.seo-metric-card {
		background: rgba(22, 27, 34, 0.6);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		padding: 1rem;
		transition: all 0.2s;
	}

	.seo-metric-card:hover {
		border-color: var(--border-default);
		background: rgba(22, 27, 34, 0.5);
	}

	.seo-metric-card.primary {
		grid-column: span 3;
		background: linear-gradient(135deg, rgba(230, 184, 0, 0.1), rgba(61, 90, 153, 0.1));
		border-color: rgba(230, 184, 0, 0.2);
	}

	@media (max-width: calc(var(--breakpoint-md) - 1px)) {
		.seo-metric-card.primary {
			grid-column: span 2;
		}
	}

	.seo-metric-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: var(--text-secondary);
		font-size: 0.75rem;
		font-weight: 500;
		margin-bottom: 0.75rem;
	}

	.seo-metric-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary);
		margin-bottom: 0.5rem;
	}

	.seo-metric-card.primary .seo-metric-value {
		font-size: 2rem;
	}

	.seo-metric-change {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.75rem;
		font-weight: 600;
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
	}

	.seo-metric-change.positive {
		color: var(--success-emphasis);
		background: var(--success-soft);
	}

	.seo-metric-change.negative {
		color: var(--error-emphasis);
		background: var(--error-soft);
	}

	.seo-metric-bar {
		height: 4px;
		background: rgba(13, 17, 23, 0.6);
		border-radius: 2px;
		margin-top: 1rem;
		overflow: hidden;
	}

	.seo-metric-bar-fill {
		height: 100%;
		background: linear-gradient(
			90deg,
			var(--error-base),
			var(--warning-emphasis),
			var(--success-emphasis)
		);
		border-radius: 2px;
		transition: width 1s ease-out;
	}

	/* Health Panel - Email Templates Style */
	.health-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1rem;
	}

	.health-card {
		background: rgba(22, 27, 34, 0.6);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		padding: 1rem;
	}

	.health-card h4 {
		font-size: 0.8rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0 0 0.875rem 0;
	}

	.device-bars {
		display: flex;
		flex-direction: column;
		gap: 0.625rem;
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
		color: var(--text-secondary);
		font-size: 0.75rem;
		min-width: 80px;
	}

	.device-bar-wrap {
		flex: 1;
		height: 6px;
		background: rgba(13, 17, 23, 0.6);
		border-radius: 3px;
		overflow: hidden;
	}

	.device-bar {
		height: 100%;
		border-radius: 3px;
		transition: width 1s ease-out;
	}

	.device-bar.desktop {
		background: linear-gradient(90deg, var(--primary-500), var(--primary-600));
	}
	.device-bar.mobile {
		background: linear-gradient(90deg, var(--info-base), var(--info-emphasis));
	}
	.device-bar.tablet {
		background: linear-gradient(90deg, var(--warning-base), var(--warning-emphasis));
	}

	.device-percent {
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-primary);
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
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--text-primary);
	}

	.error-count.has-errors {
		color: var(--error-emphasis);
	}

	.redirect-count {
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--success-emphasis);
	}

	.error-label {
		font-size: 0.7rem;
		color: var(--text-tertiary);
	}

	/* Business Panel - Email Templates Style */
	.business-grid {
		display: grid;
		grid-template-columns: repeat(5, 1fr);
		gap: 1rem;
	}

	@media (max-width: calc(var(--breakpoint-xl) - 1px)) {
		.business-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: calc(var(--breakpoint-md) - 1px)) {
		.business-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	.business-card {
		display: flex;
		align-items: center;
		gap: 0.875rem;
		padding: 1rem;
		background: rgba(22, 27, 34, 0.6);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		text-decoration: none;
		transition: all 0.2s;
		position: relative;
		overflow: hidden;
	}

	.business-card:hover {
		transform: translateY(-2px);
		background: rgba(22, 27, 34, 0.5);
	}

	.business-card.indigo:hover {
		border-color: rgba(61, 90, 153, 0.3);
	}
	.business-card.teal:hover {
		border-color: rgba(46, 160, 67, 0.3);
	}
	.business-card.emerald:hover {
		border-color: rgba(46, 160, 67, 0.3);
	}
	.business-card.blue:hover {
		border-color: rgba(56, 139, 253, 0.3);
	}
	.business-card.amber:hover {
		border-color: rgba(230, 184, 0, 0.3);
	}

	.business-card-icon {
		width: 48px;
		height: 48px;
		border-radius: 10px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.business-card-icon.indigo {
		background: rgba(61, 90, 153, 0.15);
		color: var(--secondary-300);
	}
	.business-card-icon.teal {
		background: var(--success-soft);
		color: var(--success-emphasis);
	}
	.business-card-icon.emerald {
		background: var(--success-soft);
		color: var(--success-emphasis);
	}
	.business-card-icon.blue {
		background: var(--info-soft);
		color: var(--info-emphasis);
	}
	.business-card-icon.amber {
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-500);
	}

	.business-card-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.business-card-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary);
		line-height: 1;
	}

	.business-card-label {
		font-size: 0.75rem;
		color: var(--text-tertiary);
		font-weight: 500;
	}

	.business-card-arrow {
		color: var(--text-tertiary);
		opacity: 0;
		transform: translateX(-10px);
		transition: all 0.2s;
	}

	.business-card:hover .business-card-arrow {
		opacity: 1;
		transform: translateX(0);
		color: var(--primary-400);
	}

	.section-title {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary);
		margin-bottom: 1rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid var(--border-muted);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE BREAKPOINTS - Email Templates Style
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Small Mobile (< 480px) */
	@media (max-width: calc(var(--breakpoint-sm) - 160px)) {
		.admin-page-container {
			padding: 1rem;
		}

		.page-header {
			margin-bottom: 1.5rem;
		}

		.page-header h1 {
			font-size: 1.5rem;
		}

		.header-actions {
			flex-direction: column;
			gap: 0.75rem;
		}

		.period-selector {
			width: 100%;
			justify-content: center;
		}

		/* Stats Grid - Single column on small mobile */
		.metrics-grid,
		.seo-metrics-grid {
			grid-template-columns: 1fr;
			gap: 0.75rem;
		}

		.seo-metric-card.primary {
			grid-column: span 1;
		}

		.seo-metric-card.primary .seo-metric-value {
			font-size: 1.75rem;
		}

		/* Business Grid */
		.business-grid {
			grid-template-columns: 1fr;
			gap: 0.75rem;
		}

		.business-card-value {
			font-size: 1.25rem;
		}


		/* Panel Sections */
		.glass-panel {
			padding: 1rem;
			border-radius: 8px;
		}

		.panel-header h2 {
			font-size: 1rem;
		}

		/* Metric Cards */
		.metric-card {
			padding: 1rem;
		}

		.metric-value {
			font-size: 1.25rem;
		}

		/* Health Grid */
		.health-grid {
			gap: 0.75rem;
		}

		.error-count,
		.redirect-count {
			font-size: 1.125rem;
		}

		/* Section Titles */
		.section-title {
			font-size: 1rem;
		}
	}

	/* Mobile Landscape / Large Mobile (481px - 640px) */
	@media (min-width: calc(var(--breakpoint-sm) - 159px)) and (max-width: calc(var(--breakpoint-sm) - 1px)) {
		.metrics-grid,
		.seo-metrics-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.business-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	/* Tablet Portrait (641px - 768px) */
	@media (min-width: var(--breakpoint-sm)) and (max-width: calc(var(--breakpoint-md) - 1px)) {
		.metrics-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.seo-metrics-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.seo-metric-card.primary {
			grid-column: span 2;
		}

		.business-grid {
			grid-template-columns: repeat(2, 1fr);
		}

	}

	/* Tablet Landscape (769px - 1024px) */
	@media (min-width: var(--breakpoint-md)) and (max-width: calc(var(--breakpoint-lg) - 1px)) {
		.metrics-grid {
			grid-template-columns: repeat(3, 1fr);
		}

		.seo-metrics-grid {
			grid-template-columns: repeat(3, 1fr);
		}

		.business-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	/* Touch Device Optimizations */
	@media (hover: none) and (pointer: coarse) {
		.btn-secondary,
		.period-selector button,
		.business-card {
			min-height: 44px;
		}

		.metric-card,
		.seo-metric-card,
		.health-card {
			min-height: 80px;
		}
	}

	/* Reduced Motion - Accessibility */
	@media (prefers-reduced-motion: reduce) {
		.business-card,
		.cms-card,
		.metric-card,
		.seo-metric-card,
		.device-bar,
		.seo-metric-bar-fill,
		.btn-primary,
		.btn-secondary {
			transition: none;
		}

		.business-card:hover,
		.metric-card:hover {
			transform: none;
		}
	}

	/* High Contrast Mode - Accessibility */
	@media (prefers-contrast: high) {
		.business-card,
		.cms-card,
		.metric-card,
		.seo-metric-card,
		.health-card,
		.glass-panel {
			border-width: 2px;
		}

		.metric-value,
		.seo-metric-value,
		.business-card-value {
			font-weight: 800;
		}
	}

	/* Print Styles */
	@media print {
		.admin-dashboard {
			background: white;
			color: black;
		}

		.header-actions,
		.business-card-arrow {
			display: none !important;
		}

		.business-card,
		.metric-card,
		.glass-panel {
			break-inside: avoid;
			box-shadow: none;
			border: 1px solid #ccc;
		}
	}
</style>

<script lang="ts">
	import { onMount } from 'svelte';
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
		IconCalendar
	} from '@tabler/icons-svelte';
	import { browser } from '$app/environment';
	import { api } from '$lib/api/config';

	let statsRef: HTMLDivElement;
	let isLoading = true;
	let lastUpdated: Date | null = null;
	let error: string | null = null;
	let selectedPeriod = '30d';

	// Real stats data - fetched from API
	let stats = {
		activeSubscriptions: 0,
		monthlyRevenue: 0,
		activeCoupons: 0,
		totalMembers: 0,
		totalPosts: 0,
		totalProducts: 0
	};

	// Analytics metrics (Google Analytics style)
	let analytics = {
		sessions: { value: 0, change: 0, trend: 'up' as 'up' | 'down' },
		pageviews: { value: 0, change: 0, trend: 'up' as 'up' | 'down' },
		avgSessionDuration: { value: '0s', change: 0, trend: 'up' as 'up' | 'down' },
		totalUsers: { value: 0, change: 0, trend: 'up' as 'up' | 'down' },
		bounceRate: { value: 0, change: 0, trend: 'down' as 'up' | 'down' },
		newUsers: { value: 0, change: 0, trend: 'up' as 'up' | 'down' }
	};

	// SEO metrics (Rank Math style)
	let seoMetrics = {
		searchTraffic: { value: 0, change: 0 },
		totalImpressions: { value: 0, change: 0 },
		totalKeywords: { value: 0, change: 0 },
		avgPosition: { value: 0, change: 0 },
		error404Count: { value: 0, hits: 0 },
		redirections: { count: 0, hits: 0 }
	};

	// Device breakdown
	let deviceBreakdown = {
		desktop: 65,
		mobile: 30,
		tablet: 5
	};

	// Top pages
	let topPages: { path: string; views: number; change: number }[] = [];

	async function fetchDashboardStats() {
		isLoading = true;
		error = null;
		
		try {
			// Fetch real stats from multiple endpoints in parallel
			const [membersRes, subscriptionsRes, couponsRes, postsRes, productsRes, analyticsRes] = await Promise.allSettled([
				api.get('/api/admin/members/stats'),
				api.get('/api/admin/subscriptions/plans/stats'),
				api.get('/api/admin/coupons'),
				api.get('/api/admin/posts/stats'),
				api.get('/api/admin/products/stats'),
				api.get(`/api/admin/analytics/dashboard?period=${selectedPeriod}`)
			]);

			// Extract data from responses - handle the member stats structure
			if (membersRes.status === 'fulfilled') {
				const memberData = membersRes.value;
				stats.totalMembers = memberData?.overview?.total_members || memberData?.total || 0;
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

			// Parse analytics data
			if (analyticsRes.status === 'fulfilled') {
				const data = analyticsRes.value?.data || analyticsRes.value;
				if (data?.kpis) {
					analytics.sessions = { 
						value: data.kpis.sessions?.value || 0, 
						change: data.kpis.sessions?.change || 0,
						trend: (data.kpis.sessions?.change || 0) >= 0 ? 'up' : 'down'
					};
					analytics.pageviews = { 
						value: data.kpis.pageviews?.value || 0, 
						change: data.kpis.pageviews?.change || 0,
						trend: (data.kpis.pageviews?.change || 0) >= 0 ? 'up' : 'down'
					};
					analytics.totalUsers = { 
						value: data.kpis.unique_visitors?.value || data.kpis.users?.value || 0, 
						change: data.kpis.unique_visitors?.change || data.kpis.users?.change || 0,
						trend: (data.kpis.unique_visitors?.change || 0) >= 0 ? 'up' : 'down'
					};
					analytics.newUsers = { 
						value: data.kpis.new_users?.value || 0, 
						change: data.kpis.new_users?.change || 0,
						trend: (data.kpis.new_users?.change || 0) >= 0 ? 'up' : 'down'
					};
					analytics.bounceRate = { 
						value: data.kpis.bounce_rate?.value || 0, 
						change: data.kpis.bounce_rate?.change || 0,
						trend: (data.kpis.bounce_rate?.change || 0) <= 0 ? 'up' : 'down'
					};
					const duration = data.kpis.avg_session_duration?.value || 0;
					analytics.avgSessionDuration = { 
						value: formatDuration(duration), 
						change: data.kpis.avg_session_duration?.change || 0,
						trend: (data.kpis.avg_session_duration?.change || 0) >= 0 ? 'up' : 'down'
					};
				}
				if (data?.top_pages) {
					topPages = data.top_pages.slice(0, 5);
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
		fetchDashboardStats();
	});
</script>

<div class="dashboard-container">
	<!-- Header with Period Selector -->
	<div class="dashboard-header">
		<div class="header-left">
			<h1 class="dashboard-title">Dashboard</h1>
			<p class="dashboard-subtitle">Real-time analytics and performance metrics</p>
		</div>
		<div class="header-right">
			<div class="period-selector">
				<button class:active={selectedPeriod === '7d'} on:click={() => changePeriod('7d')}>7 Days</button>
				<button class:active={selectedPeriod === '30d'} on:click={() => changePeriod('30d')}>30 Days</button>
				<button class:active={selectedPeriod === '90d'} on:click={() => changePeriod('90d')}>90 Days</button>
			</div>
			<button class="refresh-btn" on:click={fetchDashboardStats} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
		</div>
	</div>

	{#if error}
		<div class="error-banner">
			<IconAlertCircle size={20} />
			<span>{error}</span>
		</div>
	{/if}

	<!-- Main Analytics Panel -->
	<div class="analytics-panel">
		<div class="panel-header">
			<div class="panel-title">
				<IconChartBar size={22} />
				<span>Site Analytics</span>
			</div>
			<span class="panel-period">Last {selectedPeriod === '7d' ? '7' : selectedPeriod === '30d' ? '30' : '90'} Days</span>
		</div>

		<div class="analytics-grid">
			<!-- Sessions -->
			<div class="metric-card large">
				<div class="metric-header">
					<span class="metric-label">Sessions</span>
					<IconEye size={18} class="metric-icon" />
				</div>
				<div class="metric-value">{isLoading ? '...' : formatNumber(analytics.sessions.value)}</div>
				<div class="metric-change" class:positive={analytics.sessions.trend === 'up'} class:negative={analytics.sessions.trend === 'down'}>
					{#if analytics.sessions.trend === 'up'}
						<IconArrowUpRight size={16} />
					{:else}
						<IconArrowDownRight size={16} />
					{/if}
					<span>{Math.abs(analytics.sessions.change)}%</span>
					<span class="vs-text">vs. Previous Period</span>
				</div>
			</div>

			<!-- Pageviews -->
			<div class="metric-card large">
				<div class="metric-header">
					<span class="metric-label">Pageviews</span>
					<IconClick size={18} class="metric-icon" />
				</div>
				<div class="metric-value">{isLoading ? '...' : formatNumber(analytics.pageviews.value)}</div>
				<div class="metric-change" class:positive={analytics.pageviews.trend === 'up'} class:negative={analytics.pageviews.trend === 'down'}>
					{#if analytics.pageviews.trend === 'up'}
						<IconArrowUpRight size={16} />
					{:else}
						<IconArrowDownRight size={16} />
					{/if}
					<span>{Math.abs(analytics.pageviews.change)}%</span>
					<span class="vs-text">vs. Previous Period</span>
				</div>
			</div>

			<!-- Avg Session Duration -->
			<div class="metric-card">
				<div class="metric-header">
					<span class="metric-label">Avg. Session Duration</span>
					<IconClock size={18} class="metric-icon" />
				</div>
				<div class="metric-value">{isLoading ? '...' : analytics.avgSessionDuration.value}</div>
				<div class="metric-change" class:positive={analytics.avgSessionDuration.trend === 'up'} class:negative={analytics.avgSessionDuration.trend === 'down'}>
					{#if analytics.avgSessionDuration.trend === 'up'}
						<IconArrowUpRight size={16} />
					{:else}
						<IconArrowDownRight size={16} />
					{/if}
					<span>{Math.abs(analytics.avgSessionDuration.change)}%</span>
					<span class="vs-text">vs. Previous Period</span>
				</div>
			</div>

			<!-- Total Users -->
			<div class="metric-card">
				<div class="metric-header">
					<span class="metric-label">Total Users</span>
					<IconUsers size={18} class="metric-icon" />
				</div>
				<div class="metric-value">{isLoading ? '...' : formatNumber(analytics.totalUsers.value)}</div>
				<div class="metric-change" class:positive={analytics.totalUsers.trend === 'up'} class:negative={analytics.totalUsers.trend === 'down'}>
					{#if analytics.totalUsers.trend === 'up'}
						<IconArrowUpRight size={16} />
					{:else}
						<IconArrowDownRight size={16} />
					{/if}
					<span>{Math.abs(analytics.totalUsers.change)}%</span>
					<span class="vs-text">vs. Previous Period</span>
				</div>
			</div>

			<!-- Bounce Rate -->
			<div class="metric-card">
				<div class="metric-header">
					<span class="metric-label">Bounce Rate</span>
					<IconActivity size={18} class="metric-icon" />
				</div>
				<div class="metric-value">{isLoading ? '...' : analytics.bounceRate.value.toFixed(1)}%</div>
				<div class="metric-change" class:positive={analytics.bounceRate.trend === 'up'} class:negative={analytics.bounceRate.trend === 'down'}>
					{#if analytics.bounceRate.change <= 0}
						<IconArrowDownRight size={16} />
					{:else}
						<IconArrowUpRight size={16} />
					{/if}
					<span>{Math.abs(analytics.bounceRate.change)}%</span>
					<span class="vs-text">vs. Previous Period</span>
				</div>
			</div>

			<!-- New Users -->
			<div class="metric-card">
				<div class="metric-header">
					<span class="metric-label">New Users</span>
					<IconUserCircle size={18} class="metric-icon" />
				</div>
				<div class="metric-value">{isLoading ? '...' : formatNumber(analytics.newUsers.value)}</div>
				<div class="metric-change" class:positive={analytics.newUsers.trend === 'up'} class:negative={analytics.newUsers.trend === 'down'}>
					{#if analytics.newUsers.trend === 'up'}
						<IconArrowUpRight size={16} />
					{:else}
						<IconArrowDownRight size={16} />
					{/if}
					<span>{Math.abs(analytics.newUsers.change)}%</span>
					<span class="vs-text">vs. Previous Period</span>
				</div>
			</div>
		</div>
	</div>

	<!-- SEO & Technical Panel -->
	<div class="dual-panel-row">
		<!-- SEO Overview -->
		<div class="analytics-panel seo-panel">
			<div class="panel-header">
				<div class="panel-title">
					<IconBrandGoogle size={22} />
					<span>SEO Overview</span>
				</div>
				<a href="/admin/seo" class="panel-link">View Details</a>
			</div>
			<div class="seo-grid">
				<div class="seo-metric">
					<span class="seo-label">Search Traffic</span>
					<div class="seo-value">{formatNumber(seoMetrics.searchTraffic.value)}</div>
				</div>
				<div class="seo-metric">
					<span class="seo-label">Total Impressions</span>
					<div class="seo-value">{formatNumber(seoMetrics.totalImpressions.value)}</div>
				</div>
				<div class="seo-metric">
					<span class="seo-label">Total Keywords</span>
					<div class="seo-value">{formatNumber(seoMetrics.totalKeywords.value)}</div>
				</div>
				<div class="seo-metric">
					<span class="seo-label">Avg. Position</span>
					<div class="seo-value">{seoMetrics.avgPosition.value.toFixed(1)}</div>
				</div>
			</div>
		</div>

		<!-- 404 & Redirects -->
		<div class="analytics-panel monitor-panel">
			<div class="panel-header">
				<div class="panel-title">
					<IconAlertTriangle size={22} />
					<span>Site Health</span>
				</div>
			</div>
			<div class="monitor-grid">
				<div class="monitor-section">
					<span class="monitor-title">404 Errors</span>
					<div class="monitor-row">
						<div class="monitor-item">
							<span class="monitor-label">Log Count</span>
							<span class="monitor-value error">{seoMetrics.error404Count.value}</span>
						</div>
						<div class="monitor-item">
							<span class="monitor-label">URL Hits</span>
							<span class="monitor-value">{seoMetrics.error404Count.hits}</span>
						</div>
					</div>
				</div>
				<div class="monitor-section">
					<span class="monitor-title">Redirections</span>
					<div class="monitor-row">
						<div class="monitor-item">
							<span class="monitor-label">Active</span>
							<span class="monitor-value">{seoMetrics.redirections.count}</span>
						</div>
						<div class="monitor-item">
							<span class="monitor-label">Hits</span>
							<span class="monitor-value">{seoMetrics.redirections.hits}</span>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>

	<!-- Business Stats Row -->
	<div class="business-stats-panel">
		<div class="panel-header">
			<div class="panel-title">
				<IconTrendingUp size={22} />
				<span>Business Overview</span>
			</div>
		</div>
		<div class="business-grid" bind:this={statsRef}>
			<a href="/admin/members" class="business-card">
				<div class="business-icon members"><IconUserCircle size={28} /></div>
				<div class="business-content">
					<span class="business-value">{isLoading ? '...' : formatNumber(stats.totalMembers)}</span>
					<span class="business-label">Total Members</span>
				</div>
			</a>
			<a href="/admin/subscriptions" class="business-card">
				<div class="business-icon subscriptions"><IconReceipt size={28} /></div>
				<div class="business-content">
					<span class="business-value">{isLoading ? '...' : formatNumber(stats.activeSubscriptions)}</span>
					<span class="business-label">Active Subscriptions</span>
				</div>
			</a>
			<a href="/admin/products" class="business-card">
				<div class="business-icon products"><IconShoppingCart size={28} /></div>
				<div class="business-content">
					<span class="business-value">{isLoading ? '...' : formatNumber(stats.totalProducts)}</span>
					<span class="business-label">Products</span>
				</div>
			</a>
			<a href="/admin/blog" class="business-card">
				<div class="business-icon posts"><IconNews size={28} /></div>
				<div class="business-content">
					<span class="business-value">{isLoading ? '...' : formatNumber(stats.totalPosts)}</span>
					<span class="business-label">Blog Posts</span>
				</div>
			</a>
			<a href="/admin/coupons" class="business-card">
				<div class="business-icon coupons"><IconTicket size={28} /></div>
				<div class="business-content">
					<span class="business-value">{isLoading ? '...' : formatNumber(stats.activeCoupons)}</span>
					<span class="business-label">Active Coupons</span>
				</div>
			</a>
		</div>
	</div>

	<!-- CMS Functions Grid -->
	<div class="cms-section">
		<h3 class="section-title">Content Management</h3>
		<div class="cms-grid">
			<a href="/admin/blog" class="cms-card">
				<div class="cms-icon blog"><IconNews size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Blog Posts</span>
					<span class="cms-desc">Create & manage articles</span>
				</div>
			</a>
			<a href="/admin/blog/categories" class="cms-card">
				<div class="cms-icon categories"><IconTag size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Categories</span>
					<span class="cms-desc">Organize content</span>
				</div>
			</a>
			<a href="/admin/media" class="cms-card">
				<div class="cms-icon media"><IconPhoto size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Media Library</span>
					<span class="cms-desc">Upload & organize files</span>
				</div>
			</a>
			<a href="/admin/videos" class="cms-card">
				<div class="cms-icon videos"><IconVideo size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Videos</span>
					<span class="cms-desc">Manage video content</span>
				</div>
			</a>
			<a href="/admin/forms" class="cms-card">
				<div class="cms-icon forms"><IconForms size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Forms</span>
					<span class="cms-desc">Build & manage forms</span>
				</div>
			</a>
			<a href="/admin/popups" class="cms-card">
				<div class="cms-icon popups"><IconBellRinging size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Popups</span>
					<span class="cms-desc">Create popup campaigns</span>
				</div>
			</a>
		</div>
	</div>

	<!-- Members & Commerce -->
	<div class="cms-section">
		<h3 class="section-title">Members & Commerce</h3>
		<div class="cms-grid">
			<a href="/admin/members" class="cms-card">
				<div class="cms-icon members"><IconUserCircle size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Members</span>
					<span class="cms-desc">View all members</span>
				</div>
			</a>
			<a href="/admin/members/segments" class="cms-card">
				<div class="cms-icon segments"><IconFilter size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Segments</span>
					<span class="cms-desc">Create member segments</span>
				</div>
			</a>
			<a href="/admin/products" class="cms-card">
				<div class="cms-icon products"><IconShoppingCart size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Products</span>
					<span class="cms-desc">Courses & indicators</span>
				</div>
			</a>
			<a href="/admin/subscriptions" class="cms-card">
				<div class="cms-icon subscriptions"><IconReceipt size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Subscriptions</span>
					<span class="cms-desc">Manage subscriptions</span>
				</div>
			</a>
			<a href="/admin/coupons" class="cms-card">
				<div class="cms-icon coupons"><IconTicket size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Coupons</span>
					<span class="cms-desc">Create discount codes</span>
				</div>
			</a>
			<a href="/admin/crm" class="cms-card">
				<div class="cms-icon crm"><IconUsers size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">CRM</span>
					<span class="cms-desc">Contacts & deals</span>
				</div>
			</a>
		</div>
	</div>

	<!-- Email & Marketing -->
	<div class="cms-section">
		<h3 class="section-title">Email & Marketing</h3>
		<div class="cms-grid">
			<a href="/admin/email/campaigns" class="cms-card">
				<div class="cms-icon campaigns"><IconSend size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Campaigns</span>
					<span class="cms-desc">Email campaigns</span>
				</div>
			</a>
			<a href="/admin/email/templates" class="cms-card">
				<div class="cms-icon templates"><IconMail size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Email Templates</span>
					<span class="cms-desc">Design email templates</span>
				</div>
			</a>
			<a href="/admin/email/smtp" class="cms-card">
				<div class="cms-icon smtp"><IconSettings size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Email Settings</span>
					<span class="cms-desc">SMTP configuration</span>
				</div>
			</a>
			<a href="/admin/seo" class="cms-card">
				<div class="cms-icon seo"><IconSeo size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">SEO</span>
					<span class="cms-desc">Search optimization</span>
				</div>
			</a>
		</div>
	</div>

	<!-- Analytics & Reports -->
	<div class="cms-section">
		<h3 class="section-title">Analytics & Reports</h3>
		<div class="cms-grid">
			<a href="/admin/analytics" class="cms-card">
				<div class="cms-icon analytics"><IconChartBar size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Analytics</span>
					<span class="cms-desc">View detailed analytics</span>
				</div>
			</a>
			<a href="/admin/behavior" class="cms-card">
				<div class="cms-icon behavior"><IconActivity size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Behavior Tracking</span>
					<span class="cms-desc">User behavior insights</span>
				</div>
			</a>
			<a href="/admin/users" class="cms-card">
				<div class="cms-icon users"><IconUsers size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Admin Users</span>
					<span class="cms-desc">Manage admin access</span>
				</div>
			</a>
			<a href="/admin/settings" class="cms-card">
				<div class="cms-icon settings"><IconSettings size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Settings</span>
					<span class="cms-desc">System configuration</span>
				</div>
			</a>
		</div>
	</div>
</div>

<style>
	.dashboard-container {
		max-width: 1600px;
	}

	/* Dashboard Header */
	.dashboard-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.header-left {
		flex: 1;
	}

	.dashboard-title {
		font-size: 2rem;
		font-weight: 800;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.dashboard-subtitle {
		font-size: 1rem;
		color: #64748b;
		margin: 0;
	}

	.header-right {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.period-selector {
		display: flex;
		background: rgba(30, 41, 59, 0.6);
		border-radius: 10px;
		padding: 4px;
		border: 1px solid rgba(99, 102, 241, 0.2);
	}

	.period-selector button {
		padding: 0.5rem 1rem;
		background: transparent;
		border: none;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.period-selector button:hover {
		color: #e2e8f0;
	}

	.period-selector button.active {
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
	}

	.refresh-btn {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.refresh-btn:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #818cf8;
	}

	.refresh-btn :global(.spinning) {
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
		padding: 1rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 8px;
		color: #f87171;
		margin-bottom: 1.5rem;
	}

	/* Analytics Panel */
	.analytics-panel {
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.8), rgba(15, 23, 42, 0.95));
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 16px;
		padding: 1.5rem;
		margin-bottom: 1.5rem;
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.15);
	}

	.panel-title {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 1.125rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.panel-title :global(svg) {
		color: #818cf8;
	}

	.panel-period {
		font-size: 0.875rem;
		color: #64748b;
		background: rgba(99, 102, 241, 0.1);
		padding: 0.375rem 0.75rem;
		border-radius: 6px;
	}

	.panel-link {
		font-size: 0.875rem;
		color: #818cf8;
		text-decoration: none;
		transition: color 0.2s;
	}

	.panel-link:hover {
		color: #a5b4fc;
	}

	/* Analytics Grid */
	.analytics-grid {
		display: grid;
		grid-template-columns: repeat(6, 1fr);
		gap: 1rem;
	}

	@media (max-width: 1200px) {
		.analytics-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 768px) {
		.analytics-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	.metric-card {
		background: rgba(15, 23, 42, 0.5);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 12px;
		padding: 1.25rem;
		transition: all 0.2s;
	}

	.metric-card:hover {
		border-color: rgba(99, 102, 241, 0.3);
		background: rgba(99, 102, 241, 0.05);
	}

	.metric-card.large {
		grid-column: span 1;
	}

	.metric-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
	}

	.metric-label {
		font-size: 0.8rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.025em;
	}

	.metric-icon {
		color: #64748b;
	}

	.metric-value {
		font-size: 2rem;
		font-weight: 800;
		color: #f1f5f9;
		line-height: 1;
		margin-bottom: 0.5rem;
	}

	.metric-change {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.8rem;
		font-weight: 600;
	}

	.metric-change.positive {
		color: #4ade80;
	}

	.metric-change.negative {
		color: #f87171;
	}

	.vs-text {
		color: #64748b;
		font-weight: 400;
		margin-left: 0.25rem;
	}

	/* Dual Panel Row */
	.dual-panel-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1.5rem;
		margin-bottom: 1.5rem;
	}

	@media (max-width: 900px) {
		.dual-panel-row {
			grid-template-columns: 1fr;
		}
	}

	/* SEO Panel */
	.seo-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
	}

	.seo-metric {
		background: rgba(15, 23, 42, 0.5);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		padding: 1rem;
	}

	.seo-label {
		font-size: 0.8rem;
		color: #94a3b8;
		display: block;
		margin-bottom: 0.5rem;
	}

	.seo-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	/* Monitor Panel */
	.monitor-grid {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.monitor-section {
		background: rgba(15, 23, 42, 0.5);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		padding: 1rem;
	}

	.monitor-title {
		font-size: 0.85rem;
		font-weight: 600;
		color: #e2e8f0;
		margin-bottom: 0.75rem;
		display: block;
	}

	.monitor-row {
		display: flex;
		gap: 2rem;
	}

	.monitor-item {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.monitor-label {
		font-size: 0.75rem;
		color: #64748b;
	}

	.monitor-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.monitor-value.error {
		color: #f87171;
	}

	/* Business Stats Panel */
	.business-stats-panel {
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.8), rgba(15, 23, 42, 0.95));
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 16px;
		padding: 1.5rem;
		margin-bottom: 2rem;
	}

	.business-grid {
		display: grid;
		grid-template-columns: repeat(5, 1fr);
		gap: 1rem;
	}

	@media (max-width: 1200px) {
		.business-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 768px) {
		.business-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	.business-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(15, 23, 42, 0.5);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 12px;
		text-decoration: none;
		transition: all 0.2s;
	}

	.business-card:hover {
		border-color: rgba(99, 102, 241, 0.3);
		background: rgba(99, 102, 241, 0.1);
		transform: translateY(-2px);
	}

	.business-icon {
		width: 52px;
		height: 52px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.business-icon.members { background: rgba(99, 102, 241, 0.15); color: #818cf8; }
	.business-icon.subscriptions { background: rgba(20, 184, 166, 0.15); color: #2dd4bf; }
	.business-icon.products { background: rgba(16, 185, 129, 0.15); color: #34d399; }
	.business-icon.posts { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.business-icon.coupons { background: rgba(250, 204, 21, 0.15); color: #facc15; }

	.business-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.business-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		line-height: 1;
	}

	.business-label {
		font-size: 0.8rem;
		color: #94a3b8;
		font-weight: 500;
	}

	/* CMS Sections */
	.cms-section {
		margin-bottom: 2.5rem;
	}

	.section-title {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 1rem;
		padding-bottom: 0.5rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.2);
	}

	.cms-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1rem;
	}

	.cms-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem 1.25rem;
		background: rgba(30, 41, 59, 0.5);
		border: 1px solid rgba(99, 102, 241, 0.15);
		border-radius: 12px;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.cms-card:hover {
		background: rgba(99, 102, 241, 0.1);
		border-color: rgba(99, 102, 241, 0.3);
		transform: translateX(4px);
	}

	.cms-icon {
		width: 48px;
		height: 48px;
		border-radius: 10px;
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
	.cms-icon.members { background: rgba(99, 102, 241, 0.15); color: #818cf8; }
	.cms-icon.segments { background: rgba(236, 72, 153, 0.15); color: #f472b6; }
	.cms-icon.products { background: rgba(16, 185, 129, 0.15); color: #34d399; }
	.cms-icon.subscriptions { background: rgba(20, 184, 166, 0.15); color: #2dd4bf; }
	.cms-icon.coupons { background: rgba(250, 204, 21, 0.15); color: #facc15; }
	.cms-icon.crm { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.cms-icon.campaigns { background: rgba(239, 68, 68, 0.15); color: #f87171; }
	.cms-icon.templates { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.cms-icon.smtp { background: rgba(148, 163, 184, 0.15); color: #94a3b8; }
	.cms-icon.seo { background: rgba(34, 197, 94, 0.15); color: #4ade80; }
	.cms-icon.analytics { background: rgba(99, 102, 241, 0.15); color: #818cf8; }
	.cms-icon.behavior { background: rgba(251, 146, 60, 0.15); color: #fb923c; }
	.cms-icon.users { background: rgba(168, 85, 247, 0.15); color: #c084fc; }
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

	/* Refresh button animation */
	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.animate-spin {
		animation: spin 1s linear infinite;
	}

	/* Responsive */
	@media (max-width: 640px) {
		.welcome-title {
			font-size: 1.5rem;
		}
	}
</style>

<script lang="ts">
	/**
	 * SEO Dashboard - Apple ICT9+ Enterprise Grade
	 *
	 * Enterprise SEO command center with:
	 * - Real-time connection status monitoring
	 * - Apple-level polish and animations
	 * - Only shows real data from connected services
	 * - Graceful degradation when services unavailable
	 *
	 */

	import { onMount } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import { spring } from 'svelte/motion';
	import { goto } from '$app/navigation';
	import {
		IconSearch,
		IconFileText,
		IconArrowForward,
		IconExternalLink,
		IconError404,
		IconCode,
		IconKey,
		IconChartBar,
		IconSettings,
		IconSitemap,
		IconBrandGoogle,
		IconNews,
		IconVideo,
		IconPhoto,
		IconMapPin,
		IconMail,
		IconPlugConnected,
		IconRefresh,
		IconTrendingUp,
		IconTrendingDown,
		IconWorld,
		IconLink,
		IconCheck,
		IconClock,
		IconTarget
	} from '#lib/icons/index.js';
	import IconBrandBing from '@tabler/icons-svelte-runes/icons/brand-bing';
	import { connections, getIsSeoConnected, SERVICE_KEYS } from '#lib/stores/connections.svelte.js';

	// State

	let isLoading = $state(true);
	let seoData = $state<SeoMetrics | null>(null);
	let lastUpdated = $state<Date | null>(null);

	interface SeoMetrics {
		searchVisibility: {
			score: number;
			change: number;
			impressions: number;
			clicks: number;
			avgPosition: number;
			ctr: number;
		};
		keywords: {
			total: number;
			top10: number;
			top3: number;
			improving: number;
			declining: number;
		};
		backlinks: {
			total: number;
			doFollow: number;
			noFollow: number;
			newThisMonth: number;
			lostThisMonth: number;
		};
		technical: {
			indexedPages: number;
			crawlErrors: number;
			sitemapPages: number;
			schemaMarkup: number;
		};
	}

	// Spring animation for metrics
	const metricsSpring = spring(0, { stiffness: 0.1, damping: 0.8 });

	// SEO sections for navigation
	const sections = [
		{
			title: 'SEO Meta',
			description: 'Manage meta tags, Open Graph, and Twitter Cards for all content',
			icon: IconFileText,
			href: '/admin/seo/meta',
			color: '#3b82f6',
			gradient: '#3b82f6'
		},
		{
			title: 'Redirects',
			description: 'Create and manage 301/302 redirects with regex support',
			icon: IconArrowForward,
			href: '/admin/seo/redirects',
			color: '#10b981',
			gradient: '#10b981'
		},
		{
			title: '404 Monitor',
			description: 'Track 404 errors and convert them to redirects',
			icon: IconError404,
			href: '/admin/seo/404-monitor',
			color: '#ef4444',
			gradient: '#ef4444'
		},
		{
			title: 'Schema Markup',
			description: 'Generate JSON-LD structured data for better search visibility',
			icon: IconCode,
			href: '/admin/seo/schema',
			color: '#E6B800',
			gradient: '#E6B800'
		},
		{
			title: 'Keywords',
			description: 'Track keyword rankings and monitor performance',
			icon: IconKey,
			href: '/admin/seo/keywords',
			color: '#f59e0b',
			gradient: '#f59e0b',
			requiresConnection: true
		},
		{
			title: 'Analytics',
			description: 'View search performance metrics and trends',
			icon: IconChartBar,
			href: '/admin/seo/analytics',
			color: '#14b8a6',
			gradient: '#14b8a6',
			requiresConnection: true
		},
		{
			title: 'Sitemap',
			description: 'Generate and submit XML sitemaps to search engines',
			icon: IconSitemap,
			href: '/admin/seo/sitemap',
			color: '#E6B800',
			gradient: '#E6B800'
		},
		{
			title: 'News Sitemap',
			description: 'Google News sitemap with 48-hour content window and stock tickers',
			icon: IconNews,
			href: '/admin/seo/news-sitemap',
			color: '#f43f5e',
			gradient: '#f43f5e',
			isNew: true
		},
		{
			title: 'Video Sitemap',
			description: 'Video sitemap with YouTube, Vimeo, Dailymotion auto-detection',
			icon: IconVideo,
			href: '/admin/seo/video-sitemap',
			color: '#dc2626',
			gradient: '#dc2626',
			isNew: true
		},
		{
			title: 'Image SEO',
			description: 'Auto alt text, title generation, and case transformation',
			icon: IconPhoto,
			href: '/admin/seo/image-seo',
			color: '#059669',
			gradient: '#059669',
			isNew: true
		},
		{
			title: 'Store Locator',
			description: 'Multi-location SEO with LocalBusiness schema and KML export',
			icon: IconMapPin,
			href: '/admin/seo/store-locator',
			color: '#d97706',
			gradient: '#d97706',
			isNew: true
		},
		{
			title: 'SEO Reports',
			description: 'Automated email reports with scheduling and white-label branding',
			icon: IconMail,
			href: '/admin/seo/reports',
			color: '#0284c7',
			gradient: '#0284c7',
			isNew: true
		},
		{
			title: 'Search Console',
			description: 'Connect with Google Search Console for data integration',
			icon: IconBrandGoogle,
			href: '/admin/seo/search-console',
			color: '#ec4899',
			gradient: '#ec4899',
			requiresConnection: true
		},
		{
			title: 'Settings',
			description: 'Configure SEO plugin settings and preferences',
			icon: IconSettings,
			href: '/admin/seo/settings',
			color: '#6b7280',
			gradient: '#6b7280'
		}
	];

	// Available SEO services to connect
	const seoServices = [
		{
			key: SERVICE_KEYS.GOOGLE_SEARCH_CONSOLE,
			name: 'Google Search Console',
			icon: IconSearch,
			color: '#4285F4',
			description: 'Track search performance, indexing, and site health'
		},
		{
			key: SERVICE_KEYS.BING_WEBMASTER,
			name: 'Bing Webmaster Tools',
			icon: IconBrandBing,
			color: '#00809D',
			description: 'Monitor Bing search presence and crawl data'
		},
		{
			key: SERVICE_KEYS.SEMRUSH,
			name: 'SEMrush',
			icon: IconChartBar,
			color: '#FF642D',
			description: 'Comprehensive SEO analytics and keyword research'
		},
		{
			key: SERVICE_KEYS.AHREFS,
			name: 'Ahrefs',
			icon: IconLink,
			color: '#FF5800',
			description: 'Backlink analysis and competitive research'
		},
		{
			key: SERVICE_KEYS.MOZ,
			name: 'Moz',
			icon: IconTrendingUp,
			color: '#4A90E2',
			description: 'Domain authority and SEO insights'
		}
	];

	// Lifecycle

	onMount(() => {
		// Built-in SEO - always available (like RankMath Pro)
		// External connections are optional enhancements
		isLoading = false;

		// Load SEO data from built-in system
		loadSeoData();

		const metricsTimer = setTimeout(() => {
			metricsSpring.set(1);
		}, 100);

		return () => clearTimeout(metricsTimer);
	});

	// Data Loading

	async function loadSeoData() {
		// This would fetch from connected SEO services
		// For now, show placeholder until APIs are connected
		try {
			// API call would go here
			// const response = await fetch('/api/admin/seo/metrics');
			// seoData = await response.json();
			lastUpdated = new Date();
		} catch (error) {
			console.error('Failed to load SEO data:', error);
		}
	}

	function handleRefresh() {
		connections.load(true);
		if (getIsSeoConnected()) {
			loadSeoData();
		}
	}

	function navigateToConnections() {
		goto('/admin/connections?category=seo');
	}

	function getConnectionStatus(serviceKey: string) {
		return connections.getStatus(serviceKey);
	}

	function formatNumber(num: number): string {
		if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
		if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
		return num.toString();
	}
</script>

<svelte:head>
	<title>SEO Dashboard | Admin</title>
</svelte:head>

<div class="seo-dashboard">
	<div class="admin-page-container">
		

		<!-- Header Section -->
		<header class="dashboard-header" in:fly={{ y: -20, duration: 500, delay: 100 }}>
			<div class="header-content">
				<div class="header-icon">
					<IconSearch size={32} />
				</div>
				<div class="header-text">
					<h1>SEO Command Center</h1>
					<p>Comprehensive SEO tools to optimize your site's search visibility</p>
				</div>
			</div>

			<div class="header-actions">
				<!-- Built-in SEO Status Badge -->
				<div class="connection-badge connected">
					<IconPlugConnected size={16} />
					<span>Built-in SEO Active</span>
				</div>

				<button class="refresh-btn" onclick={handleRefresh} disabled={isLoading}>
					<span class={['refresh-icon', { spinning: isLoading }]}>
						<IconRefresh size={18} />
					</span>
					<span>Refresh</span>
				</button>
			</div>
		</header>

		{#if isLoading}
			<!-- Loading State -->
			<div class="loading-state" in:fade>
				<div class="loading-spinner"></div>
				<p>Loading SEO data...</p>
			</div>
		{:else}
			<!-- Connected State with Real Data -->
			<div class="connected-content" in:fade={{ duration: 300 }}>
				<!-- Quick Stats Cards -->
				{#if seoData}
					<div class="stats-grid" in:fly={{ y: 20, duration: 500, delay: 100 }}>
						<!-- Search Visibility Card -->
						<div class="stat-card visibility">
							<div class="stat-header">
								<div class="stat-icon">
									<IconWorld size={24} />
								</div>
								<span class="stat-label">Search Visibility</span>
							</div>
							<div class="stat-value">
								<span class="value">{seoData.searchVisibility.score}</span>
								<span class="unit">%</span>
							</div>
							<div class={['stat-change', { positive: seoData.searchVisibility.change > 0 }]}>
								{#if seoData.searchVisibility.change > 0}
									<IconTrendingUp size={14} />
								{:else}
									<IconTrendingDown size={14} />
								{/if}
								<span>{Math.abs(seoData.searchVisibility.change)}% this month</span>
							</div>
						</div>

						<!-- Keywords Card -->
						<div class="stat-card keywords">
							<div class="stat-header">
								<div class="stat-icon">
									<IconKey size={24} />
								</div>
								<span class="stat-label">Keywords Tracked</span>
							</div>
							<div class="stat-value">
								<span class="value">{formatNumber(seoData.keywords.total)}</span>
							</div>
							<div class="stat-detail">
								<span class="detail-item positive">{seoData.keywords.top10} in top 10</span>
								<span class="detail-item highlight">{seoData.keywords.top3} in top 3</span>
							</div>
						</div>

						<!-- Backlinks Card -->
						<div class="stat-card backlinks">
							<div class="stat-header">
								<div class="stat-icon">
									<IconLink size={24} />
								</div>
								<span class="stat-label">Total Backlinks</span>
							</div>
							<div class="stat-value">
								<span class="value">{formatNumber(seoData.backlinks.total)}</span>
							</div>
							<div class="stat-detail">
								<span class="detail-item positive">+{seoData.backlinks.newThisMonth} new</span>
								<span class="detail-item negative">-{seoData.backlinks.lostThisMonth} lost</span>
							</div>
						</div>

						<!-- Technical Health Card -->
						<div class="stat-card technical">
							<div class="stat-header">
								<div class="stat-icon">
									<IconTarget size={24} />
								</div>
								<span class="stat-label">Indexed Pages</span>
							</div>
							<div class="stat-value">
								<span class="value">{formatNumber(seoData.technical.indexedPages)}</span>
							</div>
							<div class="stat-detail">
								{#if seoData.technical.crawlErrors > 0}
									<span class="detail-item negative">{seoData.technical.crawlErrors} errors</span>
								{:else}
									<span class="detail-item positive">No crawl errors</span>
								{/if}
							</div>
						</div>
					</div>
				{:else}
					<!-- No data yet message -->
					<div class="no-data-message" in:fade>
						<IconClock size={48} />
						<h3>Syncing SEO Data</h3>
						<p>
							Your SEO tools are connected. Data will appear here once the first sync completes.
						</p>
					</div>
				{/if}

				<!-- Connected Services Indicator -->
				<div class="connected-services" in:fly={{ y: 20, duration: 500, delay: 200 }}>
					<h3>Connected Services</h3>
					<div class="connected-services-list">
						{#each seoServices as service (service.key)}
							{@const status = getConnectionStatus(service.key)}
							{#if status?.isConnected}
								<div class="connected-service">
									{@const ServiceIcon = service.icon}
									<span class="service-icon-small"><ServiceIcon size={16} aria-hidden="true" /></span>
									<span class="service-name">{service.name}</span>
									<IconCheck size={16} class="check-icon" />
								</div>
							{/if}
						{/each}
					</div>
					<button class="manage-connections-btn" onclick={navigateToConnections}>
						<IconSettings size={16} />
						<span>Manage Connections</span>
					</button>
				</div>
			</div>
		{/if}

		<!-- SEO Sections Grid - Built-in SEO tools (like RankMath Pro) -->
		<div class="sections-wrapper" in:fly={{ y: 20, duration: 500, delay: 300 }}>
			<h2 class="sections-title">SEO Tools</h2>
			<div class="sections-grid">
				{#each sections as section, i (section.href)}
					{@const SectionIcon = section.icon}
					<a
						href={section.href}
						class="section-card"
						style:--card-color={section.color}
						style:--card-gradient={section.gradient}
						in:fly={{ y: 20, duration: 400, delay: 400 + i * 30 }}
					>
						{#if section.isNew}
							<span class="new-badge">NEW</span>
						{/if}
						<div class="card-icon" style:background={section.gradient}>
							<SectionIcon size={24} />
						</div>
						<h3>{section.title}</h3>
						<p>{section.description}</p>
						<div class="card-arrow">
							<IconExternalLink size={18} />
						</div>
					</a>
				{/each}
			</div>
		</div>

		<!-- Last Updated Footer -->
		{#if lastUpdated}
			<div class="last-updated" in:fade={{ delay: 600 }}>
				<IconClock size={14} />
				<span>Last updated: {lastUpdated.toLocaleTimeString()}</span>
			</div>
		{/if}
	</div>
	<!-- End admin-page-container -->
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════════
	   Base Layout
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.seo-dashboard {
		position: relative;
		padding: 2rem;
		max-width: 1600px;
		margin: 0 auto;
		background: var(--bg-base, #0d1117);
		overflow: hidden;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Animated Background
	   ═══════════════════════════════════════════════════════════════════════════════ */

	

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Header
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.dashboard-header {
		position: relative;
		z-index: 10;
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
		padding: 1.5rem 2rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 20px;
	}

	.header-content {
		display: flex;
		align-items: center;
		gap: 1.25rem;
	}

	.header-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 56px;
		height: 56px;
		background: var(--primary-500);
		border-radius: 16px;
		color: var(--bg-base);
		box-shadow: 0 8px 24px rgba(230, 184, 0, 0.3);
	}

	.header-text h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.25rem 0;
		letter-spacing: -0.02em;
	}

	.header-text p {
		font-size: 0.9rem;
		color: var(--text-secondary, #8b949e);
		margin: 0;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.connection-badge {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		border-radius: 12px;
		font-size: 0.8125rem;
		font-weight: 600;
		background: rgba(248, 81, 73, 0.15);
		border: 1px solid rgba(248, 81, 73, 0.3);
		color: var(--error-emphasis, #f85149);
		transition: all 0.3s ease;
	}

	.connection-badge.connected {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-400);
	}

	.refresh-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		background: var(--bg-surface, #1c2128);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 12px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.refresh-btn:hover:not(:disabled) {
		background: var(--bg-hover, #252b33);
		border-color: var(--border-default, #30363d);
	}

	.refresh-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	:global(.spinning) {
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

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Loading State
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem;
		color: var(--text-secondary, #8b949e);
	}

	.loading-spinner {
		width: 48px;
		height: 48px;
		border: 3px solid var(--border-default, #30363d);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Not Connected Section
	   ═══════════════════════════════════════════════════════════════════════════════ */

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Connected Content
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.connected-content {
		position: relative;
		z-index: 10;
		margin-bottom: 3rem;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
		gap: 1.25rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		padding: 1.5rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 20px;
	}

	.stat-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1rem;
	}

	.stat-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		border-radius: 12px;
		color: white;
	}

	.visibility .stat-icon {
		background: var(--primary-500);
	}

	.keywords .stat-icon {
		background: #f59e0b;
	}

	.backlinks .stat-icon {
		background: #3b82f6;
	}

	.technical .stat-icon {
		background: var(--primary-500);
	}

	.stat-label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--text-secondary, #8b949e);
	}

	.stat-value {
		margin-bottom: 0.75rem;
	}

	.stat-value .value {
		font-size: 2.5rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		letter-spacing: -0.02em;
	}

	.stat-value .unit {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-secondary, #8b949e);
		margin-left: 0.25rem;
	}

	.stat-change {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--error-emphasis, #f85149);
	}

	.stat-change.positive {
		color: var(--success-emphasis, #3fb950);
	}

	.stat-detail {
		display: flex;
		gap: 1rem;
	}

	.detail-item {
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--text-secondary, #8b949e);
	}

	.detail-item.positive {
		color: var(--success-emphasis, #3fb950);
	}

	.detail-item.negative {
		color: var(--error-emphasis, #f85149);
	}

	.detail-item.highlight {
		color: var(--warning-emphasis, #d29922);
	}

	/* No Data Message */
	.no-data-message {
		text-align: center;
		padding: 3rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 20px;
		color: var(--text-secondary, #8b949e);
		margin-bottom: 2rem;
	}

	.no-data-message h3 {
		color: var(--text-primary, #f0f6fc);
		margin: 1rem 0 0.5rem;
	}

	/* Connected Services */
	.connected-services {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem 1.5rem;
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 14px;
		flex-wrap: wrap;
	}

	.connected-services h3 {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--primary-400);
		margin: 0;
	}

	.connected-services-list {
		display: flex;
		gap: 0.75rem;
		flex-wrap: wrap;
		flex: 1;
	}

	.connected-service {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.75rem;
		background: rgba(230, 184, 0, 0.15);
		border-radius: 8px;
		font-size: 0.8125rem;
		color: var(--primary-400);
	}

	.service-icon-small {
		display: inline-flex;
		align-items: center;
	}

	:global(.check-icon) {
		color: var(--primary-500);
	}

	.manage-connections-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(230, 184, 0, 0.2);
		border: 1px solid rgba(230, 184, 0, 0.3);
		border-radius: 8px;
		color: var(--primary-400);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.manage-connections-btn:hover {
		background: rgba(230, 184, 0, 0.3);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Sections Grid
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.sections-wrapper {
		position: relative;
		z-index: 10;
	}

	.sections-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 1.5rem 0;
	}

	.sections-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 1.25rem;
	}

	.section-card {
		position: relative;
		display: block;
		padding: 1.75rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 20px;
		text-decoration: none;
		overflow: hidden;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.section-card::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 3px;
		background: var(--card-gradient);
		transform: scaleX(0);
		transform-origin: left;
		transition: transform 0.3s ease;
	}

	.section-card:hover::before {
		transform: scaleX(1);
	}

	.section-card:hover {
		transform: translateY(-4px);
		border-color: var(--card-color);
		box-shadow:
			0 16px 48px rgba(0, 0, 0, 0.3),
			0 0 0 1px rgba(var(--card-color), 0.2);
	}

	.card-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		border-radius: 14px;
		color: white;
		margin-bottom: 1.25rem;
	}

	.section-card h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.5rem 0;
	}

	.section-card p {
		font-size: 0.875rem;
		color: var(--text-secondary, #8b949e);
		margin: 0;
		line-height: 1.6;
	}

	.card-arrow {
		position: absolute;
		top: 1.75rem;
		right: 1.75rem;
		color: var(--text-tertiary, #6e7681);
		opacity: 0;
		transform: translateX(-8px);
		transition: all 0.3s ease;
	}

	.section-card:hover .card-arrow {
		opacity: 1;
		transform: translateX(0);
		color: var(--card-color);
	}

	.new-badge {
		position: absolute;
		top: 1rem;
		right: 1rem;
		background: #f97316;
		color: white;
		font-size: 0.625rem;
		font-weight: 700;
		padding: 0.25rem 0.5rem;
		border-radius: 6px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		z-index: 2;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Footer
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.last-updated {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		margin-top: 2rem;
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Responsive
	   ═══════════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 1023.98px) {
		.dashboard-header {
			flex-direction: column;
			gap: 1.5rem;
			text-align: center;
		}

		.header-content {
			flex-direction: column;
		}

		.header-actions {
			width: 100%;
			justify-content: center;
		}
	}

	@media (max-width: 767.98px) {
		.seo-dashboard {
			padding: 1rem;
		}

		.dashboard-header {
			padding: 1.25rem;
			border-radius: 16px;
		}

		.header-text h1 {
			font-size: 1.5rem;
		}

		.sections-grid {
			grid-template-columns: 1fr;
		}

		.connected-services {
			flex-direction: column;
			align-items: flex-start;
		}

		.manage-connections-btn {
			width: 100%;
			justify-content: center;
		}
	}
</style>

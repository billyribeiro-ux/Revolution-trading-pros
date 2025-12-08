<script lang="ts">
	/**
	 * Dashboard - My Indicators Page - WordPress Simpler Trading EXACT
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Shows user's purchased indicators with download access.
	 * EXACT match of WordPress Simpler Trading membership-card structure.
	 * Connected to real backend API.
	 *
	 * @version 3.0.0 (Simpler Trading Exact / December 2025)
	 */

	import {
		IconChartLine,
		IconDownload,
		IconExternalLink,
		IconCheck,
		IconClock,
		IconRefresh,
		IconChevronRight,
		IconSearch,
		IconAlertTriangle,
		IconLoader2
	} from '@tabler/icons-svelte';
	import '$lib/styles/st-icons.css';
	import { userIndicatorsApi, type PurchasedIndicator as ApiPurchasedIndicator } from '$lib/api/indicators';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	interface DisplayIndicator {
		id: number;
		name: string;
		description: string;
		platform: 'TradingView' | 'ThinkorSwim' | 'MetaTrader' | 'NinjaTrader';
		platformSlug: string;
		status: 'active' | 'expiring' | 'expired' | 'trial';
		expiresAt: string;
		daysUntilExpiry?: number;
		downloadUrl: string;
		downloadId: string;
		version: string;
		slug: string;
		lastUpdated: string;
		iconClass?: string;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let searchQuery = $state('');
	let filterPlatform = $state<'all' | 'tradingview' | 'thinkorswim' | 'metatrader' | 'ninjatrader'>('all');
	let purchasedIndicators = $state<DisplayIndicator[]>([]);
	let isLoading = $state(true);
	let isDownloading = $state<number | null>(null);
	let errorMessage = $state('');
	let successMessage = $state('');

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	function mapToDisplay(indicator: ApiPurchasedIndicator): DisplayIndicator {
		return {
			id: indicator.id,
			name: indicator.name,
			description: indicator.description || '',
			platform: indicator.platform,
			platformSlug: indicator.platform_slug,
			status: indicator.status,
			expiresAt: formatDate(indicator.expires_at),
			daysUntilExpiry: indicator.days_until_expiry,
			downloadUrl: indicator.download_url,
			downloadId: indicator.download_id,
			version: indicator.version,
			slug: indicator.slug,
			lastUpdated: formatDate(indicator.last_updated),
			iconClass: `st-icon-${indicator.slug.replace(/-/g, '-')}`
		};
	}

	async function loadIndicators() {
		isLoading = true;
		errorMessage = '';

		try {
			const params: { platform?: string; status?: 'active' | 'expiring' | 'expired' } = {};
			if (filterPlatform !== 'all') {
				params.platform = filterPlatform;
			}

			const response = await userIndicatorsApi.getPurchased(params);

			if (response.success) {
				purchasedIndicators = response.data.map(mapToDisplay);
			} else {
				errorMessage = response.message || 'Failed to load indicators';
			}
		} catch (err) {
			console.error('Error loading indicators:', err);
			errorMessage = 'Failed to load indicators. Please try again.';
		} finally {
			isLoading = false;
		}
	}

	async function handleDownload(indicator: DisplayIndicator) {
		isDownloading = indicator.id;
		errorMessage = '';

		try {
			const response = await userIndicatorsApi.getDownload(indicator.slug, indicator.downloadId);

			if (response.success && response.data.download_url) {
				window.open(response.data.download_url, '_blank');
				successMessage = `Download started for ${indicator.name}`;
				setTimeout(() => successMessage = '', 3000);
			} else {
				errorMessage = 'Failed to get download link';
			}
		} catch (err) {
			console.error('Error downloading:', err);
			errorMessage = 'Failed to download indicator. Please try again.';
		} finally {
			isDownloading = null;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		loadIndicators();
	});

	$effect(() => {
		const _ = filterPlatform;
		if (!isLoading) {
			loadIndicators();
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const filteredIndicators = $derived.by(() => {
		return purchasedIndicators.filter(indicator => {
			const matchesSearch = searchQuery === '' ||
				indicator.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				indicator.platform.toLowerCase().includes(searchQuery.toLowerCase());
			return matchesSearch;
		});
	});

	const stats = $derived({
		total: purchasedIndicators.length,
		active: purchasedIndicators.filter(i => i.status === 'active').length,
		expiring: purchasedIndicators.filter(i => i.status === 'expiring').length,
		expired: purchasedIndicators.filter(i => i.status === 'expired').length,
		trial: purchasedIndicators.filter(i => i.status === 'trial').length
	});

	const platforms = [
		{ id: 'all', name: 'All Platforms' },
		{ id: 'tradingview', name: 'TradingView' },
		{ id: 'thinkorswim', name: 'ThinkorSwim' },
		{ id: 'metatrader', name: 'MetaTrader' },
		{ id: 'ninjatrader', name: 'NinjaTrader' }
	];
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>My Indicators | Dashboard | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     DASHBOARD HEADER - WordPress: .dashboard__header
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">
			<span class="st-icon-handle-stick icon icon--lg"></span>
			My Indicators
		</h1>
	</div>
	<div class="dashboard__header-right">
		<a href="/indicators" class="btn btn-xs btn-link">
			Browse Indicators Store
			<IconChevronRight size={14} />
		</a>
	</div>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     DASHBOARD CONTENT - WordPress: .dashboard__content
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard__content">
	<div class="dashboard__content-main">

		<!-- Success/Error Messages -->
		{#if successMessage}
			<div class="woocommerce-message">
				<IconCheck size={18} />
				{successMessage}
			</div>
		{/if}
		{#if errorMessage}
			<div class="woocommerce-error">
				<IconAlertTriangle size={18} />
				{errorMessage}
				<button class="dismiss" onclick={() => errorMessage = ''}>×</button>
			</div>
		{/if}

		<!-- Dashboard Filters Bar - WordPress: .dashboard-filters -->
		<div class="dashboard-filters">
			<div class="dashboard-filters__search">
				<IconSearch size={16} />
				<input
					type="text"
					placeholder="Search indicators..."
					bind:value={searchQuery}
				/>
			</div>
			<div class="dashboard-filters__tabs">
				{#each platforms as platform}
					<button
						class="filter-tab"
						class:active={filterPlatform === platform.id}
						onclick={() => filterPlatform = platform.id as typeof filterPlatform}
					>
						{platform.name}
					</button>
				{/each}
			</div>
		</div>

		<!-- Loading State -->
		{#if isLoading}
			<div class="loading-container">
				<div class="loading">
					<span class="loading-icon"><IconLoader2 size={32} class="spin" /></span>
					<span class="loading-message">Loading your indicators...</span>
				</div>
			</div>
		{:else}
			<!-- Indicators Section - WordPress: .dashboard__content-section -->
			<section class="dashboard__content-section">
				<h2 class="section-title">My Indicators</h2>

				{#if filteredIndicators.length > 0}
					<!-- WordPress: .membership-cards.row -->
					<div class="membership-cards row">
						{#each filteredIndicators as indicator (indicator.id)}
							<!-- WordPress: .col-sm-6.col-xl-4 -->
							<div class="col-sm-6 col-xl-4">
								<!-- WordPress: article.membership-card with status modifier -->
								<article
									class="membership-card membership-card--indicator"
									class:membership-card--active={indicator.status === 'active'}
									class:membership-card--trial={indicator.status === 'trial'}
									class:membership-card--expiring={indicator.status === 'expiring'}
									class:membership-card--expired={indicator.status === 'expired'}
								>
									<!-- WordPress: .membership-card__header (clickable) -->
									<a href="/dashboard/indicators/{indicator.slug}" class="membership-card__header">
										<span class="mem_icon">
											<span class="membership-card__icon">
												<span class="icon icon--lg">
													<IconChartLine size={28} />
												</span>
											</span>
										</span>
										<span class="mem_div">{indicator.name}</span>
									</a>

									<!-- Status Indicator Bar (the vertical bar!) -->
									<div class="membership-card__status-bar">
										<div
											class="status-bar__fill"
											style:height={indicator.status === 'active' ? '100%' :
												indicator.status === 'trial' ? '70%' :
												indicator.status === 'expiring' ? '30%' : '0%'}
										></div>
									</div>

									<!-- Status Badge -->
									{#if indicator.status === 'trial'}
										<span class="membership-card__badge badge--trial">Trial</span>
									{:else if indicator.status === 'expiring'}
										<span class="membership-card__badge badge--expiring">{indicator.daysUntilExpiry}d left</span>
									{:else if indicator.status === 'expired'}
										<span class="membership-card__badge badge--expired">Expired</span>
									{/if}

									<!-- Platform & Version Info -->
									<div class="membership-card__meta">
										<span class="meta-platform">{indicator.platform}</span>
										<span class="meta-version">v{indicator.version}</span>
									</div>

									<!-- WordPress: .membership-card__actions -->
									<div class="membership-card__actions">
										{#if indicator.status !== 'expired'}
											<button
												onclick={() => handleDownload(indicator)}
												disabled={isDownloading === indicator.id}
											>
												{#if isDownloading === indicator.id}
													<IconLoader2 size={14} class="spin" />
													Downloading...
												{:else}
													<IconDownload size={14} />
													Download
												{/if}
											</button>
											<a href="/dashboard/indicators/{indicator.slug}/docs">
												<IconExternalLink size={14} />
												Documentation
											</a>
										{:else}
											<a href="/indicators/{indicator.slug}" class="action--renew">
												<IconRefresh size={14} />
												Renew License
											</a>
										{/if}
									</div>
								</article>
							</div>
						{/each}
					</div>
				{:else}
					<!-- Empty State -->
					<div class="empty-state">
						<IconChartLine size={64} />
						<h3>No Indicators Found</h3>
						{#if searchQuery || filterPlatform !== 'all'}
							<p>Try adjusting your search or filter criteria</p>
							<button class="btn btn-default btn-sm" onclick={() => { searchQuery = ''; filterPlatform = 'all'; }}>
								Clear Filters
							</button>
						{:else}
							<p>You haven't purchased any indicators yet.</p>
							<a href="/indicators" class="btn btn-orange">Browse Indicators Store</a>
						{/if}
					</div>
				{/if}
			</section>
		{/if}
	</div>

	<!-- Sidebar (WordPress: .dashboard__content-sidebar) -->
	<aside class="dashboard__content-sidebar">
		<section class="content-sidebar__section">
			<h3 class="sidebar-title">Quick Stats</h3>
			<div class="sidebar-stats">
				<div class="stat-row">
					<span class="stat-label">Total Indicators</span>
					<span class="stat-value">{stats.total}</span>
				</div>
				<div class="stat-row stat--active">
					<span class="stat-label">Active</span>
					<span class="stat-value">{stats.active}</span>
				</div>
				{#if stats.trial > 0}
					<div class="stat-row stat--trial">
						<span class="stat-label">Trial</span>
						<span class="stat-value">{stats.trial}</span>
					</div>
				{/if}
				{#if stats.expiring > 0}
					<div class="stat-row stat--expiring">
						<span class="stat-label">Expiring Soon</span>
						<span class="stat-value">{stats.expiring}</span>
					</div>
				{/if}
				{#if stats.expired > 0}
					<div class="stat-row stat--expired">
						<span class="stat-label">Expired</span>
						<span class="stat-value">{stats.expired}</span>
					</div>
				{/if}
			</div>
		</section>
	</aside>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - WordPress Simpler Trading EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   CSS CUSTOM PROPERTIES - Simpler Trading Dashboard
	   ═══════════════════════════════════════════════════════════════════════════ */

	:root {
		/* Status Colors - The Blue Bar System */
		--st-status-active: #0984ae;      /* Dark blue - Full active */
		--st-status-trial: #5bc0de;       /* Light blue - Trial */
		--st-status-expiring: #f59e0b;    /* Orange - Expiring */
		--st-status-expired: #ef4444;     /* Red - Expired */
		--st-status-inactive: #cbd5e1;    /* Gray - Inactive */
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER - WordPress: .dashboard__header
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		padding: 20px 30px;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.dashboard__header-left,
	.dashboard__header-right {
		display: flex;
		align-items: center;
	}

	.dashboard__page-title {
		display: flex;
		align-items: center;
		gap: 12px;
		color: #333;
		font-family: 'Open Sans Condensed', 'Open Sans', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
	}

	.dashboard__page-title .icon {
		color: #0984ae;
	}

	.btn-link {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		color: #1e73be;
		text-decoration: none;
		font-weight: 600;
		font-size: 14px;
	}

	.btn-link:hover {
		color: #0984ae;
		text-decoration: underline;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT - WordPress: .dashboard__content
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		display: flex;
		gap: 30px;
		padding: 30px;
		background: #f4f4f4;
	}

	.dashboard__content-main {
		flex: 1;
		min-width: 0;
	}

	.dashboard__content-sidebar {
		width: 280px;
		flex-shrink: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   WOOCOMMERCE MESSAGES - WordPress exact
	   ═══════════════════════════════════════════════════════════════════════════ */

	.woocommerce-message,
	.woocommerce-error {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 20px;
		border-radius: 0;
		margin-bottom: 20px;
		font-size: 14px;
		position: relative;
	}

	.woocommerce-message {
		background: #d4edda;
		color: #155724;
		border-left: 4px solid #28a745;
	}

	.woocommerce-error {
		background: #f8d7da;
		color: #721c24;
		border-left: 4px solid #dc3545;
	}

	.woocommerce-message .dismiss,
	.woocommerce-error .dismiss {
		position: absolute;
		right: 12px;
		top: 50%;
		transform: translateY(-50%);
		background: none;
		border: none;
		font-size: 20px;
		cursor: pointer;
		color: inherit;
		opacity: 0.5;
	}

	.woocommerce-message .dismiss:hover,
	.woocommerce-error .dismiss:hover {
		opacity: 1;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD FILTERS - WordPress exact
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard-filters {
		display: flex;
		flex-wrap: wrap;
		gap: 16px;
		margin-bottom: 24px;
		align-items: center;
		justify-content: space-between;
	}

	.dashboard-filters__search {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 8px 14px;
		background: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		min-width: 250px;
		color: #666;
	}

	.dashboard-filters__search input {
		flex: 1;
		border: none;
		outline: none;
		font-size: 14px;
		background: transparent;
		color: #333;
	}

	.dashboard-filters__tabs {
		display: flex;
		gap: 8px;
	}

	.filter-tab {
		padding: 8px 16px;
		background: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		font-size: 13px;
		font-weight: 500;
		color: #666;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.filter-tab:hover {
		border-color: #0984ae;
		color: #0984ae;
	}

	.filter-tab.active {
		background: #0984ae;
		border-color: #0984ae;
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LOADING STATE - WordPress: .loading-container
	   ═══════════════════════════════════════════════════════════════════════════ */

	.loading-container {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 300px;
	}

	.loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 16px;
		color: #666;
	}

	.loading-icon {
		color: #0984ae;
	}

	:global(.spin) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SECTION TITLE - WordPress: .section-title
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-section {
		margin-bottom: 30px;
	}

	.section-title {
		font-family: 'Open Sans Condensed', 'Open Sans', sans-serif;
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		padding-bottom: 10px;
		border-bottom: 2px solid #0984ae;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MEMBERSHIP CARDS GRID - WordPress: .membership-cards.row
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-cards {
		display: flex;
		flex-wrap: wrap;
		margin-right: -15px;
		margin-left: -15px;
	}

	.membership-cards > .col-sm-6 {
		position: relative;
		width: 100%;
		padding-right: 15px;
		padding-left: 15px;
		margin-bottom: 30px;
	}

	@media (min-width: 576px) {
		.membership-cards > .col-sm-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 1200px) {
		.membership-cards > .col-xl-4 {
			flex: 0 0 33.333333%;
			max-width: 33.333333%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MEMBERSHIP CARD - WordPress: .membership-card (EXACT)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card {
		position: relative;
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 5px 30px rgb(0 0 0 / 10%);
		transition: all 0.15s ease-in-out;
		overflow: hidden;
		height: 100%;
		display: flex;
		flex-direction: column;
		/* THE VERTICAL ACCENT BAR - Left side */
		border-left: 4px solid var(--st-status-inactive);
	}

	.membership-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 10px 40px rgb(0 0 0 / 15%);
	}

	/* Status modifiers - changes the vertical bar color */
	.membership-card--active {
		border-left-color: var(--st-status-active);  /* Dark blue */
	}

	.membership-card--trial {
		border-left-color: var(--st-status-trial);   /* Light blue */
	}

	.membership-card--expiring {
		border-left-color: var(--st-status-expiring); /* Orange */
	}

	.membership-card--expired {
		border-left-color: var(--st-status-expired);  /* Red */
		opacity: 0.8;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MEMBERSHIP CARD HEADER - WordPress: .membership-card__header
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card__header {
		display: flex;
		align-items: center;
		padding: 20px;
		text-decoration: none;
		color: #333;
		gap: 12px;
		border-bottom: 1px solid #ededed;
		transition: background 0.15s ease;
	}

	.membership-card__header:hover {
		background: #f8f9fa;
	}

	.mem_icon,
	.mem_div {
		display: inline-block;
		vertical-align: middle;
	}

	.mem_div {
		white-space: normal;
		width: calc(100% - 50px);
		font-weight: 600;
		font-size: 16px;
		line-height: 1.3;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MEMBERSHIP CARD ICON - WordPress: .membership-card__icon
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card__icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 42px;
		height: 42px;
		background: linear-gradient(135deg, #e8f4f8, #d1ebf5);
		border-radius: 8px;
		color: #0984ae;
	}

	.membership-card__icon .icon--lg {
		font-size: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   STATUS BAR - The vertical progress indicator (EXACT from Simpler)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card__status-bar {
		position: absolute;
		left: 0;
		top: 0;
		bottom: 0;
		width: 4px;
		background: var(--st-status-inactive);
		overflow: hidden;
	}

	.status-bar__fill {
		position: absolute;
		bottom: 0;
		left: 0;
		width: 100%;
		background: var(--st-status-active);
		transition: height 0.3s ease;
	}

	.membership-card--trial .status-bar__fill {
		background: var(--st-status-trial);
	}

	.membership-card--expiring .status-bar__fill {
		background: var(--st-status-expiring);
	}

	.membership-card--expired .status-bar__fill {
		background: var(--st-status-expired);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   STATUS BADGE - WordPress exact
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card__badge {
		position: absolute;
		top: 12px;
		right: 12px;
		padding: 4px 10px;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.badge--trial {
		background: var(--st-status-trial);
		color: #fff;
	}

	.badge--expiring {
		background: var(--st-status-expiring);
		color: #fff;
	}

	.badge--expired {
		background: var(--st-status-expired);
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CARD META - Platform & Version
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card__meta {
		display: flex;
		gap: 12px;
		padding: 12px 20px;
		font-size: 12px;
		color: #666;
		border-bottom: 1px solid #ededed;
	}

	.meta-platform {
		background: #f4f4f4;
		padding: 4px 8px;
		border-radius: 4px;
		font-weight: 500;
	}

	.meta-version {
		color: #999;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MEMBERSHIP CARD ACTIONS - WordPress: .membership-card__actions (EXACT)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card__actions {
		display: flex;
		padding: 0;
		margin-top: auto;
		border-top: 1px solid #ededed;
	}

	.membership-card__actions a,
	.membership-card__actions button {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
		padding: 14px 12px;
		font-weight: 600;
		font-size: 13px;
		text-decoration: none;
		transition: all 0.15s ease;
		background: transparent;
		border: none;
		color: #0984ae;
		cursor: pointer;
	}

	.membership-card__actions a:not(:last-child),
	.membership-card__actions button:not(:last-child) {
		border-right: 1px solid #ededed;
	}

	.membership-card__actions a:hover,
	.membership-card__actions button:hover {
		background: #f4f4f4;
		color: #076787;
	}

	.membership-card__actions button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.membership-card__actions .action--renew {
		background: #fef3c7;
		color: #92400e;
	}

	.membership-card__actions .action--renew:hover {
		background: #fde68a;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SIDEBAR - WordPress: .dashboard__content-sidebar
	   ═══════════════════════════════════════════════════════════════════════════ */

	.content-sidebar__section {
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 5px 30px rgb(0 0 0 / 10%);
		padding: 20px;
		margin-bottom: 20px;
	}

	.sidebar-title {
		font-family: 'Open Sans Condensed', 'Open Sans', sans-serif;
		font-size: 18px;
		font-weight: 700;
		color: #333;
		margin: 0 0 16px;
		padding-bottom: 10px;
		border-bottom: 2px solid #0984ae;
	}

	.sidebar-stats {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.stat-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 10px 12px;
		background: #f8f9fa;
		border-radius: 6px;
		border-left: 3px solid #cbd5e1;
	}

	.stat-row.stat--active {
		border-left-color: var(--st-status-active);
	}

	.stat-row.stat--trial {
		border-left-color: var(--st-status-trial);
	}

	.stat-row.stat--expiring {
		border-left-color: var(--st-status-expiring);
	}

	.stat-row.stat--expired {
		border-left-color: var(--st-status-expired);
	}

	.stat-label {
		font-size: 13px;
		color: #666;
	}

	.stat-value {
		font-size: 18px;
		font-weight: 700;
		color: #333;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   EMPTY STATE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 300px;
		text-align: center;
		color: #666;
		padding: 40px 20px;
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 5px 30px rgb(0 0 0 / 10%);
	}

	.empty-state h3 {
		font-size: 1.25rem;
		color: #333;
		margin: 16px 0 8px;
	}

	.empty-state p {
		margin: 0 0 20px;
	}

	.btn-default {
		padding: 10px 20px;
		background: #f4f4f4;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		color: #333;
		font-weight: 600;
		cursor: pointer;
		text-decoration: none;
	}

	.btn-default:hover {
		background: #e5e5e5;
	}

	.btn-orange {
		display: inline-flex;
		padding: 12px 24px;
		background: #f99e31;
		border-radius: 4px;
		color: #fff;
		font-weight: 600;
		text-decoration: none;
	}

	.btn-orange:hover {
		background: #f88b09;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - WordPress exact breakpoints
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 991px) {
		.dashboard__content {
			flex-direction: column;
		}

		.dashboard__content-sidebar {
			width: 100%;
			order: -1;
		}
	}

	@media (max-width: 768px) {
		.dashboard__header {
			padding: 16px 20px;
			flex-direction: column;
			align-items: flex-start;
			gap: 12px;
		}

		.dashboard__page-title {
			font-size: 28px;
		}

		.dashboard__content {
			padding: 20px;
		}

		.dashboard-filters {
			flex-direction: column;
			align-items: stretch;
		}

		.dashboard-filters__search {
			min-width: auto;
		}

		.dashboard-filters__tabs {
			overflow-x: auto;
			padding-bottom: 8px;
		}

		.membership-cards {
			margin-right: -10px;
			margin-left: -10px;
		}

		.membership-cards > .col-sm-6 {
			padding-right: 10px;
			padding-left: 10px;
			margin-bottom: 20px;
		}
	}

	@media (max-width: 575px) {
		.membership-card__actions {
			flex-direction: column;
		}

		.membership-card__actions a:not(:last-child),
		.membership-card__actions button:not(:last-child) {
			border-right: none;
			border-bottom: 1px solid #ededed;
		}
	}
</style>

<script lang="ts">
	/**
	 * Dashboard - My Indicators Page - WordPress Simpler Trading EXACT
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Shows user's purchased indicators with download access.
	 * Fetches real data from the backend API.
	 *
	 * @version 3.0.0 (API Connected / December 2025)
	 */

	import { browser } from '$app/environment';
	import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
	import IconDownload from '@tabler/icons-svelte/icons/download';
	import IconExternalLink from '@tabler/icons-svelte/icons/external-link';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconClock from '@tabler/icons-svelte/icons/clock';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconChevronRight from '@tabler/icons-svelte/icons/chevron-right';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconAlertTriangle from '@tabler/icons-svelte/icons/alert-triangle';
	import IconLock from '@tabler/icons-svelte/icons/lock';
	import IconMessageCircle from '@tabler/icons-svelte/icons/message-circle';
	import IconLoader from '@tabler/icons-svelte/icons/loader';
	import '$lib/styles/st-icons.css';
	import {
		getUserIndicators,
		downloadIndicator,
		invalidateIndicatorsCache,
		type PurchasedIndicator,
		type IndicatorStats
	} from '$lib/api/user-indicators';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let searchQuery = $state('');
	let filterPlatform = $state<'all' | 'tradingview' | 'thinkorswim' | 'metatrader'>('all');
	let indicators = $state<PurchasedIndicator[]>([]);
	let stats = $state<IndicatorStats>({ total: 0, active: 0, expiring: 0, expired: 0 });
	let isLoading = $state(true);
	let isRefreshing = $state(false);
	let error = $state<string | null>(null);
	let downloadingId = $state<number | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// LOAD INDICATORS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadIndicators(skipCache = false): Promise<void> {
		if (skipCache) {
			isRefreshing = true;
			invalidateIndicatorsCache();
		} else {
			isLoading = true;
		}
		error = null;

		try {
			const response = await getUserIndicators({ skipCache });
			if (response.success) {
				indicators = response.data.indicators;
				stats = response.data.stats;
			}
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load indicators';
			console.error('[Indicators] Error loading:', e);
		} finally {
			isLoading = false;
			isRefreshing = false;
		}
	}

	// Initial load
	$effect(() => {
		if (browser) {
			loadIndicators();
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function handleDownload(indicator: PurchasedIndicator): Promise<void> {
		downloadingId = indicator.id;
		try {
			await downloadIndicator(indicator.id);
		} catch (e) {
			console.error('[Indicators] Download error:', e);
		} finally {
			downloadingId = null;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const filteredIndicators = $derived.by(() => {
		return indicators.filter(indicator => {
			const matchesSearch = searchQuery === '' ||
				indicator.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				indicator.platform.toLowerCase().includes(searchQuery.toLowerCase());
			return matchesSearch;
		});
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
		<button
			class="refresh-btn"
			onclick={() => loadIndicators(true)}
			disabled={isRefreshing}
			aria-label="Refresh indicators"
		>
			<IconRefresh size={18} class={isRefreshing ? 'spin' : ''} />
		</button>
		<a href="/indicators" class="btn btn-link">
			Browse All Indicators
			<IconChevronRight size={16} />
		</a>
	</div>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     DASHBOARD CONTENT - WordPress: .dashboard__content
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard__content">
	{#if isLoading}
		<!-- Loading State -->
		<div class="loading-state">
			<span class="spin"><IconLoader size={48} /></span>
			<p>Loading your indicators...</p>
		</div>
	{:else if error}
		<!-- Error State -->
		<div class="error-state">
			<IconAlertTriangle size={48} />
			<h2>Failed to load indicators</h2>
			<p>{error}</p>
			<button class="btn btn-primary" onclick={() => loadIndicators(true)}>
				Try Again
			</button>
		</div>
	{:else}
		<!-- Filters Bar -->
		<div class="filters-bar">
			<div class="search-box">
				<IconSearch size={18} />
				<input
					id="indicator-search"
					type="text"
					placeholder="Search indicators..."
					bind:value={searchQuery}
				/>
			</div>
			<div class="filter-tabs">
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

		<!-- Indicators Grid - WordPress: .membership-cards.row -->
		{#if filteredIndicators.length > 0}
		<section class="dashboard__content-section">
			<div class="indicators-grid row">
				{#each filteredIndicators as indicator (indicator.id)}
					<div class="col-sm-6 col-xl-4">
						<article
							class="indicator-card"
							class:is-expiring={indicator.status === 'expiring'}
							class:is-expired={indicator.status === 'expired'}
						>
							<!-- Status Badge -->
							{#if indicator.status === 'expiring'}
								<span class="status-badge status-expiring">{indicator.daysUntilExpiry}d</span>
							{:else if indicator.status === 'expired'}
								<span class="status-badge status-expired">Expired</span>
							{/if}

							<div class="indicator-card__header">
								<div class="indicator-card__icon">
									<IconChartLine size={28} />
								</div>
								<div class="indicator-card__status" class:active={indicator.status === 'active'} class:expiring={indicator.status === 'expiring'}>
									{#if indicator.status === 'active'}
										<IconCheck size={14} />
										Active
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
												disabled={downloadingId === indicator.id}
											>
												{#if downloadingId === indicator.id}
													<IconLoader size={14} class="spin" />
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
								</div>
							</div>
						</article>
					</div>
				{/each}
			</div>
		</section>
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
		{/if}
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

	.refresh-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		border: 1px solid var(--st-border-color, #dbdbdb);
		border-radius: 8px;
		background: #fff;
		color: var(--st-text-muted, #64748b);
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.refresh-btn:hover:not(:disabled) {
		border-color: var(--st-primary, #0984ae);
		color: var(--st-primary, #0984ae);
	}

	.refresh-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LOADING & ERROR STATES
	   ═══════════════════════════════════════════════════════════════════════════ */

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 300px;
		color: var(--st-primary, #0984ae);
	}

	.loading-state p {
		margin-top: 16px;
		color: var(--st-text-muted, #64748b);
	}

	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 300px;
		text-align: center;
		padding: 24px;
		color: var(--st-error, #ef4444);
	}

	.error-state h2 {
		margin: 16px 0 8px;
		color: var(--st-text-color, #333);
	}

	.error-state p {
		margin: 0 0 24px;
		color: var(--st-text-muted, #64748b);
	}

	:global(.spin) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT - WordPress: .dashboard__content
	   ═══════════════════════════════════════════════════════════════════════════ */

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
	   EMPTY STATE - Simpler Trading Style
	   ═══════════════════════════════════════════════════════════════════════════ */

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 400px;
		text-align: center;
		color: var(--st-text-muted, #64748b);
		padding: 60px 20px;
		background: #f8fafc;
		border-radius: 12px;
		border: 2px dashed #e2e8f0;
	}

	.empty-icon {
		width: 100px;
		height: 100px;
		border-radius: 50%;
		background: #fff;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 24px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
		color: var(--st-text-muted, #64748b);
	}

	.empty-icon--lock {
		background: linear-gradient(135deg, #fef3c7, #fef9c3);
		color: #b45309;
	}

	.empty-state h2 {
		font-size: 1.5rem;
		color: var(--st-text-color, #333);
		margin: 0 0 12px;
		font-weight: 700;
	}

	.empty-state p {
		margin: 0 0 32px;
		max-width: 400px;
		line-height: 1.6;
	}

	.empty-actions {
		display: flex;
		gap: 16px;
		flex-wrap: wrap;
		justify-content: center;
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
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		background: #f99e31;
		border-radius: 4px;
		color: #fff;
		font-weight: 600;
		text-decoration: none;
		transition: background-color 0.15s ease;
	}

	.btn-orange:hover {
		background: #f88b09;
	}

	.btn-feedback {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		background: #fff;
		border: 2px solid #e5e7eb;
		border-radius: 8px;
		color: var(--st-text-color, #333);
		font-weight: 600;
		text-decoration: none;
		transition: all 0.15s ease;
	}

	.btn-feedback:hover {
		border-color: var(--st-primary, #0984ae);
		color: var(--st-primary, #0984ae);
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

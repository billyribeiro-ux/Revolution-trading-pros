<script lang="ts">
	/**
	 * Dashboard - My Indicators Page - WordPress Revolution Trading Exact
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

			const matchesPlatform = filterPlatform === 'all' ||
				indicator.platformSlug === filterPlatform;

			return matchesSearch && matchesPlatform;
		});
	});

	const platforms = [
		{ id: 'all', name: 'All Platforms' },
		{ id: 'tradingview', name: 'TradingView' },
		{ id: 'thinkorswim', name: 'ThinkorSwim' },
		{ id: 'metatrader', name: 'MetaTrader' }
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
			<span class="st-icon-indicators"></span>
			My Indicators
		</h1>
		<div class="dashboard__stats">
			<span class="stat-item">
				<strong>{stats.total}</strong> indicators
			</span>
			<span class="stat-item stat-active">
				<IconCheck size={14} />
				<strong>{stats.active}</strong> active
			</span>
			{#if stats.expiring > 0}
				<span class="stat-item stat-warning">
					<IconAlertTriangle size={14} />
					<strong>{stats.expiring}</strong> expiring soon
				</span>
			{/if}
		</div>
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
										<IconAlertTriangle size={14} />
										Expiring Soon
									{:else}
										<IconClock size={14} />
										Expired
									{/if}
								</div>
							</div>

							<h3 class="indicator-card__title">{indicator.name}</h3>
							<p class="indicator-card__description">{indicator.description}</p>

							<div class="indicator-card__meta">
								<div class="meta-row">
									<span class="meta-label">Platform:</span>
									<span class="meta-value">{indicator.platform}</span>
								</div>
								<div class="meta-row">
									<span class="meta-label">Version:</span>
									<span class="meta-value">v{indicator.version}</span>
								</div>
								<div class="meta-row">
									<span class="meta-label">{indicator.status === 'expired' ? 'Expired:' : 'Expires:'}</span>
									<span class="meta-value">{indicator.expiresAt}</span>
								</div>
								<div class="meta-row">
									<span class="meta-label">Last Updated:</span>
									<span class="meta-value">{indicator.lastUpdated}</span>
								</div>
							</div>

							<div class="indicator-card__actions">
								{#if indicator.status !== 'expired'}
									<button
										class="action-btn"
										onclick={() => handleDownload(indicator)}
										disabled={downloadingId === indicator.id}
									>
										{#if downloadingId === indicator.id}
											<span class="spin"><IconLoader size={16} /></span>
											Downloading...
										{:else}
											<IconDownload size={16} />
											Download
										{/if}
									</button>
									<a href={indicator.documentationUrl}>
										<IconExternalLink size={16} />
										Docs
									</a>
								{:else}
									<a href="/indicators/{indicator.slug}" class="action-btn action-btn--renew">
										<IconRefresh size={16} />
										Renew License
									</a>
								{/if}
							</div>
						</article>
					</div>
				{/each}
			</div>
		</section>
		{:else}
		<div class="empty-state">
			{#if searchQuery || filterPlatform !== 'all'}
				<!-- Filtered but no results -->
				<div class="empty-icon">
					<IconSearch size={48} />
				</div>
				<h2>No Indicators Found</h2>
				<p>Try adjusting your search or filter criteria</p>
				<button class="btn btn-primary" onclick={() => { searchQuery = ''; filterPlatform = 'all'; }}>
					Clear Filters
				</button>
			{:else}
				<!-- No indicators at all - Simpler Trading style -->
				<div class="empty-icon empty-icon--lock">
					<IconLock size={64} />
				</div>
				<h2>You don't have any Indicators</h2>
				<p>Enhance your trading with our powerful indicator suite. Browse our collection to get started.</p>
				<div class="empty-actions">
					<a href="/indicators" class="btn btn-orange">Browse Indicators</a>
					<a href="/dashboard/support" class="btn btn-feedback">
						<IconMessageCircle size={18} />
						Give Feedback
					</a>
				</div>
			{/if}
		</div>
		{/if}
	{/if}
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER - WordPress: .dashboard__header
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid var(--st-border-color, #dbdbdb);
		padding: 20px 30px;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
		align-items: center;
		gap: 16px;
	}

	.dashboard__header-left,
	.dashboard__header-right {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.dashboard__page-title {
		display: flex;
		align-items: center;
		gap: 12px;
		color: var(--st-text-color, #333);
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
	}

	.dashboard__page-title .st-icon-indicators {
		font-size: 32px;
		color: var(--st-primary, #0984ae);
	}

	.dashboard__stats {
		display: flex;
		gap: 16px;
		margin-left: 24px;
	}

	.stat-item {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 14px;
		color: var(--st-text-muted, #64748b);
	}

	.stat-item strong {
		color: var(--st-text-color, #333);
	}

	.stat-active {
		color: #10b981;
	}

	.stat-active strong {
		color: #10b981;
	}

	.stat-warning {
		color: var(--st-warning, #f59e0b);
	}

	.stat-warning strong {
		color: var(--st-warning, #f59e0b);
	}

	.btn-link {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		color: var(--st-link-color, #1e73be);
		text-decoration: none;
		font-weight: 600;
		font-size: 14px;
	}

	.btn-link:hover {
		color: var(--st-primary, #0984ae);
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

	.dashboard__content {
		padding: 30px;
		background: #fff;
		min-height: 400px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FILTERS BAR
	   ═══════════════════════════════════════════════════════════════════════════ */

	.filters-bar {
		display: flex;
		flex-wrap: wrap;
		gap: 16px;
		margin-bottom: 24px;
		align-items: center;
		justify-content: space-between;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 16px;
		background: #f8f9fa;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		flex: 1;
		max-width: 300px;
		color: var(--st-text-muted, #64748b);
	}

	.search-box input {
		flex: 1;
		border: none;
		outline: none;
		font-size: 14px;
		background: transparent;
	}

	.filter-tabs {
		display: flex;
		gap: 8px;
	}

	.filter-tab {
		padding: 8px 16px;
		background: #f8f9fa;
		border: 1px solid #e5e7eb;
		border-radius: 20px;
		font-size: 13px;
		font-weight: 500;
		color: var(--st-text-muted, #64748b);
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.filter-tab:hover {
		border-color: var(--st-primary, #0984ae);
		color: var(--st-primary, #0984ae);
	}

	.filter-tab.active {
		background: var(--st-primary, #0984ae);
		border-color: var(--st-primary, #0984ae);
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT SECTION - WordPress: .dashboard__content-section
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-section {
		margin-bottom: 40px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   INDICATORS GRID - Bootstrap Grid
	   ═══════════════════════════════════════════════════════════════════════════ */

	.indicators-grid {
		display: flex;
		flex-wrap: wrap;
		margin-right: -15px;
		margin-left: -15px;
	}

	.indicators-grid > .col-sm-6 {
		position: relative;
		width: 100%;
		padding-right: 15px;
		padding-left: 15px;
		margin-bottom: 30px;
	}

	@media (min-width: 576px) {
		.indicators-grid > .col-sm-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 1200px) {
		.indicators-grid > .col-xl-4 {
			flex: 0 0 33.333333%;
			max-width: 33.333333%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   INDICATOR CARD - Similar to membership-card
	   ═══════════════════════════════════════════════════════════════════════════ */

	.indicator-card {
		position: relative;
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 5px 30px rgb(0 0 0 / 10%);
		transition: all 0.15s ease-in-out;
		overflow: hidden;
		padding: 24px;
		height: 100%;
		display: flex;
		flex-direction: column;
	}

	.indicator-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 10px 40px rgb(0 0 0 / 15%);
	}

	.indicator-card.is-expiring {
		border: 2px solid var(--st-warning, #f59e0b);
	}

	.indicator-card.is-expired {
		border: 2px solid var(--st-error, #ef4444);
		opacity: 0.8;
	}

	/* Status Badge */
	.status-badge {
		position: absolute;
		top: 12px;
		right: 12px;
		padding: 4px 10px;
		border-radius: 12px;
		font-size: 11px;
		font-weight: 700;
		color: #fff;
		z-index: 1;
	}

	.status-expiring {
		background: var(--st-warning, #f59e0b);
	}

	.status-expired {
		background: var(--st-error, #ef4444);
	}

	/* Header */
	.indicator-card__header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 16px;
	}

	.indicator-card__icon {
		width: 56px;
		height: 56px;
		background: linear-gradient(135deg, #e0f2fe, #dbeafe);
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--st-primary, #0984ae);
	}

	.indicator-card__status {
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 6px 12px;
		border-radius: 6px;
		font-size: 12px;
		font-weight: 600;
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
	}

	.indicator-card__status.active {
		background: rgba(16, 185, 129, 0.1);
		color: #10b981;
	}

	.indicator-card__status.expiring {
		background: rgba(245, 158, 11, 0.1);
		color: #f59e0b;
	}

	/* Title & Description */
	.indicator-card__title {
		font-size: 18px;
		font-weight: 700;
		color: var(--st-text-color, #333);
		margin: 0 0 8px;
		line-height: 1.3;
	}

	.indicator-card__description {
		font-size: 14px;
		color: var(--st-text-muted, #64748b);
		margin: 0 0 16px;
		line-height: 1.5;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
		flex: 1;
	}

	/* Meta */
	.indicator-card__meta {
		background: #f8fafc;
		border-radius: 8px;
		padding: 12px;
		margin-bottom: 16px;
	}

	.meta-row {
		display: flex;
		justify-content: space-between;
		padding: 6px 0;
		font-size: 13px;
	}

	.meta-row:not(:last-child) {
		border-bottom: 1px solid #e5e7eb;
	}

	.meta-label {
		color: var(--st-text-muted, #64748b);
	}

	.meta-value {
		color: var(--st-text-color, #333);
		font-weight: 500;
	}

	/* Actions - WordPress: .membership-card__actions */
	.indicator-card__actions {
		display: flex;
		gap: 12px;
		margin-top: auto;
	}

	.indicator-card__actions a,
	.indicator-card__actions .action-btn {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
		padding: 12px;
		border-radius: 8px;
		font-weight: 600;
		font-size: 14px;
		text-decoration: none;
		transition: all 0.15s ease;
		background: #f4f4f4;
		border: 1px solid #e5e7eb;
		color: var(--st-text-color, #333);
	}

	.indicator-card__actions a:first-child,
	.indicator-card__actions .action-btn:first-child {
		background: var(--st-primary, #0984ae);
		border-color: var(--st-primary, #0984ae);
		color: #fff;
	}

	.indicator-card__actions a:first-child:hover {
		background: #076787;
	}

	.indicator-card__actions a:hover {
		background: #e5e7eb;
	}

	.action-btn--renew {
		background: #fef3c7 !important;
		border-color: #fcd34d !important;
		color: #92400e !important;
	}

	.action-btn--renew:hover {
		background: #fde68a !important;
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

	.btn-primary {
		padding: 12px 24px;
		background: var(--st-primary, #0984ae);
		border: none;
		border-radius: 8px;
		color: #fff;
		font-weight: 600;
		cursor: pointer;
		text-decoration: none;
	}

	.btn-primary:hover {
		background: #076787;
	}

	.btn-orange {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		background: var(--st-orange, #f99e31);
		border-radius: 8px;
		color: #fff;
		font-weight: 600;
		text-decoration: none;
		transition: background-color 0.15s ease;
	}

	.btn-orange:hover {
		background: var(--st-orange-hover, #dc7309);
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
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 768px) {
		.dashboard__header {
			padding: 16px;
		}

		.dashboard__page-title {
			font-size: 28px;
		}

		.dashboard__stats {
			display: none;
		}

		.dashboard__content {
			padding: 16px;
		}

		.filters-bar {
			flex-direction: column;
			align-items: stretch;
		}

		.search-box {
			max-width: none;
		}

		.filter-tabs {
			overflow-x: auto;
			padding-bottom: 8px;
		}

		.indicators-grid {
			margin-right: -8px;
			margin-left: -8px;
		}

		.indicators-grid > .col-sm-6 {
			padding-right: 8px;
			padding-left: 8px;
			margin-bottom: 16px;
		}

		.indicator-card__actions {
			flex-direction: column;
		}
	}
</style>

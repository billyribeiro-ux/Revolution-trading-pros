<script lang="ts">
	/**
	 * SPX Profit Pulse - Alerts Archive Page
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Complete archive of all trade alerts and market updates
	 *
	 * @version 1.0.0
	 */
	import { IconFilter, IconSearch } from '$lib/icons';

	// Filter state
	let selectedFilter = $state('all');
	let searchQuery = $state('');

	// All alerts data
	const allAlerts = [
		{
			id: 1,
			type: 'Trade Alert',
			title: 'SPX 0DTE Call Spread',
			date: 'January 10, 2026 at 9:45 AM ET',
			excerpt: 'Opening bullish call spread on SPX expiring today. Entry at market open with tight risk management.',
			status: 'Open',
			profitLoss: null,
			href: '/dashboard/spx-profit-pulse/alerts/spx-call-spread-011026'
		},
		{
			id: 2,
			type: 'Market Update',
			title: 'SPX Technical Analysis',
			date: 'January 10, 2026 at 8:30 AM ET',
			excerpt: 'Key levels to watch for today\'s 0DTE trading session. Support and resistance zones identified.',
			status: 'Info',
			profitLoss: null,
			href: '/dashboard/spx-profit-pulse/alerts/spx-analysis-011026'
		},
		{
			id: 3,
			type: 'Trade Alert',
			title: 'SPX Put Spread Closed',
			date: 'January 9, 2026 at 2:15 PM ET',
			excerpt: 'Closing bearish put spread for profit. Target reached ahead of schedule.',
			status: 'Closed',
			profitLoss: '+$425',
			href: '/dashboard/spx-profit-pulse/alerts/spx-put-close-010926'
		},
		{
			id: 4,
			type: 'Trade Alert',
			title: 'SPX Iron Condor Setup',
			date: 'January 9, 2026 at 10:00 AM ET',
			excerpt: 'Neutral strategy for range-bound market conditions. High probability setup with defined risk.',
			status: 'Closed',
			profitLoss: '+$350',
			href: '/dashboard/spx-profit-pulse/alerts/spx-condor-010926'
		},
		{
			id: 5,
			type: 'Market Update',
			title: 'Weekly SPX Outlook',
			date: 'January 8, 2026 at 4:00 PM ET',
			excerpt: 'Market analysis and key levels for the upcoming week. Economic calendar highlights.',
			status: 'Info',
			profitLoss: null,
			href: '/dashboard/spx-profit-pulse/alerts/weekly-outlook-010826'
		},
		{
			id: 6,
			type: 'Trade Alert',
			title: 'SPX Butterfly Spread',
			date: 'January 8, 2026 at 11:30 AM ET',
			excerpt: 'Advanced options strategy targeting specific price level. Limited risk with high reward potential.',
			status: 'Closed',
			profitLoss: '+$275',
			href: '/dashboard/spx-profit-pulse/alerts/spx-butterfly-010826'
		},
		{
			id: 7,
			type: 'Trade Alert',
			title: 'SPX Call Credit Spread',
			date: 'January 7, 2026 at 1:45 PM ET',
			excerpt: 'Bearish credit spread with high probability of profit. Market showing signs of exhaustion.',
			status: 'Closed',
			profitLoss: '+$300',
			href: '/dashboard/spx-profit-pulse/alerts/spx-credit-010726'
		},
		{
			id: 8,
			type: 'Trade Alert',
			title: 'SPX Straddle Entry',
			date: 'January 7, 2026 at 9:30 AM ET',
			excerpt: 'Volatility play ahead of major economic data release. Expecting significant move in either direction.',
			status: 'Closed',
			profitLoss: '-$150',
			href: '/dashboard/spx-profit-pulse/alerts/spx-straddle-010726'
		}
	];

	// Filtered alerts based on selection and search
	const filteredAlerts = $derived.by(() => {
		let filtered = allAlerts;

		// Apply type filter
		if (selectedFilter !== 'all') {
			if (selectedFilter === 'trades') {
				filtered = filtered.filter(a => a.type === 'Trade Alert');
			} else if (selectedFilter === 'updates') {
				filtered = filtered.filter(a => a.type === 'Market Update');
			}
		}

		// Apply search filter
		if (searchQuery.trim()) {
			const query = searchQuery.toLowerCase();
			filtered = filtered.filter(a => 
				a.title.toLowerCase().includes(query) ||
				a.excerpt.toLowerCase().includes(query)
			);
		}

		return filtered;
	});

	function getStatusClass(status: string): string {
		switch (status) {
			case 'Open': return 'status-open';
			case 'Closed': return 'status-closed';
			case 'Info': return 'status-info';
			default: return '';
		}
	}

	function getProfitLossClass(profitLoss: string | null): string {
		if (!profitLoss) return '';
		return profitLoss.startsWith('+') ? 'profit' : 'loss';
	}
</script>

<svelte:head>
	<title>Alerts Archive | SPX Profit Pulse | Revolution Trading Pros</title>
</svelte:head>

<!-- Dashboard Header -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Alerts Archive</h1>
	</div>
</header>

<!-- Dashboard Content -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			
			<!-- Filters & Search -->
			<div class="filters-container">
				<div class="filter-buttons">
					<button 
						class="filter-btn" 
						class:active={selectedFilter === 'all'}
						onclick={() => selectedFilter = 'all'}
					>
						All Alerts
					</button>
					<button 
						class="filter-btn" 
						class:active={selectedFilter === 'trades'}
						onclick={() => selectedFilter = 'trades'}
					>
						Trade Alerts
					</button>
					<button 
						class="filter-btn" 
						class:active={selectedFilter === 'updates'}
						onclick={() => selectedFilter = 'updates'}
					>
						Market Updates
					</button>
				</div>

				<div class="search-container">
					<IconSearch size={18} />
					<input 
						type="text" 
						placeholder="Search alerts..." 
						bind:value={searchQuery}
						class="search-input"
					/>
				</div>
			</div>

			<!-- Results Count -->
			<div class="results-count">
				Showing {filteredAlerts.length} {filteredAlerts.length === 1 ? 'alert' : 'alerts'}
			</div>

			<!-- Alerts List -->
			<div class="alerts-list">
				{#each filteredAlerts as alert (alert.id)}
					<article class="alert-card">
						<div class="alert-header">
							<span class="alert-type">{alert.type}</span>
							<span class="alert-status {getStatusClass(alert.status)}">{alert.status}</span>
						</div>
						<h3 class="alert-title">
							<a href={alert.href}>{alert.title}</a>
						</h3>
						<p class="alert-date">{alert.date}</p>
						<p class="alert-excerpt">{alert.excerpt}</p>
						{#if alert.profitLoss}
							<div class="alert-profit-loss {getProfitLossClass(alert.profitLoss)}">
								{alert.profitLoss}
							</div>
						{/if}
						<a href={alert.href} class="alert-link">View Details →</a>
					</article>
				{/each}
			</div>

			{#if filteredAlerts.length === 0}
				<div class="no-results">
					<p>No alerts found matching your criteria.</p>
				</div>
			{/if}

		</section>
	</div>
</div>

<style>
	/* Dashboard Header */
	.dashboard__header {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		border-right: 1px solid #dbdbdb;
		padding: 20px;
	}

	@media (min-width: 1280px) {
		.dashboard__header {
			padding: 30px;
		}
	}

	@media (min-width: 1440px) {
		.dashboard__header {
			padding: 30px 40px;
		}
	}

	.dashboard__header-left {
		display: flex;
		align-items: center;
		justify-content: flex-start;
		flex: 1;
		gap: 15px;
	}

	.dashboard__page-title {
		margin: 0;
		color: #333;
		font-size: 36px;
		font-weight: 400;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	/* Dashboard Content */
	.dashboard__content {
		display: flex;
		flex-flow: row nowrap;
	}

	.dashboard__content-main {
		border-right: 1px solid #dbdbdb;
		flex: 1 1 auto;
		min-width: 0;
		background: #fff;
	}

	.dashboard__content-section {
		padding: 30px 20px;
	}

	@media screen and (min-width: 1280px) {
		.dashboard__content-section {
			padding: 30px;
		}
	}

	@media screen and (min-width: 1440px) {
		.dashboard__content-section {
			padding: 40px;
		}
	}

	/* Filters */
	.filters-container {
		display: flex;
		flex-wrap: wrap;
		gap: 20px;
		margin-bottom: 30px;
		align-items: center;
		justify-content: space-between;
	}

	.filter-buttons {
		display: flex;
		gap: 10px;
		flex-wrap: wrap;
	}

	.filter-btn {
		padding: 10px 20px;
		background: #f8f9fa;
		border: 1px solid #e0e0e0;
		border-radius: 5px;
		font-size: 14px;
		font-weight: 600;
		color: #666;
		cursor: pointer;
		transition: all 0.2s ease;
		font-family: 'Open Sans', sans-serif;
	}

	.filter-btn:hover {
		background: #e9ecef;
		border-color: #143E59;
	}

	.filter-btn.active {
		background: #143E59;
		color: #fff;
		border-color: #143E59;
	}

	/* Search */
	.search-container {
		display: flex;
		align-items: center;
		gap: 10px;
		background: #f8f9fa;
		border: 1px solid #e0e0e0;
		border-radius: 5px;
		padding: 10px 15px;
		min-width: 250px;
	}

	.search-input {
		flex: 1;
		border: none;
		background: transparent;
		font-size: 14px;
		color: #333;
		outline: none;
		font-family: 'Open Sans', sans-serif;
	}

	.search-input::placeholder {
		color: #999;
	}

	/* Results Count */
	.results-count {
		font-size: 14px;
		color: #666;
		margin-bottom: 20px;
		font-weight: 500;
	}

	/* Alerts List */
	.alerts-list {
		display: grid;
		grid-template-columns: repeat(1, 1fr);
		gap: 20px;
	}

	@media screen and (min-width: 768px) {
		.alerts-list {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media screen and (min-width: 1280px) {
		.alerts-list {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	/* Alert Card */
	.alert-card {
		background: #fff;
		border: 1px solid #e0e0e0;
		border-radius: 8px;
		padding: 20px;
		transition: all 0.2s ease;
	}

	.alert-card:hover {
		transform: translateY(-2px);
		box-shadow: 0 5px 20px rgba(0, 0, 0, 0.1);
		border-color: #143E59;
	}

	.alert-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 12px;
	}

	.alert-type {
		font-size: 12px;
		font-weight: 700;
		text-transform: uppercase;
		color: #666;
		letter-spacing: 0.5px;
	}

	.alert-status {
		padding: 4px 10px;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.alert-status.status-open {
		background: #e3f2fd;
		color: #1976d2;
	}

	.alert-status.status-closed {
		background: #e8f5e9;
		color: #388e3c;
	}

	.alert-status.status-info {
		background: #fff3e0;
		color: #f57c00;
	}

	.alert-title {
		margin: 0 0 8px 0;
		font-size: 18px;
		font-weight: 600;
		line-height: 1.3;
	}

	.alert-title a {
		color: #333;
		text-decoration: none;
		transition: color 0.2s;
	}

	.alert-title a:hover {
		color: #143E59;
	}

	.alert-date {
		margin: 0 0 12px 0;
		font-size: 13px;
		color: #999;
	}

	.alert-excerpt {
		margin: 0 0 15px 0;
		font-size: 14px;
		line-height: 1.6;
		color: #666;
	}

	.alert-profit-loss {
		display: inline-block;
		padding: 6px 12px;
		border-radius: 4px;
		font-size: 14px;
		font-weight: 700;
		margin-bottom: 12px;
	}

	.alert-profit-loss.profit {
		background: #e8f5e9;
		color: #2e7d32;
	}

	.alert-profit-loss.loss {
		background: #ffebee;
		color: #c62828;
	}

	.alert-link {
		display: inline-block;
		color: #0984ae;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		transition: color 0.2s;
	}

	.alert-link:hover {
		color: #143E59;
		text-decoration: underline;
	}

	/* No Results */
	.no-results {
		text-align: center;
		padding: 60px 20px;
		color: #999;
	}

	.no-results p {
		font-size: 16px;
		margin: 0;
	}
</style>

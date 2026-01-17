<script lang="ts">
	/**
	 * Explosive Swings - Alerts Archive Page
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Complete archive of all swing trade alerts and market updates
	 *
	 * @version 1.1.0
	 * @svelte5 Fully compliant with Svelte 5 Nov/Dec 2025 best practices
	 */
	import { onMount } from 'svelte';
	import { IconFilter, IconSearch } from '$lib/icons';
	import type { RoomAlert } from '$lib/types/trading';

	// Props interface for SSR data - Svelte 5 best practice
	interface Props {
		data: {
			alerts?: any[];
		};
	}

	let { data }: Props = $props();

	// Svelte 5 $state for filter state
	let selectedFilter = $state('all');
	let searchQuery = $state('');
	let apiAlerts = $state<RoomAlert[]>([]);
	let isLoading = $state(false);
	let pagination = $state({ total: 0, limit: 50, offset: 0 });

	const ROOM_SLUG = 'explosive-swings';

	// Fetch alerts from API
	async function fetchAlerts() {
		isLoading = true;
		try {
			const params = new URLSearchParams({
				limit: pagination.limit.toString(),
				offset: pagination.offset.toString()
			});

			const response = await fetch(`/api/alerts/${ROOM_SLUG}?${params}`);
			const data = await response.json();
			if (data.success) {
				apiAlerts = data.data;
				pagination.total = data.total;
			}
		} catch (err) {
			console.error('Failed to fetch alerts:', err);
		} finally {
			isLoading = false;
		}
	}

	onMount(() => {
		fetchAlerts();
	});

	// All alerts data
	const allAlerts = [
		{
			id: 1,
			type: 'Trade Alert',
			title: 'NVDA Swing Trade Setup',
			date: 'January 10, 2026 at 2:30 PM ET',
			excerpt:
				'Opening swing position on NVDA with bullish momentum. Multi-day hold targeting key resistance levels.',
			status: 'Open',
			profitLoss: null,
			href: '/dashboard/explosive-swings/alerts/nvda-swing-011026'
		},
		{
			id: 2,
			type: 'Market Update',
			title: 'Weekly Swing Trade Outlook',
			date: 'January 10, 2026 at 9:00 AM ET',
			excerpt:
				'Key swing trade setups for the week ahead. Technical analysis on major tech stocks and market leaders.',
			status: 'Info',
			profitLoss: null,
			href: '/dashboard/explosive-swings/alerts/weekly-outlook-011026'
		},
		{
			id: 3,
			type: 'Trade Alert',
			title: 'TSLA Position Closed',
			date: 'January 9, 2026 at 3:45 PM ET',
			excerpt: 'Closing TSLA swing trade for profit. Target reached after 5-day hold.',
			status: 'Closed',
			profitLoss: '+$1,250',
			href: '/dashboard/explosive-swings/alerts/tsla-close-010926'
		},
		{
			id: 4,
			type: 'Trade Alert',
			title: 'AMZN Breakout Play',
			date: 'January 9, 2026 at 11:15 AM ET',
			excerpt: 'AMZN breaking above key resistance. Swing trade entry with momentum confirmation.',
			status: 'Open',
			profitLoss: null,
			href: '/dashboard/explosive-swings/alerts/amzn-breakout-010926'
		},
		{
			id: 5,
			type: 'Market Update',
			title: 'Sector Rotation Analysis',
			date: 'January 8, 2026 at 4:30 PM ET',
			excerpt:
				'Identifying the strongest sectors for swing trading opportunities. Technology and healthcare showing strength.',
			status: 'Info',
			profitLoss: null,
			href: '/dashboard/explosive-swings/alerts/sector-analysis-010826'
		},
		{
			id: 6,
			type: 'Trade Alert',
			title: 'GOOGL Options Swing',
			date: 'January 8, 2026 at 10:00 AM ET',
			excerpt:
				'Multi-week options swing trade on GOOGL. Targeting earnings catalyst with defined risk.',
			status: 'Closed',
			profitLoss: '+$875',
			href: '/dashboard/explosive-swings/alerts/googl-options-010826'
		},
		{
			id: 7,
			type: 'Trade Alert',
			title: 'META Momentum Play',
			date: 'January 7, 2026 at 1:00 PM ET',
			excerpt: 'META showing strong momentum after earnings. Swing trade with 7-10 day target.',
			status: 'Closed',
			profitLoss: '+$1,450',
			href: '/dashboard/explosive-swings/alerts/meta-momentum-010726'
		},
		{
			id: 8,
			type: 'Trade Alert',
			title: 'AAPL Pullback Entry',
			date: 'January 7, 2026 at 9:45 AM ET',
			excerpt:
				'AAPL pulling back to support. High-probability swing entry with favorable risk/reward.',
			status: 'Closed',
			profitLoss: '-$320',
			href: '/dashboard/explosive-swings/alerts/aapl-pullback-010726'
		}
	];

	// Transform API alerts to display format
	function formatAlertDate(dateString: string): string {
		const date = new Date(dateString);
		return (
			date.toLocaleDateString('en-US', {
				month: 'long',
				day: 'numeric',
				year: 'numeric'
			}) +
			' at ' +
			date.toLocaleTimeString('en-US', {
				hour: 'numeric',
				minute: '2-digit',
				timeZoneName: 'short'
			})
		);
	}

	// Derive display alerts from API or fallback
	const displayAlerts = $derived(
		apiAlerts.length > 0
			? apiAlerts.map((a) => ({
					id: a.id,
					type: a.alert_type === 'UPDATE' ? 'Market Update' : 'Trade Alert',
					alertType: a.alert_type,
					title: a.title,
					date: formatAlertDate(a.published_at),
					excerpt: a.message,
					status: a.alert_type === 'UPDATE' ? 'Info' : a.alert_type === 'EXIT' ? 'Closed' : 'Open',
					profitLoss: null as string | null, // Would come from linked trade
					href: `/dashboard/explosive-swings/alerts/${a.id}`,
					tosString: a.tos_string,
					ticker: a.ticker
				}))
			: allAlerts
	);

	// Filtered alerts based on selection and search
	const filteredAlerts = $derived.by(() => {
		let filtered = displayAlerts;

		// Apply type filter
		if (selectedFilter !== 'all') {
			if (selectedFilter === 'trades') {
				filtered = filtered.filter((a) => a.type === 'Trade Alert');
			} else if (selectedFilter === 'updates') {
				filtered = filtered.filter((a) => a.type === 'Market Update');
			}
		}

		// Apply search filter
		if (searchQuery.trim()) {
			const query = searchQuery.toLowerCase();
			filtered = filtered.filter(
				(a) => a.title.toLowerCase().includes(query) || a.excerpt.toLowerCase().includes(query)
			);
		}

		return filtered;
	});

	function getStatusClass(status: string): string {
		switch (status) {
			case 'Open':
				return 'status-open';
			case 'Closed':
				return 'status-closed';
			case 'Info':
				return 'status-info';
			default:
				return '';
		}
	}

	function getProfitLossClass(profitLoss: string | null): string {
		if (!profitLoss) return '';
		return profitLoss.startsWith('+') ? 'profit' : 'loss';
	}
</script>

<svelte:head>
	<title>Alerts Archive | Explosive Swings | Revolution Trading Pros</title>
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
						onclick={() => (selectedFilter = 'all')}
					>
						All Alerts
					</button>
					<button
						class="filter-btn"
						class:active={selectedFilter === 'trades'}
						onclick={() => (selectedFilter = 'trades')}
					>
						Trade Alerts
					</button>
					<button
						class="filter-btn"
						class:active={selectedFilter === 'updates'}
						onclick={() => (selectedFilter = 'updates')}
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
				Showing {filteredAlerts.length}
				{filteredAlerts.length === 1 ? 'alert' : 'alerts'}
			</div>

			<!-- Alerts List -->
			<div class="alerts-list">
				{#each filteredAlerts as alert (alert.id)}
					<article class="alert-card">
						<div class="alert-header">
							<div class="alert-header-left">
								<span class="alert-type">{alert.type}</span>
								{#if 'ticker' in alert && alert.ticker}
									<span class="alert-ticker">{alert.ticker}</span>
								{/if}
							</div>
							<span class="alert-status {getStatusClass(alert.status)}">{alert.status}</span>
						</div>
						<h3 class="alert-title">
							<a href={alert.href}>{alert.title}</a>
						</h3>
						<p class="alert-date">{alert.date}</p>
						{#if 'tosString' in alert && alert.tosString}
							<div class="tos-display">
								<code>{alert.tosString}</code>
							</div>
						{/if}
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
	.dashboard__header {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		border-right: 1px solid #dbdbdb;
		padding: 20px;
	}

	@media (min-width: 1024px) {
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

	@media screen and (min-width: 1024px) {
		.dashboard__content-section {
			padding: 30px;
		}
	}

	@media screen and (min-width: 1440px) {
		.dashboard__content-section {
			padding: 40px;
		}
	}

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
		border-color: #143e59;
	}

	.filter-btn.active {
		background: #143e59;
		color: #fff;
		border-color: #143e59;
	}

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

	.results-count {
		font-size: 14px;
		color: #666;
		margin-bottom: 20px;
		font-weight: 500;
	}

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

	@media screen and (min-width: 1024px) {
		.alerts-list {
			grid-template-columns: repeat(3, 1fr);
		}
	}

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
		border-color: #143e59;
	}

	.alert-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 12px;
	}

	.alert-header-left {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	.alert-ticker {
		font-size: 14px;
		font-weight: 700;
		color: #143e59;
		background: #e0f2fe;
		padding: 4px 10px;
		border-radius: 4px;
	}

	.tos-display {
		background: #1a1a2e;
		border-radius: 6px;
		padding: 10px 14px;
		margin-bottom: 12px;
	}

	.tos-display code {
		color: #22c55e;
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
		font-size: 12px;
		font-weight: 600;
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
		color: #143e59;
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
		color: #143e59;
		text-decoration: underline;
	}

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

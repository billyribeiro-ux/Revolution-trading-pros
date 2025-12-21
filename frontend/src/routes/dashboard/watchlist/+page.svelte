<script lang="ts">
	/**
	 * Dashboard - Weekly Watchlist Page
	 * Follows WordPress Revolution Trading patterns exactly
	 */
	import SEOHead from '$lib/components/SEOHead.svelte';

	// Current week info
	const currentWeek = {
		startDate: 'December 2, 2024',
		endDate: 'December 6, 2024',
		publishedAt: 'Sunday, December 1, 2024 at 6:00 PM ET'
	};

	// Watchlist categories
	type WatchlistItem = {
		symbol: string;
		name: string;
		sector: string;
		setup: string;
		entryZone: string;
		target: string;
		stopLoss: string;
		notes: string;
		priority: 'high' | 'medium' | 'low';
	};

	const bullishSetups: WatchlistItem[] = [
		{
			symbol: 'NVDA',
			name: 'NVIDIA Corporation',
			sector: 'Technology',
			setup: 'Bull Flag Breakout',
			entryZone: '$480 - $485',
			target: '$520',
			stopLoss: '$465',
			notes: 'Strong AI momentum, watch for volume confirmation above $485',
			priority: 'high'
		},
		{
			symbol: 'MSFT',
			name: 'Microsoft Corporation',
			sector: 'Technology',
			setup: 'Support Bounce',
			entryZone: '$370 - $375',
			target: '$395',
			stopLoss: '$362',
			notes: 'Testing key support, Copilot catalyst ahead',
			priority: 'high'
		},
		{
			symbol: 'AMZN',
			name: 'Amazon.com Inc',
			sector: 'Consumer Discretionary',
			setup: 'Ascending Triangle',
			entryZone: '$185 - $188',
			target: '$200',
			stopLoss: '$180',
			notes: 'AWS growth story intact, holiday season catalyst',
			priority: 'medium'
		},
		{
			symbol: 'META',
			name: 'Meta Platforms Inc',
			sector: 'Communication Services',
			setup: 'Consolidation Breakout',
			entryZone: '$560 - $570',
			target: '$600',
			stopLoss: '$545',
			notes: 'AI advertising momentum, reels growth',
			priority: 'medium'
		}
	];

	const bearishSetups: WatchlistItem[] = [
		{
			symbol: 'DIS',
			name: 'Walt Disney Co',
			sector: 'Communication Services',
			setup: 'Descending Channel',
			entryZone: '$95 - $98',
			target: '$85',
			stopLoss: '$102',
			notes: 'Streaming losses continue, parks slowdown',
			priority: 'medium'
		},
		{
			symbol: 'BA',
			name: 'Boeing Company',
			sector: 'Industrials',
			setup: 'Head & Shoulders',
			entryZone: '$240 - $245',
			target: '$215',
			stopLoss: '$255',
			notes: 'Production issues, 737 MAX concerns',
			priority: 'low'
		}
	];

	const optionsPlays = [
		{
			symbol: 'SPY',
			strategy: 'Iron Condor',
			expiration: 'Dec 20',
			strikes: '575/580 - 605/610',
			premium: '$1.85',
			maxProfit: '$185',
			maxLoss: '$315',
			notes: 'Range-bound market expected through FOMC'
		},
		{
			symbol: 'QQQ',
			strategy: 'Bull Call Spread',
			expiration: 'Dec 27',
			strikes: '490/500',
			premium: '$4.20',
			maxProfit: '$580',
			maxLoss: '$420',
			notes: 'Tech momentum play, Santa rally potential'
		}
	];

	// Past watchlists archive
	const pastWatchlists = [
		{ week: 'Nov 25 - Nov 29', winners: 4, losers: 1, winRate: '80%' },
		{ week: 'Nov 18 - Nov 22', winners: 5, losers: 2, winRate: '71%' },
		{ week: 'Nov 11 - Nov 15', winners: 3, losers: 1, winRate: '75%' },
		{ week: 'Nov 4 - Nov 8', winners: 6, losers: 2, winRate: '75%' }
	];

	let activeTab = $state<'bullish' | 'bearish' | 'options'>('bullish');
</script>

<SEOHead
	title="Weekly Watchlist - Dashboard"
	description="This week's top stock picks and trade setups from Revolution Trading Pros experts."
	noindex
/>

<div class="dashboard__page">
	<!-- Page Header -->
	<header class="dashboard__header">
		<div class="dashboard__header-left">
			<nav class="dashboard__breadcrumb">
				<a href="/dashboard">Dashboard</a>
				<span class="separator">/</span>
				<span class="current">Weekly Watchlist</span>
			</nav>
			<h1 class="dashboard__page-title">
				<span class="dashboard__nav-item-icon st-icon-trade-of-the-week"></span>
				Weekly Watchlist
			</h1>
		</div>
		<div class="dashboard__header-right">
			<span class="watchlist__week-label">Week of {currentWeek.startDate} - {currentWeek.endDate}</span>
		</div>
	</header>

	<!-- Published Info -->
	<div class="watchlist__published">
		<span class="watchlist__published-icon">📅</span>
		<span>Published: {currentWeek.publishedAt}</span>
	</div>

	<!-- Intro Section -->
	<section class="watchlist__intro">
		<h2 class="watchlist__intro-title">This Week's Market Outlook</h2>
		<p class="watchlist__intro-text">
			Markets are showing resilience heading into December. With the Fed's final meeting of the year approaching,
			we're watching for a potential Santa rally. Key levels to watch: SPY 600 resistance, QQQ 500 psychological level.
			Focus on tech and AI plays while maintaining defensive positions in consumer staples.
		</p>
		<div class="watchlist__stats">
			<div class="watchlist__stat">
				<span class="watchlist__stat-value">6</span>
				<span class="watchlist__stat-label">Stock Setups</span>
			</div>
			<div class="watchlist__stat">
				<span class="watchlist__stat-value">2</span>
				<span class="watchlist__stat-label">Options Plays</span>
			</div>
			<div class="watchlist__stat">
				<span class="watchlist__stat-value">75%</span>
				<span class="watchlist__stat-label">Last Month Win Rate</span>
			</div>
		</div>
	</section>

	<!-- Tabs -->
	<div class="watchlist__tabs">
		<button
			class="watchlist__tab"
			class:active={activeTab === 'bullish'}
			onclick={() => activeTab = 'bullish'}
		>
			<span class="tab-icon bullish">↑</span>
			Bullish Setups ({bullishSetups.length})
		</button>
		<button
			class="watchlist__tab"
			class:active={activeTab === 'bearish'}
			onclick={() => activeTab = 'bearish'}
		>
			<span class="tab-icon bearish">↓</span>
			Bearish Setups ({bearishSetups.length})
		</button>
		<button
			class="watchlist__tab"
			class:active={activeTab === 'options'}
			onclick={() => activeTab = 'options'}
		>
			<span class="tab-icon options">⚡</span>
			Options Plays ({optionsPlays.length})
		</button>
	</div>

	<!-- Content -->
	<section class="dashboard__content-section">
		{#if activeTab === 'bullish' || activeTab === 'bearish'}
			{@const items = activeTab === 'bullish' ? bullishSetups : bearishSetups}
			<div class="watchlist__table-wrapper">
				<table class="watchlist__table">
					<thead>
						<tr>
							<th>Symbol</th>
							<th>Setup</th>
							<th>Entry Zone</th>
							<th>Target</th>
							<th>Stop Loss</th>
							<th>Priority</th>
						</tr>
					</thead>
					<tbody>
						{#each items as item}
							<tr>
								<td>
									<div class="watchlist__symbol">
										<span class="symbol-ticker">{item.symbol}</span>
										<span class="symbol-name">{item.name}</span>
										<span class="symbol-sector">{item.sector}</span>
									</div>
								</td>
								<td>
									<span class="setup-name">{item.setup}</span>
								</td>
								<td>
									<span class="price-zone">{item.entryZone}</span>
								</td>
								<td>
									<span class="price-target {activeTab}">{item.target}</span>
								</td>
								<td>
									<span class="price-stop">{item.stopLoss}</span>
								</td>
								<td>
									<span class="priority-badge {item.priority}">{item.priority}</span>
								</td>
							</tr>
							<tr class="notes-row">
								<td colspan="6">
									<div class="watchlist__notes">
										<strong>Notes:</strong> {item.notes}
									</div>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{:else}
			<div class="watchlist__options-grid">
				{#each optionsPlays as play}
					<div class="watchlist__option-card">
						<div class="option-card__header">
							<span class="option-card__symbol">{play.symbol}</span>
							<span class="option-card__strategy">{play.strategy}</span>
						</div>
						<div class="option-card__body">
							<div class="option-card__row">
								<span class="label">Expiration</span>
								<span class="value">{play.expiration}</span>
							</div>
							<div class="option-card__row">
								<span class="label">Strikes</span>
								<span class="value">{play.strikes}</span>
							</div>
							<div class="option-card__row">
								<span class="label">Premium</span>
								<span class="value">{play.premium}</span>
							</div>
							<div class="option-card__row highlight">
								<span class="label">Max Profit</span>
								<span class="value profit">{play.maxProfit}</span>
							</div>
							<div class="option-card__row highlight">
								<span class="label">Max Loss</span>
								<span class="value loss">{play.maxLoss}</span>
							</div>
						</div>
						<div class="option-card__notes">
							{play.notes}
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</section>

	<!-- Disclaimer -->
	<section class="watchlist__disclaimer">
		<h4>Risk Disclaimer</h4>
		<p>
			The stocks and options listed are for educational purposes only and do not constitute financial advice.
			All trades involve risk. Past performance does not guarantee future results. Always do your own research
			and consult with a licensed financial advisor before making investment decisions.
		</p>
	</section>

	<!-- Past Watchlists -->
	<section class="dashboard__content-section">
		<h2 class="dashboard__section-title">Past Watchlists Performance</h2>
		<div class="watchlist__archive">
			{#each pastWatchlists as week}
				<div class="watchlist__archive-item">
					<span class="archive-week">{week.week}</span>
					<div class="archive-stats">
						<span class="archive-winners">✓ {week.winners} Winners</span>
						<span class="archive-losers">✗ {week.losers} Losers</span>
						<span class="archive-winrate">{week.winRate}</span>
					</div>
					<button type="button" class="archive-link">View Details →</button>
				</div>
			{/each}
		</div>
	</section>
</div>

<style>
	.dashboard__page {
		padding: 0;
	}

	/* Header */
	.dashboard__header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.dashboard__breadcrumb {
		font-size: 0.875rem;
		color: var(--st-color-gray-400, #9ca3af);
		margin-bottom: 0.5rem;
	}

	.dashboard__breadcrumb a {
		color: var(--st-color-gray-400, #9ca3af);
		text-decoration: none;
	}

	.dashboard__breadcrumb a:hover {
		color: var(--st-color-orange, #f97316);
	}

	.dashboard__breadcrumb .separator {
		margin: 0 0.5rem;
	}

	.dashboard__breadcrumb .current {
		color: var(--st-color-white, #fff);
	}

	.dashboard__page-title {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--st-color-white, #fff);
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.watchlist__week-label {
		background: var(--st-color-gray-800, #1f2937);
		padding: 0.5rem 1rem;
		border-radius: 8px;
		font-size: 0.875rem;
		color: var(--st-color-gray-300, #d1d5db);
	}

	/* Published Info */
	.watchlist__published {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: var(--st-color-gray-400, #9ca3af);
		margin-bottom: 2rem;
	}

	/* Intro Section */
	.watchlist__intro {
		background: var(--st-color-gray-800, #1f2937);
		border-radius: 1rem;
		padding: 2rem;
		margin-bottom: 2rem;
	}

	.watchlist__intro-title {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--st-color-white, #fff);
		margin-bottom: 1rem;
	}

	.watchlist__intro-text {
		font-size: 1rem;
		color: var(--st-color-gray-300, #d1d5db);
		line-height: 1.7;
		margin-bottom: 1.5rem;
	}

	.watchlist__stats {
		display: flex;
		gap: 2rem;
		flex-wrap: wrap;
	}

	.watchlist__stat {
		display: flex;
		flex-direction: column;
	}

	.watchlist__stat-value {
		font-size: 2rem;
		font-weight: 700;
		color: var(--st-color-orange, #f97316);
	}

	.watchlist__stat-label {
		font-size: 0.875rem;
		color: var(--st-color-gray-400, #9ca3af);
	}

	/* Tabs */
	.watchlist__tabs {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
	}

	.watchlist__tab {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: var(--st-color-gray-800, #1f2937);
		border: 1px solid var(--st-color-gray-700, #374151);
		border-radius: 8px;
		color: var(--st-color-gray-300, #d1d5db);
		font-size: 0.9375rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.watchlist__tab:hover {
		border-color: var(--st-color-gray-600, #4b5563);
	}

	.watchlist__tab.active {
		background: var(--st-color-orange, #f97316);
		border-color: var(--st-color-orange, #f97316);
		color: var(--st-color-white, #fff);
	}

	.tab-icon {
		font-weight: 700;
	}

	.tab-icon.bullish {
		color: var(--st-color-green, #22c55e);
	}

	.tab-icon.bearish {
		color: var(--st-color-red, #ef4444);
	}

	.watchlist__tab.active .tab-icon {
		color: var(--st-color-white, #fff);
	}

	/* Table */
	.watchlist__table-wrapper {
		overflow-x: auto;
	}

	.watchlist__table {
		width: 100%;
		border-collapse: collapse;
	}

	.watchlist__table th {
		text-align: left;
		padding: 1rem;
		background: var(--st-color-gray-800, #1f2937);
		color: var(--st-color-gray-400, #9ca3af);
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		border-bottom: 1px solid var(--st-color-gray-700, #374151);
	}

	.watchlist__table td {
		padding: 1rem;
		border-bottom: 1px solid var(--st-color-gray-800, #1f2937);
		vertical-align: top;
	}

	.watchlist__table tbody tr:hover {
		background: var(--st-color-gray-800, #1f2937);
	}

	.watchlist__symbol {
		display: flex;
		flex-direction: column;
	}

	.symbol-ticker {
		font-size: 1.125rem;
		font-weight: 700;
		color: var(--st-color-white, #fff);
	}

	.symbol-name {
		font-size: 0.875rem;
		color: var(--st-color-gray-400, #9ca3af);
	}

	.symbol-sector {
		font-size: 0.75rem;
		color: var(--st-color-gray-500, #6b7280);
	}

	.setup-name {
		color: var(--st-color-gray-300, #d1d5db);
	}

	.price-zone {
		color: var(--st-color-gray-300, #d1d5db);
		font-family: monospace;
	}

	.price-target {
		font-weight: 600;
		font-family: monospace;
	}

	.price-target.bullish {
		color: var(--st-color-green, #22c55e);
	}

	.price-target.bearish {
		color: var(--st-color-red, #ef4444);
	}

	.price-stop {
		color: var(--st-color-red, #ef4444);
		font-family: monospace;
	}

	.priority-badge {
		display: inline-block;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.priority-badge.high {
		background: rgba(239, 68, 68, 0.2);
		color: #ef4444;
	}

	.priority-badge.medium {
		background: rgba(249, 115, 22, 0.2);
		color: #f97316;
	}

	.priority-badge.low {
		background: rgba(156, 163, 175, 0.2);
		color: #9ca3af;
	}

	.notes-row {
		background: var(--st-color-gray-900, #111827) !important;
	}

	.notes-row:hover {
		background: var(--st-color-gray-900, #111827) !important;
	}

	.watchlist__notes {
		font-size: 0.875rem;
		color: var(--st-color-gray-400, #9ca3af);
		padding: 0.5rem 0;
	}

	.watchlist__notes strong {
		color: var(--st-color-gray-300, #d1d5db);
	}

	/* Options Grid */
	.watchlist__options-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(min(100%, 320px), 1fr));
		gap: 1.5rem;
	}

	.watchlist__option-card {
		background: var(--st-color-gray-800, #1f2937);
		border: 1px solid var(--st-color-gray-700, #374151);
		border-radius: 1rem;
		overflow: hidden;
	}

	.option-card__header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem;
		background: var(--st-color-gray-700, #374151);
	}

	.option-card__symbol {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--st-color-white, #fff);
	}

	.option-card__strategy {
		background: var(--st-color-orange, #f97316);
		padding: 0.375rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--st-color-white, #fff);
	}

	.option-card__body {
		padding: 1.25rem;
	}

	.option-card__row {
		display: flex;
		justify-content: space-between;
		padding: 0.5rem 0;
		border-bottom: 1px solid var(--st-color-gray-700, #374151);
	}

	.option-card__row:last-child {
		border-bottom: none;
	}

	.option-card__row .label {
		color: var(--st-color-gray-400, #9ca3af);
		font-size: 0.875rem;
	}

	.option-card__row .value {
		color: var(--st-color-white, #fff);
		font-weight: 500;
		font-family: monospace;
	}

	.option-card__row .value.profit {
		color: var(--st-color-green, #22c55e);
	}

	.option-card__row .value.loss {
		color: var(--st-color-red, #ef4444);
	}

	.option-card__row.highlight {
		background: rgba(255, 255, 255, 0.02);
		margin: 0 -1.25rem;
		padding: 0.75rem 1.25rem;
	}

	.option-card__notes {
		padding: 1rem 1.25rem;
		background: var(--st-color-gray-900, #111827);
		font-size: 0.875rem;
		color: var(--st-color-gray-400, #9ca3af);
	}

	/* Disclaimer */
	.watchlist__disclaimer {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 0.75rem;
		padding: 1.25rem;
		margin-bottom: 2rem;
	}

	.watchlist__disclaimer h4 {
		color: var(--st-color-red, #ef4444);
		font-size: 0.875rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
	}

	.watchlist__disclaimer p {
		color: var(--st-color-gray-400, #9ca3af);
		font-size: 0.8125rem;
		line-height: 1.6;
		margin: 0;
	}

	/* Archive */
	.dashboard__section-title {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--st-color-white, #fff);
		margin-bottom: 1.5rem;
	}

	.watchlist__archive {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.watchlist__archive-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.25rem;
		background: var(--st-color-gray-800, #1f2937);
		border-radius: 0.75rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.archive-week {
		font-weight: 600;
		color: var(--st-color-white, #fff);
	}

	.archive-stats {
		display: flex;
		gap: 1.5rem;
	}

	.archive-winners {
		color: var(--st-color-green, #22c55e);
		font-size: 0.875rem;
	}

	.archive-losers {
		color: var(--st-color-red, #ef4444);
		font-size: 0.875rem;
	}

	.archive-winrate {
		background: rgba(34, 197, 94, 0.2);
		color: var(--st-color-green, #22c55e);
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.archive-link {
		background: none;
		border: none;
		padding: 0;
		cursor: pointer;
		color: var(--st-color-orange, #f97316);
		text-decoration: none;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.archive-link:hover {
		text-decoration: underline;
	}

	/* Responsive */
	@media (max-width: 767px) {
		.watchlist__table th,
		.watchlist__table td {
			padding: 0.75rem 0.5rem;
			font-size: 0.8125rem;
		}

		.watchlist__options-grid {
			grid-template-columns: 1fr;
		}

		.archive-stats {
			width: 100%;
			justify-content: space-between;
		}
	}
</style>

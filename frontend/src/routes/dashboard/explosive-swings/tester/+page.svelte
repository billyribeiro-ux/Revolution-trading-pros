<script lang="ts">
	/**
	 * Explosive Swings - ULTIMATE Dashboard
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Apple ICT 7 Principal Engineer Grade - January 2026
	 * State-of-the-art UI/UX with advanced features
	 * 
	 * @version 2.0.0 - Ultimate Edition
	 */
	import { onMount } from 'svelte';
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';

	// SSR data
	let { data } = $props();

	// Filter state
	let selectedFilter = $state('all');
	let activeTab = $state('alerts');

	// Featured Video of the Week
	const featuredVideo = {
		title: "This Week's Market Outlook & Top Swing Setups",
		date: 'January 13, 2026',
		duration: '24:35',
		thumbnail: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
		href: '/dashboard/explosive-swings/video/weekly-outlook-011326',
		description: "Join us for this week's comprehensive breakdown of the hottest swing trade setups. We cover NVDA, TSLA, AMZN, and reveal our top picks for the week ahead.",
		timestamps: [
			{ time: '0:00', label: 'Market Overview' },
			{ time: '5:24', label: 'NVDA Setup Analysis' },
			{ time: '10:15', label: 'TSLA Breakout Play' },
			{ time: '15:40', label: 'AMZN Entry Strategy' },
			{ time: '20:30', label: 'Risk Management Tips' }
		]
	};

	// This Week's Watchlist
	const watchlistTickers = [
		{ symbol: 'NVDA', entry: '$142.50', target: '$158.00', stop: '$136.00', status: 'active', change: '+2.4%' },
		{ symbol: 'TSLA', entry: '$248.00', target: '$275.00', stop: '$235.00', status: 'watching', change: '+1.8%' },
		{ symbol: 'AMZN', entry: '$185.00', target: '$198.00', stop: '$178.00', status: 'active', change: '+0.9%' },
		{ symbol: 'GOOGL', entry: '$175.50', target: '$188.00', stop: '$168.00', status: 'watching', change: '-0.3%' },
		{ symbol: 'AAPL', entry: '$182.00', target: '$195.00', stop: '$175.00', status: 'closed', change: '+3.2%' }
	];

	// Recent Trades
	const recentTrades = [
		{ symbol: 'MSFT', type: 'WIN', profit: '+$2,450', percent: '+8.2%', duration: '5 days', setup: 'Breakout' },
		{ symbol: 'META', type: 'WIN', profit: '+$1,820', percent: '+6.1%', duration: '3 days', setup: 'Momentum' },
		{ symbol: 'AMD', type: 'LOSS', profit: '-$680', percent: '-2.3%', duration: '2 days', setup: 'Reversal' },
		{ symbol: 'NFLX', type: 'WIN', profit: '+$3,100', percent: '+9.5%', duration: '7 days', setup: 'Earnings' }
	];

	// Alerts with tags
	const alerts = [
		{
			id: 1,
			type: 'Trade Alert',
			title: 'NVDA Swing Trade Setup',
			date: 'January 10, 2026 • 2:30 PM ET',
			excerpt: 'Opening swing position on NVDA with bullish momentum. Multi-day hold targeting key resistance levels.',
			href: '/dashboard/explosive-swings/alerts/nvda-swing-011026',
			tags: ['momentum', 'tech', 'breakout'],
			isNew: true
		},
		{
			id: 2,
			type: 'Market Update',
			title: 'Weekly Swing Trade Outlook',
			date: 'January 10, 2026 • 9:00 AM ET',
			excerpt: 'Key swing trade setups for the week ahead. Technical analysis on major tech stocks.',
			href: '/dashboard/explosive-swings/alerts/weekly-outlook-011026',
			tags: ['tech'],
			isNew: true
		},
		{
			id: 3,
			type: 'Trade Alert',
			title: 'TSLA Position Closed - WIN',
			date: 'January 9, 2026 • 3:45 PM ET',
			excerpt: 'Closing TSLA swing trade for +$2,450 profit. Target reached after 5-day hold.',
			href: '/dashboard/explosive-swings/alerts/tsla-close-010926',
			tags: ['momentum'],
			isNew: false
		},
		{
			id: 4,
			type: 'Trade Alert',
			title: 'AMZN Breakout Play',
			date: 'January 9, 2026 • 11:15 AM ET',
			excerpt: 'AMZN breaking above key resistance. Swing trade entry with momentum confirmation.',
			href: '/dashboard/explosive-swings/alerts/amzn-breakout-010926',
			tags: ['breakout', 'momentum', 'tech'],
			isNew: false
		},
		{
			id: 5,
			type: 'Trade Alert',
			title: 'GOOGL Options Swing',
			date: 'January 8, 2026 • 10:00 AM ET',
			excerpt: 'Multi-week options swing trade on GOOGL. Targeting earnings catalyst.',
			href: '/dashboard/explosive-swings/alerts/googl-options-010826',
			tags: ['options', 'earnings', 'tech'],
			isNew: false
		}
	];

	// Filtered alerts
	const filteredAlerts = $derived(
		selectedFilter === 'all'
			? alerts
			: alerts.filter(alert => alert.tags?.includes(selectedFilter))
	);

	// Performance stats
	const stats = {
		winRate: 82,
		totalProfit: 18750,
		totalTrades: 28,
		avgHold: 4.2,
		bestTrade: 3100,
		streak: 5
	};
</script>

<svelte:head>
	<title>Explosive Swings - Ultimate Dashboard | Revolution Trading Pros</title>
</svelte:head>

<TradingRoomHeader 
	roomName="Explosive Swings" 
	startHereUrl="/dashboard/explosive-swings/start-here" 
/>

<div class="ultimate-dashboard">
	<!-- HERO: Featured Video of the Week -->
	<section class="hero-section">
		<div class="hero-badge">📺 THIS WEEK'S FEATURED VIDEO</div>
		<div class="hero-content">
			<div class="hero-video">
				<div class="video-thumbnail" style="background-image: url('{featuredVideo.thumbnail}')">
					<div class="play-button">
						<svg viewBox="0 0 24 24" fill="currentColor">
							<path d="M8 5v14l11-7z"/>
						</svg>
					</div>
					<div class="video-duration">{featuredVideo.duration}</div>
				</div>
			</div>
			<div class="hero-info">
				<h1>{featuredVideo.title}</h1>
				<p class="hero-date">{featuredVideo.date}</p>
				<p class="hero-description">{featuredVideo.description}</p>
				
				<div class="timestamps">
					<h4>Jump to Section</h4>
					<div class="timestamp-list">
						{#each featuredVideo.timestamps as ts}
							<a href="{featuredVideo.href}?t={ts.time}" class="timestamp-item">
								<span class="ts-time">{ts.time}</span>
								<span class="ts-label">{ts.label}</span>
							</a>
						{/each}
					</div>
				</div>
				
				<a href={featuredVideo.href} class="hero-cta">
					Watch Full Video
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M5 12h14M12 5l7 7-7 7"/>
					</svg>
				</a>
			</div>
		</div>
	</section>

	<!-- QUICK STATS BAR -->
	<section class="stats-bar">
		<div class="stat-item">
			<div class="stat-value">{stats.winRate}%</div>
			<div class="stat-label">Win Rate</div>
		</div>
		<div class="stat-divider"></div>
		<div class="stat-item">
			<div class="stat-value">+${stats.totalProfit.toLocaleString()}</div>
			<div class="stat-label">30-Day Profit</div>
		</div>
		<div class="stat-divider"></div>
		<div class="stat-item">
			<div class="stat-value">{stats.totalTrades}</div>
			<div class="stat-label">Total Trades</div>
		</div>
		<div class="stat-divider"></div>
		<div class="stat-item">
			<div class="stat-value">{stats.avgHold} days</div>
			<div class="stat-label">Avg Hold</div>
		</div>
		<div class="stat-divider"></div>
		<div class="stat-item highlight">
			<div class="stat-value">🔥 {stats.streak}</div>
			<div class="stat-label">Win Streak</div>
		</div>
	</section>

	<!-- MAIN CONTENT -->
	<div class="main-content">
		<!-- LEFT: Watchlist & Trades -->
		<div class="content-left">
			<!-- This Week's Watchlist -->
			<section class="card watchlist-card">
				<div class="card-header">
					<h2>📊 This Week's Watchlist</h2>
					<a href="/dashboard/explosive-swings/watchlist" class="view-all">View All →</a>
				</div>
				<div class="watchlist-table">
					<div class="table-header">
						<span>Symbol</span>
						<span>Entry</span>
						<span>Target</span>
						<span>Stop</span>
						<span>Status</span>
					</div>
					{#each watchlistTickers as ticker}
						<div class="table-row">
							<span class="ticker-symbol">
								{ticker.symbol}
								<small class:positive={ticker.change.startsWith('+')} class:negative={ticker.change.startsWith('-')}>{ticker.change}</small>
							</span>
							<span>{ticker.entry}</span>
							<span class="target">{ticker.target}</span>
							<span class="stop">{ticker.stop}</span>
							<span class="status status--{ticker.status}">{ticker.status}</span>
						</div>
					{/each}
				</div>
			</section>

			<!-- Recent Trades -->
			<section class="card trades-card">
				<div class="card-header">
					<h2>📈 Recent Trades</h2>
					<a href="/dashboard/explosive-swings/trade-tracker" class="view-all">Trade Tracker →</a>
				</div>
				<div class="trades-grid">
					{#each recentTrades as trade}
						<div class="trade-item trade--{trade.type.toLowerCase()}">
							<div class="trade-symbol">{trade.symbol}</div>
							<div class="trade-result">
								<span class="trade-type">{trade.type}</span>
								<span class="trade-profit">{trade.profit}</span>
							</div>
							<div class="trade-meta">
								<span>{trade.percent}</span>
								<span>•</span>
								<span>{trade.duration}</span>
								<span>•</span>
								<span class="trade-setup">{trade.setup}</span>
							</div>
						</div>
					{/each}
				</div>
			</section>

			<!-- Performance Charts -->
			<section class="card charts-card">
				<div class="card-header">
					<h2>📉 Performance Analytics</h2>
				</div>
				<div class="charts-grid">
					<!-- Profit Trend -->
					<div class="chart-box">
						<h4>Cumulative Profit (8 Weeks)</h4>
						<svg viewBox="0 0 400 180" class="trend-chart">
							<defs>
								<linearGradient id="profitFill" x1="0%" y1="0%" x2="0%" y2="100%">
									<stop offset="0%" style="stop-color:#22c55e;stop-opacity:0.4" />
									<stop offset="100%" style="stop-color:#22c55e;stop-opacity:0" />
								</linearGradient>
							</defs>
							<path d="M40,140 L85,125 L130,105 L175,90 L220,95 L265,70 L310,55 L355,40 L355,160 L40,160 Z" fill="url(#profitFill)"/>
							<polyline points="40,140 85,125 130,105 175,90 220,95 265,70 310,55 355,40" fill="none" stroke="#22c55e" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
							<circle cx="355" cy="40" r="6" fill="#22c55e"/>
							<text x="355" y="30" text-anchor="middle" class="chart-highlight">$18.7k</text>
						</svg>
					</div>
					<!-- Win/Loss -->
					<div class="chart-box">
						<h4>Setup Performance</h4>
						<div class="donut-container">
							<svg viewBox="0 0 120 120" class="donut-chart">
								<circle cx="60" cy="60" r="45" fill="none" stroke="#e5e7eb" stroke-width="12"/>
								<circle cx="60" cy="60" r="45" fill="none" stroke="#22c55e" stroke-width="12" 
									stroke-dasharray="232" stroke-dashoffset="42" transform="rotate(-90 60 60)"/>
								<text x="60" y="55" text-anchor="middle" class="donut-value">82%</text>
								<text x="60" y="72" text-anchor="middle" class="donut-label">Win Rate</text>
							</svg>
							<div class="donut-legend">
								<div><span class="dot win"></span>Wins: 23</div>
								<div><span class="dot loss"></span>Losses: 5</div>
							</div>
						</div>
					</div>
				</div>
			</section>
		</div>

		<!-- RIGHT: Alerts Feed -->
		<div class="content-right">
			<section class="card alerts-card">
				<div class="card-header">
					<h2>🔔 Alerts & Updates</h2>
				</div>
				
				<!-- Filter Pills -->
				<div class="filter-pills">
					<button class="pill" class:active={selectedFilter === 'all'} onclick={() => selectedFilter = 'all'}>All</button>
					<button class="pill" class:active={selectedFilter === 'momentum'} onclick={() => selectedFilter = 'momentum'}>Momentum</button>
					<button class="pill" class:active={selectedFilter === 'breakout'} onclick={() => selectedFilter = 'breakout'}>Breakout</button>
					<button class="pill" class:active={selectedFilter === 'earnings'} onclick={() => selectedFilter = 'earnings'}>Earnings</button>
					<button class="pill" class:active={selectedFilter === 'options'} onclick={() => selectedFilter = 'options'}>Options</button>
					<button class="pill" class:active={selectedFilter === 'tech'} onclick={() => selectedFilter = 'tech'}>Tech</button>
				</div>

				<!-- Alerts List -->
				<div class="alerts-feed">
					{#each filteredAlerts as alert}
						<a href={alert.href} class="alert-item" class:is-new={alert.isNew}>
							{#if alert.isNew}
								<span class="new-badge">NEW</span>
							{/if}
							<div class="alert-type">{alert.type}</div>
							<h3>{alert.title}</h3>
							<p>{alert.excerpt}</p>
							<div class="alert-footer">
								<span class="alert-date">{alert.date}</span>
								<div class="alert-tags">
									{#each alert.tags as tag}
										<span class="tag">{tag}</span>
									{/each}
								</div>
							</div>
						</a>
					{/each}
				</div>

				<a href="/dashboard/explosive-swings/alerts" class="view-all-btn">
					View All Alerts
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16">
						<path d="M5 12h14M12 5l7 7-7 7"/>
					</svg>
				</a>
			</section>

			<!-- Quick Resources -->
			<section class="card resources-card">
				<div class="card-header">
					<h2>📚 Resources</h2>
				</div>
				<div class="resources-grid">
					<a href="/dashboard/explosive-swings/video-library" class="resource-item">
						<span class="resource-icon">��</span>
						<span>Video Library</span>
					</a>
					<a href="/dashboard/explosive-swings/trade-tracker" class="resource-item">
						<span class="resource-icon">📊</span>
						<span>Trade Tracker</span>
					</a>
					<a href="/guides/swing-entry" class="resource-item">
						<span class="resource-icon">📖</span>
						<span>Entry Guide</span>
					</a>
					<a href="/guides/risk-management" class="resource-item">
						<span class="resource-icon">🛡️</span>
						<span>Risk Management</span>
					</a>
					<a href="/guides/exit-strategies" class="resource-item">
						<span class="resource-icon">🎯</span>
						<span>Exit Strategies</span>
					</a>
					<a href="/dashboard/account" class="resource-item">
						<span class="resource-icon">⚙️</span>
						<span>Alert Settings</span>
					</a>
				</div>
			</section>
		</div>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * ULTIMATE DASHBOARD - State of the Art UI/UX
	 * ═══════════════════════════════════════════════════════════════════════════ */
	
	.ultimate-dashboard {
		background: #f8fafc;
		min-height: 100vh;
	}

	/* HERO SECTION */
	.hero-section {
		background: linear-gradient(135deg, #143E59 0%, #1e5a7e 50%, #0984ae 100%);
		padding: 40px 30px;
		color: #fff;
		text-align: center;
	}

	.hero-badge {
		display: inline-block;
		background: rgba(255, 255, 255, 0.15);
		backdrop-filter: blur(10px);
		padding: 8px 20px;
		border-radius: 50px;
		font-size: 12px;
		font-weight: 700;
		letter-spacing: 0.1em;
		margin-bottom: 30px;
		border: 1px solid rgba(255, 255, 255, 0.2);
	}

	.hero-content {
		display: flex;
		gap: 40px;
		max-width: 1200px;
		margin: 0 auto;
		align-items: flex-start;
		text-align: left;
	}

	.hero-video {
		flex: 0 0 500px;
	}

	.video-thumbnail {
		position: relative;
		width: 100%;
		padding-bottom: 56.25%;
		background-size: cover;
		background-position: center;
		border-radius: 16px;
		overflow: hidden;
		box-shadow: 0 20px 60px rgba(0, 0, 0, 0.4);
		cursor: pointer;
		transition: transform 0.3s ease;
	}

	.video-thumbnail:hover {
		transform: scale(1.02);
	}

	.video-thumbnail::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(to top, rgba(0,0,0,0.6) 0%, transparent 50%);
	}

	.play-button {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 80px;
		height: 80px;
		background: rgba(255, 255, 255, 0.95);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #143E59;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
		transition: all 0.3s ease;
	}

	.play-button svg {
		width: 32px;
		height: 32px;
		margin-left: 4px;
	}

	.video-thumbnail:hover .play-button {
		transform: translate(-50%, -50%) scale(1.1);
		background: #fff;
	}

	.video-duration {
		position: absolute;
		bottom: 15px;
		right: 15px;
		background: rgba(0, 0, 0, 0.8);
		padding: 6px 12px;
		border-radius: 6px;
		font-size: 13px;
		font-weight: 600;
	}

	.hero-info {
		flex: 1;
		text-align: center;
	}

	.hero-info h1 {
		font-size: 32px;
		font-weight: 700;
		margin: 0 0 12px 0;
		font-family: 'Montserrat', sans-serif;
		line-height: 1.3;
	}

	.hero-date {
		font-size: 14px;
		opacity: 0.8;
		margin: 0 0 20px 0;
	}

	.hero-description {
		font-size: 16px;
		line-height: 1.7;
		opacity: 0.95;
		margin: 0 0 30px 0;
	}

	.timestamps {
		background: rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 20px;
		margin-bottom: 30px;
		text-align: center;
	}

	.timestamps h4 {
		font-size: 12px;
		font-weight: 700;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		margin: 0 0 15px 0;
		opacity: 0.8;
	}

	.timestamp-list {
		display: flex;
		flex-wrap: wrap;
		gap: 10px;
		justify-content: center;
	}

	.timestamp-item {
		display: flex;
		align-items: center;
		gap: 8px;
		background: rgba(255, 255, 255, 0.1);
		padding: 8px 14px;
		border-radius: 8px;
		font-size: 13px;
		color: #fff;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.timestamp-item:hover {
		background: rgba(255, 255, 255, 0.2);
	}

	.ts-time {
		font-weight: 700;
		color: #F69532;
	}

	.hero-cta {
		display: inline-flex;
		align-items: center;
		gap: 10px;
		background: #F69532;
		color: #fff;
		padding: 16px 32px;
		border-radius: 12px;
		font-size: 16px;
		font-weight: 700;
		text-decoration: none;
		transition: all 0.3s ease;
		box-shadow: 0 10px 30px rgba(246, 149, 50, 0.4);
	}

	.hero-cta:hover {
		background: #e8850d;
		transform: translateY(-3px);
		box-shadow: 0 15px 40px rgba(246, 149, 50, 0.5);
	}

	.hero-cta svg {
		width: 20px;
		height: 20px;
	}

	@media (max-width: 1024px) {
		.hero-content {
			flex-direction: column;
			text-align: center;
		}

		.hero-video {
			flex: none;
			width: 100%;
			max-width: 600px;
			margin: 0 auto;
		}

		.hero-info {
			text-align: center;
		}
	}

	/* STATS BAR */
	.stats-bar {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 30px;
		background: #fff;
		padding: 25px 30px;
		border-bottom: 1px solid #e5e7eb;
		flex-wrap: wrap;
	}

	.stat-item {
		text-align: center;
	}

	.stat-value {
		font-size: 28px;
		font-weight: 700;
		color: #143E59;
		font-family: 'Montserrat', sans-serif;
	}

	.stat-label {
		font-size: 12px;
		color: #666;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-top: 4px;
	}

	.stat-item.highlight .stat-value {
		color: #F69532;
	}

	.stat-divider {
		width: 1px;
		height: 40px;
		background: #e5e7eb;
	}

	@media (max-width: 768px) {
		.stats-bar {
			gap: 20px;
		}

		.stat-divider {
			display: none;
		}

		.stat-value {
			font-size: 22px;
		}
	}

	/* MAIN CONTENT */
	.main-content {
		display: flex;
		gap: 30px;
		padding: 30px;
		max-width: 1400px;
		margin: 0 auto;
	}

	.content-left {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 30px;
		min-width: 0;
	}

	.content-right {
		width: 420px;
		flex-shrink: 0;
		display: flex;
		flex-direction: column;
		gap: 30px;
	}

	@media (max-width: 1200px) {
		.main-content {
			flex-direction: column;
		}

		.content-right {
			width: 100%;
		}
	}

	/* CARDS */
	.card {
		background: #fff;
		border-radius: 16px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.06);
		overflow: hidden;
	}

	.card-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 20px 25px;
		border-bottom: 1px solid #f0f0f0;
	}

	.card-header h2 {
		font-size: 18px;
		font-weight: 700;
		margin: 0;
		color: #333;
		font-family: 'Montserrat', sans-serif;
		text-align: center;
		flex: 1;
	}

	.view-all {
		font-size: 13px;
		color: #0984ae;
		text-decoration: none;
		font-weight: 600;
		transition: color 0.2s;
	}

	.view-all:hover {
		color: #143E59;
	}

	/* WATCHLIST TABLE */
	.watchlist-table {
		padding: 0 25px 25px;
	}

	.table-header, .table-row {
		display: grid;
		grid-template-columns: 1.2fr 1fr 1fr 1fr 0.8fr;
		gap: 15px;
		padding: 15px 0;
		text-align: center;
	}

	.table-header {
		font-size: 11px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: #888;
		border-bottom: 1px solid #f0f0f0;
	}

	.table-row {
		border-bottom: 1px solid #f5f5f5;
		font-size: 14px;
		align-items: center;
	}

	.table-row:last-child {
		border-bottom: none;
	}

	.ticker-symbol {
		font-weight: 700;
		color: #143E59;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 2px;
	}

	.ticker-symbol small {
		font-size: 11px;
		font-weight: 600;
	}

	.ticker-symbol small.positive {
		color: #22c55e;
	}

	.ticker-symbol small.negative {
		color: #ef4444;
	}

	.target {
		color: #22c55e;
		font-weight: 600;
	}

	.stop {
		color: #ef4444;
		font-weight: 600;
	}

	.status {
		font-size: 11px;
		font-weight: 700;
		text-transform: uppercase;
		padding: 4px 10px;
		border-radius: 20px;
	}

	.status--active {
		background: #dcfce7;
		color: #166534;
	}

	.status--watching {
		background: #fef3c7;
		color: #92400e;
	}

	.status--closed {
		background: #f3f4f6;
		color: #6b7280;
	}

	/* TRADES GRID */
	.trades-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 15px;
		padding: 25px;
	}

	@media (max-width: 640px) {
		.trades-grid {
			grid-template-columns: 1fr;
		}
	}

	.trade-item {
		background: #f8fafc;
		border-radius: 12px;
		padding: 18px;
		text-align: center;
		border: 1px solid #e5e7eb;
		transition: all 0.2s ease;
	}

	.trade-item:hover {
		transform: translateY(-3px);
		box-shadow: 0 8px 25px rgba(0, 0, 0, 0.08);
	}

	.trade-item.trade--win {
		border-left: 4px solid #22c55e;
	}

	.trade-item.trade--loss {
		border-left: 4px solid #ef4444;
	}

	.trade-symbol {
		font-size: 20px;
		font-weight: 700;
		color: #143E59;
		margin-bottom: 8px;
	}

	.trade-result {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 10px;
		margin-bottom: 8px;
	}

	.trade-type {
		font-size: 11px;
		font-weight: 700;
		padding: 3px 8px;
		border-radius: 4px;
	}

	.trade--win .trade-type {
		background: #dcfce7;
		color: #166534;
	}

	.trade--loss .trade-type {
		background: #fee2e2;
		color: #991b1b;
	}

	.trade-profit {
		font-size: 16px;
		font-weight: 700;
	}

	.trade--win .trade-profit {
		color: #22c55e;
	}

	.trade--loss .trade-profit {
		color: #ef4444;
	}

	.trade-meta {
		font-size: 12px;
		color: #666;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
	}

	.trade-setup {
		background: #e5e7eb;
		padding: 2px 8px;
		border-radius: 4px;
		font-weight: 600;
	}

	/* CHARTS */
	.charts-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 25px;
		padding: 25px;
	}

	@media (max-width: 768px) {
		.charts-grid {
			grid-template-columns: 1fr;
		}
	}

	.chart-box {
		text-align: center;
	}

	.chart-box h4 {
		font-size: 13px;
		font-weight: 600;
		color: #666;
		margin: 0 0 15px 0;
	}

	.trend-chart {
		width: 100%;
		height: auto;
	}

	.chart-highlight {
		font-size: 12px;
		font-weight: 700;
		fill: #22c55e;
	}

	.donut-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 15px;
	}

	.donut-chart {
		width: 120px;
		height: 120px;
	}

	.donut-value {
		font-size: 22px;
		font-weight: 700;
		fill: #143E59;
	}

	.donut-label {
		font-size: 10px;
		fill: #666;
	}

	.donut-legend {
		display: flex;
		gap: 20px;
		font-size: 13px;
	}

	.donut-legend .dot {
		display: inline-block;
		width: 10px;
		height: 10px;
		border-radius: 50%;
		margin-right: 6px;
	}

	.donut-legend .dot.win {
		background: #22c55e;
	}

	.donut-legend .dot.loss {
		background: #ef4444;
	}

	/* ALERTS CARD */
	.filter-pills {
		display: flex;
		gap: 8px;
		padding: 15px 25px;
		flex-wrap: wrap;
		justify-content: center;
		border-bottom: 1px solid #f0f0f0;
	}

	.pill {
		background: #f3f4f6;
		border: none;
		padding: 8px 16px;
		border-radius: 20px;
		font-size: 13px;
		font-weight: 600;
		color: #666;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.pill:hover {
		background: #e5e7eb;
	}

	.pill.active {
		background: #143E59;
		color: #fff;
	}

	.alerts-feed {
		padding: 15px 25px;
		display: flex;
		flex-direction: column;
		gap: 15px;
		max-height: 500px;
		overflow-y: auto;
	}

	.alert-item {
		display: block;
		background: #f8fafc;
		border-radius: 12px;
		padding: 18px;
		text-decoration: none;
		color: inherit;
		border: 1px solid #e5e7eb;
		transition: all 0.2s ease;
		position: relative;
		text-align: center;
	}

	.alert-item:hover {
		border-color: #143E59;
		transform: translateY(-2px);
		box-shadow: 0 8px 25px rgba(0, 0, 0, 0.08);
	}

	.alert-item.is-new {
		border-color: #F69532;
		background: #fffbf5;
	}

	.new-badge {
		position: absolute;
		top: -8px;
		right: 15px;
		background: #F69532;
		color: #fff;
		font-size: 10px;
		font-weight: 700;
		padding: 4px 10px;
		border-radius: 10px;
	}

	.alert-type {
		font-size: 11px;
		font-weight: 700;
		text-transform: uppercase;
		color: #0984ae;
		margin-bottom: 8px;
	}

	.alert-item h3 {
		font-size: 15px;
		font-weight: 700;
		margin: 0 0 8px 0;
		color: #333;
	}

	.alert-item p {
		font-size: 13px;
		color: #666;
		line-height: 1.5;
		margin: 0 0 12px 0;
	}

	.alert-footer {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 15px;
		flex-wrap: wrap;
	}

	.alert-date {
		font-size: 11px;
		color: #888;
	}

	.alert-tags {
		display: flex;
		gap: 6px;
		justify-content: center;
	}

	.tag {
		background: #e5e7eb;
		font-size: 10px;
		font-weight: 600;
		padding: 3px 8px;
		border-radius: 4px;
		color: #555;
		text-transform: uppercase;
	}

	.view-all-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		margin: 15px 25px 25px;
		padding: 14px;
		background: #143E59;
		color: #fff;
		border-radius: 10px;
		text-decoration: none;
		font-weight: 600;
		font-size: 14px;
		transition: all 0.2s ease;
	}

	.view-all-btn:hover {
		background: #0f2d42;
	}

	/* RESOURCES */
	.resources-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 12px;
		padding: 20px;
	}

	.resource-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
		padding: 18px 12px;
		background: #f8fafc;
		border-radius: 12px;
		text-decoration: none;
		color: #333;
		font-size: 12px;
		font-weight: 600;
		transition: all 0.2s ease;
		text-align: center;
		border: 1px solid transparent;
	}

	.resource-item:hover {
		background: #fff;
		border-color: #143E59;
		transform: translateY(-3px);
		box-shadow: 0 8px 20px rgba(0, 0, 0, 0.08);
	}

	.resource-icon {
		font-size: 24px;
	}

	@media (max-width: 480px) {
		.resources-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>

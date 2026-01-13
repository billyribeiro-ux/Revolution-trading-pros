<script lang="ts">
	/**
	 * Explosive Swings Dashboard - Trading Room
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Alert service dashboard for Explosive Swings members
	 * Follows Day Trading Room pattern with reusable components
	 *
	 * @version 1.0.0
	 */
	import { onMount } from 'svelte';
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import LatestUpdates from '$lib/components/dashboard/LatestUpdates.svelte';

	// SSR data from +page.server.ts
	let { data } = $props();

	// Filter state
	let selectedFilter = $state('all');
	const filterOptions = ['all', 'momentum', 'earnings', 'options', 'tech-sector', 'breakout'];

	// Latest alerts and updates with tags
	const articles = [
		{
			id: 1,
			type: 'Trade Alert',
			title: 'NVDA Swing Trade Setup',
			date: 'January 10, 2026 at 2:30 PM ET',
			excerpt: 'Opening swing position on NVDA with bullish momentum. Multi-day hold targeting key resistance levels.',
			href: '/dashboard/explosive-swings/alerts/nvda-swing-011026',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false,
			tags: ['momentum', 'tech-sector', 'breakout']
		},
		{
			id: 2,
			type: 'Market Update',
			title: 'Weekly Swing Trade Outlook',
			date: 'January 10, 2026 at 9:00 AM ET',
			excerpt: 'Key swing trade setups for the week ahead. Technical analysis on major tech stocks and market leaders.',
			href: '/dashboard/explosive-swings/alerts/weekly-outlook-011026',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false,
			tags: ['tech-sector']
		},
		{
			id: 3,
			type: 'Trade Alert',
			title: 'TSLA Position Closed',
			date: 'January 9, 2026 at 3:45 PM ET',
			excerpt: 'Closing TSLA swing trade for profit. Target reached after 5-day hold.',
			href: '/dashboard/explosive-swings/alerts/tsla-close-010926',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false,
			tags: ['momentum']
		},
		{
			id: 4,
			type: 'Trade Alert',
			title: 'AMZN Breakout Play',
			date: 'January 9, 2026 at 11:15 AM ET',
			excerpt: 'AMZN breaking above key resistance. Swing trade entry with momentum confirmation.',
			href: '/dashboard/explosive-swings/alerts/amzn-breakout-010926',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false,
			tags: ['breakout', 'momentum', 'tech-sector']
		},
		{
			id: 5,
			type: 'Market Update',
			title: 'Sector Rotation Analysis',
			date: 'January 8, 2026 at 4:30 PM ET',
			excerpt: 'Identifying the strongest sectors for swing trading opportunities. Technology and healthcare showing strength.',
			href: '/dashboard/explosive-swings/alerts/sector-analysis-010826',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false,
			tags: ['tech-sector']
		},
		{
			id: 6,
			type: 'Trade Alert',
			title: 'GOOGL Options Swing',
			date: 'January 8, 2026 at 10:00 AM ET',
			excerpt: 'Multi-week options swing trade on GOOGL. Targeting earnings catalyst with defined risk.',
			href: '/dashboard/explosive-swings/alerts/googl-options-010826',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false,
			tags: ['options', 'earnings', 'tech-sector']
		}
	];

	// Filtered articles based on selected filter
	const filteredArticles = $derived(
		selectedFilter === 'all'
			? articles
			: articles.filter(article => article.tags?.includes(selectedFilter))
	);

	// Google Calendar integration
	onMount(() => {
		const script = document.createElement('script');
		script.src = 'https://apis.google.com/js/api.js';
		script.onload = () => {
			setTimeout(initCalendar, 100);
		};
		script.onerror = () => {
			console.warn('Failed to load Google Calendar API');
		};
		document.head.appendChild(script);
	});

	function initCalendar() {
		const CLIENT_ID = '656301048421-g2s2jvb2pe772mnj8j8it67eirh4jq1f.apps.googleusercontent.com';
		const API_KEY = 'AIzaSyBTC-zYg65B6xD8ezr4gMWCeUNk7y2Hlrw';
		const DISCOVERY_DOCS = ['https://www.googleapis.com/discovery/v1/apis/calendar/v3/rest'];
		const SCOPES = 'https://www.googleapis.com/auth/calendar.readonly';

		// @ts-ignore
		if (typeof gapi === 'undefined' || !gapi.client) {
			console.warn('Google API not loaded yet');
			return;
		}

		try {
			// @ts-ignore
			gapi.load('client', () => {
				// @ts-ignore
				gapi.client.init({
					apiKey: API_KEY,
					clientId: CLIENT_ID,
					discoveryDocs: DISCOVERY_DOCS,
					scope: SCOPES
				}).then(() => {
					// @ts-ignore
					return gapi.client.calendar.events.list({
						'calendarId': 'simpleroptions.com_sabio00har0rd4odbrsa705904@group.calendar.google.com',
						'timeMin': (new Date()).toISOString(),
						'showDeleted': false,
						'singleEvents': true,
						'maxResults': 10,
						'orderBy': 'startTime',
						'fields': 'items(summary,start/dateTime)'
					});
				}).then((response: any) => {
					const dateOptions: Intl.DateTimeFormatOptions = {
						month: 'short',
						day: 'numeric',
						year: 'numeric',
						hour: 'numeric',
						minute: 'numeric',
						timeZoneName: 'short'
					};
					const container = document.querySelector('.room-sched');
					if (container && response.result.items) {
						for (let i = 0; i < response.result.items.length; i++) {
							const eventStart = new Date(response.result.items[i].start.dateTime);
							const eventHtml = `<h4>${response.result.items[i].summary}</h4><span>${eventStart.toLocaleString('en-US', dateOptions)}</span>`;
							container.innerHTML += eventHtml;
						}
					}
				}).catch((error: any) => {
					console.warn('Calendar Error:', error);
				});
			});
		} catch (error) {
			console.warn('Failed to initialize Google Calendar:', error);
		}
	}
</script>

<svelte:head>
	<title>Explosive Swings TESTER | Revolution Trading Pros</title>
	<script src="https://apis.google.com/js/api.js"></script>
</svelte:head>

<TradingRoomHeader 
	roomName="Explosive Swings" 
	startHereUrl="/dashboard/explosive-swings/start-here" 
/>

<!-- DASHBOARD CONTENT -->
<div class="dashboard__content">
	<div class="dashboard__content-main">

		<!-- PINNED WEEKLY WATCHLIST BANNER -->
		<section class="dashboard__content-section-member">
			<div class="watchlist-banner">
				<div class="watchlist-banner__icon">📊</div>
				<div class="watchlist-banner__content">
					<h3>This Week's Watchlist</h3>
					<p>Top swing trade setups for the week: NVDA, TSLA, AMZN, GOOGL, AAPL</p>
				</div>
				<a href="/dashboard/explosive-swings/watchlist" class="watchlist-banner__btn">View Full List</a>
			</div>
		</section>

		<!-- WELCOME SECTION -->
		<section class="dashboard__content-section-member">
			<div class="welcome-banner">
				<h2>Welcome to Explosive Swings</h2>
				<p>Get high-probability swing trade alerts on stocks and options with explosive momentum. All trades include detailed entry, exit, and risk management strategies for multi-day holds.</p>
			</div>
		</section>

		<!-- LATEST ALERTS SECTION WITH FILTERS -->
		<section class="dashboard__content-section u--background-color-white">
			<div class="alerts-header">
				<h3 class="section-heading">Latest Alerts & Updates</h3>
				<div class="filter-tabs">
					<button 
						class="filter-tab" 
						class:active={selectedFilter === 'all'}
						onclick={() => selectedFilter = 'all'}
					>
						All
					</button>
					<button 
						class="filter-tab" 
						class:active={selectedFilter === 'momentum'}
						onclick={() => selectedFilter = 'momentum'}
					>
						Momentum
					</button>
					<button 
						class="filter-tab" 
						class:active={selectedFilter === 'earnings'}
						onclick={() => selectedFilter = 'earnings'}
					>
						Earnings Play
					</button>
					<button 
						class="filter-tab" 
						class:active={selectedFilter === 'options'}
						onclick={() => selectedFilter = 'options'}
					>
						Options
					</button>
					<button 
						class="filter-tab" 
						class:active={selectedFilter === 'tech-sector'}
						onclick={() => selectedFilter = 'tech-sector'}
					>
						Tech Sector
					</button>
					<button 
						class="filter-tab" 
						class:active={selectedFilter === 'breakout'}
						onclick={() => selectedFilter = 'breakout'}
					>
						Breakout
					</button>
				</div>
			</div>
			<LatestUpdates items={filteredArticles} title="" roomSlug="explosive-swings" />
		</section>

		<!-- PERFORMANCE METRICS SECTION -->
		<section class="dashboard__content-section u--background-color-white">
			<h3 class="section-heading">Recent Performance</h3>
			<div class="metrics-grid">
				<div class="metric-card">
					<div class="metric-value">82%</div>
					<div class="metric-label">Win Rate (30 Days)</div>
				</div>
				<div class="metric-card">
					<div class="metric-value">+$18,750</div>
					<div class="metric-label">Total Profit (30 Days)</div>
				</div>
				<div class="metric-card">
					<div class="metric-value">28</div>
					<div class="metric-label">Total Trades</div>
				</div>
				<div class="metric-card">
					<div class="metric-value">3.2:1</div>
					<div class="metric-label">Avg Risk/Reward</div>
				</div>
			</div>
		</section>

		<!-- PERFORMANCE CHARTS SECTION -->
		<section class="dashboard__content-section">
			<div class="charts-grid">
				<!-- Profit Trend Chart -->
				<div class="chart-card">
					<h4 class="chart-title">Profit Trend (Last 8 Weeks)</h4>
					<div class="chart-container">
						<svg viewBox="0 0 400 200" class="trend-chart">
							<!-- Grid lines -->
							<line x1="50" y1="20" x2="50" y2="170" stroke="#e0e0e0" stroke-width="1"/>
							<line x1="50" y1="170" x2="380" y2="170" stroke="#e0e0e0" stroke-width="1"/>
							<line x1="50" y1="95" x2="380" y2="95" stroke="#e0e0e0" stroke-width="1" stroke-dasharray="4"/>
							<line x1="50" y1="45" x2="380" y2="45" stroke="#e0e0e0" stroke-width="1" stroke-dasharray="4"/>
							<line x1="50" y1="130" x2="380" y2="130" stroke="#e0e0e0" stroke-width="1" stroke-dasharray="4"/>
							
							<!-- Y-axis labels -->
							<text x="45" y="175" text-anchor="end" class="chart-label">$0</text>
							<text x="45" y="135" text-anchor="end" class="chart-label">$5k</text>
							<text x="45" y="100" text-anchor="end" class="chart-label">$10k</text>
							<text x="45" y="50" text-anchor="end" class="chart-label">$20k</text>
							
							<!-- Gradient fill -->
							<defs>
								<linearGradient id="profitGradient" x1="0%" y1="0%" x2="0%" y2="100%">
									<stop offset="0%" style="stop-color:#22c55e;stop-opacity:0.3" />
									<stop offset="100%" style="stop-color:#22c55e;stop-opacity:0" />
								</linearGradient>
							</defs>
							
							<!-- Area fill -->
							<path d="M70,140 L115,120 L160,100 L205,85 L250,90 L295,65 L340,50 L340,170 L70,170 Z" fill="url(#profitGradient)"/>
							
							<!-- Trend line -->
							<polyline 
								points="70,140 115,120 160,100 205,85 250,90 295,65 340,50" 
								fill="none" 
								stroke="#22c55e" 
								stroke-width="3"
								stroke-linecap="round"
								stroke-linejoin="round"
							/>
							
							<!-- Data points -->
							<circle cx="70" cy="140" r="5" fill="#22c55e" class="data-point"/>
							<circle cx="115" cy="120" r="5" fill="#22c55e" class="data-point"/>
							<circle cx="160" cy="100" r="5" fill="#22c55e" class="data-point"/>
							<circle cx="205" cy="85" r="5" fill="#22c55e" class="data-point"/>
							<circle cx="250" cy="90" r="5" fill="#22c55e" class="data-point"/>
							<circle cx="295" cy="65" r="5" fill="#22c55e" class="data-point"/>
							<circle cx="340" cy="50" r="5" fill="#22c55e" class="data-point"/>
							
							<!-- X-axis labels -->
							<text x="70" y="188" text-anchor="middle" class="chart-label">W1</text>
							<text x="115" y="188" text-anchor="middle" class="chart-label">W2</text>
							<text x="160" y="188" text-anchor="middle" class="chart-label">W3</text>
							<text x="205" y="188" text-anchor="middle" class="chart-label">W4</text>
							<text x="250" y="188" text-anchor="middle" class="chart-label">W5</text>
							<text x="295" y="188" text-anchor="middle" class="chart-label">W6</text>
							<text x="340" y="188" text-anchor="middle" class="chart-label">W7</text>
						</svg>
					</div>
				</div>

				<!-- Setup Success Pie Chart -->
				<div class="chart-card">
					<h4 class="chart-title">Setup Success Breakdown</h4>
					<div class="chart-container pie-container">
						<svg viewBox="0 0 200 200" class="pie-chart">
							<!-- Pie segments - Breakouts 35%, Momentum 28%, Earnings 22%, Reversals 15% -->
							<circle cx="100" cy="100" r="80" fill="transparent" stroke="#22c55e" stroke-width="40" 
								stroke-dasharray="175.93 326.73" stroke-dashoffset="0" class="pie-segment"/>
							<circle cx="100" cy="100" r="80" fill="transparent" stroke="#3b82f6" stroke-width="40" 
								stroke-dasharray="140.74 361.92" stroke-dashoffset="-175.93" class="pie-segment"/>
							<circle cx="100" cy="100" r="80" fill="transparent" stroke="#f59e0b" stroke-width="40" 
								stroke-dasharray="110.58 392.08" stroke-dashoffset="-316.67" class="pie-segment"/>
							<circle cx="100" cy="100" r="80" fill="transparent" stroke="#ef4444" stroke-width="40" 
								stroke-dasharray="75.4 427.26" stroke-dashoffset="-427.25" class="pie-segment"/>
							
							<!-- Center circle for donut effect -->
							<circle cx="100" cy="100" r="50" fill="#fff"/>
							<text x="100" y="95" text-anchor="middle" class="pie-center-value">82%</text>
							<text x="100" y="115" text-anchor="middle" class="pie-center-label">Win Rate</text>
						</svg>
						<div class="pie-legend">
							<div class="legend-item">
								<span class="legend-dot" style="background: #22c55e;"></span>
								<span>Breakouts (35%)</span>
							</div>
							<div class="legend-item">
								<span class="legend-dot" style="background: #3b82f6;"></span>
								<span>Momentum (28%)</span>
							</div>
							<div class="legend-item">
								<span class="legend-dot" style="background: #f59e0b;"></span>
								<span>Earnings (22%)</span>
							</div>
							<div class="legend-item">
								<span class="legend-dot" style="background: #ef4444;"></span>
								<span>Reversals (15%)</span>
							</div>
						</div>
					</div>
				</div>
			</div>
		</section>

	</div>

	<!-- SIDEBAR -->
	<aside class="dashboard__content-sidebar">
		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">
				Alert Schedule
				<p class="pssubject" style="font-size: 10px;margin-top: 15px;text-transform: initial;text-align: center;">Schedule is subject to change.</p>
			</h4>
			<div class="script-container">
				<div class="room-sched"></div>
			</div>
		</section>

		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">My Alerts</h4>
			<ul class="link-list">
				<li><a href="/dashboard/explosive-swings/favorites">⭐ Favorites (3)</a></li>
				<li><a href="/dashboard/explosive-swings/alerts">📋 All Alerts</a></li>
				<li><a href="/dashboard/explosive-swings/video-library">🎥 Video Library</a></li>
				<li><a href="/dashboard/explosive-swings/trade-tracker">📊 Trade Tracker</a></li>
			</ul>
		</section>

		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">Strategy Guides</h4>
			<ul class="link-list">
				<li><a href="/guides/swing-entry">📖 Entry Strategies</a></li>
				<li><a href="/guides/risk-management">🛡️ Risk Management</a></li>
				<li><a href="/guides/exit-strategies">🎯 Exit Strategies</a></li>
				<li><a href="/guides/position-sizing">💰 Position Sizing</a></li>
			</ul>
		</section>

		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">Quick Links</h4>
			<ul class="link-list">
				<li><a href="https://intercom.help/simpler-trading/en/" target="_blank">Support</a></li>
				<li><a href="/tutorials" target="_blank">Platform Tutorials</a></li>
				<li><a href="/blog" target="_blank">Trading Blog</a></li>
			</ul>
		</section>

		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">Alert Settings</h4>
			<ul class="link-list">
				<li><a href="/dashboard/account">Notification Preferences</a></li>
				<li><a href="/dashboard/account">SMS Alerts</a></li>
				<li><a href="/dashboard/account">Email Alerts</a></li>
			</ul>
		</section>
	</aside>
</div>

<style>
	.dashboard__content {
		display: flex;
		flex-flow: row nowrap;
	}

	.dashboard__content-main {
		border-right: 1px solid #dbdbdb;
		flex: 1 1 auto;
		min-width: 0;
	}

	.dashboard__content-section-member {
		padding: 30px 20px;
	}

	@media screen and (min-width: 1024px) {
		.dashboard__content-section-member {
			padding: 30px;
		}
	}

	@media screen and (min-width: 1440px) {
		.dashboard__content-section-member {
			padding: 40px;
		}
	}

	/* Pinned Watchlist Banner */
	.watchlist-banner {
		background: linear-gradient(135deg, #F69532 0%, #f7941d 100%);
		color: #fff;
		padding: 25px 30px;
		border-radius: 8px;
		box-shadow: 0 4px 20px rgba(246, 149, 50, 0.3);
		display: flex;
		align-items: center;
		gap: 20px;
		margin-bottom: 20px;
	}

	.watchlist-banner__icon {
		font-size: 48px;
		line-height: 1;
		flex-shrink: 0;
	}

	.watchlist-banner__content {
		flex: 1;
	}

	.watchlist-banner__content h3 {
		margin: 0 0 8px 0;
		font-size: 24px;
		font-weight: 700;
		font-family: 'Montserrat', sans-serif;
	}

	.watchlist-banner__content p {
		margin: 0;
		font-size: 15px;
		opacity: 0.95;
		line-height: 1.5;
	}

	.watchlist-banner__btn {
		background: #fff;
		color: #F69532;
		padding: 12px 24px;
		border-radius: 6px;
		font-weight: 600;
		text-decoration: none;
		font-size: 14px;
		transition: all 0.2s ease;
		flex-shrink: 0;
		white-space: nowrap;
	}

	.watchlist-banner__btn:hover {
		background: #f5f5f5;
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
	}

	@media (max-width: 768px) {
		.watchlist-banner {
			flex-direction: column;
			text-align: center;
			gap: 15px;
		}

		.watchlist-banner__btn {
			width: 100%;
		}
	}

	.welcome-banner {
		background: linear-gradient(135deg, #143E59 0%, #0984ae 100%);
		color: #fff;
		padding: 40px;
		border-radius: 8px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
	}

	.welcome-banner h2 {
		margin: 0 0 15px 0;
		font-size: 32px;
		font-weight: 600;
		font-family: 'Montserrat', sans-serif;
	}

	.welcome-banner p {
		margin: 0;
		font-size: 16px;
		line-height: 1.6;
		opacity: 0.95;
	}

	.dashboard__content-section {
		padding: 30px 20px;
		overflow-x: auto;
		overflow-y: hidden;
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

	.u--background-color-white {
		background-color: #fff !important;
	}

	/* Alerts Header with Filters */
	.alerts-header {
		margin-bottom: 30px;
	}

	.section-heading {
		margin: 0 0 20px 0;
		font-size: 24px;
		font-weight: 600;
		color: #333;
		font-family: 'Montserrat', sans-serif;
	}

	.filter-tabs {
		display: flex;
		gap: 10px;
		flex-wrap: wrap;
	}

	.filter-tab {
		background: #f5f5f5;
		color: #666;
		border: 2px solid transparent;
		padding: 10px 20px;
		border-radius: 25px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
		font-family: 'Open Sans', sans-serif;
	}

	.filter-tab:hover {
		background: #e8e8e8;
		color: #333;
	}

	.filter-tab.active {
		background: #143E59;
		color: #fff;
		border-color: #143E59;
	}

	@media (max-width: 640px) {
		.filter-tabs {
			gap: 8px;
		}

		.filter-tab {
			padding: 8px 16px;
			font-size: 13px;
		}
	}

	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(1, 1fr);
		gap: 20px;
	}

	@media screen and (min-width: 640px) {
		.metrics-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media screen and (min-width: 1024px) {
		.metrics-grid {
			grid-template-columns: repeat(4, 1fr);
		}
	}

	.metric-card {
		background: #f8f9fa;
		padding: 25px;
		border-radius: 8px;
		text-align: center;
		border: 1px solid #e0e0e0;
		transition: all 0.2s ease;
	}

	.metric-card:hover {
		transform: translateY(-2px);
		box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
	}

	.metric-value {
		font-size: 32px;
		font-weight: 700;
		color: #143E59;
		margin-bottom: 8px;
		font-family: 'Montserrat', sans-serif;
	}

	.metric-label {
		font-size: 14px;
		color: #666;
		font-weight: 500;
	}

	/* Charts Grid */
	.charts-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 30px;
	}

	@media (min-width: 1024px) {
		.charts-grid {
			grid-template-columns: 1fr 1fr;
		}
	}

	.chart-card {
		background: #fff;
		border-radius: 12px;
		padding: 25px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
		border: 1px solid #e5e7eb;
	}

	.chart-title {
		margin: 0 0 20px 0;
		font-size: 18px;
		font-weight: 600;
		color: #333;
		font-family: 'Montserrat', sans-serif;
	}

	.chart-container {
		width: 100%;
	}

	.trend-chart {
		width: 100%;
		height: auto;
	}

	.chart-label {
		font-size: 11px;
		fill: #666;
		font-family: 'Open Sans', sans-serif;
	}

	.data-point {
		transition: r 0.2s ease;
		cursor: pointer;
	}

	.data-point:hover {
		r: 8;
	}

	/* Pie Chart */
	.pie-container {
		display: flex;
		align-items: center;
		gap: 30px;
		flex-wrap: wrap;
		justify-content: center;
	}

	.pie-chart {
		width: 180px;
		height: 180px;
		transform: rotate(-90deg);
	}

	.pie-segment {
		transition: opacity 0.2s ease;
	}

	.pie-segment:hover {
		opacity: 0.8;
	}

	.pie-center-value {
		font-size: 24px;
		font-weight: 700;
		fill: #143E59;
		font-family: 'Montserrat', sans-serif;
		transform: rotate(90deg);
		transform-origin: 100px 100px;
	}

	.pie-center-label {
		font-size: 12px;
		fill: #666;
		font-family: 'Open Sans', sans-serif;
		transform: rotate(90deg);
		transform-origin: 100px 100px;
	}

	.pie-legend {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 10px;
		font-size: 14px;
		color: #333;
		font-family: 'Open Sans', sans-serif;
	}

	.legend-dot {
		width: 12px;
		height: 12px;
		border-radius: 50%;
		flex-shrink: 0;
	}

	@media (max-width: 640px) {
		.pie-container {
			flex-direction: column;
		}

		.pie-legend {
			flex-direction: row;
			flex-wrap: wrap;
			justify-content: center;
			gap: 15px;
		}
	}

	.dashboard__content-sidebar {
		display: block;
		width: 260px;
		flex: 0 0 auto;
		margin-top: -1px;
		background: #fff;
		border-right: 1px solid #dbdbdb;
		border-top: 1px solid #dbdbdb;
		font-family: 'Open Sans', sans-serif;
		font-size: 14px;
		line-height: 1.6;
	}

	.dashboard__content-sidebar {
		display: none;
	}

	@media (min-width: 1080px) {
		.dashboard__content-sidebar {
			display: block;
		}
	}

	.content-sidebar__section {
		padding: 20px 30px 20px 20px;
		border-bottom: 1px solid #dbdbdb;
	}

	.content-sidebar__heading {
		padding: 15px 20px;
		margin: -20px -30px 20px -20px;
		font-size: 14px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: #333;
		background: #ededed;
		border-bottom: 1px solid #dbdbdb;
		line-height: 1.4;
	}

	.pssubject {
		font-size: 10px;
		margin-top: 15px;
		text-transform: initial;
	}

	.script-container {
		margin: 0;
	}

	.room-sched {
		margin: 0;
	}

	.link-list {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.link-list li {
		margin-bottom: 12px;
	}

	.link-list a {
		color: #0984ae;
		text-decoration: none;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
		transition: color 0.2s;
	}

	.link-list a:hover {
		color: #076787;
		text-decoration: underline;
	}

	@media (min-width: 993px) {
		.dashboard__content {
			flex-direction: row;
		}
	}
</style>

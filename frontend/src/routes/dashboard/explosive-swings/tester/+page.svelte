<script lang="ts">
	/**
	 * Explosive Swings - ULTIMATE Dashboard v2
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Member-first design - What do traders need?
	 * 1. This week's video breakdown
	 * 2. The exact trade plan (entries, targets, stops, options)
	 * 
	 * @version 2.1.0
	 */
	import { onMount } from 'svelte';
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';

	let { data } = $props();

	// Hero tabs: Video or Entries
	let heroTab = $state<'video' | 'entries'>('video');

	// Alert filters
	let selectedFilter = $state('all');

	// This Week's Featured Content
	const weeklyContent = {
		title: "Week of January 13, 2026",
		videoTitle: "Weekly Breakdown: Top Swing Setups",
		videoUrl: "https://player.vimeo.com/video/123456789",
		thumbnail: 'https://placehold.co/1280x720/143E59/FFFFFF/png?text=Weekly+Video+Breakdown',
		duration: '24:35',
		publishedDate: 'January 13, 2026 at 9:00 AM ET'
	};

	// Trade Plan - The Google Sheet equivalent
	const tradePlan = [
		{
			ticker: 'NVDA',
			bias: 'BULLISH',
			entry: '$142.50',
			target1: '$148.00',
			target2: '$152.00',
			target3: '$158.00',
			runner: '$165.00+',
			stop: '$136.00',
			optionsStrike: '$145 Call',
			optionsExp: 'Jan 24, 2026',
			notes: 'Breakout above consolidation. Wait for pullback to entry.'
		},
		{
			ticker: 'TSLA',
			bias: 'BULLISH',
			entry: '$248.00',
			target1: '$255.00',
			target2: '$265.00',
			target3: '$275.00',
			runner: '$290.00+',
			stop: '$235.00',
			optionsStrike: '$250 Call',
			optionsExp: 'Jan 31, 2026',
			notes: 'Momentum building. Earnings catalyst ahead.'
		},
		{
			ticker: 'AMZN',
			bias: 'BULLISH',
			entry: '$185.00',
			target1: '$190.00',
			target2: '$195.00',
			target3: '$198.00',
			runner: '$205.00+',
			stop: '$178.00',
			optionsStrike: '$185 Call',
			optionsExp: 'Jan 24, 2026',
			notes: 'Breaking resistance. Strong volume confirmation.'
		},
		{
			ticker: 'GOOGL',
			bias: 'NEUTRAL',
			entry: '$175.50',
			target1: '$180.00',
			target2: '$185.00',
			target3: '$188.00',
			runner: '$195.00+',
			stop: '$168.00',
			optionsStrike: '$177.50 Call',
			optionsExp: 'Feb 7, 2026',
			notes: 'Watching for breakout. Not triggered yet.'
		},
		{
			ticker: 'META',
			bias: 'BULLISH',
			entry: '$585.00',
			target1: '$600.00',
			target2: '$615.00',
			target3: '$630.00',
			runner: '$650.00+',
			stop: '$565.00',
			optionsStrike: '$590 Call',
			optionsExp: 'Jan 24, 2026',
			notes: 'Strong trend. Buy dips to support.'
		},
		{
			ticker: 'AMD',
			bias: 'BEARISH',
			entry: '$125.00',
			target1: '$120.00',
			target2: '$115.00',
			target3: '$110.00',
			runner: '$100.00',
			stop: '$132.00',
			optionsStrike: '$122 Put',
			optionsExp: 'Jan 31, 2026',
			notes: 'Breakdown in progress. Short on bounces.'
		}
	];

	// Quick Stats
	const stats = {
		winRate: 82,
		weeklyProfit: '+$4,850',
		activeTrades: 4,
		closedThisWeek: 2
	};

	// Recent Alerts
	const alerts = [
		{
			id: 1,
			type: 'ENTRY',
			ticker: 'NVDA',
			title: 'Opening NVDA Swing Position',
			time: 'Today at 10:32 AM',
			message: 'Entering NVDA at $142.50. First target $148, stop at $136. See trade plan for full details.',
			isNew: true
		},
		{
			id: 2,
			type: 'UPDATE',
			ticker: 'TSLA',
			title: 'TSLA Approaching Entry Zone',
			time: 'Today at 9:15 AM',
			message: 'TSLA pulling back to our entry zone. Be ready. Will alert when triggered.',
			isNew: true
		},
		{
			id: 3,
			type: 'EXIT',
			ticker: 'MSFT',
			title: 'Closing MSFT for +8.2%',
			time: 'Yesterday at 3:45 PM',
			message: 'Taking profits on MSFT. Hit second target. +$2,450 on this trade.',
			isNew: false
		},
		{
			id: 4,
			type: 'ENTRY',
			ticker: 'META',
			title: 'META Entry Triggered',
			time: 'Yesterday at 11:20 AM',
			message: 'META hit our entry at $585. Position active. Targets in trade plan.',
			isNew: false
		},
		{
			id: 5,
			type: 'UPDATE',
			ticker: 'AMD',
			title: 'AMD Short Setup Active',
			time: 'Jan 10 at 2:30 PM',
			message: 'Bearish setup triggered on AMD. Short at $125 with stop at $132.',
			isNew: false
		}
	];

	// Filter alerts
	const filteredAlerts = $derived(
		selectedFilter === 'all' 
			? alerts 
			: alerts.filter(a => a.type.toLowerCase() === selectedFilter)
	);

	// Latest Updates - Video updates for entries/exits
	const latestUpdates = [
		{
			id: 1,
			title: 'NVDA Entry Alert - Opening Swing Position',
			date: 'January 13, 2026 at 10:32 AM ET',
			excerpt: 'Entering NVDA at $142.50. Watch this video breakdown for entry reasoning, targets, and risk management.',
			href: '/dashboard/explosive-swings/updates/nvda-entry-011326',
			image: 'https://placehold.co/640x360/22c55e/FFFFFF/png?text=NVDA+ENTRY',
			isVideo: true,
			duration: '8:45'
		},
		{
			id: 2,
			title: 'MSFT Exit - Closing for +8.2% Profit',
			date: 'January 12, 2026 at 3:45 PM ET',
			excerpt: 'Taking profits on MSFT swing trade. Hit second target. Watch the exit strategy breakdown and lessons learned.',
			href: '/dashboard/explosive-swings/updates/msft-exit-011226',
			image: 'https://placehold.co/640x360/3b82f6/FFFFFF/png?text=MSFT+EXIT',
			isVideo: true,
			duration: '6:20'
		},
		{
			id: 3,
			title: 'META Entry - Momentum Play Setup',
			date: 'January 11, 2026 at 11:20 AM ET',
			excerpt: 'META triggered our entry at $585. Full breakdown of the momentum setup and what to watch for.',
			href: '/dashboard/explosive-swings/updates/meta-entry-011126',
			image: 'https://placehold.co/640x360/22c55e/FFFFFF/png?text=META+ENTRY',
			isVideo: true,
			duration: '7:15'
		},
		{
			id: 4,
			title: 'TSLA Update - Approaching Entry Zone',
			date: 'January 11, 2026 at 9:15 AM ET',
			excerpt: 'TSLA pulling back to our entry zone. Technical analysis and what levels to watch.',
			href: '/dashboard/explosive-swings/updates/tsla-update-011126',
			image: 'https://placehold.co/640x360/f59e0b/FFFFFF/png?text=TSLA+UPDATE',
			isVideo: true,
			duration: '5:30'
		},
		{
			id: 5,
			title: 'AMD Short Setup - Bearish Breakdown',
			date: 'January 10, 2026 at 2:30 PM ET',
			excerpt: 'AMD short position triggered at $125. Breakdown of the bearish setup and downside targets.',
			href: '/dashboard/explosive-swings/updates/amd-short-011026',
			image: 'https://placehold.co/640x360/ef4444/FFFFFF/png?text=AMD+SHORT',
			isVideo: true,
			duration: '9:10'
		},
		{
			id: 6,
			title: 'AMZN Entry - Breakout Confirmation',
			date: 'January 9, 2026 at 11:15 AM ET',
			excerpt: 'AMZN breaking above key resistance. Entry at $185 with full technical breakdown.',
			href: '/dashboard/explosive-swings/updates/amzn-entry-010926',
			image: 'https://placehold.co/640x360/22c55e/FFFFFF/png?text=AMZN+ENTRY',
			isVideo: true,
			duration: '7:45'
		}
	];
</script>

<svelte:head>
	<title>Explosive Swings Dashboard | Revolution Trading Pros</title>
</svelte:head>

<TradingRoomHeader 
	roomName="Explosive Swings" 
	startHereUrl="/dashboard/explosive-swings/start-here" 
/>

<div class="dashboard">
	<!-- ═══════════════════════════════════════════════════════════════════════════
	     HERO: This Week's Content with Tabs (Video | Entries)
	     ═══════════════════════════════════════════════════════════════════════════ -->
	<section class="hero">
		<div class="hero-header">
			<h1>{weeklyContent.title}</h1>
			<div class="hero-tabs">
				<button 
					class="hero-tab" 
					class:active={heroTab === 'video'}
					onclick={() => heroTab = 'video'}
				>
					<svg viewBox="0 0 24 24" fill="currentColor" width="18" height="18">
						<path d="M8 5v14l11-7z"/>
					</svg>
					Video Breakdown
				</button>
				<button 
					class="hero-tab" 
					class:active={heroTab === 'entries'}
					onclick={() => heroTab = 'entries'}
				>
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18">
						<rect x="3" y="3" width="18" height="18" rx="2"/>
						<path d="M3 9h18M9 21V9"/>
					</svg>
					Trade Plan & Entries
				</button>
			</div>
		</div>

		<div class="hero-content">
			{#if heroTab === 'video'}
				<!-- VIDEO TAB -->
				<div class="video-container">
					<div class="video-player" style="background-image: url('{weeklyContent.thumbnail}')">
						<div class="video-overlay">
							<button class="play-btn" aria-label="Play video">
								<svg viewBox="0 0 24 24" fill="currentColor">
									<path d="M8 5v14l11-7z"/>
								</svg>
							</button>
						</div>
						<div class="video-duration">{weeklyContent.duration}</div>
					</div>
					<div class="video-info">
						<h2>{weeklyContent.videoTitle}</h2>
						<p>Published {weeklyContent.publishedDate}</p>
						<a href="/dashboard/explosive-swings/video/weekly" class="watch-btn">
							Watch Full Video
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18">
								<path d="M5 12h14M12 5l7 7-7 7"/>
							</svg>
						</a>
					</div>
				</div>
			{:else}
				<!-- ENTRIES TAB - Trade Plan Sheet -->
				<div class="entries-container">
					<div class="entries-header">
						<h2>This Week's Trade Plan</h2>
						<p>Complete breakdown with entries, targets, stops, and options plays</p>
					</div>
					<div class="trade-sheet-wrapper">
						<table class="trade-sheet">
							<thead>
								<tr>
									<th>Ticker</th>
									<th>Bias</th>
									<th>Entry</th>
									<th>Target 1</th>
									<th>Target 2</th>
									<th>Target 3</th>
									<th>Runner</th>
									<th>Stop</th>
									<th>Options</th>
									<th>Exp</th>
								</tr>
							</thead>
							<tbody>
								{#each tradePlan as trade}
									<tr>
										<td class="ticker-cell">
											<strong>{trade.ticker}</strong>
										</td>
										<td>
											<span class="bias bias--{trade.bias.toLowerCase()}">{trade.bias}</span>
										</td>
										<td class="entry-cell">{trade.entry}</td>
										<td class="target-cell">{trade.target1}</td>
										<td class="target-cell">{trade.target2}</td>
										<td class="target-cell">{trade.target3}</td>
										<td class="runner-cell">{trade.runner}</td>
										<td class="stop-cell">{trade.stop}</td>
										<td class="options-cell">{trade.optionsStrike}</td>
										<td class="exp-cell">{trade.optionsExp}</td>
									</tr>
									<tr class="notes-row">
										<td colspan="10">
											<span class="notes-label">Notes:</span> {trade.notes}
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
					<div class="sheet-footer">
						<a href="https://docs.google.com/spreadsheets/d/your-sheet-id" target="_blank" class="google-sheet-link">
							<svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor">
								<path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zM7 17h2v-7H7v7zm4 0h2V7h-2v10zm4 0h2v-4h-2v4z"/>
							</svg>
							Open in Google Sheets
						</a>
					</div>
				</div>
			{/if}
		</div>
	</section>

	<!-- ═══════════════════════════════════════════════════════════════════════════
	     QUICK STATS
	     ═══════════════════════════════════════════════════════════════════════════ -->
	<section class="stats-bar">
		<div class="stat">
			<span class="stat-value">{stats.winRate}%</span>
			<span class="stat-label">Win Rate</span>
		</div>
		<div class="stat">
			<span class="stat-value green">{stats.weeklyProfit}</span>
			<span class="stat-label">This Week</span>
		</div>
		<div class="stat">
			<span class="stat-value">{stats.activeTrades}</span>
			<span class="stat-label">Active Trades</span>
		</div>
		<div class="stat">
			<span class="stat-value">{stats.closedThisWeek}</span>
			<span class="stat-label">Closed This Week</span>
		</div>
	</section>

	<!-- ═══════════════════════════════════════════════════════════════════════════
	     MAIN CONTENT: Alerts + Sidebar
	     ═══════════════════════════════════════════════════════════════════════════ -->
	<div class="main-grid">
		<!-- ALERTS FEED -->
		<section class="alerts-section">
			<div class="section-header">
				<h2>Live Alerts</h2>
				<div class="filter-pills">
					<button class="pill" class:active={selectedFilter === 'all'} onclick={() => selectedFilter = 'all'}>All</button>
					<button class="pill" class:active={selectedFilter === 'entry'} onclick={() => selectedFilter = 'entry'}>Entries</button>
					<button class="pill" class:active={selectedFilter === 'exit'} onclick={() => selectedFilter = 'exit'}>Exits</button>
					<button class="pill" class:active={selectedFilter === 'update'} onclick={() => selectedFilter = 'update'}>Updates</button>
				</div>
			</div>

			<div class="alerts-list">
				{#each filteredAlerts as alert}
					<div class="alert-card" class:is-new={alert.isNew}>
						{#if alert.isNew}
							<span class="new-badge">NEW</span>
						{/if}
						<div class="alert-header">
							<span class="alert-type alert-type--{alert.type.toLowerCase()}">{alert.type}</span>
							<span class="alert-ticker">{alert.ticker}</span>
							<span class="alert-time">{alert.time}</span>
						</div>
						<h3>{alert.title}</h3>
						<p>{alert.message}</p>
					</div>
				{/each}
			</div>

			<a href="/dashboard/explosive-swings/alerts" class="view-all-link">
				View All Alerts →
			</a>
		</section>

		<!-- SIDEBAR -->
		<aside class="sidebar">
			<!-- Performance -->
			<div class="sidebar-card">
				<h3>30-Day Performance</h3>
				<div class="perf-chart">
					<svg viewBox="0 0 200 100" class="mini-chart">
						<defs>
							<linearGradient id="chartGrad" x1="0%" y1="0%" x2="0%" y2="100%">
								<stop offset="0%" style="stop-color:#22c55e;stop-opacity:0.3"/>
								<stop offset="100%" style="stop-color:#22c55e;stop-opacity:0"/>
							</linearGradient>
						</defs>
						<path d="M0,80 L30,70 L60,55 L90,45 L120,50 L150,30 L180,20 L200,15 L200,100 L0,100 Z" fill="url(#chartGrad)"/>
						<polyline points="0,80 30,70 60,55 90,45 120,50 150,30 180,20 200,15" fill="none" stroke="#22c55e" stroke-width="2.5"/>
					</svg>
					<div class="perf-total">+$18,750</div>
				</div>
				<div class="perf-stats">
					<div><span>23</span> Wins</div>
					<div><span>5</span> Losses</div>
					<div><span>82%</span> Win Rate</div>
				</div>
			</div>

			<!-- Quick Links -->
			<div class="sidebar-card">
				<h3>Resources</h3>
				<div class="quick-links">
					<a href="/dashboard/explosive-swings/video-library">🎬 Video Library</a>
					<a href="/dashboard/explosive-swings/trade-tracker">📊 Trade Tracker</a>
					<a href="/guides/swing-trading">📖 Swing Guide</a>
					<a href="/dashboard/account">⚙️ Alert Settings</a>
				</div>
			</div>

			<!-- Support -->
			<div class="sidebar-card support-card">
				<h3>Need Help?</h3>
				<p>Questions about a trade or setup?</p>
				<a href="https://intercom.help/simpler-trading/en/" target="_blank" class="support-btn">
					Contact Support
				</a>
			</div>
		</aside>
	</div>

	<!-- ═══════════════════════════════════════════════════════════════════════════
	     LATEST UPDATES - Video Updates for Entries/Exits
	     ═══════════════════════════════════════════════════════════════════════════ -->
	<section class="latest-updates">
		<div class="updates-header">
			<h2>Latest Updates</h2>
			<p>Video breakdowns as we enter and exit trades</p>
		</div>
		<div class="updates-grid">
			{#each latestUpdates as update}
				<a href={update.href} class="update-card">
					<div class="update-thumbnail" style="background-image: url('{update.image}')">
						<div class="play-overlay">
							<svg viewBox="0 0 24 24" fill="currentColor" width="48" height="48">
								<path d="M8 5v14l11-7z"/>
							</svg>
						</div>
						<div class="update-duration">{update.duration}</div>
					</div>
					<div class="update-content">
						<h3>{update.title}</h3>
						<p class="update-date">{update.date}</p>
						<p class="update-excerpt">{update.excerpt}</p>
					</div>
				</a>
			{/each}
		</div>
	</section>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD BASE
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard {
		background: #f5f7fa;
		min-height: 100vh;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   HERO SECTION - The Main Event
	   ═══════════════════════════════════════════════════════════════════════════ */
	.hero {
		background: linear-gradient(135deg, #F69532 0%, #e8850d 100%);
		padding: 0;
	}

	.hero-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 25px 40px;
		border-bottom: 1px solid rgba(255,255,255,0.2);
		flex-wrap: wrap;
		gap: 20px;
	}

	.hero-header h1 {
		color: #fff;
		font-size: 28px;
		font-weight: 700;
		margin: 0;
		font-family: 'Montserrat', sans-serif;
		text-align: center;
		flex: 1;
	}

	.hero-tabs {
		display: flex;
		gap: 10px;
	}

	.hero-tab {
		display: flex;
		align-items: center;
		gap: 8px;
		background: rgba(255,255,255,0.15);
		border: 2px solid rgba(255,255,255,0.3);
		color: #fff;
		padding: 12px 24px;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.hero-tab:hover {
		background: rgba(255,255,255,0.25);
	}

	.hero-tab.active {
		background: #fff;
		color: #F69532;
		border-color: #fff;
	}

	.hero-content {
		padding: 40px;
	}

	/* VIDEO TAB */
	.video-container {
		display: flex;
		gap: 40px;
		align-items: center;
		max-width: 1200px;
		margin: 0 auto;
	}

	.video-player {
		flex: 0 0 55%;
		position: relative;
		padding-bottom: 31%;
		background-size: cover;
		background-position: center;
		border-radius: 16px;
		overflow: hidden;
		box-shadow: 0 20px 60px rgba(0,0,0,0.3);
	}

	.video-overlay {
		position: absolute;
		inset: 0;
		background: rgba(0,0,0,0.3);
		display: flex;
		align-items: center;
		justify-content: center;
		transition: background 0.3s;
	}

	.video-player:hover .video-overlay {
		background: rgba(0,0,0,0.4);
	}

	.play-btn {
		width: 80px;
		height: 80px;
		background: #fff;
		border: none;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		transition: transform 0.3s;
		box-shadow: 0 10px 40px rgba(0,0,0,0.3);
	}

	.play-btn svg {
		width: 32px;
		height: 32px;
		color: #F69532;
		margin-left: 4px;
	}

	.video-player:hover .play-btn {
		transform: scale(1.1);
	}

	.video-duration {
		position: absolute;
		bottom: 15px;
		right: 15px;
		background: rgba(0,0,0,0.8);
		color: #fff;
		padding: 6px 12px;
		border-radius: 6px;
		font-size: 13px;
		font-weight: 600;
	}

	.video-info {
		flex: 1;
		color: #fff;
		text-align: center;
	}

	.video-info h2 {
		font-size: 32px;
		font-weight: 700;
		margin: 0 0 10px 0;
		font-family: 'Montserrat', sans-serif;
	}

	.video-info p {
		font-size: 14px;
		opacity: 0.9;
		margin: 0 0 30px 0;
	}

	.watch-btn {
		display: inline-flex;
		align-items: center;
		gap: 10px;
		background: #143E59;
		color: #fff;
		padding: 16px 32px;
		border-radius: 10px;
		font-size: 16px;
		font-weight: 700;
		text-decoration: none;
		transition: all 0.3s;
	}

	.watch-btn:hover {
		background: #0f2d42;
		transform: translateY(-2px);
	}

	/* ENTRIES TAB - Trade Sheet */
	.entries-container {
		max-width: 1400px;
		margin: 0 auto;
	}

	.entries-header {
		text-align: center;
		color: #fff;
		margin-bottom: 30px;
	}

	.entries-header h2 {
		font-size: 28px;
		font-weight: 700;
		margin: 0 0 8px 0;
		font-family: 'Montserrat', sans-serif;
	}

	.entries-header p {
		font-size: 15px;
		opacity: 0.9;
		margin: 0;
	}

	.trade-sheet-wrapper {
		background: #fff;
		border-radius: 16px;
		overflow: hidden;
		box-shadow: 0 20px 60px rgba(0,0,0,0.2);
	}

	.trade-sheet {
		width: 100%;
		border-collapse: collapse;
		font-size: 14px;
	}

	.trade-sheet thead {
		background: #143E59;
		color: #fff;
	}

	.trade-sheet th {
		padding: 16px 12px;
		text-align: center;
		font-weight: 700;
		font-size: 12px;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.trade-sheet tbody tr:nth-child(4n+1),
	.trade-sheet tbody tr:nth-child(4n+2) {
		background: #f8fafc;
	}

	.trade-sheet td {
		padding: 14px 12px;
		text-align: center;
		border-bottom: 1px solid #e5e7eb;
	}

	.ticker-cell strong {
		font-size: 16px;
		color: #143E59;
	}

	.bias {
		display: inline-block;
		padding: 4px 10px;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 700;
	}

	.bias--bullish {
		background: #dcfce7;
		color: #166534;
	}

	.bias--bearish {
		background: #fee2e2;
		color: #991b1b;
	}

	.bias--neutral {
		background: #fef3c7;
		color: #92400e;
	}

	.entry-cell {
		font-weight: 700;
		color: #143E59;
	}

	.target-cell {
		color: #22c55e;
		font-weight: 600;
	}

	.runner-cell {
		color: #0ea5e9;
		font-weight: 700;
	}

	.stop-cell {
		color: #ef4444;
		font-weight: 600;
	}

	.options-cell {
		font-weight: 600;
		color: #7c3aed;
	}

	.exp-cell {
		font-size: 12px;
		color: #666;
	}

	.notes-row td {
		text-align: left;
		padding: 10px 20px 16px;
		font-size: 13px;
		color: #666;
		background: transparent !important;
		border-bottom: 2px solid #e5e7eb;
	}

	.notes-label {
		font-weight: 700;
		color: #333;
	}

	.sheet-footer {
		padding: 20px;
		text-align: center;
	}

	.google-sheet-link {
		display: inline-flex;
		align-items: center;
		gap: 10px;
		background: #143E59;
		color: #fff;
		padding: 14px 28px;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 700;
		text-decoration: none;
		transition: all 0.3s;
	}

	.google-sheet-link:hover {
		background: #0f2d42;
	}

	@media (max-width: 1024px) {
		.video-container {
			flex-direction: column;
		}

		.video-player {
			flex: none;
			width: 100%;
			padding-bottom: 56.25%;
		}

		.trade-sheet-wrapper {
			overflow-x: auto;
		}

		.trade-sheet {
			min-width: 900px;
		}
	}

	@media (max-width: 768px) {
		.hero-header {
			flex-direction: column;
			text-align: center;
		}

		.hero-tabs {
			width: 100%;
			justify-content: center;
		}

		.hero-tab {
			flex: 1;
			justify-content: center;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   STATS BAR
	   ═══════════════════════════════════════════════════════════════════════════ */
	.stats-bar {
		display: flex;
		justify-content: center;
		gap: 60px;
		padding: 30px;
		background: #fff;
		border-bottom: 1px solid #e5e7eb;
		flex-wrap: wrap;
	}

	.stat {
		text-align: center;
	}

	.stat-value {
		display: block;
		font-size: 32px;
		font-weight: 700;
		color: #143E59;
		font-family: 'Montserrat', sans-serif;
	}

	.stat-value.green {
		color: #22c55e;
	}

	.stat-label {
		font-size: 13px;
		color: #666;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	@media (max-width: 640px) {
		.stats-bar {
			gap: 30px;
		}

		.stat-value {
			font-size: 24px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MAIN GRID
	   ═══════════════════════════════════════════════════════════════════════════ */
	.main-grid {
		display: grid;
		grid-template-columns: 1fr 340px;
		gap: 30px;
		padding: 30px;
		max-width: 1400px;
		margin: 0 auto;
	}

	@media (max-width: 1024px) {
		.main-grid {
			grid-template-columns: 1fr;
		}
	}

	/* ALERTS SECTION */
	.alerts-section {
		background: #fff;
		border-radius: 16px;
		padding: 25px;
		box-shadow: 0 4px 20px rgba(0,0,0,0.06);
	}

	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 25px;
		flex-wrap: wrap;
		gap: 15px;
	}

	.section-header h2 {
		font-size: 20px;
		font-weight: 700;
		margin: 0;
		color: #333;
		font-family: 'Montserrat', sans-serif;
		text-align: center;
	}

	.filter-pills {
		display: flex;
		gap: 8px;
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
		transition: all 0.2s;
	}

	.pill:hover {
		background: #e5e7eb;
	}

	.pill.active {
		background: #143E59;
		color: #fff;
	}

	.alerts-list {
		display: flex;
		flex-direction: column;
		gap: 15px;
	}

	.alert-card {
		background: #f8fafc;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		padding: 20px;
		position: relative;
		transition: all 0.2s;
		text-align: center;
	}

	.alert-card:hover {
		border-color: #143E59;
		transform: translateY(-2px);
		box-shadow: 0 8px 25px rgba(0,0,0,0.08);
	}

	.alert-card.is-new {
		background: #fffbf5;
		border-color: #F69532;
	}

	.new-badge {
		position: absolute;
		top: -10px;
		right: 15px;
		background: #F69532;
		color: #fff;
		font-size: 10px;
		font-weight: 700;
		padding: 4px 12px;
		border-radius: 10px;
	}

	.alert-header {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 12px;
		margin-bottom: 12px;
		flex-wrap: wrap;
	}

	.alert-type {
		font-size: 11px;
		font-weight: 700;
		padding: 4px 10px;
		border-radius: 4px;
	}

	.alert-type--entry {
		background: #dcfce7;
		color: #166534;
	}

	.alert-type--exit {
		background: #dbeafe;
		color: #1e40af;
	}

	.alert-type--update {
		background: #fef3c7;
		color: #92400e;
	}

	.alert-ticker {
		font-size: 16px;
		font-weight: 700;
		color: #143E59;
	}

	.alert-time {
		font-size: 12px;
		color: #888;
	}

	.alert-card h3 {
		font-size: 16px;
		font-weight: 700;
		margin: 0 0 8px 0;
		color: #333;
	}

	.alert-card p {
		font-size: 14px;
		color: #666;
		line-height: 1.6;
		margin: 0;
	}

	.view-all-link {
		display: block;
		text-align: center;
		margin-top: 20px;
		color: #0984ae;
		font-weight: 600;
		text-decoration: none;
	}

	.view-all-link:hover {
		color: #143E59;
	}

	/* SIDEBAR */
	.sidebar {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.sidebar-card {
		background: #fff;
		border-radius: 16px;
		padding: 25px;
		box-shadow: 0 4px 20px rgba(0,0,0,0.06);
		text-align: center;
	}

	.sidebar-card h3 {
		font-size: 16px;
		font-weight: 700;
		margin: 0 0 20px 0;
		color: #333;
		font-family: 'Montserrat', sans-serif;
	}

	.mini-chart {
		width: 100%;
		height: 80px;
	}

	.perf-total {
		font-size: 28px;
		font-weight: 700;
		color: #22c55e;
		margin: 15px 0;
		font-family: 'Montserrat', sans-serif;
	}

	.perf-stats {
		display: flex;
		justify-content: center;
		gap: 20px;
		font-size: 13px;
		color: #666;
	}

	.perf-stats span {
		font-weight: 700;
		color: #143E59;
	}

	.quick-links {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.quick-links a {
		display: block;
		padding: 12px 15px;
		background: #f8fafc;
		border-radius: 8px;
		color: #333;
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
		transition: all 0.2s;
		text-align: center;
	}

	.quick-links a:hover {
		background: #143E59;
		color: #fff;
	}

	.support-card {
		background: linear-gradient(135deg, #143E59, #0984ae);
		color: #fff;
	}

	.support-card h3 {
		color: #fff;
	}

	.support-card p {
		font-size: 14px;
		opacity: 0.9;
		margin: 0 0 20px 0;
	}

	.support-btn {
		display: inline-block;
		background: #fff;
		color: #143E59;
		padding: 12px 24px;
		border-radius: 8px;
		font-weight: 700;
		text-decoration: none;
		transition: all 0.2s;
	}

	.support-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 5px 20px rgba(0,0,0,0.2);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LATEST UPDATES SECTION
	   ═══════════════════════════════════════════════════════════════════════════ */
	.latest-updates {
		background: #fff;
		padding: 60px 30px;
		border-top: 1px solid #e5e7eb;
	}

	.updates-header {
		text-align: center;
		max-width: 800px;
		margin: 0 auto 50px;
	}

	.updates-header h2 {
		font-size: 32px;
		font-weight: 700;
		margin: 0 0 12px 0;
		color: #333;
		font-family: 'Montserrat', sans-serif;
	}

	.updates-header p {
		font-size: 16px;
		color: #666;
		margin: 0;
	}

	.updates-grid {
		display: grid;
		grid-template-columns: repeat(1, 1fr);
		gap: 30px;
		max-width: 1400px;
		margin: 0 auto;
	}

	@media (min-width: 640px) {
		.updates-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (min-width: 1024px) {
		.updates-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.update-card {
		background: #fff;
		border-radius: 16px;
		overflow: hidden;
		box-shadow: 0 4px 20px rgba(0,0,0,0.08);
		transition: all 0.3s ease;
		text-decoration: none;
		color: inherit;
		display: block;
	}

	.update-card:hover {
		transform: translateY(-8px);
		box-shadow: 0 12px 40px rgba(0,0,0,0.15);
	}

	.update-thumbnail {
		position: relative;
		width: 100%;
		padding-bottom: 56.25%;
		background-size: cover;
		background-position: center;
		overflow: hidden;
	}

	.play-overlay {
		position: absolute;
		inset: 0;
		background: rgba(0,0,0,0.4);
		display: flex;
		align-items: center;
		justify-content: center;
		transition: background 0.3s;
	}

	.update-card:hover .play-overlay {
		background: rgba(0,0,0,0.6);
	}

	.play-overlay svg {
		color: #fff;
		opacity: 0.9;
		transition: all 0.3s;
	}

	.update-card:hover .play-overlay svg {
		opacity: 1;
		transform: scale(1.1);
	}

	.update-duration {
		position: absolute;
		bottom: 12px;
		right: 12px;
		background: rgba(0,0,0,0.85);
		color: #fff;
		padding: 5px 10px;
		border-radius: 6px;
		font-size: 12px;
		font-weight: 600;
	}

	.update-content {
		padding: 20px;
		text-align: center;
	}

	.update-content h3 {
		font-size: 16px;
		font-weight: 700;
		margin: 0 0 8px 0;
		color: #333;
		line-height: 1.4;
	}

	.update-date {
		font-size: 12px;
		color: #888;
		margin: 0 0 12px 0;
	}

	.update-excerpt {
		font-size: 14px;
		color: #666;
		line-height: 1.6;
		margin: 0;
	}
</style>

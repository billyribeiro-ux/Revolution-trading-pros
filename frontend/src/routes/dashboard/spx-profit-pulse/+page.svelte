<script lang="ts">
	/**
	 * SPX Profit Pulse Dashboard - Pixel-Perfect WordPress Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Apple ICT Level 7 Principal Engineer Implementation
	 * SSR-first with real API data
	 *
	 * @version 3.0.0 - January 2026 - Full API Integration
	 */
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import LatestUpdates from '$lib/components/dashboard/LatestUpdates.svelte';
	import WeeklyWatchlist from '$lib/components/dashboard/WeeklyWatchlist.svelte';

	// SSR data from +page.server.ts
	let props = $props();
	let data = $derived(props.data);

	// Transform SSR alerts to LatestUpdates format
	const latestUpdatesItems = $derived(
		(data.alerts || []).map((alert: any) => ({
			id: alert.id,
			type: 'Daily Video',
			title: alert.title,
			date: alert.date,
			excerpt: alert.excerpt,
			href: alert.href,
			image:
				alert.image ||
				alert.video?.thumbnail ||
				'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: alert.is_video ?? true
		}))
	);

	// Get trading rooms for header dropdown
	const tradingRooms = $derived(data.tradingRooms || []);
</script>

<svelte:head>
	<title>SPX Profit Pulse Dashboard | Revolution Trading Pros</title>
</svelte:head>

<TradingRoomHeader
	roomName="SPX Profit Pulse"
	startHereUrl="/dashboard/spx-profit-pulse/start-here"
	{tradingRooms}
/>

<!-- DASHBOARD CONTENT - Two Column Layout with Sidebar -->
<div class="dashboard-wrapper">
	<div class="main-grid">
		<!-- MAIN CONTENT -->
		<main class="main-content">
			<!-- MAIN CONTENT SECTION - Video + Featured Cards -->
			<section class="dashboard__content-section-member">
				<!-- Welcome Video -->
				<div class="video-container">
					<video
						controls
						poster="https://cdn.simplertrading.com/2025/07/02134158/FTR-Jonathan.png"
						class="welcome-video"
					>
						<source
							src="https://simpler-options.s3.amazonaws.com/Tr3ndy/TrendySPXQuickstart2025.mp4"
							type="video/mp4"
						/>
						Your browser doesn't support HTML5 video.
					</video>
				</div>

				<!-- Featured Cards - Exact WordPress Match -->
				<div class="featured_cards">
					<div class="card-col">
						<div class="img_1">
							<h2 class="card_title">Start Here</h2>
							<p class="featured-desc">
								Key steps for getting started with your SPX Profit Pulse membership
							</p>
							<div class="card-button-wrapper">
								<a href="/dashboard/spx-profit-pulse/start-here">CHECK IT OUT</a>
							</div>
						</div>
					</div>
					<div class="card-col">
						<div class="img_2">
							<h2 class="card_title_2">Meet the Traders</h2>
							<p>Here you'll find Strategies and more!</p>
							<div class="card-button-wrapper">
								<a class="aclass" href="/dashboard/spx-profit-pulse/billy-ribeiro/">Meet Billy</a>
							</div>
						</div>
					</div>
					<div class="card-col">
						<div class="img_3">
							<h2 class="card_title_3">Continued Education</h2>
							<p class="featured-desc">
								Grow your trading skills and learn to read the market better.
							</p>
							<div class="card-button-wrapper">
								<a target="_blank" href="https://www.youtube.com/@Revolutiontradingpros"
									>LEARN MORE</a
								>
							</div>
						</div>
					</div>
				</div>
			</section>

			<!-- Latest Updates Section - Exact WordPress Match -->
			<section class="dashboard__content-section">
				<LatestUpdates
					items={latestUpdatesItems}
					title="Latest Updates"
					roomSlug="spx-profit-pulse"
					buttonText="Watch Now"
				/>
			</section>

			<!-- Weekly Watchlist Section -->
			<div class="dashboard__content-section u--background-color-white">
				<WeeklyWatchlist />
			</div>
		</main>

		<!-- SIDEBAR -->
		<aside class="sidebar" aria-label="Performance sidebar">
			<!-- 30-Day Performance -->
			<div class="sidebar-card">
				<h3>30-Day Performance</h3>
				<div class="perf-chart">
					<svg viewBox="0 0 200 100" class="mini-chart" aria-hidden="true">
						<defs>
							<linearGradient id="chartGrad" x1="0%" y1="0%" x2="0%" y2="100%">
								<stop offset="0%" style="stop-color:#22c55e;stop-opacity:0.3" />
								<stop offset="100%" style="stop-color:#22c55e;stop-opacity:0" />
							</linearGradient>
						</defs>
						<path
							d="M0,80 L30,70 L60,55 L90,45 L120,50 L150,30 L180,20 L200,15 L200,100 L0,100 Z"
							fill="url(#chartGrad)"
						/>
						<polyline
							points="0,80 30,70 60,55 90,45 120,50 150,30 180,20 200,15"
							fill="none"
							stroke="#22c55e"
							stroke-width="2.5"
						/>
					</svg>
					<div class="perf-total">+$12,450</div>
				</div>
				<div class="perf-stats">
					<div><span>87%</span> Win Rate</div>
					<div><span>24</span> Active</div>
					<div><span>18</span> Closed</div>
				</div>
			</div>

			<!-- Resources -->
			<div class="sidebar-card">
				<h3>Resources</h3>
				<div class="quick-links">
					<a href="/dashboard/spx-profit-pulse/video-library">🎬 Video Library</a>
					<a href="/dashboard/spx-profit-pulse/trades">📊 Trade Tracker</a>
					<a href="/dashboard/spx-profit-pulse/favorites">⭐ My Favorites</a>
					<a href="/api/export/watchlist?room_slug=spx-profit-pulse&format=csv" download
						>📥 Export CSV</a
					>
					<a href="/dashboard/account">⚙️ Alert Settings</a>
				</div>
			</div>

			<!-- Support -->
			<div class="sidebar-card support-card">
				<h3>Need Help?</h3>
				<p>Questions about SPX 0DTE trading?</p>
				<a
					href="https://intercom.help/simpler-trading/en/"
					target="_blank"
					rel="noopener noreferrer"
					class="support-btn"
				>
					Contact Support
				</a>
			</div>
		</aside>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   SPX PROFIT PULSE DASHBOARD - Two Column Layout with Sidebar
	   Matching Explosive Swings Dashboard Structure
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard-wrapper {
		background: #f5f7fa;
		min-height: 100vh;
		padding: 30px;
	}

	/* Two-column grid layout */
	.main-grid {
		display: grid;
		grid-template-columns: 1fr 340px;
		gap: 30px;
		max-width: 1400px;
		margin: 0 auto;
		align-items: start;
	}

	/* lg: 1024px - Collapse to single column on tablets and below */
	@media (max-width: 1023px) {
		.main-grid {
			grid-template-columns: 1fr;
		}
	}

	.main-content {
		width: 100%;
	}

	/* Main Content Section */
	.dashboard__content-section-member {
		padding: 20px;
	}

	/* md: 768px - Tablets */
	@media screen and (min-width: 768px) {
		.dashboard__content-section-member {
			padding: 30px;
		}
	}

	/* xl: 1280px - Large desktops */
	@media screen and (min-width: 1280px) {
		.dashboard__content-section-member {
			padding: 40px;
		}
	}

	/* Video Container - Constrained to 1024x575 max
	   CRITICAL: aspect-ratio reserves space BEFORE video loads to prevent layout shift */
	.video-container {
		margin-bottom: 30px;
		max-width: 1024px;
		margin-left: auto;
		margin-right: auto;
		width: 100%;
		aspect-ratio: 16 / 9;
		background: #000;
		border-radius: 8px;
		overflow: hidden;
	}

	.welcome-video {
		width: 100%;
		height: 100%;
		display: block;
		border-radius: 8px;
		object-fit: contain;
		background: #000;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FEATURED CARDS - Centered above video, 2 on top row, 1 centered below
	   ═══════════════════════════════════════════════════════════════════════════ */
	.featured_cards {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 20px;
		max-width: 1024px;
		margin: 0 auto 30px auto;
	}

	/* Mobile: 1 column, full width */
	.card-col {
		display: flex;
		flex: 1 1 100%;
		min-width: 280px;
		max-width: 360px;
	}

	/* md: 768px - Tablets: 2 cards on top, 1 centered below */
	@media screen and (min-width: 768px) {
		.card-col {
			flex: 0 1 360px;
			max-width: 360px;
		}

		/* Third card: force to new row and center */
		.card-col:nth-child(3) {
			flex: 0 1 360px;
			margin-left: auto;
			margin-right: auto;
		}
	}

	/* Card Title Styles - Exact WordPress Match */
	.card_title {
		background-color: #2e88e2;
		opacity: 0.81;
		border-radius: 8px 8px 0 0;
		padding: 12px 15px;
		font-weight: 700;
		font-size: 20px;
		margin: 0;
		color: #fff;
	}

	.card_title_2 {
		background-color: #284556;
		opacity: 0.81;
		border-radius: 8px 8px 0 0;
		padding: 12px 15px;
		font-weight: 700;
		font-size: 20px;
		margin: 0;
		color: #fff;
	}

	.card_title_3 {
		background-color: #062946;
		border-radius: 8px 8px 0 0;
		opacity: 0.81;
		padding: 12px 15px;
		font-weight: 700;
		font-size: 20px;
		margin: 0;
		color: #fff;
	}

	/* Card Content Paragraphs */
	.featured_cards p {
		padding: 0 15px;
		margin: 10px 0;
		color: #fff;
		font-size: 13px;
		line-height: 1.4;
	}

	.featured-desc {
		min-height: 36px;
	}

	/* Card Buttons - Exact WordPress Match */
	.featured_cards a {
		color: #fff !important;
		display: inline-block;
		font-family: 'Open Sans Condensed', sans-serif;
		font-weight: 700;
		background: transparent linear-gradient(180deg, #ffb834 0%, #c68000 100%) 0% 0% no-repeat
			padding-box;
		border-radius: 50px;
		padding: 5px 30px;
		margin-bottom: 20px;
		text-decoration: none;
		transition:
			transform 0.2s ease,
			box-shadow 0.2s ease;
	}

	.featured_cards a:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(198, 128, 0, 0.4);
	}

	/* Card Button Wrapper - Centers buttons */
	.card-button-wrapper {
		display: flex;
		justify-content: center;
		align-items: center;
		padding: 0 15px;
	}

	/* Card Background Images - 360x212 on large devices */
	.img_1,
	.img_2,
	.img_3 {
		width: 100%;
		height: 212px;
		border-radius: 8px;
		background-position: center;
		background-repeat: no-repeat;
		background-size: cover;
		color: #ffffff;
		text-align: center;
		display: flex;
		flex-direction: column;
	}

	.img_1 {
		background-image: url('https://cdn.simplertrading.com/2021/08/12114451/start-here-top-card-bg.png');
	}

	.img_2 {
		background-image: url('https://cdn.simplertrading.com/2021/08/12123709/meet-the-traders-top-card-bg.png');
	}

	.img_3 {
		background-image: url('https://cdn.simplertrading.com/2021/08/12123711/continued-ed-top-card-bg.png');
	}

	/* Weekly Watchlist Section */
	.dashboard__content-section {
		padding: 30px 20px;
	}

	/* md: 768px - Tablets */
	@media screen and (min-width: 768px) {
		.dashboard__content-section {
			padding: 30px;
		}
	}

	/* xl: 1280px - Large desktops */
	@media screen and (min-width: 1280px) {
		.dashboard__content-section {
			padding: 40px;
		}
	}

	.u--background-color-white {
		background-color: #fff !important;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SIDEBAR - Matching Explosive Swings Dashboard
	   ═══════════════════════════════════════════════════════════════════════════ */
	.sidebar {
		display: flex;
		flex-direction: column;
		gap: 20px;
		position: sticky;
		top: 20px;
	}

	.sidebar-card {
		background: #fff;
		border-radius: 16px;
		padding: 25px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.06);
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
		color: #143e59;
	}

	.quick-links {
		display: flex;
		flex-direction: column;
		gap: 12px;
		text-align: left;
	}

	.quick-links a {
		color: #143e59;
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
		transition: all 0.2s;
		padding: 8px 12px;
		border-radius: 8px;
		display: block;
	}

	.quick-links a:hover {
		background: #f8fafc;
		color: #0984ae;
		transform: translateX(4px);
	}

	.support-card {
		background: linear-gradient(135deg, #143e59 0%, #1e5175 100%);
		color: white;
	}

	.support-card h3 {
		color: white;
	}

	.support-card p {
		color: rgba(255, 255, 255, 0.9);
		font-size: 14px;
		margin-bottom: 15px;
	}

	.support-btn {
		display: inline-block;
		background: white;
		color: #143e59;
		padding: 10px 20px;
		border-radius: 8px;
		font-weight: 700;
		font-size: 14px;
		text-decoration: none;
		transition: all 0.2s;
	}

	.support-btn:hover {
		background: #f8fafc;
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
	}

	/* lg: 1024px - Collapse sidebar on tablets and below */
	@media (max-width: 1023px) {
		.sidebar {
			position: static;
		}
	}
</style>

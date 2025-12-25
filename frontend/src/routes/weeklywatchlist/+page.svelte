<script lang="ts">
	/**
	 * Weekly Watchlist Page - 100% PIXEL-PERFECT Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Exact replica of Simpler Trading Weekly Watchlist page
	 * @version 1.0.0 - Pixel Perfect
	 */

	// Current week's watchlist data
	const currentWatchlist = {
		week: 'December 23-27, 2025',
		trader: 'TG Watkins',
		video: '/videos/weekly-watchlist-12-23-2025.mp4',
		poster: 'https://cdn.simplertrading.com/2020/11/01132413/Store-Card-MasteringTheTrade-Room.jpg',
		stocks: [
			{ symbol: 'AAPL', name: 'Apple Inc.', direction: 'Bullish', entry: '$195.50', target: '$210.00', stop: '$188.00' },
			{ symbol: 'NVDA', name: 'NVIDIA Corporation', direction: 'Bullish', entry: '$485.00', target: '$520.00', stop: '$465.00' },
			{ symbol: 'TSLA', name: 'Tesla Inc.', direction: 'Neutral', entry: '$245.00', target: '$265.00', stop: '$230.00' },
			{ symbol: 'SPY', name: 'S&P 500 ETF', direction: 'Bullish', entry: '$475.00', target: '$490.00', stop: '$465.00' },
			{ symbol: 'QQQ', name: 'Nasdaq 100 ETF', direction: 'Bullish', entry: '$410.00', target: '$430.00', stop: '$395.00' }
		]
	};
</script>

<svelte:head>
	<title>Weekly Watchlist | Revolution Trading Pros</title>
	<meta name="description" content="Weekly trading watchlist with top stock picks and market analysis from our expert traders." />
</svelte:head>

<!-- WordPress EXACT Structure -->
<div class="page-template-default page logged-in woocommerce-page">

	<!-- Breadcrumbs -->
	<nav id="breadcrumbs" class="breadcrumbs">
		<div class="container-fluid">
			<ul>
				<li class="item-home"><a class="breadcrumb-link breadcrumb-home" href="/" title="Home">Home</a></li>
				<li class="separator separator-home"> / </li>
				<li class="item-dashboard"><a class="breadcrumb-link" href="/dashboard" title="Dashboard">Dashboard</a></li>
				<li class="separator"> / </li>
				<li class="item-current"><strong class="breadcrumb-current">Weekly Watchlist</strong></li>
			</ul>
		</div>
	</nav>

	<!-- Page Content -->
	<div id="page" class="hfeed site grid-parent">
		<div id="content" class="site-content">

			<!-- Page Header -->
			<header class="page-header-section">
				<div class="container-fluid">
					<h1 class="page-title">Weekly Watchlist</h1>
					<p class="page-subtitle">Week of {currentWatchlist.week} with {currentWatchlist.trader}</p>
				</div>
			</header>

			<!-- Main Content -->
			<section class="wl-page-content">
				<div class="container-fluid">
					<div class="row">
						<!-- Video Section -->
						<div class="col-xs-12 col-lg-8">
							<div class="video-section">
								<div class="video-card">
									<div class="video-wrapper">
										<div class="video-placeholder">
											<img src={currentWatchlist.poster} alt="Weekly Watchlist Video" />
											<button class="play-button" aria-label="Play video">
												<svg viewBox="0 0 24 24" width="48" height="48">
													<path fill="currentColor" d="M8 5v14l11-7z"/>
												</svg>
											</button>
										</div>
									</div>
									<div class="video-info">
										<h2>Weekly Watchlist Rundown</h2>
										<p class="video-meta">Posted: December 22, 2025 | Duration: 15:42</p>
									</div>
								</div>
							</div>

							<!-- Watchlist Table -->
							<div class="watchlist-section">
								<h3 class="section-title">This Week's Watchlist</h3>
								<div class="watchlist-table-wrapper">
									<table class="watchlist-table">
										<thead>
											<tr>
												<th>Symbol</th>
												<th>Name</th>
												<th>Direction</th>
												<th>Entry</th>
												<th>Target</th>
												<th>Stop</th>
											</tr>
										</thead>
										<tbody>
											{#each currentWatchlist.stocks as stock}
												<tr>
													<td class="symbol-cell">
														<span class="stock-symbol">{stock.symbol}</span>
													</td>
													<td class="name-cell">{stock.name}</td>
													<td class="direction-cell">
														<span class="direction-badge direction-{stock.direction.toLowerCase()}">{stock.direction}</span>
													</td>
													<td class="price-cell">{stock.entry}</td>
													<td class="price-cell target">{stock.target}</td>
													<td class="price-cell stop">{stock.stop}</td>
												</tr>
											{/each}
										</tbody>
									</table>
								</div>
							</div>
						</div>

						<!-- Sidebar -->
						<div class="col-xs-12 col-lg-4">
							<div class="sidebar">
								<!-- Quick Links Card -->
								<div class="sidebar-card">
									<h4 class="card-title">Quick Links</h4>
									<ul class="quick-links">
										<li><a href="/weeklywatchlist/archive">View Watchlist Archive</a></li>
										<li><a href="/weeklywatchlist/rundown-archive">View Rundown Archive</a></li>
										<li><a href="/dashboard/day-trading-room">Back to Trading Room</a></li>
									</ul>
								</div>

								<!-- Trader Info Card -->
								<div class="sidebar-card trader-card">
									<h4 class="card-title">Your Analyst</h4>
									<div class="trader-info">
										<div class="trader-avatar">
											<img src="https://cdn.simplertrading.com/traders/tg-watkins.jpg" alt={currentWatchlist.trader} />
										</div>
										<div class="trader-details">
											<h5 class="trader-name">{currentWatchlist.trader}</h5>
											<p class="trader-role">Lead Market Analyst</p>
										</div>
									</div>
									<p class="trader-bio">TG Watkins brings over 20 years of trading experience, specializing in technical analysis and swing trading strategies.</p>
								</div>

								<!-- Disclaimer Card -->
								<div class="sidebar-card disclaimer-card">
									<h4 class="card-title">Disclaimer</h4>
									<p class="disclaimer-text">The information provided is for educational purposes only and should not be construed as investment advice. Past performance is not indicative of future results.</p>
								</div>
							</div>
						</div>
					</div>
				</div>
			</section>

		</div>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   100% PIXEL-PERFECT STYLES - Matching Simpler Trading
	   ═══════════════════════════════════════════════════════════════════════════ */

	:global(body) {
		font-family: 'Open Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
		margin: 0;
		padding: 0;
		background: #f4f4f4;
	}

	/* Breadcrumbs */
	.breadcrumbs {
		background: #f4f4f4;
		padding: 12px 0;
		border-bottom: 1px solid #dbdbdb;
	}

	.breadcrumbs ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		align-items: center;
	}

	.breadcrumbs li {
		font-size: 14px;
		color: #666;
	}

	.breadcrumbs a {
		color: #0984ae;
		text-decoration: none;
	}

	.breadcrumbs a:hover {
		text-decoration: underline;
	}

	.separator {
		margin: 0 8px;
	}

	/* Container */
	.container-fluid {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 20px;
	}

	/* Page Header */
	.page-header-section {
		padding: 40px 0 30px;
	}

	.page-title {
		font-family: 'Open Sans Condensed', 'Open Sans', sans-serif;
		font-size: 36px;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0 0 8px;
		text-transform: uppercase;
	}

	.page-subtitle {
		color: #666;
		font-size: 16px;
		font-weight: 500;
		margin: 0;
	}

	/* Grid */
	.row {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.col-xs-12 {
		padding: 0 15px;
		flex: 0 0 100%;
		max-width: 100%;
	}

	@media (min-width: 992px) {
		.col-lg-8 {
			flex: 0 0 66.666667%;
			max-width: 66.666667%;
		}

		.col-lg-4 {
			flex: 0 0 33.333333%;
			max-width: 33.333333%;
		}
	}

	/* Video Section */
	.video-section {
		margin-bottom: 30px;
	}

	.video-card {
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
	}

	.video-wrapper {
		position: relative;
		width: 100%;
		padding-bottom: 56.25%;
		background: #1a1a1a;
	}

	.video-placeholder {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
	}

	.video-placeholder img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.play-button {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 80px;
		height: 80px;
		background: rgba(249, 158, 49, 0.9);
		border: none;
		border-radius: 50%;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #fff;
		transition: all 0.2s ease;
	}

	.play-button:hover {
		background: #f99e31;
		transform: translate(-50%, -50%) scale(1.1);
	}

	.video-info {
		padding: 20px;
	}

	.video-info h2 {
		font-size: 20px;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0 0 8px;
	}

	.video-meta {
		font-size: 14px;
		color: #666;
		margin: 0;
	}

	/* Watchlist Section */
	.watchlist-section {
		background: #fff;
		border-radius: 8px;
		padding: 24px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		margin-bottom: 30px;
	}

	.section-title {
		font-size: 18px;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0 0 20px;
		padding-bottom: 12px;
		border-bottom: 2px solid #0984ae;
	}

	.watchlist-table-wrapper {
		overflow-x: auto;
	}

	.watchlist-table {
		width: 100%;
		border-collapse: collapse;
	}

	.watchlist-table th {
		text-align: left;
		padding: 12px 16px;
		font-size: 12px;
		font-weight: 700;
		color: #666;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		background: #f9f9f9;
		border-bottom: 1px solid #dbdbdb;
	}

	.watchlist-table td {
		padding: 16px;
		font-size: 14px;
		color: #333;
		border-bottom: 1px solid #ededed;
	}

	.watchlist-table tbody tr:hover {
		background: #f9f9f9;
	}

	.stock-symbol {
		font-weight: 700;
		color: #0984ae;
		font-size: 15px;
	}

	.name-cell {
		color: #666;
	}

	.direction-badge {
		display: inline-block;
		padding: 4px 12px;
		border-radius: 25px;
		font-size: 11px;
		font-weight: 700;
		text-transform: uppercase;
	}

	.direction-bullish {
		background: #d4edda;
		color: #155724;
	}

	.direction-bearish {
		background: #f8d7da;
		color: #721c24;
	}

	.direction-neutral {
		background: #fff3cd;
		color: #856404;
	}

	.price-cell {
		font-weight: 600;
		font-family: 'Roboto Mono', monospace;
	}

	.price-cell.target {
		color: #28a745;
	}

	.price-cell.stop {
		color: #dc3545;
	}

	/* Sidebar */
	.sidebar {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.sidebar-card {
		background: #fff;
		border-radius: 8px;
		padding: 20px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
	}

	.card-title {
		font-size: 16px;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0 0 16px;
		padding-bottom: 12px;
		border-bottom: 1px solid #ededed;
	}

	.quick-links {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	.quick-links li {
		margin-bottom: 10px;
	}

	.quick-links a {
		display: flex;
		align-items: center;
		color: #0984ae;
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
	}

	.quick-links a:hover {
		color: #076787;
		text-decoration: underline;
	}

	/* Trader Card */
	.trader-info {
		display: flex;
		align-items: center;
		gap: 16px;
		margin-bottom: 16px;
	}

	.trader-avatar {
		width: 64px;
		height: 64px;
		border-radius: 50%;
		overflow: hidden;
		background: #f4f4f4;
	}

	.trader-avatar img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.trader-name {
		font-size: 16px;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0 0 4px;
	}

	.trader-role {
		font-size: 13px;
		color: #666;
		margin: 0;
	}

	.trader-bio {
		font-size: 14px;
		color: #666;
		line-height: 1.6;
		margin: 0;
	}

	/* Disclaimer */
	.disclaimer-card {
		background: #fff8e6;
		border-left: 4px solid #f99e31;
	}

	.disclaimer-text {
		font-size: 13px;
		color: #666;
		line-height: 1.6;
		margin: 0;
	}

	/* Responsive */
	@media (max-width: 991px) {
		.sidebar {
			margin-top: 30px;
		}
	}

	@media (max-width: 768px) {
		.page-title {
			font-size: 28px;
		}

		.watchlist-table th,
		.watchlist-table td {
			padding: 10px 8px;
			font-size: 12px;
		}
	}
</style>

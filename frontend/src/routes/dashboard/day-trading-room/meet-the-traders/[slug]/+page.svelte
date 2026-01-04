<!--
	URL: /dashboard/day-trading-room/meet-the-traders/[slug]

	Individual Trader Profile Page - WordPress Pixel-Perfect Implementation
	═══════════════════════════════════════════════════════════════════════════
	Matches WordPress meet-the-trader-john-carter implementation exactly.

	@version 2.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';

	interface Trader {
		id: string;
		name: string;
		slug: string;
		title: string;
		image: string;
		quote: string;
		whoIs: string;
		tradingApproach: string;
	}

	const traders: Trader[] = [
		{
			id: 'john-carter',
			name: 'John Carter',
			slug: 'john-carter',
			title: 'Founder of Simpler Trading®',
			image: 'https://cdn.simplertrading.com/2021/10/08113910/john-headshot-2020.png',
			quote: '"I think there are a lot of parallels between being a good trader and living a good life. The market truly is the ultimate psychologist."',
			whoIs: 'In 1999, tired of talking to his goldfish while trading alone in his office, John Carter launched the company that has evolved into Simpler Trading. He\'s the trading expert everyone turns to when the markets go awry (as they often do), and many of our traders have taken ideas and strategies from him and morphed them into thier own strategies. He\'s truly one of the best with a level of charisma that draws people in, which has transformed him into the famous day trader he is today.',
			tradingApproach: 'As for his trading strategy, it combines expert technical analysis with an overall macro, fundamental view. However, what makes him a truely fantastic mentor on top of his vast knowledge, is his ability to make you feel as though you\'re talking to an old friend.'
		},
		{
			id: 'henry-gambell',
			name: 'Henry Gambell',
			slug: 'henry-gambell',
			title: 'Director of Options',
			image: 'https://cdn.simplertrading.com/2021/10/08114059/henry-headshot-2020.png',
			quote: '"Every trade is an opportunity to learn something new about yourself and the markets."',
			whoIs: 'Henry Gambell is the Director of Options at Simpler Trading. He focuses on high-probability options setups using technical analysis and risk management principles. His methodical approach has helped countless traders improve their skills.',
			tradingApproach: 'Henry\'s trading style emphasizes patience and discipline, waiting for the right setups before entering trades. He is known for his ability to break down complex concepts for traders of all levels.'
		},
		{
			id: 'taylor-horton',
			name: 'Taylor Horton',
			slug: 'taylor-horton',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2021/10/08114206/taylor-headshot-2020.png',
			quote: '"Success in trading comes from consistency and discipline, not from hitting home runs."',
			whoIs: 'Taylor Horton is a Senior Trader at Simpler Trading specializing in day trading and swing trading strategies. He focuses on momentum plays and breakout patterns that have proven successful over time.',
			tradingApproach: 'Taylor brings energy and enthusiasm to his trading sessions, helping members identify actionable setups in real-time with a focus on risk management.'
		},
		{
			id: 'bruce-marshall',
			name: 'Bruce Marshall',
			slug: 'bruce-marshall',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2021/10/08113753/bruce-headshot-2020.png',
			quote: '"Options give you leverage, but risk management keeps you in the game."',
			whoIs: 'Bruce Marshall is a Senior Trader with expertise in options strategies and portfolio management. He focuses on income-generating strategies and risk-defined trades that work in various market conditions.',
			tradingApproach: 'Bruce\'s approach emphasizes consistency and capital preservation, making him a favorite among traders looking for steady returns.'
		},
		{
			id: 'danielle-shay',
			name: 'Danielle Shay',
			slug: 'danielle-shay',
			title: 'Director of Options',
			image: 'https://cdn.simplertrading.com/2021/10/08113828/danielle-headshot-2020.png',
			quote: '"The best trades are the ones where you manage your risk before thinking about reward."',
			whoIs: 'Danielle Shay is the Director of Options at Simpler Trading. She specializes in momentum-based options strategies and volatility trading, helping traders navigate fast-moving markets.',
			tradingApproach: 'Danielle is known for her ability to identify high-probability setups in fast-moving markets. She focuses on helping traders understand options Greeks and position sizing.'
		},
		{
			id: 'allison-ostrander',
			name: 'Allison Ostrander',
			slug: 'allison-ostrander',
			title: 'Director of Risk Tolerance',
			image: 'https://cdn.simplertrading.com/2021/10/08113700/allison-headshot-2020.png',
			quote: '"Small accounts can grow into big accounts with the right mindset and strategy."',
			whoIs: 'Allison Ostrander is the Director of Risk Tolerance at Simpler Trading. She specializes in helping traders with smaller accounts grow their portfolios responsibly without taking excessive risks.',
			tradingApproach: 'Allison\'s approach emphasizes proper position sizing, risk management, and building a sustainable trading practice that can last for years.'
		},
		{
			id: 'sam-shames',
			name: 'Sam Shames',
			slug: 'sam-shames',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2021/10/08114128/sam-headshot-2020.png',
			quote: '"The futures market rewards those who are prepared and punishes those who are not."',
			whoIs: 'Sam Shames is a Senior Trader specializing in futures and day trading. He focuses on ES and NQ futures with a momentum-based approach that has proven effective in various market conditions.',
			tradingApproach: 'Sam is known for his quick analysis and ability to adapt to changing market conditions, helping traders stay ahead of the curve.'
		},
		{
			id: 'kody-ashmore',
			name: 'Kody Ashmore',
			slug: 'kody-ashmore',
			title: 'Senior Futures Trader',
			image: 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg',
			quote: '"Every day in the futures market is a new opportunity to execute your edge."',
			whoIs: 'Kody Ashmore is a Senior Futures Trader at Simpler Trading. He specializes in ES and NQ futures with a focus on intraday momentum setups that capture quick, high-probability moves.',
			tradingApproach: 'Kody\'s trading style is aggressive yet disciplined, with strict risk management rules. He is known for his daily trading sessions where he shares live setups with members.'
		},
		{
			id: 'raghee-horner',
			name: 'Raghee Horner',
			slug: 'raghee-horner',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2021/10/08114038/raghee-headshot-2020.png',
			quote: '"The trend is your friend, but only if you know how to find it."',
			whoIs: 'Raghee Horner is a Senior Trader with over 30 years of trading experience. She specializes in forex and futures trading with a focus on technical analysis that has stood the test of time.',
			tradingApproach: 'Raghee is known for her proprietary indicators and trading methodology. She has authored multiple books on trading and has helped thousands of traders improve their skills.'
		}
	];

	const traderPills = [
		{ label: 'Overview', path: '' },
		{ label: 'Chart Setups', path: '/chart-setups' },
		{ label: 'Trading Plan', path: '/trading-plan' },
		{ label: 'Trading Strategies', path: '/trading-strategies' },
		{ label: 'Trader Store', path: '/trader-store' }
	];

	let trader: Trader | null = null;
	let loading = true;
	let currentSlug = '';

	onMount(() => {
		currentSlug = $page.params.slug;
		trader = traders.find(t => t.slug === currentSlug) || null;
		loading = false;
	});

	$: if ($page.params.slug !== currentSlug) {
		currentSlug = $page.params.slug;
		trader = traders.find(t => t.slug === currentSlug) || null;
	}
</script>

<svelte:head>
	{#if trader}
		<title>{trader.name} | Meet the Traders | Revolution Trading Pros</title>
		<meta name="description" content="{trader.name} - {trader.title}. {trader.quote}" />
	{:else}
		<title>Trader Not Found | Revolution Trading Pros</title>
	{/if}
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			{#if loading}
				<div class="loading-container">
					<div class="loading">
						<span class="loading-icon"></span>
						<span class="loading-message">Loading...</span>
					</div>
				</div>
			{:else if trader}
				<!-- Trader Header Section - fl-row-bg-color -->
				<div class="fl-row fl-row-full-width fl-row-bg-color">
					<div class="fl-row-content-wrap">
						<div class="fl-row-content fl-row-fixed-width fl-node-content">
							<div class="fl-col-group fl-col-group-equal-height fl-col-group-align-bottom">
								<!-- Left Column - Photo -->
								<div class="fl-col fl-col-small">
									<div class="fl-col-content fl-node-content">
										<div class="fl-module fl-module-photo fl-visible-desktop-medium">
											<div class="fl-module-content fl-node-content">
												<div class="fl-photo fl-photo-align-center">
													<div class="fl-photo-content fl-photo-img-png">
														<img 
															class="fl-photo-img" 
															src={trader.image} 
															alt={trader.name}
															width="2000" 
															height="2000"
														/>
													</div>
												</div>
											</div>
										</div>
									</div>
								</div>
								<!-- Right Column - Info -->
								<div class="fl-col">
									<div class="fl-col-content fl-node-content">
										<div class="fl-module fl-module-heading">
											<div class="fl-module-content fl-node-content">
												<h2 class="fl-heading">
													<span class="fl-heading-text trader-name">{trader.name}</span>
												</h2>
											</div>
										</div>
										<div class="fl-module fl-module-heading">
											<div class="fl-module-content fl-node-content">
												<h2 class="fl-heading">
													<span class="fl-heading-text trader-title">{trader.title}</span>
												</h2>
											</div>
										</div>
										<div class="fl-module fl-module-separator">
											<div class="fl-module-content fl-node-content">
												<div class="fl-separator"></div>
											</div>
										</div>
										<div class="fl-module fl-module-rich-text">
											<div class="fl-module-content fl-node-content">
												<div class="fl-rich-text">
													<p><em>{trader.quote}</em></p>
												</div>
											</div>
										</div>
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>

				<!-- Trader Pills Navigation -->
				<div class="fl-row fl-row-full-width fl-row-bg-color pills-section">
					<div class="fl-row-content-wrap">
						<div class="fl-row-content fl-row-fixed-width fl-node-content">
							<div class="fl-col-group">
								<div class="fl-col">
									<div class="fl-col-content fl-node-content">
										<div class="fl-module fl-module-html">
											<div class="fl-module-content fl-node-content">
												<div class="fl-html">
													<div class="trader_pills">
														{#each traderPills as pill}
															<div class="trader_pill">
																<a href="/dashboard/day-trading-room/meet-the-traders/{trader.slug}{pill.path === '/trader-store' ? '' : pill.path}"
																   class:active={pill.path === ''}
																   on:click|preventDefault={() => {
																	   if (pill.path === '/trader-store') {
																		   window.location.href = `/dashboard/day-trading-room/trader-store/${trader?.slug}`;
																	   }
																   }}>
																	{pill.label}
																</a>
															</div>
														{/each}
													</div>
												</div>
											</div>
										</div>
									</div>
								</div>
							</div>

							<!-- Who is Section -->
							<div class="fl-col-group">
								<div class="fl-col">
									<div class="fl-col-content fl-node-content">
										<div class="fl-module fl-module-heading">
											<div class="fl-module-content fl-node-content">
												<h2 class="fl-heading">
													<span class="fl-heading-text section-title">Who is {trader.name.split(' ')[0]}?</span>
												</h2>
											</div>
										</div>
										<div class="fl-module fl-module-rich-text">
											<div class="fl-module-content fl-node-content">
												<div class="fl-rich-text">
													<p>{trader.whoIs}</p>
													<p>{trader.tradingApproach}</p>
												</div>
											</div>
										</div>
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>
			{:else}
				<div class="not-found">
					<h1>Trader Not Found</h1>
					<p>The trader you're looking for could not be found.</p>
					<a href="/dashboard/day-trading-room/meet-the-traders" class="btn btn-default">
						← Back to All Traders
					</a>
				</div>
			{/if}
		</section>
	</div>
</div>

<!-- Weekly Watchlist Section -->
{#if trader}
<div class="dashboard__content-section u--background-color-white">
	<section>
		<div class="row">
			<div class="col-sm-6 col-lg-5">
				<h2 class="section-title-alt section-title-alt--underline">Weekly Watchlist</h2>
				<div class="hidden-md d-lg-none pb-2">
					<a href="/dashboard/day-trading-room/weekly-watchlist">
						<img src="https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg" alt="Weekly Watchlist image" class="u--border-radius" />
					</a>
				</div>
				<h4 class="h5 u--font-weight-bold">Weekly Watchlist with David Starr</h4>
				<div class="u--hide-read-more">
					<p>Week of December 29, 2025.</p>
				</div>
				<a href="/dashboard/day-trading-room/weekly-watchlist" class="btn btn-tiny btn-default">Watch Now</a>
			</div>
			<div class="col-sm-6 col-lg-7 hidden-xs hidden-sm d-none d-lg-block">
				<a href="/dashboard/day-trading-room/weekly-watchlist">
					<img src="https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg" alt="Weekly Watchlist image" class="u--border-radius" />
				</a>
			</div>
		</div>
	</section>
</div>
{/if}

<style>
	/* Dashboard Content Section */
	.dashboard__content {
		padding: 0;
	}

	.dashboard__content-main {
		padding: 0;
	}

	.dashboard__content-section {
		padding: 0;
	}

	/* Loading */
	.loading-container {
		display: flex;
		justify-content: center;
		align-items: center;
		min-height: 300px;
	}

	.loading {
		text-align: center;
		color: #666;
	}

	.loading-icon {
		display: inline-block;
		width: 40px;
		height: 40px;
		border: 3px solid #e6e6e6;
		border-top-color: #143E59;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	/* FL Row - Beaver Builder Style Classes */
	.fl-row {
		margin: 0;
		padding: 0;
		width: 100%;
	}

	.fl-row-full-width {
		width: 100%;
	}

	.fl-row-bg-color {
		background-color: #E5E9F4;
	}

	.fl-row-content-wrap {
		max-width: 1200px;
		margin: 0 auto;
		padding: 40px 20px;
	}

	.fl-row-content {
		width: 100%;
	}

	.fl-row-fixed-width {
		max-width: 1100px;
		margin: 0 auto;
	}

	/* FL Column Groups */
	.fl-col-group {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.fl-col-group-equal-height {
		align-items: stretch;
	}

	.fl-col-group-align-bottom {
		align-items: flex-end;
	}

	/* FL Columns */
	.fl-col {
		flex: 1;
		padding: 0 15px;
		min-width: 0;
	}

	.fl-col-small {
		flex: 0 0 300px;
		max-width: 300px;
	}

	.fl-col-content {
		height: 100%;
	}

	.fl-node-content {
		position: relative;
	}

	/* FL Modules */
	.fl-module {
		margin-bottom: 20px;
	}

	.fl-module:last-child {
		margin-bottom: 0;
	}

	.fl-module-content {
		position: relative;
	}

	/* FL Photo */
	.fl-module-photo {
		margin-bottom: 0;
	}

	.fl-photo {
		display: block;
	}

	.fl-photo-align-center {
		text-align: center;
	}

	.fl-photo-content {
		display: inline-block;
		line-height: 0;
	}

	.fl-photo-img-png {
		background: transparent;
	}

	.fl-photo-img {
		max-width: 100%;
		height: auto;
		width: 250px;
		border-radius: 50%;
	}

	/* FL Headings */
	.fl-heading {
		margin: 0;
		padding: 0;
		line-height: 1.2;
	}

	.fl-heading-text {
		display: block;
	}

	.fl-heading-text.trader-name {
		font-size: 36px;
		font-weight: 700;
		color: #333;
	}

	.fl-heading-text.trader-title {
		font-size: 18px;
		font-weight: 400;
		color: #666;
		margin-top: 5px;
	}

	.fl-heading-text.section-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin-bottom: 15px;
	}

	/* FL Separator */
	.fl-module-separator {
		margin: 20px 0;
	}

	.fl-separator {
		border-top: 2px solid #F69532;
		width: 60px;
	}

	/* FL Rich Text */
	.fl-rich-text {
		font-size: 16px;
		line-height: 1.7;
		color: #444;
	}

	.fl-rich-text p {
		margin: 0 0 15px;
	}

	.fl-rich-text p:last-child {
		margin-bottom: 0;
	}

	.fl-rich-text em {
		font-style: italic;
		color: #555;
	}

	/* Trader Pills */
	.pills-section {
		background-color: #fff;
	}

	.pills-section .fl-row-content-wrap {
		padding-top: 30px;
	}

	.trader_pills {
		display: flex;
		flex-wrap: wrap;
		gap: 10px;
		margin-bottom: 30px;
	}

	.trader_pill {
		display: inline-block;
	}

	.trader_pill a {
		display: inline-block;
		padding: 10px 20px;
		background: #f5f5f5;
		color: #333;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 25px;
		border: 1px solid #e0e0e0;
		transition: all 0.2s ease;
	}

	.trader_pill a:hover {
		background: #143E59;
		color: #fff;
		border-color: #143E59;
	}

	.trader_pill a.active {
		background: #143E59;
		color: #fff;
		border-color: #143E59;
	}

	/* Not Found */
	.not-found {
		text-align: center;
		padding: 60px 20px;
		background: #fff;
	}

	.not-found h1 {
		font-size: 28px;
		margin-bottom: 15px;
		color: #333;
	}

	.not-found p {
		color: #666;
		margin-bottom: 25px;
	}

	/* Weekly Watchlist Section */
	.dashboard__content-section.u--background-color-white {
		background: #fff;
		padding: 40px 20px;
		border-top: 1px solid #e6e6e6;
	}

	.dashboard__content-section.u--background-color-white section {
		max-width: 1100px;
		margin: 0 auto;
	}

	.row {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.col-sm-6 {
		flex: 0 0 50%;
		max-width: 50%;
		padding: 0 15px;
	}

	.col-lg-5 {
		flex: 0 0 41.666667%;
		max-width: 41.666667%;
	}

	.col-lg-7 {
		flex: 0 0 58.333333%;
		max-width: 58.333333%;
	}

	.section-title-alt {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
	}

	.section-title-alt--underline {
		padding-bottom: 15px;
		border-bottom: 3px solid #F69532;
	}

	.h5 {
		font-size: 18px;
		margin: 0 0 10px;
	}

	.u--font-weight-bold {
		font-weight: 700;
	}

	.u--border-radius {
		border-radius: 8px;
		width: 100%;
		height: auto;
	}

	.u--hide-read-more p {
		color: #666;
		font-size: 14px;
		margin: 0 0 15px;
	}

	.hidden-md,
	.d-lg-none {
		display: none;
	}

	.d-none {
		display: none;
	}

	.d-lg-block {
		display: block;
	}

	.pb-2 {
		padding-bottom: 0.5rem;
	}

	/* Buttons */
	.btn {
		display: inline-block;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
		cursor: pointer;
	}

	.btn-default {
		background: #143E59;
		color: #fff;
		border: 1px solid #143E59;
	}

	.btn-default:hover {
		background: #0c2638;
		border-color: #0c2638;
	}

	.btn-tiny {
		padding: 8px 16px;
		font-size: 13px;
	}

	/* Responsive */
	@media (max-width: 991px) {
		.fl-col-small {
			flex: 0 0 200px;
			max-width: 200px;
		}

		.fl-photo-img {
			width: 180px;
		}

		.fl-heading-text.trader-name {
			font-size: 28px;
		}

		.col-lg-5,
		.col-lg-7 {
			flex: 0 0 50%;
			max-width: 50%;
		}

		.d-lg-block {
			display: none;
		}

		.d-lg-none {
			display: block;
		}
	}

	@media (max-width: 767px) {
		.fl-col-group {
			flex-direction: column;
		}

		.fl-col-small {
			flex: 0 0 100%;
			max-width: 100%;
			text-align: center;
			margin-bottom: 20px;
		}

		.fl-col {
			flex: 0 0 100%;
			max-width: 100%;
		}

		.fl-photo-img {
			width: 150px;
		}

		.fl-heading-text.trader-name {
			font-size: 24px;
			text-align: center;
		}

		.fl-heading-text.trader-title {
			text-align: center;
		}

		.fl-separator {
			margin: 15px auto;
		}

		.fl-rich-text {
			text-align: center;
		}

		.trader_pills {
			justify-content: center;
		}

		.col-sm-6 {
			flex: 0 0 100%;
			max-width: 100%;
			margin-bottom: 20px;
		}

		.col-sm-6:last-child {
			margin-bottom: 0;
		}

		.hidden-xs,
		.hidden-sm {
			display: none !important;
		}
	}
</style>

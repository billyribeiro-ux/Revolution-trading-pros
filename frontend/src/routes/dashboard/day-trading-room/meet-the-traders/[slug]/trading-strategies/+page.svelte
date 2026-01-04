<!--
	URL: /dashboard/day-trading-room/meet-the-traders/[slug]/trading-strategies

	Trader Trading Strategies Page - WordPress Pixel-Perfect Implementation
	═══════════════════════════════════════════════════════════════════════════
	Trading strategies and techniques from individual traders.

	@version 1.0.0
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
	}

	interface Strategy {
		id: string;
		name: string;
		description: string;
		difficulty: 'Beginner' | 'Intermediate' | 'Advanced';
		timeframe: string;
		markets: string[];
		keyPoints: string[];
	}

	const traderData: { [key: string]: Trader } = {
		'john-carter': {
			id: 'john-carter',
			name: 'John Carter',
			slug: 'john-carter',
			title: 'Founder of Simpler Trading®',
			image: 'https://cdn.simplertrading.com/2021/10/08113910/john-headshot-2020.png',
			quote: '"I think there are a lot of parallels between being a good trader and living a good life. The market truly is the ultimate psychologist."'
		},
		'henry-gambell': {
			id: 'henry-gambell',
			name: 'Henry Gambell',
			slug: 'henry-gambell',
			title: 'Director of Options',
			image: 'https://cdn.simplertrading.com/2021/10/08114059/henry-headshot-2020.png',
			quote: '"Every trade is an opportunity to learn something new about yourself and the markets."'
		},
		'taylor-horton': {
			id: 'taylor-horton',
			name: 'Taylor Horton',
			slug: 'taylor-horton',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2021/10/08114206/taylor-headshot-2020.png',
			quote: '"Success in trading comes from consistency and discipline, not from hitting home runs."'
		},
		'bruce-marshall': {
			id: 'bruce-marshall',
			name: 'Bruce Marshall',
			slug: 'bruce-marshall',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2021/10/08113753/bruce-headshot-2020.png',
			quote: '"Options give you leverage, but risk management keeps you in the game."'
		},
		'danielle-shay': {
			id: 'danielle-shay',
			name: 'Danielle Shay',
			slug: 'danielle-shay',
			title: 'Director of Options',
			image: 'https://cdn.simplertrading.com/2021/10/08113828/danielle-headshot-2020.png',
			quote: '"The best trades are the ones where you manage your risk before thinking about reward."'
		},
		'allison-ostrander': {
			id: 'allison-ostrander',
			name: 'Allison Ostrander',
			slug: 'allison-ostrander',
			title: 'Director of Risk Tolerance',
			image: 'https://cdn.simplertrading.com/2021/10/08113700/allison-headshot-2020.png',
			quote: '"Small accounts can grow into big accounts with the right mindset and strategy."'
		},
		'sam-shames': {
			id: 'sam-shames',
			name: 'Sam Shames',
			slug: 'sam-shames',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2021/10/08114128/sam-headshot-2020.png',
			quote: '"The futures market rewards those who are prepared and punishes those who are not."'
		},
		'kody-ashmore': {
			id: 'kody-ashmore',
			name: 'Kody Ashmore',
			slug: 'kody-ashmore',
			title: 'Senior Futures Trader',
			image: 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg',
			quote: '"Every day in the futures market is a new opportunity to execute your edge."'
		},
		'raghee-horner': {
			id: 'raghee-horner',
			name: 'Raghee Horner',
			slug: 'raghee-horner',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2021/10/08114038/raghee-headshot-2020.png',
			quote: '"The trend is your friend, but only if you know how to find it."'
		}
	};

	const sampleStrategies: Strategy[] = [
		{
			id: '1',
			name: 'Squeeze Pro Momentum',
			description: 'Identify explosive momentum moves using the Squeeze Pro indicator. This strategy focuses on catching breakouts after periods of low volatility consolidation.',
			difficulty: 'Intermediate',
			timeframe: 'Daily / 4-Hour',
			markets: ['SPY', 'QQQ', 'Individual Stocks'],
			keyPoints: [
				'Wait for squeeze to fire (dots change color)',
				'Confirm with momentum histogram direction',
				'Enter on first pullback after breakout',
				'Use ATR-based stops for position sizing'
			]
		},
		{
			id: '2',
			name: 'Voodoo Lines Support/Resistance',
			description: 'Use proprietary Voodoo Lines to identify key support and resistance levels that institutional traders watch.',
			difficulty: 'Beginner',
			timeframe: 'All Timeframes',
			markets: ['All Markets'],
			keyPoints: [
				'Identify major Voodoo Line levels',
				'Watch for price reaction at these levels',
				'Look for confluence with other indicators',
				'Trade bounces or breaks with confirmation'
			]
		},
		{
			id: '3',
			name: 'Options Momentum Swing',
			description: 'Capture multi-day momentum moves using options for leveraged exposure with defined risk.',
			difficulty: 'Advanced',
			timeframe: 'Daily',
			markets: ['High Beta Stocks', 'ETFs'],
			keyPoints: [
				'Select options 30-45 DTE for optimal theta decay',
				'Use delta 0.50-0.70 for good leverage',
				'Scale into positions on pullbacks',
				'Take profits at 50-100% gain'
			]
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
	let strategies: Strategy[] = [];
	let loading = true;
	let currentSlug = '';

	onMount(() => {
		currentSlug = $page.params.slug;
		trader = traderData[currentSlug] || null;
		strategies = sampleStrategies;
		loading = false;
	});

	$: if ($page.params.slug !== currentSlug) {
		currentSlug = $page.params.slug;
		trader = traderData[currentSlug] || null;
	}

	function getDifficultyColor(difficulty: string): string {
		switch (difficulty) {
			case 'Beginner': return '#28a745';
			case 'Intermediate': return '#ffc107';
			case 'Advanced': return '#dc3545';
			default: return '#666';
		}
	}
</script>

<svelte:head>
	{#if trader}
		<title>Trading Strategies | {trader.name} | Revolution Trading Pros</title>
		<meta name="description" content="Trading strategies and techniques from {trader.name}." />
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
				<!-- Trader Header Section -->
				<div class="fl-row fl-row-full-width fl-row-bg-color">
					<div class="fl-row-content-wrap">
						<div class="fl-row-content fl-row-fixed-width fl-node-content">
							<div class="fl-col-group fl-col-group-equal-height fl-col-group-align-bottom">
								<div class="fl-col fl-col-small">
									<div class="fl-col-content fl-node-content">
										<div class="fl-module fl-module-photo fl-visible-desktop-medium">
											<div class="fl-module-content fl-node-content">
												<div class="fl-photo fl-photo-align-center">
													<div class="fl-photo-content fl-photo-img-png">
														<img class="fl-photo-img" src={trader.image} alt={trader.name} width="2000" height="2000" />
													</div>
												</div>
											</div>
										</div>
									</div>
								</div>
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
																<a href="/dashboard/day-trading-room/meet-the-traders/{trader.slug}{pill.path}"
																   class:active={pill.path === '/trading-strategies'}
																   on:click|preventDefault={() => {
																	   if (pill.path === '/trader-store') {
																		   window.location.href = `/dashboard/day-trading-room/trader-store/${trader?.slug}`;
																	   } else {
																		   window.location.href = `/dashboard/day-trading-room/meet-the-traders/${trader?.slug}${pill.path}`;
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

							<!-- Page Content -->
							<div class="fl-col-group">
								<div class="fl-col">
									<div class="fl-col-content fl-node-content">
										<div class="fl-module fl-module-heading">
											<div class="fl-module-content fl-node-content">
												<h2 class="fl-heading">
													<span class="fl-heading-text section-title">{trader.name.split(' ')[0]}'s Trading Strategies</span>
												</h2>
											</div>
										</div>
										<div class="fl-module fl-module-rich-text">
											<div class="fl-module-content fl-node-content">
												<div class="fl-rich-text">
													<p>Explore the trading strategies developed and refined by {trader.name}. Each strategy includes detailed entry and exit criteria, risk management rules, and best practices.</p>
												</div>
											</div>
										</div>
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>

				<!-- Strategies Content -->
				<div class="content-container">
					<div class="strategies-grid">
						{#each strategies as strategy (strategy.id)}
							<article class="strategy-card">
								<header class="card-header">
									<h3 class="strategy-name">{strategy.name}</h3>
									<span class="difficulty-badge" style="background-color: {getDifficultyColor(strategy.difficulty)}">
										{strategy.difficulty}
									</span>
								</header>
								<div class="card-body">
									<p class="strategy-description">{strategy.description}</p>
									
									<div class="strategy-meta">
										<div class="meta-item">
											<span class="meta-label">Timeframe</span>
											<span class="meta-value">{strategy.timeframe}</span>
										</div>
										<div class="meta-item">
											<span class="meta-label">Markets</span>
											<span class="meta-value">{strategy.markets.join(', ')}</span>
										</div>
									</div>

									<div class="key-points">
										<h4>Key Points</h4>
										<ul>
											{#each strategy.keyPoints as point}
												<li>{point}</li>
											{/each}
										</ul>
									</div>
								</div>
								<footer class="card-footer">
									<a href="/dashboard/day-trading-room/trader-store/{trader.slug}" class="btn btn-xs btn-default">Learn This Strategy</a>
								</footer>
							</article>
						{/each}
					</div>

					{#if strategies.length === 0}
						<div class="empty-state">
							<p>No strategies available at this time. Check back soon!</p>
						</div>
					{/if}
				</div>
			{:else}
				<div class="not-found">
					<h1>Trader Not Found</h1>
					<p>The trader you're looking for could not be found.</p>
					<a href="/dashboard/day-trading-room/meet-the-traders" class="btn btn-default">← Back to All Traders</a>
				</div>
			{/if}
		</section>
	</div>
</div>

<style>
	.dashboard__content { padding: 0; }
	.dashboard__content-main { padding: 0; }
	.dashboard__content-section { padding: 0; }

	.loading-container { display: flex; justify-content: center; align-items: center; min-height: 300px; }
	.loading { text-align: center; color: #666; }
	.loading-icon { display: inline-block; width: 40px; height: 40px; border: 3px solid #e6e6e6; border-top-color: #143E59; border-radius: 50%; animation: spin 1s linear infinite; }
	@keyframes spin { to { transform: rotate(360deg); } }

	.fl-row { margin: 0; padding: 0; width: 100%; }
	.fl-row-full-width { width: 100%; }
	.fl-row-bg-color { background-color: #E5E9F4; }
	.fl-row-content-wrap { max-width: 1200px; margin: 0 auto; padding: 40px 20px; }
	.fl-row-content { width: 100%; }
	.fl-row-fixed-width { max-width: 1100px; margin: 0 auto; }

	.fl-col-group { display: flex; flex-wrap: wrap; margin: 0 -15px; }
	.fl-col-group-equal-height { align-items: stretch; }
	.fl-col-group-align-bottom { align-items: flex-end; }
	.fl-col { flex: 1; padding: 0 15px; min-width: 0; }
	.fl-col-small { flex: 0 0 300px; max-width: 300px; }
	.fl-col-content { height: 100%; }
	.fl-node-content { position: relative; }

	.fl-module { margin-bottom: 20px; }
	.fl-module:last-child { margin-bottom: 0; }
	.fl-module-content { position: relative; }
	.fl-module-photo { margin-bottom: 0; }
	.fl-photo { display: block; }
	.fl-photo-align-center { text-align: center; }
	.fl-photo-content { display: inline-block; line-height: 0; }
	.fl-photo-img-png { background: transparent; }
	.fl-photo-img { max-width: 100%; height: auto; width: 250px; border-radius: 50%; }

	.fl-heading { margin: 0; padding: 0; line-height: 1.2; }
	.fl-heading-text { display: block; }
	.fl-heading-text.trader-name { font-size: 36px; font-weight: 700; color: #333; }
	.fl-heading-text.trader-title { font-size: 18px; font-weight: 400; color: #666; margin-top: 5px; }
	.fl-heading-text.section-title { font-size: 24px; font-weight: 700; color: #333; margin-bottom: 15px; }

	.fl-module-separator { margin: 20px 0; }
	.fl-separator { border-top: 2px solid #F69532; width: 60px; }

	.fl-rich-text { font-size: 16px; line-height: 1.7; color: #444; }
	.fl-rich-text p { margin: 0 0 15px; }
	.fl-rich-text p:last-child { margin-bottom: 0; }
	.fl-rich-text em { font-style: italic; color: #555; }

	.pills-section { background-color: #fff; }
	.pills-section .fl-row-content-wrap { padding-top: 30px; }
	.trader_pills { display: flex; flex-wrap: wrap; gap: 10px; margin-bottom: 30px; }
	.trader_pill { display: inline-block; }
	.trader_pill a { display: inline-block; padding: 10px 20px; background: #f5f5f5; color: #333; font-size: 14px; font-weight: 600; text-decoration: none; border-radius: 25px; border: 1px solid #e0e0e0; transition: all 0.2s ease; }
	.trader_pill a:hover { background: #143E59; color: #fff; border-color: #143E59; }
	.trader_pill a.active { background: #143E59; color: #fff; border-color: #143E59; }

	.content-container { max-width: 1100px; margin: 0 auto; padding: 30px 20px; }

	.strategies-grid { display: grid; grid-template-columns: repeat(1, 1fr); gap: 25px; }

	.strategy-card { background: #fff; border: 1px solid #e6e6e6; border-radius: 8px; overflow: hidden; transition: box-shadow 0.3s ease; }
	.strategy-card:hover { box-shadow: 0 5px 20px rgba(0, 0, 0, 0.08); }

	.card-header { display: flex; justify-content: space-between; align-items: center; padding: 20px; background: #f9f9f9; border-bottom: 1px solid #e6e6e6; }
	.strategy-name { font-size: 20px; font-weight: 700; color: #333; margin: 0; }
	.difficulty-badge { display: inline-block; padding: 5px 12px; color: #fff; font-size: 11px; font-weight: 600; border-radius: 3px; text-transform: uppercase; }

	.card-body { padding: 20px; }
	.strategy-description { font-size: 15px; color: #555; line-height: 1.7; margin: 0 0 20px; }

	.strategy-meta { display: flex; gap: 30px; margin-bottom: 20px; padding-bottom: 20px; border-bottom: 1px solid #e6e6e6; }
	.meta-item { display: flex; flex-direction: column; gap: 4px; }
	.meta-label { font-size: 11px; text-transform: uppercase; color: #999; font-weight: 600; }
	.meta-value { font-size: 14px; color: #333; font-weight: 500; }

	.key-points h4 { font-size: 14px; font-weight: 700; color: #333; margin: 0 0 10px; text-transform: uppercase; }
	.key-points ul { margin: 0; padding: 0 0 0 20px; }
	.key-points li { font-size: 14px; color: #555; line-height: 1.8; margin-bottom: 5px; }
	.key-points li:last-child { margin-bottom: 0; }

	.card-footer { padding: 15px 20px; background: #fafafa; border-top: 1px solid #e6e6e6; }

	.btn { display: inline-block; padding: 10px 20px; font-size: 14px; font-weight: 600; text-align: center; text-decoration: none; border-radius: 4px; transition: all 0.2s ease; cursor: pointer; }
	.btn-xs { padding: 8px 16px; font-size: 12px; }
	.btn-default { background: #143E59; color: #fff; border: 1px solid #143E59; }
	.btn-default:hover { background: #0c2638; border-color: #0c2638; }

	.empty-state { text-align: center; padding: 60px 20px; color: #666; }
	.not-found { text-align: center; padding: 60px 20px; background: #fff; }
	.not-found h1 { font-size: 28px; margin-bottom: 15px; color: #333; }
	.not-found p { color: #666; margin-bottom: 25px; }

	@media (max-width: 991px) {
		.fl-col-small { flex: 0 0 200px; max-width: 200px; }
		.fl-photo-img { width: 180px; }
		.fl-heading-text.trader-name { font-size: 28px; }
	}

	@media (max-width: 767px) {
		.fl-col-group { flex-direction: column; }
		.fl-col-small { flex: 0 0 100%; max-width: 100%; text-align: center; margin-bottom: 20px; }
		.fl-col { flex: 0 0 100%; max-width: 100%; }
		.fl-photo-img { width: 150px; }
		.fl-heading-text.trader-name { font-size: 24px; text-align: center; }
		.fl-heading-text.trader-title { text-align: center; }
		.fl-separator { margin: 15px auto; }
		.fl-rich-text { text-align: center; }
		.trader_pills { justify-content: center; }
		.card-header { flex-direction: column; gap: 10px; text-align: center; }
		.strategy-meta { flex-direction: column; gap: 15px; }
	}
</style>

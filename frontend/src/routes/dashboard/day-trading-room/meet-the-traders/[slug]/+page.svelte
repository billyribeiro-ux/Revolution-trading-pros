<!--
	URL: /dashboard/day-trading-room/meet-the-traders/[slug]

	Individual Trader Profile Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation

	Individual trader profile matching WordPress implementation.

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
		bio: string;
		fullBio: string;
		specialties: string[];
		tradingStyle: string;
		experience: string;
		favoriteMarkets: string[];
		socialLinks: { platform: string; url: string }[];
	}

	const traders: Trader[] = [
		{
			id: 'john-carter',
			name: 'John Carter',
			slug: 'john-carter',
			title: 'Founder & CEO',
			image: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
			bio: 'John Carter is the founder of Simpler Trading and has been trading for over 25 years.',
			fullBio: `John Carter is the founder of Simpler Trading and has been actively trading since 1996. He is a best-selling author of "Mastering the Trade" and is known for developing the Squeeze Pro indicator. John specializes in options trading and market cycles, helping traders identify high-probability setups.

His approach focuses on understanding market cycles and using momentum-based strategies to capture significant moves. He has been featured on CNBC, Bloomberg, and various trading publications.`,
			specialties: ['Options', 'Market Cycles', 'Squeeze Pro'],
			tradingStyle: 'Momentum & Cycles',
			experience: '25+ years',
			favoriteMarkets: ['SPY', 'QQQ', 'TSLA', 'AMZN'],
			socialLinks: [{ platform: 'Twitter', url: '#' }]
		},
		{
			id: 'henry-gambell',
			name: 'Henry Gambell',
			slug: 'henry-gambell',
			title: 'Director of Options',
			image: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			bio: 'Henry Gambell specializes in options trading and technical analysis.',
			fullBio: `Henry Gambell is the Director of Options at Simpler Trading. He focuses on high-probability options setups using technical analysis and risk management principles.

Henry's trading style emphasizes patience and discipline, waiting for the right setups before entering trades. He is known for his methodical approach to market analysis and his ability to break down complex concepts for traders of all levels.`,
			specialties: ['Options', 'Technical Analysis', 'Risk Management'],
			tradingStyle: 'Technical & Methodical',
			experience: '15+ years',
			favoriteMarkets: ['SPY', 'IWM', 'Individual Stocks'],
			socialLinks: [{ platform: 'Twitter', url: '#' }]
		},
		{
			id: 'taylor-horton',
			name: 'Taylor Horton',
			slug: 'taylor-horton',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			bio: 'Taylor Horton is known for his day trading strategies and market analysis techniques.',
			fullBio: `Taylor Horton is a Senior Trader at Simpler Trading specializing in day trading and swing trading strategies. He focuses on momentum plays and breakout patterns.

Taylor brings energy and enthusiasm to his trading sessions, helping members identify actionable setups in real-time.`,
			specialties: ['Day Trading', 'Market Analysis', 'Trade Setups'],
			tradingStyle: 'Momentum Day Trading',
			experience: '10+ years',
			favoriteMarkets: ['ES', 'NQ', 'Growth Stocks'],
			socialLinks: [{ platform: 'Twitter', url: '#' }]
		},
		{
			id: 'bruce-marshall',
			name: 'Bruce Marshall',
			slug: 'bruce-marshall',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			bio: 'Bruce Marshall focuses on options strategies and portfolio management.',
			fullBio: `Bruce Marshall is a Senior Trader with expertise in options strategies and portfolio management. He focuses on income-generating strategies and risk-defined trades.

Bruce's approach emphasizes consistency and capital preservation, making him a favorite among traders looking for steady returns.`,
			specialties: ['Options Strategies', 'Portfolio Management'],
			tradingStyle: 'Income Generation',
			experience: '20+ years',
			favoriteMarkets: ['ETFs', 'Blue Chips'],
			socialLinks: [{ platform: 'Twitter', url: '#' }]
		},
		{
			id: 'danielle-shay',
			name: 'Danielle Shay',
			slug: 'danielle-shay',
			title: 'Director of Options',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			bio: 'Danielle Shay specializes in options trading with a focus on momentum and volatility strategies.',
			fullBio: `Danielle Shay is the Director of Options at Simpler Trading. She specializes in momentum-based options strategies and volatility trading.

Danielle is known for her ability to identify high-probability setups in fast-moving markets. She focuses on helping traders understand options Greeks and position sizing.`,
			specialties: ['Options', 'Momentum Trading', 'Volatility'],
			tradingStyle: 'Momentum Options',
			experience: '12+ years',
			favoriteMarkets: ['High Beta Stocks', 'SPY', 'QQQ'],
			socialLinks: [{ platform: 'Twitter', url: '#' }]
		},
		{
			id: 'allison-ostrander',
			name: 'Allison Ostrander',
			slug: 'allison-ostrander',
			title: 'Director of Risk Tolerance',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			bio: 'Allison Ostrander focuses on risk management and small account trading strategies.',
			fullBio: `Allison Ostrander is the Director of Risk Tolerance at Simpler Trading. She specializes in helping traders with smaller accounts grow their portfolios responsibly.

Allison's approach emphasizes proper position sizing, risk management, and building a sustainable trading practice.`,
			specialties: ['Risk Management', 'Small Account Trading'],
			tradingStyle: 'Conservative Growth',
			experience: '10+ years',
			favoriteMarkets: ['Small Cap Stocks', 'ETFs'],
			socialLinks: [{ platform: 'Twitter', url: '#' }]
		},
		{
			id: 'sam-shames',
			name: 'Sam Shames',
			slug: 'sam-shames',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			bio: 'Sam Shames specializes in futures and day trading strategies.',
			fullBio: `Sam Shames is a Senior Trader specializing in futures and day trading. He focuses on ES and NQ futures with a momentum-based approach.

Sam is known for his quick analysis and ability to adapt to changing market conditions.`,
			specialties: ['Futures', 'Day Trading'],
			tradingStyle: 'Futures Day Trading',
			experience: '8+ years',
			favoriteMarkets: ['ES', 'NQ', 'CL'],
			socialLinks: [{ platform: 'Twitter', url: '#' }]
		},
		{
			id: 'kody-ashmore',
			name: 'Kody Ashmore',
			slug: 'kody-ashmore',
			title: 'Senior Futures Trader',
			image: 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg',
			bio: 'Kody Ashmore is an expert in futures trading and day trading strategies.',
			fullBio: `Kody Ashmore is a Senior Futures Trader at Simpler Trading. He specializes in ES and NQ futures with a focus on intraday momentum setups.

Kody's trading style is aggressive yet disciplined, with strict risk management rules. He is known for his daily trading sessions where he shares live setups with members.`,
			specialties: ['Futures', 'Day Trading', 'ES/NQ'],
			tradingStyle: 'Aggressive Day Trading',
			experience: '10+ years',
			favoriteMarkets: ['ES', 'NQ', 'RTY'],
			socialLinks: [{ platform: 'Twitter', url: '#' }]
		},
		{
			id: 'raghee-horner',
			name: 'Raghee Horner',
			slug: 'raghee-horner',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			bio: 'Raghee Horner specializes in forex and futures trading with decades of experience.',
			fullBio: `Raghee Horner is a Senior Trader with over 30 years of trading experience. She specializes in forex and futures trading with a focus on technical analysis.

Raghee is known for her proprietary indicators and trading methodology. She has authored multiple books on trading and has helped thousands of traders improve their skills.`,
			specialties: ['Forex', 'Futures', 'Technical Analysis'],
			tradingStyle: 'Technical Trend Following',
			experience: '30+ years',
			favoriteMarkets: ['EUR/USD', 'GBP/USD', 'ES'],
			socialLinks: [{ platform: 'Twitter', url: '#' }]
		}
	];

	let trader: Trader | null = null;
	let loading = true;

	onMount(() => {
		const slug = $page.params.slug;
		trader = traders.find(t => t.slug === slug) || null;
		loading = false;
	});
</script>

<svelte:head>
	{#if trader}
		<title>{trader.name} | Meet the Traders | Revolution Trading Pros</title>
		<meta name="description" content={trader.bio} />
	{:else}
		<title>Trader Not Found | Revolution Trading Pros</title>
	{/if}
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- Breadcrumbs -->
<nav class="breadcrumbs" aria-label="Breadcrumb">
	<ul>
		<li><a href="/dashboard">Dashboard</a></li>
		<li class="separator">/</li>
		<li><a href="/dashboard/day-trading-room">Day Trading Room</a></li>
		<li class="separator">/</li>
		<li><a href="/dashboard/day-trading-room/meet-the-traders">Meet the Traders</a></li>
		<li class="separator">/</li>
		{#if trader}
			<li class="current">{trader.name}</li>
		{:else}
			<li class="current">Not Found</li>
		{/if}
	</ul>
</nav>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		{#if loading}
			<div class="loading">Loading...</div>
		{:else if trader}
			<div class="trader-profile">
				<div class="trader-profile__header">
					<figure class="trader-profile__image">
						<img src={trader.image} alt={trader.name} />
					</figure>
					
					<div class="trader-profile__info">
						<h1 class="trader-profile__name">{trader.name}</h1>
						<p class="trader-profile__title">{trader.title}</p>
						
						<div class="trader-profile__meta">
							<div class="meta-item">
								<span class="meta-label">Experience</span>
								<span class="meta-value">{trader.experience}</span>
							</div>
							<div class="meta-item">
								<span class="meta-label">Trading Style</span>
								<span class="meta-value">{trader.tradingStyle}</span>
							</div>
						</div>
						
						<div class="trader-profile__specialties">
							{#each trader.specialties as specialty}
								<span class="specialty-tag">{specialty}</span>
							{/each}
						</div>
					</div>
				</div>
				
				<div class="trader-profile__body">
					<section class="profile-section">
						<h2>About {trader.name}</h2>
						<div class="bio-content">
							{#each trader.fullBio.split('\n\n') as paragraph}
								<p>{paragraph}</p>
							{/each}
						</div>
					</section>
					
					<section class="profile-section">
						<h2>Favorite Markets</h2>
						<div class="markets-list">
							{#each trader.favoriteMarkets as market}
								<span class="market-tag">{market}</span>
							{/each}
						</div>
					</section>
					
					<section class="profile-section">
						<h2>Resources</h2>
						<div class="resources-links">
							<a href="/dashboard/day-trading-room/trader-store/{trader.slug}" class="resource-link">
								<span class="resource-icon">🛒</span>
								<span class="resource-text">{trader.name}'s Trader Store</span>
							</a>
							<a href="/dashboard/day-trading-room/learning-center" class="resource-link">
								<span class="resource-icon">📚</span>
								<span class="resource-text">Learning Center</span>
							</a>
						</div>
					</section>
				</div>
				
				<div class="back-link">
					<a href="/dashboard/day-trading-room/meet-the-traders" class="btn btn-secondary">
						← Back to All Traders
					</a>
				</div>
			</div>
		{:else}
			<div class="not-found">
				<h1>Trader Not Found</h1>
				<p>The trader you're looking for could not be found.</p>
				<a href="/dashboard/day-trading-room/meet-the-traders" class="btn btn-primary">
					← Back to All Traders
				</a>
			</div>
		{/if}
	</div>
</div>

<style>
	/* Breadcrumbs */
	.breadcrumbs {
		background: #f5f5f5;
		padding: 12px 20px;
		font-size: 13px;
		border-bottom: 1px solid #e6e6e6;
		margin-bottom: 30px;
	}

	.breadcrumbs ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 8px;
	}

	.breadcrumbs li {
		color: #666;
	}

	.breadcrumbs li a {
		color: #1e73be;
		text-decoration: none;
	}

	.breadcrumbs li a:hover {
		text-decoration: underline;
	}

	.breadcrumbs .separator {
		color: #999;
	}

	.breadcrumbs .current {
		color: #333;
		font-weight: 600;
	}

	/* Trader Profile */
	.trader-profile {
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 8px;
		overflow: hidden;
	}

	.trader-profile__header {
		display: flex;
		gap: 30px;
		padding: 30px;
		background: linear-gradient(135deg, #143E59 0%, #0c2638 100%);
		color: #fff;
	}

	.trader-profile__image {
		width: 200px;
		height: 200px;
		margin: 0;
		border-radius: 8px;
		overflow: hidden;
		flex-shrink: 0;
	}

	.trader-profile__image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.trader-profile__info {
		flex: 1;
	}

	.trader-profile__name {
		font-size: 32px;
		font-weight: 700;
		margin: 0 0 5px;
	}

	.trader-profile__title {
		font-size: 18px;
		color: #F69532;
		font-weight: 600;
		margin: 0 0 20px;
	}

	.trader-profile__meta {
		display: flex;
		gap: 30px;
		margin-bottom: 20px;
	}

	.meta-item {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.meta-label {
		font-size: 12px;
		text-transform: uppercase;
		opacity: 0.7;
	}

	.meta-value {
		font-size: 16px;
		font-weight: 600;
	}

	.trader-profile__specialties {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
	}

	.specialty-tag {
		display: inline-block;
		padding: 6px 12px;
		background: rgba(255, 255, 255, 0.2);
		color: #fff;
		font-size: 12px;
		font-weight: 600;
		border-radius: 4px;
		text-transform: uppercase;
	}

	/* Body */
	.trader-profile__body {
		padding: 30px;
	}

	.profile-section {
		margin-bottom: 30px;
	}

	.profile-section:last-child {
		margin-bottom: 0;
	}

	.profile-section h2 {
		font-size: 20px;
		font-weight: 700;
		color: #333;
		margin: 0 0 15px;
		padding-bottom: 10px;
		border-bottom: 2px solid #e6e6e6;
	}

	.bio-content p {
		font-size: 16px;
		line-height: 1.7;
		color: #444;
		margin: 0 0 15px;
	}

	.bio-content p:last-child {
		margin-bottom: 0;
	}

	.markets-list {
		display: flex;
		flex-wrap: wrap;
		gap: 10px;
	}

	.market-tag {
		display: inline-block;
		padding: 8px 16px;
		background: #e8f4fc;
		color: #0984ae;
		font-size: 14px;
		font-weight: 600;
		border-radius: 4px;
	}

	/* Resources */
	.resources-links {
		display: flex;
		flex-wrap: wrap;
		gap: 15px;
	}

	.resource-link {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 15px 20px;
		background: #f9f9f9;
		border: 1px solid #e6e6e6;
		border-radius: 6px;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.resource-link:hover {
		background: #f0f0f0;
		border-color: #ccc;
	}

	.resource-icon {
		font-size: 24px;
	}

	.resource-text {
		font-size: 14px;
		font-weight: 600;
		color: #333;
	}

	/* Back Link */
	.back-link {
		padding: 20px 30px;
		border-top: 1px solid #e6e6e6;
		background: #f9f9f9;
	}

	.btn {
		display: inline-block;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
	}

	.btn-primary {
		background: #F69532;
		color: #fff;
	}

	.btn-primary:hover {
		background: #dc7309;
	}

	.btn-secondary {
		background: #fff;
		color: #333;
		border: 1px solid #ddd;
	}

	.btn-secondary:hover {
		background: #f5f5f5;
	}

	/* Not Found */
	.not-found {
		text-align: center;
		padding: 60px 20px;
	}

	.not-found h1 {
		font-size: 28px;
		margin-bottom: 15px;
	}

	.not-found p {
		color: #666;
		margin-bottom: 25px;
	}

	/* Loading */
	.loading {
		text-align: center;
		padding: 60px 20px;
		color: #666;
	}

	/* Responsive */
	@media (max-width: 767px) {
		.trader-profile__header {
			flex-direction: column;
			align-items: center;
			text-align: center;
		}

		.trader-profile__image {
			width: 150px;
			height: 150px;
		}

		.trader-profile__meta {
			justify-content: center;
		}

		.trader-profile__specialties {
			justify-content: center;
		}

		.trader-profile__name {
			font-size: 24px;
		}
	}
</style>

<!--
	URL: /dashboard/day-trading-room/meet-the-traders

	Meet the Traders Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation

	Directory of all traders matching WordPress implementation exactly.

	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import TradingRoomDropdown from '$lib/components/dashboard/TradingRoomDropdown.svelte';

	interface Trader {
		id: string;
		name: string;
		slug: string;
		title: string;
		image: string;
		bio: string;
		specialties: string[];
	}

	const traders: Trader[] = [
		{
			id: 'john-carter',
			name: 'John Carter',
			slug: 'john-carter',
			title: 'Founder & CEO',
			image: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
			bio: 'John Carter is the founder of Simpler Trading and has been trading for over 25 years. He is known for his expertise in options trading and market cycles.',
			specialties: ['Options', 'Market Cycles', 'Squeeze Pro']
		},
		{
			id: 'henry-gambell',
			name: 'Henry Gambell',
			slug: 'henry-gambell',
			title: 'Director of Options',
			image: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			bio: 'Henry Gambell specializes in options trading and technical analysis. He focuses on high-probability setups and risk management.',
			specialties: ['Options', 'Technical Analysis', 'Risk Management']
		},
		{
			id: 'taylor-horton',
			name: 'Taylor Horton',
			slug: 'taylor-horton',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			bio: 'Taylor Horton is known for his day trading strategies and market analysis techniques.',
			specialties: ['Day Trading', 'Market Analysis', 'Trade Setups']
		},
		{
			id: 'bruce-marshall',
			name: 'Bruce Marshall',
			slug: 'bruce-marshall',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			bio: 'Bruce Marshall focuses on options strategies and portfolio management.',
			specialties: ['Options Strategies', 'Portfolio Management']
		},
		{
			id: 'danielle-shay',
			name: 'Danielle Shay',
			slug: 'danielle-shay',
			title: 'Director of Options',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			bio: 'Danielle Shay specializes in options trading with a focus on momentum and volatility strategies.',
			specialties: ['Options', 'Momentum Trading', 'Volatility']
		},
		{
			id: 'allison-ostrander',
			name: 'Allison Ostrander',
			slug: 'allison-ostrander',
			title: 'Director of Risk Tolerance',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			bio: 'Allison Ostrander focuses on risk management and small account trading strategies.',
			specialties: ['Risk Management', 'Small Account Trading']
		},
		{
			id: 'sam-shames',
			name: 'Sam Shames',
			slug: 'sam-shames',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			bio: 'Sam Shames specializes in futures and day trading strategies.',
			specialties: ['Futures', 'Day Trading']
		},
		{
			id: 'kody-ashmore',
			name: 'Kody Ashmore',
			slug: 'kody-ashmore',
			title: 'Senior Futures Trader',
			image: 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg',
			bio: 'Kody Ashmore is an expert in futures trading and day trading strategies with a focus on ES and NQ.',
			specialties: ['Futures', 'Day Trading', 'ES/NQ']
		},
		{
			id: 'raghee-horner',
			name: 'Raghee Horner',
			slug: 'raghee-horner',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			bio: 'Raghee Horner specializes in forex and futures trading with decades of experience.',
			specialties: ['Forex', 'Futures', 'Technical Analysis']
		}
	];
</script>

<svelte:head>
	<title>Meet the Traders | Day Trading Room | Revolution Trading Pros</title>
	<meta name="description" content="Meet our expert traders and learn from their years of experience in the markets." />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Meet the Traders</h1>
	</div>
	<div class="dashboard__header-right">
		<TradingRoomDropdown />
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<p class="intro-text">
			Learn from our team of experienced traders. Each trader brings unique expertise and strategies to help you improve your trading.
		</p>

		<div class="traders-grid">
			{#each traders as trader (trader.id)}
				<article class="trader-card">
					<figure class="trader-card__image">
						<img src={trader.image} alt={trader.name} loading="lazy" />
					</figure>
					
					<div class="trader-card__content">
						<h3 class="trader-card__name">{trader.name}</h3>
						<p class="trader-card__title">{trader.title}</p>
						
						<div class="trader-card__specialties">
							{#each trader.specialties as specialty}
								<span class="specialty-tag">{specialty}</span>
							{/each}
						</div>
						
						<p class="trader-card__bio">{trader.bio}</p>
						
						<a href="/dashboard/day-trading-room/meet-the-traders/{trader.slug}" class="btn btn-primary">
							View Profile
						</a>
					</div>
				</article>
			{/each}
		</div>
	</div>
</div>

<style>
	.intro-text {
		font-size: 16px;
		color: #666;
		margin-bottom: 30px;
		line-height: 1.6;
	}

	.traders-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 30px;
	}

	.trader-card {
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 8px;
		overflow: hidden;
		transition: box-shadow 0.3s ease;
	}

	.trader-card:hover {
		box-shadow: 0 5px 20px rgba(0, 0, 0, 0.1);
	}

	.trader-card__image {
		width: 100%;
		height: 200px;
		margin: 0;
		overflow: hidden;
	}

	.trader-card__image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.trader-card__content {
		padding: 20px;
	}

	.trader-card__name {
		font-size: 20px;
		font-weight: 700;
		color: #333;
		margin: 0 0 5px;
	}

	.trader-card__title {
		font-size: 14px;
		color: #0984ae;
		font-weight: 600;
		margin: 0 0 15px;
	}

	.trader-card__specialties {
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
		margin-bottom: 15px;
	}

	.specialty-tag {
		display: inline-block;
		padding: 4px 10px;
		background: #e8f4fc;
		color: #0984ae;
		font-size: 11px;
		font-weight: 600;
		border-radius: 3px;
		text-transform: uppercase;
	}

	.trader-card__bio {
		font-size: 14px;
		color: #666;
		line-height: 1.5;
		margin: 0 0 20px;
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
		border: none;
	}

	.btn-primary:hover {
		background: #dc7309;
	}

	/* Responsive */
	@media (max-width: 1199px) {
		.traders-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 767px) {
		.traders-grid {
			grid-template-columns: 1fr;
			gap: 20px;
		}
	}
</style>

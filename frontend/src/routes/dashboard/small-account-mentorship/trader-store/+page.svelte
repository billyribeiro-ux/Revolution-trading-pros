<!--
	URL: /dashboard/day-trading-room/trader-store

	Trader Store Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation

	Directory of all trader stores matching WordPress implementation exactly.

	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import TradingRoomDropdown from '$lib/components/dashboard/TradingRoomDropdown.svelte';

	interface TraderStore {
		id: string;
		name: string;
		slug: string;
		image: string;
		description: string;
		productCount: number;
	}

	const traderStores: TraderStore[] = [
		{
			id: 'john-carter',
			name: 'John Carter',
			slug: 'john-carter',
			image:
				'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
			description: 'Premium courses, indicators, and trading tools from John Carter.',
			productCount: 12
		},
		{
			id: 'henry-gambell',
			name: 'Henry Gambell',
			slug: 'henry-gambell',
			image: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			description: 'Options trading courses and technical analysis resources.',
			productCount: 8
		},
		{
			id: 'taylor-horton',
			name: 'Taylor Horton',
			slug: 'taylor-horton',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'Day trading strategies and momentum trading courses.',
			productCount: 6
		},
		{
			id: 'bruce-marshall',
			name: 'Bruce Marshall',
			slug: 'bruce-marshall',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'Options income strategies and portfolio management courses.',
			productCount: 5
		},
		{
			id: 'danielle-shay',
			name: 'Danielle Shay',
			slug: 'danielle-shay',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'Momentum options trading and volatility strategies.',
			productCount: 7
		},
		{
			id: 'allison-ostrander',
			name: 'Allison Ostrander',
			slug: 'allison-ostrander',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'Small account trading and risk management courses.',
			productCount: 4
		},
		{
			id: 'sam-shames',
			name: 'Sam Shames',
			slug: 'sam-shames',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'Futures trading strategies and day trading courses.',
			productCount: 5
		},
		{
			id: 'kody-ashmore',
			name: 'Kody Ashmore',
			slug: 'kody-ashmore',
			image: 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg',
			description: 'ES and NQ futures trading courses and daily setups.',
			productCount: 6
		},
		{
			id: 'raghee-horner',
			name: 'Raghee Horner',
			slug: 'raghee-horner',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'Forex and futures trading courses with proprietary indicators.',
			productCount: 9
		}
	];
</script>

<svelte:head>
	<title>Trader Store | Small Account Mentorship | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Browse courses, indicators, and trading tools from our expert traders."
	/>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Trader Store</h1>
	</div>
	<div class="dashboard__header-right">
		<TradingRoomDropdown />
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<p class="intro-text">
			Browse premium courses, indicators, and trading tools from our expert traders. Each store
			features exclusive content and resources.
		</p>

		<div class="stores-grid">
			{#each traderStores as store (store.id)}
				<a href="/dashboard/small-account-mentorship/trader-store/{store.slug}" class="store-card">
					<figure class="store-card__image">
						<img src={store.image} alt={store.name} loading="lazy" />
					</figure>

					<div class="store-card__content">
						<h3 class="store-card__name">{store.name}'s Store</h3>
						<p class="store-card__description">{store.description}</p>
						<div class="store-card__meta">
							<span class="product-count">{store.productCount} Products</span>
							<span class="view-store">View Store →</span>
						</div>
					</div>
				</a>
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

	.stores-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 25px;
	}

	.store-card {
		display: block;
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 8px;
		overflow: hidden;
		text-decoration: none;
		transition: all 0.3s ease;
	}

	.store-card:hover {
		box-shadow: 0 5px 20px rgba(0, 0, 0, 0.1);
		transform: translateY(-3px);
	}

	.store-card__image {
		width: 100%;
		height: 160px;
		margin: 0;
		overflow: hidden;
		position: relative;
	}

	.store-card__image::after {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: linear-gradient(to bottom, transparent 50%, rgba(0, 0, 0, 0.5) 100%);
	}

	.store-card__image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.store-card__content {
		padding: 20px;
	}

	.store-card__name {
		font-size: 18px;
		font-weight: 700;
		color: #333;
		margin: 0 0 10px;
	}

	.store-card__description {
		font-size: 14px;
		color: #666;
		line-height: 1.5;
		margin: 0 0 15px;
	}

	.store-card__meta {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.product-count {
		font-size: 13px;
		color: #999;
	}

	.view-store {
		font-size: 14px;
		font-weight: 600;
		color: #f69532;
	}

	.store-card:hover .view-store {
		color: #dc7309;
	}

	/* Mobile-first: 1 column by default, 2 on md+, 3 on lg+ */
	.stores-grid {
		grid-template-columns: 1fr;
		gap: 20px;
	}

	@media (min-width: 768px) {
		.stores-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 30px;
		}
	}

	@media (min-width: 1200px) {
		.stores-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}
</style>

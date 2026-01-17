<!--
	URL: /dashboard/spx-profit-pulse/billy-ribeiro

	Billy Ribeiro Trader Page - SPX Profit Pulse
	═══════════════════════════════════════════════════════════════════════════
	Same pattern as day-trading-room trader-store pages.

	@version 1.0.0 - January 2026
	@author Revolution Trading Pros
-->
<script lang="ts">
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';

	interface Product {
		id: string;
		name: string;
		price: number;
		formattedPrice: string;
		image: string;
		description: string;
		url: string;
	}

	const trader = {
		name: 'Billy Ribeiro',
		slug: 'billy-ribeiro',
		title: 'SPX Trading Expert',
		image: 'https://cdn.simplertrading.com/2024/09/23132611/Jon-Generic.jpg',
		quote: '"Master the SPX market with discipline, patience, and a solid trading plan."'
	};

	const products: Product[] = [
		{
			id: 'br-1',
			name: 'SPX Profit Pulse Mastery',
			price: 1497,
			formattedPrice: '$1,497',
			image: 'https://cdn.simplertrading.com/2024/01/29162759/SimplerCentral_JM.jpg',
			description: 'Complete SPX trading system for consistent profits',
			url: '#'
		},
		{
			id: 'br-2',
			name: 'SPX Options Strategy Course',
			price: 997,
			formattedPrice: '$997',
			image: 'https://cdn.simplertrading.com/2024/01/29162759/SimplerCentral_JM.jpg',
			description: 'Learn to trade SPX options with precision and confidence',
			url: '#'
		},
		{
			id: 'br-3',
			name: 'Daily SPX Alerts Service',
			price: 297,
			formattedPrice: '$297/mo',
			image: 'https://cdn.simplertrading.com/2024/01/29162759/SimplerCentral_JM.jpg',
			description: 'Real-time SPX trade alerts directly to your device',
			url: '#'
		}
	];

	const traderPills = [
		{ label: 'Overview', path: '/dashboard/spx-profit-pulse/billy-ribeiro', active: true },
		{
			label: 'Trading Strategies',
			path: '/dashboard/spx-profit-pulse/billy-ribeiro/strategies',
			active: false
		},
		{
			label: 'Trader Store',
			path: '/dashboard/spx-profit-pulse/billy-ribeiro/store',
			active: false
		}
	];
</script>

<svelte:head>
	<title>{trader.name} | SPX Profit Pulse | Revolution Trading Pros</title>
	<meta name="description" content="Meet {trader.name}, SPX Trading Expert. {trader.quote}" />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<TradingRoomHeader
	roomName="SPX Profit Pulse"
	startHereUrl="/dashboard/spx-profit-pulse/start-here"
/>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<!-- Trader Header Section -->
			<div class="trader-header">
				<div class="trader-header__wrap">
					<div class="trader-header__content">
						<div class="trader-header__columns">
							<!-- Left Column - Photo -->
							<div class="trader-header__col trader-header__col--photo">
								<div class="trader-photo">
									<div class="trader-photo__frame">
										<img class="trader-photo__img" src={trader.image} alt={trader.name} />
									</div>
								</div>
							</div>
							<!-- Right Column - Info -->
							<div class="trader-header__col trader-header__col--info">
								<div class="trader-info">
									<h2 class="trader-info__name">{trader.name}</h2>
									<h3 class="trader-info__title">{trader.title}</h3>
									<div class="trader-info__separator"></div>
									<div class="trader-info__quote">
										<p><em>{trader.quote}</em></p>
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- Navigation Pills -->
			<nav class="trader-pills">
				{#each traderPills as pill}
					<a href={pill.path} class="trader-pill {pill.active ? 'trader-pill--active' : ''}">
						{pill.label}
					</a>
				{/each}
			</nav>

			<!-- Products Section -->
			<div class="trader-products">
				<h3 class="trader-products__title">Products from {trader.name}</h3>
				<div class="trader-products__grid">
					{#each products as product}
						<div class="product-card">
							<figure class="product-card__image">
								<img src={product.image} alt={product.name} />
							</figure>
							<div class="product-card__body">
								<h4 class="product-card__name">{product.name}</h4>
								<p class="product-card__description">{product.description}</p>
								<div class="product-card__price">{product.formattedPrice}</div>
								<a href={product.url} class="product-card__btn">Learn More</a>
							</div>
						</div>
					{/each}
				</div>
			</div>
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   BILLY RIBEIRO TRADER PAGE - SPX Profit Pulse
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		display: block;
		width: 100%;
	}

	.dashboard__content-main {
		width: 100%;
		max-width: 100%;
	}

	.dashboard__content-section {
		padding: 30px 20px;
	}

	@media screen and (min-width: 768px) {
		.dashboard__content-section {
			padding: 40px 30px;
		}
	}

	/* Trader Header */
	.trader-header {
		background: linear-gradient(135deg, #143e59 0%, #0c2638 100%);
		border-radius: 12px;
		padding: 30px;
		margin-bottom: 30px;
	}

	.trader-header__columns {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 30px;
	}

	@media screen and (min-width: 768px) {
		.trader-header__columns {
			flex-direction: row;
			align-items: flex-end;
		}
	}

	.trader-header__col--photo {
		flex-shrink: 0;
	}

	.trader-header__col--info {
		flex: 1;
	}

	/* Trader Photo */
	.trader-photo__frame {
		width: 180px;
		height: 180px;
		border-radius: 50%;
		overflow: hidden;
		border: 4px solid rgba(255, 255, 255, 0.3);
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
	}

	.trader-photo__img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	/* Trader Info */
	.trader-info {
		color: #fff;
		text-align: center;
	}

	@media screen and (min-width: 768px) {
		.trader-info {
			text-align: left;
		}
	}

	.trader-info__name {
		font-size: 2rem;
		font-weight: 700;
		margin: 0 0 8px 0;
	}

	.trader-info__title {
		font-size: 1.125rem;
		font-weight: 400;
		margin: 0 0 20px 0;
		opacity: 0.9;
	}

	.trader-info__separator {
		width: 60px;
		height: 3px;
		background: linear-gradient(90deg, #ffb834 0%, #c68000 100%);
		margin: 0 auto 20px auto;
	}

	@media screen and (min-width: 768px) {
		.trader-info__separator {
			margin: 0 0 20px 0;
		}
	}

	.trader-info__quote {
		font-size: 1rem;
		line-height: 1.6;
		opacity: 0.9;
	}

	.trader-info__quote p {
		margin: 0;
	}

	/* Navigation Pills */
	.trader-pills {
		display: flex;
		flex-wrap: wrap;
		gap: 10px;
		margin-bottom: 30px;
	}

	.trader-pill {
		padding: 10px 20px;
		background: #f5f5f5;
		border: 2px solid #ddd;
		border-radius: 25px;
		color: #333;
		text-decoration: none;
		font-size: 14px;
		font-weight: 500;
		transition: all 0.2s ease;
	}

	.trader-pill:hover {
		background: #e5e5e5;
		border-color: #ccc;
	}

	.trader-pill--active {
		background: #143e59;
		border-color: #143e59;
		color: #fff;
	}

	/* Products Section */
	.trader-products__title {
		font-size: 1.5rem;
		font-weight: 700;
		margin: 0 0 20px 0;
		color: #1a1a1a;
	}

	.trader-products__grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 20px;
	}

	@media screen and (min-width: 576px) {
		.trader-products__grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media screen and (min-width: 992px) {
		.trader-products__grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	/* Product Card */
	.product-card {
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		transition:
			box-shadow 0.3s ease,
			transform 0.2s ease;
	}

	.product-card:hover {
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	.product-card__image {
		margin: 0;
		position: relative;
		padding-bottom: 56.25%;
		overflow: hidden;
	}

	.product-card__image img {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.product-card__body {
		padding: 20px;
	}

	.product-card__name {
		font-size: 1rem;
		font-weight: 600;
		margin: 0 0 10px 0;
		color: #1a1a1a;
	}

	.product-card__description {
		font-size: 0.875rem;
		color: #666;
		margin: 0 0 15px 0;
		line-height: 1.5;
	}

	.product-card__price {
		font-size: 1.25rem;
		font-weight: 700;
		color: #143e59;
		margin-bottom: 15px;
	}

	.product-card__btn {
		display: inline-block;
		padding: 10px 24px;
		background: linear-gradient(180deg, #ffb834 0%, #c68000 100%);
		color: #fff;
		text-decoration: none;
		border-radius: 25px;
		font-size: 14px;
		font-weight: 600;
		transition:
			transform 0.2s ease,
			box-shadow 0.2s ease;
	}

	.product-card__btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(198, 128, 0, 0.4);
	}

	/* CRITICAL: Hide sidebar on SPX Profit Pulse pages */
	:global(.dashboard__content-sidebar) {
		display: none !important;
	}

	/* Ensure main content takes full width without sidebar */
	.dashboard__content {
		display: block !important;
		width: 100% !important;
	}

	.dashboard__content-main {
		width: 100% !important;
		max-width: 100% !important;
		flex: none !important;
	}
</style>

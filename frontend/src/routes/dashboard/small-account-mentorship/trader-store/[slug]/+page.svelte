<!--
	URL: /dashboard/day-trading-room/trader-store/[slug]

	Individual Trader Store Page - WordPress Pixel-Perfect Implementation
	═══════════════════════════════════════════════════════════════════════════
	Matches WordPress trader-store-john-carter implementation exactly.

	@version 2.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { page } from '$app/state';

	interface Product {
		id: string;
		name: string;
		price: number;
		formattedPrice: string;
		image: string;
		description: string;
		url: string;
		traders: string[];
		markets: string;
		category: string;
		skillLevel: string;
	}

	interface TraderStore {
		id: string;
		name: string;
		slug: string;
		title: string;
		image: string;
		quote: string;
		products: Product[];
	}

	const allProducts: Product[] = [
		{
			id: '2145107',
			name: 'The New Squeeze Pro System',
			price: 2197,
			formattedPrice: '$2,197',
			image:
				'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/08/22154121/Squeeze-Pro-Trailer-Cardxx-300x169.jpg',
			description: '',
			url: '#',
			traders: ['John Carter'],
			markets: 'Options',
			category: 'Courses',
			skillLevel: 'Intermediate'
		},
		{
			id: '2132341',
			name: 'Monster Momo Method Basic',
			price: 997,
			formattedPrice: '$997',
			image:
				'https://cdn.simplertrading.com/2025/05/27173247/Store-Card-176x112-JC-Monster-Momo-Method-300x191.jpg',
			description:
				"Discover John's NEW Tools for Targeting Big breakouts and Sudden Reversals with Clarity and Confidence",
			url: '#',
			traders: ['John Carter'],
			markets: 'Options',
			category: 'Courses',
			skillLevel: 'Intermediate'
		},
		{
			id: '2110223',
			name: 'Mastering the Trade University – All 12 Courses',
			price: 997,
			formattedPrice: '$997',
			image:
				'https://cdn.simplertrading.com/2025/05/12141240/Store-Card-JC-MasteringTheTrade-University.jpg',
			description: 'Attend 12 immersive sessions with the Simpler Trading Team',
			url: '#',
			traders: ['Danielle Shay', 'Henry Gambell', 'John Carter', 'Sam Shames', 'Taylor Horton'],
			markets: 'Options',
			category: 'Courses,Live',
			skillLevel: 'Intermediate'
		},
		{
			id: '2008204',
			name: 'Sandbox Elite Package',
			price: 997,
			formattedPrice: '$997',
			image:
				'https://cdn.simplertrading.com/2024/04/01103716/Store-Card-Sandbox-Strategy-Class-300x191.jpg',
			description: "Discover How to Exploit Wall Street's Biggest Blind Spot",
			url: '#',
			traders: ['John Carter'],
			markets: 'Options',
			category: 'Courses',
			skillLevel: 'Intermediate'
		},
		{
			id: '1836903',
			name: 'New Micro Voodoo Line Strategy Basic Bundle',
			price: 797,
			formattedPrice: '$797',
			image:
				'https://cdn.simplertrading.com/2023/03/21162031/Micro-voodoo-lines-video-bg-300x169.jpg',
			description: 'How to Automatically Predict Tradeable Levels in a Violently Choppy Market',
			url: '#',
			traders: ['David Starr', 'Henry Gambell', 'John Carter'],
			markets: 'Options',
			category: 'Courses',
			skillLevel: ''
		},
		{
			id: '1720212',
			name: "The 'Quick Hits' Strategy Basic",
			price: 797,
			formattedPrice: '$797',
			image:
				'https://cdn.simplertrading.com/2022/08/07092727/Quick-Hits-Strategy-Video-bg-300x169.jpg',
			description: "Get John's Simple New Formula for Consistent Daily Income",
			url: '#',
			traders: ['John Carter', 'Kody Ashmore'],
			markets: 'Options',
			category: 'Courses',
			skillLevel: 'Intermediate'
		},
		{
			id: 'hg-1',
			name: 'Options Technical Analysis Mastery',
			price: 697,
			formattedPrice: '$697',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'Master technical analysis for high-probability options trades',
			url: '#',
			traders: ['Henry Gambell'],
			markets: 'Options',
			category: 'Courses',
			skillLevel: 'Intermediate'
		},
		{
			id: 'th-1',
			name: 'Day Trading Momentum Strategies',
			price: 597,
			formattedPrice: '$597',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'Learn to capture momentum moves in real-time',
			url: '#',
			traders: ['Taylor Horton'],
			markets: 'Options',
			category: 'Courses',
			skillLevel: 'Intermediate'
		},
		{
			id: 'bm-1',
			name: 'Income Generation with Options',
			price: 497,
			formattedPrice: '$497',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'Build consistent income with defined-risk options strategies',
			url: '#',
			traders: ['Bruce Marshall'],
			markets: 'Options',
			category: 'Courses',
			skillLevel: 'Intermediate'
		},
		{
			id: 'ds-1',
			name: 'Momentum Options Trading',
			price: 697,
			formattedPrice: '$697',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'Capture explosive moves with momentum-based options strategies',
			url: '#',
			traders: ['Danielle Shay'],
			markets: 'Options',
			category: 'Courses',
			skillLevel: 'Intermediate'
		},
		{
			id: 'ao-1',
			name: 'Small Account Trading Strategies',
			price: 397,
			formattedPrice: '$397',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'Grow your small account with proper risk management',
			url: '#',
			traders: ['Allison Ostrander'],
			markets: 'Options',
			category: 'Courses',
			skillLevel: 'Beginner'
		},
		{
			id: 'ss-1',
			name: 'Futures Trading Essentials',
			price: 597,
			formattedPrice: '$597',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'Master the fundamentals of futures trading',
			url: '#',
			traders: ['Sam Shames'],
			markets: 'Futures',
			category: 'Courses',
			skillLevel: 'Intermediate'
		},
		{
			id: 'ka-1',
			name: 'ES/NQ Day Trading System',
			price: 797,
			formattedPrice: '$797',
			image: 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg',
			description: 'Complete system for day trading ES and NQ futures',
			url: '#',
			traders: ['Kody Ashmore'],
			markets: 'Futures',
			category: 'Courses',
			skillLevel: 'Intermediate'
		},
		{
			id: 'rh-1',
			name: 'Forex Technical Analysis',
			price: 697,
			formattedPrice: '$697',
			image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'Professional forex trading with technical analysis',
			url: '#',
			traders: ['Raghee Horner'],
			markets: 'Forex',
			category: 'Courses',
			skillLevel: 'Intermediate'
		}
	];

	const traderData: {
		[key: string]: { name: string; title: string; image: string; quote: string };
	} = {
		'john-carter': {
			name: 'John Carter',
			title: 'Founder of Simpler Trading®',
			image: 'https://cdn.simplertrading.com/2021/10/08113910/john-headshot-2020.png',
			quote:
				'"I think there are a lot of parallels between being a good trader and living a good life. The market truly is the ultimate psychologist."'
		},
		'henry-gambell': {
			name: 'Henry Gambell',
			title: 'Director of Options',
			image: 'https://cdn.simplertrading.com/2021/10/08114059/henry-headshot-2020.png',
			quote:
				'"Every trade is an opportunity to learn something new about yourself and the markets."'
		},
		'taylor-horton': {
			name: 'Taylor Horton',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2021/10/08114206/taylor-headshot-2020.png',
			quote:
				'"Success in trading comes from consistency and discipline, not from hitting home runs."'
		},
		'bruce-marshall': {
			name: 'Bruce Marshall',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2021/10/08113753/bruce-headshot-2020.png',
			quote: '"Options give you leverage, but risk management keeps you in the game."'
		},
		'danielle-shay': {
			name: 'Danielle Shay',
			title: 'Director of Options',
			image: 'https://cdn.simplertrading.com/2021/10/08113828/danielle-headshot-2020.png',
			quote:
				'"The best trades are the ones where you manage your risk before thinking about reward."'
		},
		'allison-ostrander': {
			name: 'Allison Ostrander',
			title: 'Director of Risk Tolerance',
			image: 'https://cdn.simplertrading.com/2021/10/08113700/allison-headshot-2020.png',
			quote: '"Small accounts can grow into big accounts with the right mindset and strategy."'
		},
		'sam-shames': {
			name: 'Sam Shames',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2021/10/08114128/sam-headshot-2020.png',
			quote: '"The futures market rewards those who are prepared and punishes those who are not."'
		},
		'kody-ashmore': {
			name: 'Kody Ashmore',
			title: 'Senior Futures Trader',
			image: 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg',
			quote: '"Every day in the futures market is a new opportunity to execute your edge."'
		},
		'raghee-horner': {
			name: 'Raghee Horner',
			title: 'Senior Trader',
			image: 'https://cdn.simplertrading.com/2021/10/08114038/raghee-headshot-2020.png',
			quote: '"The trend is your friend, but only if you know how to find it."'
		}
	};

	const traderPills = [
		{ label: 'Overview', path: '' },
		{ label: 'Chart Setups', path: '/chart-setups' },
		{ label: 'Trading Plan', path: '/trading-plan' },
		{ label: 'Trading Strategies', path: '/trading-strategies' },
		{ label: 'Trader Store', path: '/trader-store' }
	];

	let store = $state<TraderStore | null>(null);
	let loading = $state(true);
	let currentSlug = $state('');

	function getTraderProducts(traderName: string): Product[] {
		return allProducts.filter((p) => p.traders.includes(traderName));
	}

	// Svelte 5: Use $effect for reactive slug changes
	$effect(() => {
		const slug = page.params.slug;
		if (slug !== currentSlug) {
			currentSlug = slug ?? '';
			const trader = traderData[slug ?? ''];

			if (trader) {
				store = {
					id: slug ?? '',
					name: trader.name,
					slug: slug ?? '',
					title: trader.title,
					image: trader.image,
					quote: trader.quote,
					products: getTraderProducts(trader.name)
				};
			} else {
				store = null;
			}
			loading = false;
		}
	});
</script>

<svelte:head>
	{#if store}
		<title>{store.name}'s Trader Store | Revolution Trading Pros</title>
		<meta
			name="description"
			content="Browse courses and indicators from {store.name}. {store.quote}"
		/>
	{:else}
		<title>Store Not Found | Revolution Trading Pros</title>
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
			{:else if store}
				<!-- Trader Header Section -->
				<div class="trader-header trader-header--full-width trader-header--bg-color">
					<div class="trader-header__wrap">
						<div class="trader-header__content trader-header--fixed-width">
							<div
								class="trader-header__columns trader-header__columns--equal-height trader-header__columns--align-bottom"
							>
								<!-- Left Column - Photo -->
								<div class="trader-header__col trader-header__col--small">
									<div class="trader-header__col-content">
										<div class="trader-photo trader-photo--desktop">
											<div class="trader-photo__wrapper">
												<div class="trader-photo__frame trader-photo__frame--center">
													<div class="trader-photo__content">
														<img
															class="trader-photo__img"
															src={store.image}
															alt={store.name}
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
								<div class="trader-header__col">
									<div class="trader-header__col-content">
										<div class="trader-info__block">
											<h2 class="trader-info__heading">
												<span class="trader-info__name">{store.name}</span>
											</h2>
										</div>
										<div class="trader-info__block">
											<h2 class="trader-info__heading">
												<span class="trader-info__title">{store.title}</span>
											</h2>
										</div>
										<div class="trader-info__block trader-info__block--separator">
											<div class="trader-info__separator"></div>
										</div>
										<div class="trader-info__block">
											<div class="trader-info__quote">
												<p><em>{store.quote}</em></p>
											</div>
										</div>
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>

				<!-- Trader Pills Navigation -->
				<div class="trader-nav trader-nav--fixed-width trader-nav--bg-none">
					<div class="trader-nav__wrap">
						<div class="trader-nav__content trader-nav--fixed-width">
							<div class="trader-nav__columns">
								<div class="trader-nav__col">
									<div class="trader-nav__col-content">
										<div class="trader-nav__module">
											<div class="trader-nav__module-content">
												<div class="trader_pills">
													{#each traderPills as pill}
														<div class="trader_pill">
															<a
																href="/dashboard/day-trading-room/meet-the-traders/{store.slug}{pill.path ===
																'/trader-store'
																	? ''
																	: pill.path}"
																class:active={pill.path === '/trader-store'}
																onclick={(e) => {
																	e.preventDefault();
																	if (pill.path === '') {
																		window.location.href = `/dashboard/day-trading-room/meet-the-traders/${store?.slug}`;
																	} else if (pill.path !== '/trader-store') {
																		window.location.href = `/dashboard/day-trading-room/meet-the-traders/${store?.slug}${pill.path}`;
																	}
																}}
															>
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
					</div>
				</div>

				<!-- Product Listing -->
				<div class="products-container">
					<h1 class="u--font-size-h1 tr_title">{store.name.split(' ')[0]}'s Classes</h1>
					<div class="products u--margin-bottom-20" data-products-view="grid">
						<div class="flex-grid">
							{#each store.products as product (product.id)}
								<article
									class="product flex-grid-item store-product"
									data-traders={product.traders.join(',')}
									data-markets={product.markets}
									data-category={product.category}
									data-skill_level={product.skillLevel}
									data-price={product.price}
									data-id={product.id}
									data-name={product.name}
								>
									<div class="card">
										<figure class="card-media">
											<a
												href={product.url}
												class="card-image"
												style="background-image: url({product.image});"
											>
												<img src="https://placehold.it/540x256" alt={product.name} />
											</a>
										</figure>
										<section class="card-body">
											<h4 class="card-title">
												<a href={product.url}>
													{product.name}
												</a>
											</h4>
											<div class="card-description">
												{product.description}
											</div>
										</section>
										<footer class="card-footer">
											<div class="product-add-to-cart">
												<a
													href={product.url}
													class="btn btn-xs btn-orange"
													title="View {product.name}">Learn&nbsp;More</a
												>
											</div>
											<div class="product-price">
												<span class="woocommerce-Price-amount amount">
													<bdi
														><span class="woocommerce-Price-currencySymbol">$</span
														>{product.price.toLocaleString()}</bdi
													>
												</span>
											</div>
										</footer>
									</div>
								</article>
							{/each}
						</div>
					</div>
				</div>
			{:else}
				<div class="not-found">
					<h1>Store Not Found</h1>
					<p>The trader store you're looking for could not be found.</p>
					<a href="/dashboard/day-trading-room/trader-store" class="btn btn-default">
						← Back to All Stores
					</a>
				</div>
			{/if}
		</section>
	</div>
</div>

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
		border-top-color: #143e59;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Trader Header Section */
	.trader-header {
		margin: 0;
		padding: 0;
		width: 100%;
	}

	.trader-header--full-width {
		width: 100%;
	}

	.trader-header--bg-color {
		background-color: #e5e9f4;
	}

	.trader-nav--bg-none {
		background: #fff;
	}

	.trader-header__wrap,
	.trader-nav__wrap {
		max-width: 1200px;
		margin: 0 auto;
		padding: 40px 20px;
	}

	.trader-header__content,
	.trader-nav__content {
		width: 100%;
	}

	.trader-header--fixed-width,
	.trader-nav--fixed-width {
		max-width: 1100px;
		margin: 0 auto;
	}

	.trader-nav__wrap {
		padding: 30px 20px;
	}

	/* Trader Header Columns */
	.trader-header__columns,
	.trader-nav__columns {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.trader-header__columns--equal-height {
		align-items: stretch;
	}

	.trader-header__columns--align-bottom {
		align-items: flex-end;
	}

	/* Trader Header Cols */
	.trader-header__col,
	.trader-nav__col {
		flex: 1;
		padding: 0 15px;
		min-width: 0;
	}

	.trader-header__col--small {
		flex: 0 0 300px;
		max-width: 300px;
	}

	.trader-header__col-content,
	.trader-nav__col-content {
		height: 100%;
	}

	/* Trader Info Blocks */
	.trader-info__block {
		margin-bottom: 20px;
	}

	.trader-info__block:last-child {
		margin-bottom: 0;
	}

	/* Trader Photo */
	.trader-photo {
		margin-bottom: 0;
	}

	.trader-photo__frame {
		display: block;
	}

	.trader-photo__frame--center {
		text-align: center;
	}

	.trader-photo__content {
		display: inline-block;
		line-height: 0;
	}

	.trader-photo__img {
		max-width: 100%;
		height: auto;
		width: 250px;
		border-radius: 50%;
	}

	/* Trader Info Headings */
	.trader-info__heading {
		margin: 0;
		padding: 0;
		line-height: 1.2;
	}

	.trader-info__name {
		display: block;
		font-size: 36px;
		font-weight: 700;
		color: #333;
	}

	.trader-info__title {
		display: block;
		font-size: 18px;
		font-weight: 400;
		color: #666;
		margin-top: 5px;
	}

	/* Trader Info Separator */
	.trader-info__block--separator {
		margin: 20px 0;
	}

	.trader-info__separator {
		border-top: 2px solid #f69532;
		width: 60px;
	}

	/* Trader Info Quote */
	.trader-info__quote {
		font-size: 16px;
		line-height: 1.7;
		color: #444;
	}

	.trader-info__quote p {
		margin: 0 0 15px;
	}

	.trader-info__quote p:last-child {
		margin-bottom: 0;
	}

	.trader-info__quote em {
		font-style: italic;
		color: #555;
	}

	/* Trader Pills */
	.trader_pills {
		display: flex;
		flex-wrap: wrap;
		gap: 10px;
		margin-bottom: 0;
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
		background: #143e59;
		color: #fff;
		border-color: #143e59;
	}

	.trader_pill a.active {
		background: #143e59;
		color: #fff;
		border-color: #143e59;
	}

	/* Products Container */
	.products-container {
		max-width: 1100px;
		margin: 0 auto;
		padding: 30px 20px;
	}

	.u--font-size-h1 {
		font-size: 28px;
	}

	.tr_title {
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
	}

	.products {
		margin-bottom: 20px;
	}

	.flex-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 25px;
	}

	/* Product Card - WordPress Style */
	.product {
		display: block;
	}

	.store-product .card {
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 8px;
		overflow: hidden;
		display: flex;
		flex-direction: column;
		height: 100%;
		transition: box-shadow 0.3s ease;
	}

	.store-product .card:hover {
		box-shadow: 0 5px 20px rgba(0, 0, 0, 0.1);
	}

	.card-media {
		margin: 0;
		padding: 0;
		position: relative;
		height: 160px;
		overflow: hidden;
	}

	.card-image {
		display: block;
		width: 100%;
		height: 100%;
		background-size: cover;
		background-position: center;
		background-repeat: no-repeat;
	}

	.card-image img {
		opacity: 0;
		width: 100%;
		height: 100%;
	}

	.card-body {
		padding: 15px;
		flex: 1;
		display: flex;
		flex-direction: column;
	}

	.card-title {
		font-size: 16px;
		font-weight: 700;
		margin: 0 0 10px;
		line-height: 1.3;
	}

	.card-title a {
		color: #333;
		text-decoration: none;
	}

	.card-title a:hover {
		color: #143e59;
	}

	.card-description {
		font-size: 13px;
		color: #666;
		line-height: 1.5;
		flex: 1;
	}

	.card-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 15px;
		border-top: 1px solid #e6e6e6;
		background: #fafafa;
	}

	.product-add-to-cart {
		flex-shrink: 0;
	}

	.product-price {
		text-align: right;
	}

	.woocommerce-Price-amount {
		font-size: 18px;
		font-weight: 700;
		color: #333;
	}

	.woocommerce-Price-currencySymbol {
		font-size: 14px;
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

	.btn-xs {
		padding: 8px 16px;
		font-size: 12px;
	}

	.btn-orange {
		background: #f69532;
		color: #fff;
		border: 1px solid #f69532;
	}

	.btn-orange:hover {
		background: #dc7309;
		border-color: #dc7309;
	}

	.btn-default {
		background: #143e59;
		color: #fff;
		border: 1px solid #143e59;
	}

	.btn-default:hover {
		background: #0c2638;
		border-color: #0c2638;
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

	/* Mobile-first: stacked layout by default */
	.trader-header__columns,
	.trader-nav__columns {
		flex-direction: column;
	}

	.trader-header__col--small {
		flex: 0 0 100%;
		max-width: 100%;
		text-align: center;
		margin-bottom: 20px;
	}

	.trader-header__col,
	.trader-nav__col {
		flex: 0 0 100%;
		max-width: 100%;
	}

	.flex-grid {
		grid-template-columns: 1fr;
	}

	@media (min-width: 768px) {
		.trader-header__columns,
		.trader-nav__columns {
			flex-direction: row;
		}

		.trader-header__col--small {
			flex: 0 0 200px;
			max-width: 200px;
			text-align: left;
			margin-bottom: 0;
		}

		.flex-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (min-width: 992px) {
		.trader-header__col--small {
			flex: 0 0 300px;
			max-width: 300px;
		}

		.trader-photo__img {
			width: 220px;
		}

		.trader-info__name {
			font-size: 36px;
		}

		.flex-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 640px) {
		.trader-photo__img {
			width: 150px;
		}

		.trader-info__name {
			font-size: 24px;
			text-align: center;
		}

		.trader-info__title {
			text-align: center;
		}

		.trader-info__separator {
			margin: 15px auto;
		}

		.trader-info__quote {
			text-align: center;
		}

		.trader_pills {
			justify-content: center;
		}

		.flex-grid {
			grid-template-columns: 1fr;
		}

		.card-footer {
			flex-direction: column;
			gap: 10px;
		}

		.product-price {
			text-align: center;
		}
	}
</style>

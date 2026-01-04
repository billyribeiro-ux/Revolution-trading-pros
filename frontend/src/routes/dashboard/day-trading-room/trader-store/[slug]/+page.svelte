<!--
	URL: /dashboard/day-trading-room/trader-store/[slug]

	Individual Trader Store Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation

	Individual trader store with products matching WordPress implementation.

	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';

	interface Product {
		id: string;
		name: string;
		price: string;
		originalPrice?: string;
		image: string;
		description: string;
		type: 'course' | 'indicator' | 'bundle';
	}

	interface TraderStore {
		id: string;
		name: string;
		slug: string;
		image: string;
		title: string;
		products: Product[];
	}

	const traderStores: TraderStore[] = [
		{
			id: 'john-carter',
			name: 'John Carter',
			slug: 'john-carter',
			image: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
			title: 'Founder & CEO',
			products: [
				{ id: 'jc-1', name: 'Squeeze Pro Indicator', price: '$597', image: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg', description: 'The ultimate momentum indicator for identifying explosive moves.', type: 'indicator' },
				{ id: 'jc-2', name: 'Market Cycles Mastery', price: '$997', image: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg', description: 'Learn to predict market turns using cycle analysis.', type: 'course' },
				{ id: 'jc-3', name: 'Options Trading Bundle', price: '$1,497', originalPrice: '$2,000', image: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg', description: 'Complete options trading education package.', type: 'bundle' }
			]
		},
		{
			id: 'henry-gambell',
			name: 'Henry Gambell',
			slug: 'henry-gambell',
			image: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			title: 'Director of Options',
			products: [
				{ id: 'hg-1', name: 'Technical Analysis Mastery', price: '$497', image: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg', description: 'Master technical analysis for consistent profits.', type: 'course' },
				{ id: 'hg-2', name: 'Options Risk Management', price: '$397', image: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg', description: 'Learn to manage risk like a professional.', type: 'course' }
			]
		},
		{
			id: 'kody-ashmore',
			name: 'Kody Ashmore',
			slug: 'kody-ashmore',
			image: 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg',
			title: 'Senior Futures Trader',
			products: [
				{ id: 'ka-1', name: 'Futures Day Trading Course', price: '$697', image: 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg', description: 'Complete guide to day trading ES and NQ futures.', type: 'course' },
				{ id: 'ka-2', name: 'Daily Setups Indicator Pack', price: '$297', image: 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg', description: 'Custom indicators for daily trading setups.', type: 'indicator' }
			]
		}
	];

	// Default products for traders without specific products
	const defaultProducts: Product[] = [
		{ id: 'default-1', name: 'Trading Fundamentals Course', price: '$297', image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg', description: 'Essential trading concepts and strategies.', type: 'course' },
		{ id: 'default-2', name: 'Premium Indicator Pack', price: '$197', image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg', description: 'Custom trading indicators for your platform.', type: 'indicator' }
	];

	let store: TraderStore | null = null;
	let loading = true;

	onMount(() => {
		const slug = $page.params.slug;
		store = traderStores.find(s => s.slug === slug) || null;
		
		// Create a default store if trader exists but no specific store
		if (!store && ['taylor-horton', 'bruce-marshall', 'danielle-shay', 'allison-ostrander', 'sam-shames', 'raghee-horner'].includes(slug)) {
			const traderNames: { [key: string]: { name: string; title: string } } = {
				'taylor-horton': { name: 'Taylor Horton', title: 'Senior Trader' },
				'bruce-marshall': { name: 'Bruce Marshall', title: 'Senior Trader' },
				'danielle-shay': { name: 'Danielle Shay', title: 'Director of Options' },
				'allison-ostrander': { name: 'Allison Ostrander', title: 'Director of Risk Tolerance' },
				'sam-shames': { name: 'Sam Shames', title: 'Senior Trader' },
				'raghee-horner': { name: 'Raghee Horner', title: 'Senior Trader' }
			};
			
			store = {
				id: slug,
				name: traderNames[slug].name,
				slug: slug,
				image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
				title: traderNames[slug].title,
				products: defaultProducts
			};
		}
		
		loading = false;
	});

	function getTypeLabel(type: string): string {
		const labels: { [key: string]: string } = {
			course: 'Course',
			indicator: 'Indicator',
			bundle: 'Bundle'
		};
		return labels[type] || type;
	}
</script>

<svelte:head>
	{#if store}
		<title>{store.name}'s Store | Trader Store | Revolution Trading Pros</title>
		<meta name="description" content="Browse courses, indicators, and tools from {store.name}." />
	{:else}
		<title>Store Not Found | Revolution Trading Pros</title>
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
		<li><a href="/dashboard/day-trading-room/trader-store">Trader Store</a></li>
		<li class="separator">/</li>
		{#if store}
			<li class="current">{store.name}</li>
		{:else}
			<li class="current">Not Found</li>
		{/if}
	</ul>
</nav>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		{#if loading}
			<div class="loading">Loading...</div>
		{:else if store}
			<!-- Store Header -->
			<div class="store-header">
				<figure class="store-header__image">
					<img src={store.image} alt={store.name} />
				</figure>
				<div class="store-header__info">
					<h1 class="store-header__name">{store.name}'s Store</h1>
					<p class="store-header__title">{store.title}</p>
					<p class="store-header__count">{store.products.length} Products Available</p>
				</div>
			</div>

			<!-- Products Grid -->
			<div class="products-section">
				<h2 class="section-title">Available Products</h2>
				
				<div class="products-grid">
					{#each store.products as product (product.id)}
						<article class="product-card">
							<figure class="product-card__image">
								<img src={product.image} alt={product.name} loading="lazy" />
								<span class="product-type">{getTypeLabel(product.type)}</span>
							</figure>
							
							<div class="product-card__content">
								<h3 class="product-card__name">{product.name}</h3>
								<p class="product-card__description">{product.description}</p>
								
								<div class="product-card__pricing">
									<span class="price">{product.price}</span>
									{#if product.originalPrice}
										<span class="original-price">{product.originalPrice}</span>
									{/if}
								</div>
								
								<button type="button" class="btn btn-primary">
									Learn More
								</button>
							</div>
						</article>
					{/each}
				</div>
			</div>

			<div class="back-link">
				<a href="/dashboard/day-trading-room/trader-store" class="btn btn-secondary">
					← Back to All Stores
				</a>
				<a href="/dashboard/day-trading-room/meet-the-traders/{store.slug}" class="btn btn-outline">
					View {store.name}'s Profile
				</a>
			</div>
		{:else}
			<div class="not-found">
				<h1>Store Not Found</h1>
				<p>The trader store you're looking for could not be found.</p>
				<a href="/dashboard/day-trading-room/trader-store" class="btn btn-primary">
					← Back to All Stores
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

	/* Store Header */
	.store-header {
		display: flex;
		gap: 25px;
		padding: 25px;
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 8px;
		margin-bottom: 30px;
		align-items: center;
	}

	.store-header__image {
		width: 100px;
		height: 100px;
		margin: 0;
		border-radius: 50%;
		overflow: hidden;
		flex-shrink: 0;
	}

	.store-header__image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.store-header__info {
		flex: 1;
	}

	.store-header__name {
		font-size: 28px;
		font-weight: 700;
		color: #333;
		margin: 0 0 5px;
	}

	.store-header__title {
		font-size: 16px;
		color: #0984ae;
		font-weight: 600;
		margin: 0 0 10px;
	}

	.store-header__count {
		font-size: 14px;
		color: #666;
		margin: 0;
	}

	/* Products Section */
	.products-section {
		margin-bottom: 30px;
	}

	.section-title {
		font-size: 22px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		padding-bottom: 15px;
		border-bottom: 2px solid #e6e6e6;
	}

	.products-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 25px;
	}

	.product-card {
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 8px;
		overflow: hidden;
		transition: box-shadow 0.3s ease;
	}

	.product-card:hover {
		box-shadow: 0 5px 20px rgba(0, 0, 0, 0.1);
	}

	.product-card__image {
		width: 100%;
		height: 180px;
		margin: 0;
		overflow: hidden;
		position: relative;
	}

	.product-card__image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.product-type {
		position: absolute;
		top: 10px;
		left: 10px;
		padding: 4px 10px;
		background: #143E59;
		color: #fff;
		font-size: 11px;
		font-weight: 600;
		border-radius: 3px;
		text-transform: uppercase;
	}

	.product-card__content {
		padding: 20px;
	}

	.product-card__name {
		font-size: 18px;
		font-weight: 700;
		color: #333;
		margin: 0 0 10px;
	}

	.product-card__description {
		font-size: 14px;
		color: #666;
		line-height: 1.5;
		margin: 0 0 15px;
	}

	.product-card__pricing {
		display: flex;
		align-items: center;
		gap: 10px;
		margin-bottom: 15px;
	}

	.price {
		font-size: 22px;
		font-weight: 700;
		color: #F69532;
	}

	.original-price {
		font-size: 16px;
		color: #999;
		text-decoration: line-through;
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
		border: none;
	}

	.btn-primary {
		background: #F69532;
		color: #fff;
		width: 100%;
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

	.btn-outline {
		background: transparent;
		color: #143E59;
		border: 1px solid #143E59;
	}

	.btn-outline:hover {
		background: #143E59;
		color: #fff;
	}

	/* Back Link */
	.back-link {
		display: flex;
		gap: 15px;
		padding: 25px;
		background: #f9f9f9;
		border-radius: 8px;
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
	@media (max-width: 1199px) {
		.products-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 767px) {
		.products-grid {
			grid-template-columns: 1fr;
		}

		.store-header {
			flex-direction: column;
			text-align: center;
		}

		.back-link {
			flex-direction: column;
		}
	}
</style>

<script lang="ts">
	/**
	 * Day Trading Room - Trader Store
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Products and courses from traders
	 *
	 * @version 1.0.0
	 */
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconBuildingStore from '@tabler/icons-svelte/icons/building-store';
	import IconShoppingCart from '@tabler/icons-svelte/icons/shopping-cart';
	import IconStar from '@tabler/icons-svelte/icons/star';

	// Mock product data
	const products = [
		{
			id: 1,
			name: 'Momentum Trading Masterclass',
			trader: 'HG',
			price: 497,
			originalPrice: 697,
			rating: 4.9,
			reviews: 128,
			image: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			description: 'Learn the complete momentum trading system used by HG in the Day Trading Room.',
			category: 'Course'
		},
		{
			id: 2,
			name: 'Options Basics Bundle',
			trader: 'DShay',
			price: 297,
			originalPrice: 497,
			rating: 4.8,
			reviews: 89,
			image: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
			description: 'Everything you need to get started with options trading.',
			category: 'Bundle'
		},
		{
			id: 3,
			name: 'Supply & Demand Indicator',
			trader: 'SS',
			price: 197,
			originalPrice: null,
			rating: 4.7,
			reviews: 156,
			image: 'https://cdn.simplertrading.com/2025/05/07134553/SimplerCentral_SS.jpg',
			description: 'Custom indicator that automatically identifies supply and demand zones.',
			category: 'Indicator'
		},
		{
			id: 4,
			name: 'Risk Management Guide',
			trader: 'HG',
			price: 97,
			originalPrice: 147,
			rating: 4.9,
			reviews: 203,
			image: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			description: 'Complete guide to managing risk and protecting your trading capital.',
			category: 'eBook'
		}
	];
</script>

<svelte:head>
	<title>Trader Store - Day Trading Room | Revolution Trading Pros</title>
</svelte:head>

<div class="trader-store">
	<a href="/dashboard/day-trading-room" class="back-link">
		<IconArrowLeft size={18} />
		Back to Day Trading Room
	</a>

	<header class="page-header">
		<div class="header-icon">
			<IconBuildingStore size={32} />
		</div>
		<div class="header-content">
			<h1 class="page-title">Trader Store</h1>
			<p class="page-subtitle">Courses, indicators, and resources from our expert traders</p>
		</div>
	</header>

	<div class="products-grid">
		{#each products as product}
			<article class="product-card">
				<div class="product-image" style="background-image: url({product.image})">
					<span class="product-category">{product.category}</span>
					{#if product.originalPrice}
						<span class="product-sale">Sale</span>
					{/if}
				</div>
				<div class="product-info">
					<p class="product-trader">by {product.trader}</p>
					<h3 class="product-name">{product.name}</h3>
					<p class="product-description">{product.description}</p>
					<div class="product-rating">
						<IconStar size={16} fill="#F69532" color="#F69532" />
						<span class="rating-value">{product.rating}</span>
						<span class="rating-count">({product.reviews} reviews)</span>
					</div>
					<div class="product-pricing">
						<span class="current-price">${product.price}</span>
						{#if product.originalPrice}
							<span class="original-price">${product.originalPrice}</span>
						{/if}
					</div>
					<button class="add-to-cart-btn">
						<IconShoppingCart size={18} />
						Add to Cart
					</button>
				</div>
			</article>
		{/each}
	</div>
</div>

<style>
	.trader-store {
		padding: 30px;
		max-width: 1400px;
		margin: 0 auto;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		color: #0984ae;
		text-decoration: none;
		font-size: 14px;
		font-weight: 500;
		margin-bottom: 24px;
	}

	.back-link:hover {
		text-decoration: underline;
	}

	.page-header {
		display: flex;
		align-items: center;
		gap: 20px;
		margin-bottom: 40px;
	}

	.header-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 64px;
		height: 64px;
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
		border-radius: 16px;
		color: #fff;
	}

	.page-title {
		font-size: 28px;
		font-weight: 700;
		color: #333;
		margin: 0 0 4px;
		font-family: 'Open Sans', sans-serif;
	}

	.page-subtitle {
		font-size: 14px;
		color: #666;
		margin: 0;
	}

	.products-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 24px;
	}

	.product-card {
		background: #fff;
		border-radius: 12px;
		overflow: hidden;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		transition: all 0.2s;
	}

	.product-card:hover {
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	.product-image {
		position: relative;
		width: 100%;
		height: 180px;
		background-size: cover;
		background-position: center;
		background-color: #0984ae;
	}

	.product-category {
		position: absolute;
		top: 12px;
		left: 12px;
		padding: 4px 10px;
		background: rgba(0, 0, 0, 0.7);
		color: #fff;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
	}

	.product-sale {
		position: absolute;
		top: 12px;
		right: 12px;
		padding: 4px 10px;
		background: #F69532;
		color: #fff;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 700;
		text-transform: uppercase;
	}

	.product-info {
		padding: 20px;
	}

	.product-trader {
		font-size: 12px;
		color: #0984ae;
		margin: 0 0 8px;
		font-weight: 600;
	}

	.product-name {
		font-size: 18px;
		font-weight: 700;
		color: #333;
		margin: 0 0 8px;
		line-height: 1.3;
		font-family: 'Open Sans', sans-serif;
	}

	.product-description {
		font-size: 14px;
		color: #666;
		line-height: 1.5;
		margin: 0 0 12px;
	}

	.product-rating {
		display: flex;
		align-items: center;
		gap: 6px;
		margin-bottom: 12px;
	}

	.rating-value {
		font-size: 14px;
		font-weight: 700;
		color: #333;
	}

	.rating-count {
		font-size: 13px;
		color: #999;
	}

	.product-pricing {
		display: flex;
		align-items: center;
		gap: 10px;
		margin-bottom: 16px;
	}

	.current-price {
		font-size: 24px;
		font-weight: 700;
		color: #333;
	}

	.original-price {
		font-size: 16px;
		color: #999;
		text-decoration: line-through;
	}

	.add-to-cart-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		width: 100%;
		padding: 12px 20px;
		background: #F69532;
		color: #fff;
		border: none;
		border-radius: 4px;
		font-size: 14px;
		font-weight: 700;
		cursor: pointer;
		transition: background 0.2s;
	}

	.add-to-cart-btn:hover {
		background: #dc7309;
	}

	@media (max-width: 768px) {
		.trader-store {
			padding: 20px;
		}

		.products-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

<!--
	Trader Store Sub-Page
	═══════════════════════════════════════════════════════════════════════════
	Trader-specific products and courses. Uses shared layout.
	
	@version 2.0.0
-->
<script lang="ts">
	import { page } from '$app/state';
	import { getTraderBySlug } from '$lib/data/traders';

	interface Product {
		id: string;
		name: string;
		description: string;
		price: string;
		originalPrice?: string;
		image: string;
		category: 'Course' | 'Indicator' | 'Bundle' | 'Membership';
		featured?: boolean;
	}

	let trader = $derived(getTraderBySlug(page.params.slug));

	// Trader-specific products - in production, fetch from API
	const products: Product[] = [
		{
			id: '1',
			name: 'Squeeze Pro Indicator',
			description:
				'The ultimate momentum indicator that identifies explosive moves before they happen. Includes lifetime access and updates.',
			price: '$497',
			originalPrice: '$697',
			image:
				'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/08/22154121/Squeeze-Pro-Trailer-Cardxx-300x169.jpg',
			category: 'Indicator',
			featured: true
		},
		{
			id: '2',
			name: 'Options Mastery Course',
			description:
				'Complete options trading education from beginner to advanced. Learn to trade options like a pro with step-by-step guidance.',
			price: '$997',
			image:
				'https://cdn.simplertrading.com/2022/08/07092727/Quick-Hits-Strategy-Video-bg-300x169.jpg',
			category: 'Course'
		},
		{
			id: '3',
			name: 'Ultimate Trading Bundle',
			description:
				'Get access to all courses, indicators, and live trading room sessions. The complete package for serious traders.',
			price: '$1,997',
			originalPrice: '$2,997',
			image:
				'https://cdn.simplertrading.com/2023/03/21162031/Micro-voodoo-lines-video-bg-300x169.jpg',
			category: 'Bundle',
			featured: true
		}
	];

	function getCategoryColor(category: string): string {
		switch (category) {
			case 'Course':
				return '#143E59';
			case 'Indicator':
				return '#F69532';
			case 'Bundle':
				return '#28a745';
			case 'Membership':
				return '#6f42c1';
			default:
				return '#666';
		}
	}
</script>

<!-- Page Content -->
{#if trader}
	<div class="content-block">
		<h2 class="section-heading">{trader.name.split(' ')[0]}'s Trader Store</h2>
	</div>
	<div class="content-block">
		<div class="content-text">
			<p>
				Explore courses, indicators, and trading resources from {trader.name}. Each product is
				designed to help you become a better trader.
			</p>
		</div>
	</div>

	<!-- Products Grid -->
	<div class="products-grid">
		{#each products as product (product.id)}
			<article class="product-card" class:featured={product.featured}>
				{#if product.featured}
					<span class="featured-badge">Featured</span>
				{/if}
				<figure class="card-media">
					<div class="card-image" style="background-image: url({product.image});">
						<span
							class="category-badge"
							style="background-color: {getCategoryColor(product.category)}"
						>
							{product.category}
						</span>
					</div>
				</figure>
				<div class="card-body">
					<h3 class="product-name">{product.name}</h3>
					<p class="product-description">{product.description}</p>
					<div class="price-row">
						<span class="price">{product.price}</span>
						{#if product.originalPrice}
							<span class="original-price">{product.originalPrice}</span>
						{/if}
					</div>
				</div>
				<footer class="card-footer">
					<button class="btn btn-primary btn-block">Add to Cart</button>
					<a
						href="/dashboard/day-trading-room/meet-the-traders/{trader.slug}/trader-store/{product.id}"
						class="btn btn-outline btn-block">Learn More</a
					>
				</footer>
			</article>
		{/each}
	</div>

	{#if products.length === 0}
		<div class="empty-state">
			<p>No products available at this time. Check back soon!</p>
		</div>
	{/if}
{/if}

<style>
	.content-block {
		margin-bottom: 20px;
	}
	.content-block:last-child {
		margin-bottom: 0;
	}

	.section-heading {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 15px;
		line-height: 1.2;
	}

	.content-text {
		font-size: 16px;
		line-height: 1.7;
		color: #444;
		margin-bottom: 30px;
	}
	.content-text p {
		margin: 0;
	}

	.products-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 25px;
		margin-top: 20px;
	}

	.product-card {
		position: relative;
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 8px;
		overflow: hidden;
		transition: box-shadow 0.3s ease;
		display: flex;
		flex-direction: column;
	}
	.product-card:hover {
		box-shadow: 0 5px 20px rgba(0, 0, 0, 0.1);
	}
	.product-card.featured {
		border-color: #f69532;
		border-width: 2px;
	}

	.featured-badge {
		position: absolute;
		top: 15px;
		right: 15px;
		background: #f69532;
		color: #fff;
		padding: 5px 12px;
		font-size: 11px;
		font-weight: 600;
		border-radius: 3px;
		text-transform: uppercase;
		z-index: 10;
	}

	.card-media {
		margin: 0;
		padding: 0;
		position: relative;
		height: 180px;
		overflow: hidden;
	}
	.card-image {
		display: block;
		width: 100%;
		height: 100%;
		background-size: cover;
		background-position: center;
		background-repeat: no-repeat;
		position: relative;
	}

	.category-badge {
		position: absolute;
		bottom: 10px;
		left: 10px;
		padding: 4px 12px;
		color: #fff;
		font-size: 11px;
		font-weight: 600;
		border-radius: 3px;
		text-transform: uppercase;
	}

	.card-body {
		padding: 20px;
		flex: 1;
	}
	.product-name {
		font-size: 18px;
		font-weight: 700;
		color: #333;
		margin: 0 0 10px;
	}
	.product-description {
		font-size: 14px;
		color: #666;
		line-height: 1.6;
		margin: 0 0 15px;
	}

	.price-row {
		display: flex;
		align-items: center;
		gap: 10px;
	}
	.price {
		font-size: 24px;
		font-weight: 700;
		color: #143e59;
	}
	.original-price {
		font-size: 16px;
		color: #999;
		text-decoration: line-through;
	}

	.card-footer {
		padding: 15px 20px;
		background: #fafafa;
		border-top: 1px solid #e6e6e6;
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.btn {
		display: inline-block;
		padding: 12px 20px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
		cursor: pointer;
		border: none;
	}
	.btn-block {
		width: 100%;
	}
	.btn-primary {
		background: #f69532;
		color: #fff;
	}
	.btn-primary:hover {
		background: #dc7309;
	}
	.btn-outline {
		background: transparent;
		color: #143e59;
		border: 1px solid #143e59;
	}
	.btn-outline:hover {
		background: #143e59;
		color: #fff;
	}

	.empty-state {
		text-align: center;
		padding: 60px 20px;
		color: #666;
	}

	/* Mobile-first: 1 column by default, 2 on md+, 3 on lg+ */
	.products-grid {
		grid-template-columns: 1fr;
	}

	@media (min-width: 768px) {
		.products-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (min-width: 992px) {
		.products-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}
</style>

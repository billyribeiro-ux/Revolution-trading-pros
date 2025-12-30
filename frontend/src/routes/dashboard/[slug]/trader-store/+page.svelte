<script lang="ts">
	import type { PageData } from '@sveltejs/kit';
	import IconShoppingCart from '@tabler/icons-svelte/icons/shopping-cart';
	let { data }: { data: PageData } = $props();
	const { room, slug } = data;

	const products = [
		{ id: 1, name: 'Moxie Indicator', description: 'Advanced technical indicator for momentum.', price: '$497', image: '', url: '/products/moxie' },
		{ id: 2, name: 'Squeeze Pro', description: 'Professional squeeze indicator suite.', price: '$397', image: '', url: '/products/squeeze' },
		{ id: 3, name: 'Trading Psychology Mastery', description: 'Develop the trader mindset.', price: '$297', image: '', url: '/products/psychology' },
		{ id: 4, name: 'Options Blueprint', description: 'Complete options trading system.', price: '$597', image: '', url: '/products/options' }
	];
</script>

<svelte:head><title>Trader Store - {room.name} | Revolution Trading Pros</title></svelte:head>

<div class="trader-store">
	<header class="page-header"><h1 class="page-title">{room.name} Trader Store</h1><p class="page-subtitle">Premium tools, indicators, and courses.</p></header>
	<div class="products-grid">
		{#each products as product}
			<article class="product-card">
				<div class="product-image"></div>
				<div class="product-content">
					<h3 class="product-name">{product.name}</h3>
					<p class="product-description">{product.description}</p>
					<div class="product-footer">
						<span class="product-price">{product.price}</span>
						<a href={product.url} class="buy-btn"><IconShoppingCart size={16} /> Learn More</a>
					</div>
				</div>
			</article>
		{/each}
	</div>
</div>

<style>
	.trader-store { padding: 30px; max-width: 1200px; margin: 0 auto; }
	.page-header { margin-bottom: 40px; }
	.page-title { font-size: 28px; font-weight: 700; color: #333; margin: 0 0 8px; }
	.page-subtitle { font-size: 14px; color: #666; margin: 0; }
	.products-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 24px; }
	.product-card { background: #fff; border-radius: 12px; overflow: hidden; box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16); }
	.product-image { width: 100%; height: 180px; background: #0984ae; }
	.product-content { padding: 20px; }
	.product-name { font-size: 18px; font-weight: 700; color: #333; margin: 0 0 8px; }
	.product-description { font-size: 14px; color: #666; margin: 0 0 16px; }
	.product-footer { display: flex; align-items: center; justify-content: space-between; }
	.product-price { font-size: 20px; font-weight: 700; color: #0984ae; }
	.buy-btn { display: inline-flex; align-items: center; gap: 6px; padding: 10px 18px; background: #F69532; color: #fff; text-decoration: none; border-radius: 4px; font-size: 13px; font-weight: 600; }
</style>

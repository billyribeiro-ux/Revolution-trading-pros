<script lang="ts">
	/**
	 * Dynamic Checkout Route
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Handles direct product checkout URLs like:
	 * - /checkout/monthly-swings
	 * - /checkout/annual-room
	 * - /checkout/quarterly
	 *
	 * Adds the product to cart and redirects to the main checkout page.
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { cartStore } from '$lib/stores/cart';
	import { getProductBySlug, productToCartItem } from '$lib/data/products';

	// Get the product slug from the URL
	const slug = page.params.slug!;
	const product = getProductBySlug(slug);

	let error = $state<string | null>(null);
	let isLoading = $state(true);

	onMount(() => {
		if (!product) {
			error = `Product "${slug}" not found. Please check the URL or contact support.`;
			isLoading = false;
			return;
		}

		// Clear cart and add this product (direct checkout flow)
		cartStore.clearCart();
		cartStore.addItem(productToCartItem(product));

		// Redirect to main checkout page
		goto('/checkout', { replaceState: true });
	});
</script>

<svelte:head>
	<title>{product ? `Checkout - ${product.name}` : 'Product Not Found'} | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<main class="checkout-redirect">
	{#if isLoading && product}
		<div class="loading-container">
			<div class="loading-spinner"></div>
			<h2>Preparing your checkout...</h2>
			<p>Adding <strong>{product.name}</strong> to your cart</p>
		</div>
	{:else if error}
		<div class="error-container">
			<div class="error-icon">!</div>
			<h2>Product Not Found</h2>
			<p>{error}</p>
			<div class="error-actions">
				<a href="/" class="btn btn-secondary">Go Home</a>
				<a href="/alerts/explosive-swings" class="btn btn-primary">View Products</a>
			</div>
		</div>
	{/if}
</main>

<style>
	.checkout-redirect {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 100vh;
		background: #f4f4f4;
		padding: 20px;
	}

	.loading-container,
	.error-container {
		text-align: center;
		background: #fff;
		padding: 48px;
		border-radius: 12px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
		max-width: 400px;
	}

	/* Loading State */
	.loading-spinner {
		width: 48px;
		height: 48px;
		border: 3px solid #e5e7eb;
		border-top-color: #0984ae;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		margin: 0 auto 24px;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.loading-container h2 {
		color: #333;
		font-size: 20px;
		font-weight: 600;
		margin: 0 0 8px;
	}

	.loading-container p {
		color: #666;
		font-size: 14px;
		margin: 0;
	}

	.loading-container p strong {
		color: #0984ae;
	}

	/* Error State */
	.error-icon {
		width: 56px;
		height: 56px;
		background: #fef2f2;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		margin: 0 auto 20px;
		font-size: 28px;
		font-weight: 700;
		color: #ef4444;
	}

	.error-container h2 {
		color: #333;
		font-size: 22px;
		font-weight: 600;
		margin: 0 0 12px;
	}

	.error-container p {
		color: #666;
		font-size: 14px;
		line-height: 1.5;
		margin: 0 0 24px;
	}

	.error-actions {
		display: flex;
		gap: 12px;
		justify-content: center;
	}

	.btn {
		display: inline-block;
		padding: 12px 24px;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.btn-primary {
		background: #0984ae;
		color: #fff;
	}

	.btn-primary:hover {
		background: #076787;
	}

	.btn-secondary {
		background: #f4f4f4;
		color: #333;
		border: 1px solid #ddd;
	}

	.btn-secondary:hover {
		background: #e5e7eb;
	}
</style>

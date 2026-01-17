<!--
	High Octane Scanner Product Page
	Complete product page with pricing, features, and checkout
	
	Svelte 5 (Dec 2025):
	- Dynamic components without <svelte:component>
	- $state() and $derived() runes
	- Proper type safety
	
	@version 2.0.0 - Svelte 5 Runes
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { isAuthenticated } from '$lib/stores/auth.svelte';
	import { addToCart } from '$lib/stores/cart.svelte';
	import {
		IconCheck,
		IconShoppingCart,
		IconChartLine,
		IconBell,
		IconFilter,
		IconClock
	} from '$lib/icons';

	// Product configuration
	const product = {
		id: 'high-octane-scanner',
		name: 'High Octane Scanner',
		slug: 'high-octane-scanner',
		type: 'indicator' as const,
		description:
			'Professional-grade options scanner with real-time alerts and advanced filtering algorithms',
		longDescription: `
			The High Octane Scanner is your ultimate tool for finding high-probability options trades. 
			Built by professional traders for serious market participants, this scanner uses proprietary 
			algorithms to identify opportunities across thousands of stocks in real-time.
		`,
		price: 119,
		priceYearly: 1190,
		interval: 'monthly' as const,
		features: [
			{
				icon: IconChartLine,
				title: 'Real-Time Scanning',
				description: 'Scan thousands of options contracts in milliseconds'
			},
			{
				icon: IconFilter,
				title: 'Advanced Filters',
				description: 'Customize scans with 50+ technical indicators and criteria'
			},
			{
				icon: IconBell,
				title: 'Smart Alerts',
				description: 'Get instant notifications via SMS, email, and mobile app'
			},
			{
				icon: IconClock,
				title: 'Historical Analysis',
				description: 'Backtest strategies with 10+ years of historical data'
			}
		],
		benefits: [
			'Find high-probability trades faster',
			'Reduce analysis time by 90%',
			'Never miss a trading opportunity',
			'Access from desktop and mobile',
			'Unlimited scans and alerts',
			'Priority customer support'
		],
		images: [
			'/images/scanners/high-octane-scanner-hero.jpg',
			'/images/scanners/high-octane-scanner-filters.jpg',
			'/images/scanners/high-octane-scanner-alerts.jpg'
		]
	};

	let selectedInterval = $state<'monthly' | 'yearly'>('monthly');
	let isAddingToCart = $state(false);
	let showSuccessMessage = $state(false);

	const currentPrice = $derived(
		selectedInterval === 'monthly' ? product.price : product.priceYearly
	);
	const savingsPercent = $derived(
		Math.round((1 - product.priceYearly / (product.price * 12)) * 100)
	);

	async function handleAddToCart() {
		if (!$isAuthenticated) {
			goto(`/login?redirect=/store/scanners/${product.slug}`);
			return;
		}

		isAddingToCart = true;

		try {
			await addToCart({
				productId: product.id,
				productName: product.name,
				productType: product.type,
				price: currentPrice,
				interval: selectedInterval,
				quantity: 1
			});

			showSuccessMessage = true;
			setTimeout(() => {
				showSuccessMessage = false;
			}, 3000);
		} catch (error) {
			console.error('Failed to add to cart:', error);
			alert('Failed to add to cart. Please try again.');
		} finally {
			isAddingToCart = false;
		}
	}

	function handleBuyNow() {
		handleAddToCart();
		setTimeout(() => {
			goto('/checkout');
		}, 500);
	}
</script>

<svelte:head>
	<title>{product.name} - $119/month | Revolution Trading Pros</title>
	<meta name="description" content={product.description} />
</svelte:head>

<div class="product-page">
	<!-- Hero Section -->
	<section class="hero">
		<div class="container">
			<div class="hero-content">
				<div class="hero-text">
					<h1>{product.name}</h1>
					<p class="tagline">{product.description}</p>

					<div class="pricing-section">
						<div class="interval-toggle">
							<button
								class="interval-btn"
								class:active={selectedInterval === 'monthly'}
								onclick={() => (selectedInterval = 'monthly')}
							>
								Monthly
							</button>
							<button
								class="interval-btn"
								class:active={selectedInterval === 'yearly'}
								onclick={() => (selectedInterval = 'yearly')}
							>
								Yearly
								{#if savingsPercent > 0}
									<span class="savings-badge">Save {savingsPercent}%</span>
								{/if}
							</button>
						</div>

						<div class="price-display">
							<span class="price">${currentPrice}</span>
							<span class="interval">/{selectedInterval}</span>
						</div>

						{#if selectedInterval === 'yearly'}
							<p class="price-note">
								${Math.round(product.priceYearly / 12)}/month billed annually
							</p>
						{/if}
					</div>

					<div class="cta-buttons">
						<button class="btn-primary btn-large" onclick={handleBuyNow} disabled={isAddingToCart}>
							{#if isAddingToCart}
								Processing...
							{:else}
								Buy Now
							{/if}
						</button>

						<button
							class="btn-secondary btn-large"
							onclick={handleAddToCart}
							disabled={isAddingToCart}
						>
							<IconShoppingCart size={20} />
							Add to Cart
						</button>
					</div>

					{#if showSuccessMessage}
						<div class="success-message">
							<IconCheck size={20} />
							Added to cart successfully!
						</div>
					{/if}
				</div>

				<div class="hero-image">
					<div class="image-placeholder">
						<IconChartLine size={120} />
						<p>Scanner Interface Preview</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Features Section -->
	<section class="features-section">
		<div class="container">
			<h2>Powerful Features</h2>
			<div class="features-grid">
				{#each product.features as feature}
					<!-- Svelte 5: {@const} must be immediate child of {#each} -->
					{@const Icon = feature.icon}
					<div class="feature-card">
						<div class="feature-icon">
							<Icon size={32} />
						</div>
						<h3>{feature.title}</h3>
						<p>{feature.description}</p>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- Benefits Section -->
	<section class="benefits-section">
		<div class="container">
			<h2>What You Get</h2>
			<div class="benefits-grid">
				{#each product.benefits as benefit}
					<div class="benefit-item">
						<IconCheck size={24} class="check-icon" />
						<span>{benefit}</span>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- CTA Section -->
	<section class="cta-section">
		<div class="container">
			<h2>Ready to Start Scanning?</h2>
			<p>Join thousands of traders using the High Octane Scanner</p>
			<button class="btn-primary btn-large" onclick={handleBuyNow}>
				Get Started Now - ${currentPrice}/{selectedInterval}
			</button>
		</div>
	</section>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * PRODUCT PAGE LAYOUT
	 * Apple ICT 11+ Standards
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.product-page {
		min-height: 100vh;
		background: #ffffff;
	}

	.container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 24px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * HERO SECTION
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.hero {
		background: linear-gradient(135deg, #0a2335 0%, #0984ae 100%);
		color: white;
		padding: 120px 0 80px;
	}

	.hero-content {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 60px;
		align-items: center;
	}

	.hero-text h1 {
		font-size: 3.5rem;
		font-weight: 700;
		margin-bottom: 20px;
		line-height: 1.2;
	}

	.tagline {
		font-size: 1.25rem;
		opacity: 0.9;
		margin-bottom: 40px;
		line-height: 1.6;
	}

	/* Pricing Section */
	.pricing-section {
		background: rgba(255, 255, 255, 0.1);
		backdrop-filter: blur(10px);
		border-radius: 16px;
		padding: 32px;
		margin-bottom: 32px;
	}

	.interval-toggle {
		display: flex;
		gap: 12px;
		margin-bottom: 24px;
	}

	.interval-btn {
		flex: 1;
		padding: 12px 24px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		background: transparent;
		color: white;
		border-radius: 8px;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.3s ease;
		position: relative;
	}

	.interval-btn:hover {
		border-color: rgba(255, 255, 255, 0.6);
		background: rgba(255, 255, 255, 0.1);
	}

	.interval-btn.active {
		background: white;
		color: #0a2335;
		border-color: white;
	}

	.savings-badge {
		position: absolute;
		top: -8px;
		right: -8px;
		background: #10b981;
		color: white;
		font-size: 0.75rem;
		padding: 4px 8px;
		border-radius: 4px;
		font-weight: 700;
	}

	.price-display {
		display: flex;
		align-items: baseline;
		gap: 8px;
		margin-bottom: 8px;
	}

	.price {
		font-size: 4rem;
		font-weight: 700;
	}

	.interval {
		font-size: 1.5rem;
		opacity: 0.8;
	}

	.price-note {
		font-size: 0.875rem;
		opacity: 0.7;
	}

	/* CTA Buttons */
	.cta-buttons {
		display: flex;
		gap: 16px;
		margin-bottom: 20px;
	}

	.btn-large {
		padding: 16px 48px;
		font-size: 1.125rem;
		border-radius: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.3s ease;
		border: none;
	}

	.btn-primary {
		background: white;
		color: #0a2335;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 8px 24px rgba(255, 255, 255, 0.3);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: transparent;
		color: white;
		border: 2px solid white;
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.btn-secondary:hover:not(:disabled) {
		background: rgba(255, 255, 255, 0.1);
	}

	.success-message {
		display: flex;
		align-items: center;
		gap: 8px;
		background: #10b981;
		color: white;
		padding: 12px 20px;
		border-radius: 8px;
		font-weight: 600;
		animation: slideIn 0.3s ease;
	}

	@keyframes slideIn {
		from {
			opacity: 0;
			transform: translateY(-10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	/* Hero Image */
	.hero-image {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.image-placeholder {
		background: rgba(255, 255, 255, 0.1);
		backdrop-filter: blur(10px);
		border-radius: 16px;
		padding: 60px;
		text-align: center;
		border: 2px solid rgba(255, 255, 255, 0.2);
	}

	.image-placeholder p {
		margin-top: 20px;
		opacity: 0.8;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * FEATURES SECTION
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.features-section {
		padding: 100px 0;
		background: #f8fafc;
	}

	.features-section h2 {
		text-align: center;
		font-size: 2.5rem;
		font-weight: 700;
		color: #0a2335;
		margin-bottom: 60px;
	}

	.features-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 32px;
	}

	.feature-card {
		background: white;
		padding: 32px;
		border-radius: 12px;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
		transition:
			transform 0.3s ease,
			box-shadow 0.3s ease;
	}

	.feature-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 12px 24px rgba(0, 0, 0, 0.1);
	}

	.feature-icon {
		width: 64px;
		height: 64px;
		background: linear-gradient(135deg, #0984ae 0%, #0a2335 100%);
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		margin-bottom: 20px;
	}

	.feature-card h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #0a2335;
		margin-bottom: 12px;
	}

	.feature-card p {
		color: #64748b;
		line-height: 1.6;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * BENEFITS SECTION
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.benefits-section {
		padding: 100px 0;
		background: white;
	}

	.benefits-section h2 {
		text-align: center;
		font-size: 2.5rem;
		font-weight: 700;
		color: #0a2335;
		margin-bottom: 60px;
	}

	.benefits-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		gap: 24px;
		max-width: 900px;
		margin: 0 auto;
	}

	.benefit-item {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 20px;
		background: #f8fafc;
		border-radius: 8px;
	}

	.benefit-item :global(.check-icon) {
		color: #10b981;
		flex-shrink: 0;
	}

	.benefit-item span {
		font-size: 1.125rem;
		color: #334155;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * CTA SECTION
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.cta-section {
		padding: 100px 0;
		background: linear-gradient(135deg, #0a2335 0%, #0984ae 100%);
		color: white;
		text-align: center;
	}

	.cta-section h2 {
		font-size: 2.5rem;
		font-weight: 700;
		margin-bottom: 16px;
	}

	.cta-section p {
		font-size: 1.25rem;
		opacity: 0.9;
		margin-bottom: 40px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (width < 768px) {
		.hero {
			padding: 80px 0 60px;
		}

		.hero-content {
			grid-template-columns: 1fr;
			gap: 40px;
		}

		.hero-text h1 {
			font-size: 2.5rem;
		}

		.price {
			font-size: 3rem;
		}

		.cta-buttons {
			flex-direction: column;
		}

		.features-section,
		.benefits-section,
		.cta-section {
			padding: 60px 0;
		}

		.features-section h2,
		.benefits-section h2,
		.cta-section h2 {
			font-size: 2rem;
		}
	}
</style>

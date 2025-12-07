<script lang="ts">
	/**
	 * Thank You / Order Confirmation Page
	 * Follows WordPress Simpler Trading patterns exactly
	 */
	import SEOHead from '$lib/components/SEOHead.svelte';
	import { page } from '$app/stores';

	// Get order details from URL params or session
	let orderNumber = $state($page.url.searchParams.get('order') || 'RTP-' + Date.now().toString(36).toUpperCase());
	let productName = $state($page.url.searchParams.get('product') || 'Revolution Trading Pro Membership');

	// Sample purchased item (would come from order data)
	const purchasedItem = {
		name: productName,
		image: '/images/membership-card.jpg',
		type: 'Membership',
		nextStep: 'Access your dashboard to start learning'
	};

	// Related upsell products
	const upsellProducts = [
		{
			id: 'advanced-options',
			name: 'Advanced Options Mastery',
			description: 'Take your options trading to the next level with advanced strategies.',
			originalPrice: 497,
			salePrice: 297,
			savings: 200,
			image: '/images/courses/advanced-options.jpg',
			badge: 'MEMBERS ONLY'
		},
		{
			id: 'indicator-bundle',
			name: 'Pro Indicator Bundle',
			description: 'Get all 5 premium indicators at a special member discount.',
			originalPrice: 799,
			salePrice: 499,
			savings: 300,
			image: '/images/indicators/bundle.jpg',
			badge: 'BEST VALUE'
		}
	];
</script>

<SEOHead
	title="Thanks for Your Purchase"
	description="Welcome to Revolution Trading Pros! You've taken the first step towards strategic trading."
	noindex
/>

<!-- Breadcrumbs -->
<nav id="breadcrumbs" class="breadcrumbs">
	<div class="container-fluid">
		<ul>
			<li class="item-home">
				<a class="breadcrumb-link breadcrumb-home" href="/" title="Home">Home</a>
			</li>
			<li class="separator separator-home"> / </li>
			<li class="item-current">
				<strong class="breadcrumb-current">Order Confirmation</strong>
			</li>
		</ul>
	</div>
</nav>

<!-- Main Thank You Content -->
<main class="typ-page">
	<div class="container">

		<!-- Success Header -->
		<section class="typ-header">
			<div class="typ-header__icon">
				<span class="st-icon st-icon-check-circle"></span>
			</div>
			<h1 class="typ-header__title">Thank You for Your Order!</h1>
			<p class="typ-header__subtitle">Order #{orderNumber} has been confirmed</p>
		</section>

		<!-- Welcome Section -->
		<section class="typ-welcome">
			<div class="typ-welcome__content">
				<h2 class="typ-welcome__pretitle">WELCOME TO</h2>
				<h1 class="typ-welcome__title">Revolution Trading Pros</h1>
				<p class="typ-welcome__description">
					We're thrilled to have you join our trading community! You now have access to expert-led
					training, real-time trading insights, and a supportive community of traders.
				</p>
				<p class="typ-welcome__description">
					Your journey to consistent, profitable trading starts now. Let's make it happen together.
				</p>
			</div>
			<div class="typ-welcome__image">
				<img src="/images/welcome-trading.jpg" alt="Welcome to Revolution Trading Pros" />
			</div>
		</section>

		<!-- What You Get Section -->
		<section class="typ-benefits">
			<h2 class="typ-benefits__title">What You Get Access To</h2>
			<div class="row">
				<div class="col-sm-6 col-lg-3">
					<div class="typ-benefit-card">
						<span class="typ-benefit-card__icon st-icon st-icon-learning-center"></span>
						<h3 class="typ-benefit-card__title">Learning Center</h3>
						<p class="typ-benefit-card__text">Complete video courses and tutorials</p>
					</div>
				</div>
				<div class="col-sm-6 col-lg-3">
					<div class="typ-benefit-card">
						<span class="typ-benefit-card__icon st-icon st-icon-dashboard"></span>
						<h3 class="typ-benefit-card__title">Live Trading Room</h3>
						<p class="typ-benefit-card__text">Trade alongside our experts daily</p>
					</div>
				</div>
				<div class="col-sm-6 col-lg-3">
					<div class="typ-benefit-card">
						<span class="typ-benefit-card__icon st-icon st-icon-chatroom-archive"></span>
						<h3 class="typ-benefit-card__title">Video Archives</h3>
						<p class="typ-benefit-card__text">Access to all past trading sessions</p>
					</div>
				</div>
				<div class="col-sm-6 col-lg-3">
					<div class="typ-benefit-card">
						<span class="typ-benefit-card__icon st-icon st-icon-support"></span>
						<h3 class="typ-benefit-card__title">Community Support</h3>
						<p class="typ-benefit-card__text">Connect with fellow traders</p>
					</div>
				</div>
			</div>
		</section>

		<!-- Next Steps Section -->
		<section class="typ-next-steps">
			<h2 class="typ-next-steps__title">Your Next Steps</h2>
			<div class="typ-next-steps__list">
				<div class="typ-step">
					<span class="typ-step__number">1</span>
					<div class="typ-step__content">
						<h3 class="typ-step__title">Access Your Dashboard</h3>
						<p class="typ-step__text">Click the button below to go to your member dashboard and explore your new membership.</p>
					</div>
				</div>
				<div class="typ-step">
					<span class="typ-step__number">2</span>
					<div class="typ-step__content">
						<h3 class="typ-step__title">Watch the Getting Started Video</h3>
						<p class="typ-step__text">We've prepared a quick orientation video to help you make the most of your membership.</p>
					</div>
				</div>
				<div class="typ-step">
					<span class="typ-step__number">3</span>
					<div class="typ-step__content">
						<h3 class="typ-step__title">Join the Trading Room</h3>
						<p class="typ-step__text">Our live trading room opens at 9:00 AM ET. Join us to see our strategies in action!</p>
					</div>
				</div>
			</div>
			<div class="typ-next-steps__actions">
				<a href="/dashboard" class="btn btn-orange btn-lg">
					Go to My Dashboard
				</a>
				<a href="/dashboard/courses" class="btn btn-outline-light btn-lg">
					Browse Courses
				</a>
			</div>
		</section>

		<!-- Upsell Section -->
		<section class="typ-upsell">
			<div class="typ-upsell__header">
				<h2 class="typ-upsell__title">Exclusive Member Offers</h2>
				<p class="typ-upsell__subtitle">Special pricing available only for new members - limited time!</p>
			</div>
			<div class="row">
				{#each upsellProducts as product}
					<div class="col-md-6">
						<div class="typ-upsell-card">
							{#if product.badge}
								<span class="typ-upsell-card__badge">{product.badge}</span>
							{/if}
							<div class="typ-upsell-card__image">
								<img src={product.image} alt={product.name} />
							</div>
							<div class="typ-upsell-card__content">
								<h3 class="typ-upsell-card__title">{product.name}</h3>
								<p class="typ-upsell-card__description">{product.description}</p>
								<div class="typ-upsell-card__pricing">
									<span class="typ-upsell-card__original">${product.originalPrice}</span>
									<span class="typ-upsell-card__sale">${product.salePrice}</span>
									<span class="typ-upsell-card__savings">Save ${product.savings}!</span>
								</div>
								<a href="/checkout?product={product.id}" class="btn btn-orange btn-block">
									Add to Order
								</a>
							</div>
						</div>
					</div>
				{/each}
			</div>
		</section>

		<!-- Support Section -->
		<section class="typ-support">
			<div class="typ-support__content">
				<h3 class="typ-support__title">Need Help Getting Started?</h3>
				<p class="typ-support__text">Our support team is here to help you every step of the way.</p>
				<div class="typ-support__actions">
					<a href="/dashboard/support" class="btn btn-outline-orange">Contact Support</a>
					<a href="/resources/faq" class="btn btn-link">View FAQ</a>
				</div>
			</div>
		</section>

	</div>
</main>

<style>
	/* Breadcrumbs */
	.breadcrumbs {
		background: var(--st-color-gray-900, #111827);
		padding: 1rem 0;
		border-bottom: 1px solid var(--st-color-gray-800, #1f2937);
	}

	.breadcrumbs ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		align-items: center;
		flex-wrap: wrap;
	}

	.breadcrumbs li {
		display: inline-flex;
		align-items: center;
	}

	.breadcrumbs .separator {
		color: var(--st-color-gray-500, #6b7280);
		margin: 0 0.5rem;
	}

	.breadcrumbs a {
		color: var(--st-color-gray-400, #9ca3af);
		text-decoration: none;
		font-size: 0.875rem;
	}

	.breadcrumbs a:hover {
		color: var(--st-color-orange, #f97316);
	}

	.breadcrumbs .breadcrumb-current {
		color: var(--st-color-white, #fff);
		font-size: 0.875rem;
	}

	/* Main Page */
	.typ-page {
		background: var(--st-color-gray-900, #111827);
		min-height: 100vh;
		padding: 3rem 0;
	}

	.container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 1.5rem;
	}

	/* Header */
	.typ-header {
		text-align: center;
		padding: 3rem 0;
		border-bottom: 1px solid var(--st-color-gray-800, #1f2937);
		margin-bottom: 3rem;
	}

	.typ-header__icon {
		font-size: 4rem;
		color: var(--st-color-green, #22c55e);
		margin-bottom: 1.5rem;
	}

	.typ-header__icon .st-icon::before {
		content: "\2713";
	}

	.typ-header__title {
		font-size: 2.5rem;
		font-weight: 700;
		color: var(--st-color-white, #fff);
		margin-bottom: 0.5rem;
	}

	.typ-header__subtitle {
		font-size: 1.125rem;
		color: var(--st-color-gray-400, #9ca3af);
	}

	/* Welcome Section */
	.typ-welcome {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 3rem;
		align-items: center;
		padding: 3rem 0;
		border-bottom: 1px solid var(--st-color-gray-800, #1f2937);
		margin-bottom: 3rem;
	}

	.typ-welcome__pretitle {
		font-size: 0.875rem;
		font-weight: 600;
		letter-spacing: 0.1em;
		color: var(--st-color-orange, #f97316);
		margin-bottom: 0.5rem;
	}

	.typ-welcome__title {
		font-size: 3rem;
		font-weight: 700;
		color: var(--st-color-white, #fff);
		margin-bottom: 1.5rem;
		line-height: 1.2;
	}

	.typ-welcome__description {
		font-size: 1.125rem;
		color: var(--st-color-gray-300, #d1d5db);
		line-height: 1.7;
		margin-bottom: 1rem;
	}

	.typ-welcome__image img {
		width: 100%;
		border-radius: 1rem;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
	}

	/* Benefits Section */
	.typ-benefits {
		padding: 3rem 0;
		border-bottom: 1px solid var(--st-color-gray-800, #1f2937);
		margin-bottom: 3rem;
	}

	.typ-benefits__title {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--st-color-white, #fff);
		text-align: center;
		margin-bottom: 2rem;
	}

	.row {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -0.75rem;
	}

	.col-sm-6 {
		flex: 0 0 50%;
		max-width: 50%;
		padding: 0 0.75rem;
		margin-bottom: 1.5rem;
	}

	.col-lg-3 {
		flex: 0 0 25%;
		max-width: 25%;
	}

	.col-md-6 {
		flex: 0 0 50%;
		max-width: 50%;
		padding: 0 0.75rem;
		margin-bottom: 1.5rem;
	}

	.typ-benefit-card {
		background: var(--st-color-gray-800, #1f2937);
		border-radius: 1rem;
		padding: 2rem;
		text-align: center;
		height: 100%;
		transition: transform 0.2s ease, box-shadow 0.2s ease;
	}

	.typ-benefit-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 20px 40px rgba(0, 0, 0, 0.3);
	}

	.typ-benefit-card__icon {
		font-size: 2.5rem;
		color: var(--st-color-orange, #f97316);
		margin-bottom: 1rem;
		display: block;
	}

	.typ-benefit-card__title {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--st-color-white, #fff);
		margin-bottom: 0.5rem;
	}

	.typ-benefit-card__text {
		font-size: 0.875rem;
		color: var(--st-color-gray-400, #9ca3af);
		margin: 0;
	}

	/* Next Steps Section */
	.typ-next-steps {
		padding: 3rem 0;
		border-bottom: 1px solid var(--st-color-gray-800, #1f2937);
		margin-bottom: 3rem;
	}

	.typ-next-steps__title {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--st-color-white, #fff);
		text-align: center;
		margin-bottom: 2rem;
	}

	.typ-next-steps__list {
		max-width: 700px;
		margin: 0 auto 2rem;
	}

	.typ-step {
		display: flex;
		gap: 1.5rem;
		padding: 1.5rem;
		background: var(--st-color-gray-800, #1f2937);
		border-radius: 1rem;
		margin-bottom: 1rem;
	}

	.typ-step__number {
		flex-shrink: 0;
		width: 3rem;
		height: 3rem;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--st-color-orange, #f97316);
		color: var(--st-color-white, #fff);
		font-size: 1.25rem;
		font-weight: 700;
		border-radius: 50%;
	}

	.typ-step__title {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--st-color-white, #fff);
		margin-bottom: 0.25rem;
	}

	.typ-step__text {
		font-size: 0.9375rem;
		color: var(--st-color-gray-400, #9ca3af);
		margin: 0;
	}

	.typ-next-steps__actions {
		display: flex;
		gap: 1rem;
		justify-content: center;
		flex-wrap: wrap;
	}

	/* Upsell Section */
	.typ-upsell {
		padding: 3rem 0;
		border-bottom: 1px solid var(--st-color-gray-800, #1f2937);
		margin-bottom: 3rem;
	}

	.typ-upsell__header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.typ-upsell__title {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--st-color-white, #fff);
		margin-bottom: 0.5rem;
	}

	.typ-upsell__subtitle {
		font-size: 1rem;
		color: var(--st-color-gray-400, #9ca3af);
	}

	.typ-upsell-card {
		background: var(--st-color-gray-800, #1f2937);
		border-radius: 1rem;
		overflow: hidden;
		position: relative;
		height: 100%;
	}

	.typ-upsell-card__badge {
		position: absolute;
		top: 1rem;
		right: 1rem;
		background: var(--st-color-orange, #f97316);
		color: var(--st-color-white, #fff);
		font-size: 0.75rem;
		font-weight: 700;
		padding: 0.375rem 0.75rem;
		border-radius: 9999px;
		z-index: 1;
	}

	.typ-upsell-card__image {
		height: 200px;
		background: var(--st-color-gray-700, #374151);
		overflow: hidden;
	}

	.typ-upsell-card__image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.typ-upsell-card__content {
		padding: 1.5rem;
	}

	.typ-upsell-card__title {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--st-color-white, #fff);
		margin-bottom: 0.5rem;
	}

	.typ-upsell-card__description {
		font-size: 0.875rem;
		color: var(--st-color-gray-400, #9ca3af);
		margin-bottom: 1rem;
	}

	.typ-upsell-card__pricing {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1rem;
		flex-wrap: wrap;
	}

	.typ-upsell-card__original {
		font-size: 1rem;
		color: var(--st-color-gray-500, #6b7280);
		text-decoration: line-through;
	}

	.typ-upsell-card__sale {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--st-color-white, #fff);
	}

	.typ-upsell-card__savings {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--st-color-green, #22c55e);
	}

	/* Support Section */
	.typ-support {
		text-align: center;
		padding: 3rem;
		background: var(--st-color-gray-800, #1f2937);
		border-radius: 1rem;
	}

	.typ-support__title {
		font-size: 1.5rem;
		font-weight: 600;
		color: var(--st-color-white, #fff);
		margin-bottom: 0.5rem;
	}

	.typ-support__text {
		font-size: 1rem;
		color: var(--st-color-gray-400, #9ca3af);
		margin-bottom: 1.5rem;
	}

	.typ-support__actions {
		display: flex;
		gap: 1rem;
		justify-content: center;
		flex-wrap: wrap;
	}

	/* Buttons */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 0.875rem 1.75rem;
		font-size: 1rem;
		font-weight: 600;
		text-decoration: none;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s ease;
		border: none;
	}

	.btn-orange {
		background: linear-gradient(135deg, var(--st-color-orange, #f97316), #ea580c);
		color: var(--st-color-white, #fff);
	}

	.btn-orange:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 20px rgba(249, 115, 22, 0.3);
	}

	.btn-outline-light {
		background: transparent;
		border: 2px solid var(--st-color-gray-600, #4b5563);
		color: var(--st-color-white, #fff);
	}

	.btn-outline-light:hover {
		background: var(--st-color-gray-800, #1f2937);
		border-color: var(--st-color-gray-500, #6b7280);
	}

	.btn-outline-orange {
		background: transparent;
		border: 2px solid var(--st-color-orange, #f97316);
		color: var(--st-color-orange, #f97316);
	}

	.btn-outline-orange:hover {
		background: var(--st-color-orange, #f97316);
		color: var(--st-color-white, #fff);
	}

	.btn-link {
		background: transparent;
		color: var(--st-color-gray-400, #9ca3af);
		padding: 0.875rem 1rem;
	}

	.btn-link:hover {
		color: var(--st-color-white, #fff);
	}

	.btn-lg {
		padding: 1rem 2rem;
		font-size: 1.125rem;
	}

	.btn-block {
		display: block;
		width: 100%;
	}

	/* Responsive */
	@media (max-width: 991px) {
		.col-lg-3 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (max-width: 767px) {
		.typ-welcome {
			grid-template-columns: 1fr;
		}

		.typ-welcome__title {
			font-size: 2rem;
		}

		.typ-header__title {
			font-size: 1.75rem;
		}

		.col-sm-6, .col-lg-3, .col-md-6 {
			flex: 0 0 100%;
			max-width: 100%;
		}

		.typ-step {
			flex-direction: column;
			text-align: center;
		}

		.typ-step__number {
			margin: 0 auto;
		}
	}
</style>

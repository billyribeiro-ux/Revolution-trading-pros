<script lang="ts">
	/**
	 * Thank You / Order Confirmation Page
	 * Apple ICT 11+ Principal Engineer Grade - January 2026
	 *
	 * Complete order confirmation with:
	 * - Full order details from API
	 * - Itemized purchase breakdown
	 * - Next steps guidance
	 * - Relevant upsell offers
	 *
	 * @version 3.0.0 - Complete Order Details Display
	 */
	import SEOHead from '$lib/components/SEOHead.svelte';
	import type { PageData } from './$types';
	import type { OrderDetail, OrderItem } from './+page';

	// Receive data from load function using Svelte 5 $props()
	interface Props {
		data: PageData;
	}
	let { data }: Props = $props();

	// Derive order details with fallbacks
	let orderNumber = $derived(data.orderDetail?.order_number || data.orderNumber || 'RTP-' + Date.now().toString(36).toUpperCase());
	let orderDetail = $derived(data.orderDetail as OrderDetail | null);
	let fetchError = $derived(data.fetchError);

	// Format currency
	function formatCurrency(amount: number, currency: string = 'USD'): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: currency
		}).format(amount);
	}

	// Format date
	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	// Get product type label
	function getProductTypeLabel(type: string | null): string {
		const labels: Record<string, string> = {
			'membership': 'Membership',
			'trading-room': 'Trading Room',
			'alert-service': 'Alert Service',
			'course': 'Course',
			'indicator': 'Indicator',
			'premium-report': 'Premium Report',
			'weekly-watchlist': 'Weekly Watchlist'
		};
		return type ? labels[type] || type : 'Product';
	}

	// Get access URL based on product type
	function getAccessUrl(item: OrderItem): string {
		const urls: Record<string, string> = {
			'membership': '/account/memberships',
			'trading-room': '/live-trading-rooms',
			'alert-service': '/alerts',
			'course': item.product_slug ? `/courses/${item.product_slug}` : '/courses',
			'indicator': item.product_slug ? `/indicators/${item.product_slug}` : '/my-indicators',
			'premium-report': '/premium-reports',
			'weekly-watchlist': '/weekly-watchlist'
		};
		return item.product_type ? urls[item.product_type] || '/account' : '/account';
	}

	// Status badge styling
	function getStatusClass(status: string): string {
		const classes: Record<string, string> = {
			'completed': 'badge-success',
			'pending': 'badge-warning',
			'processing': 'badge-info',
			'refunded': 'badge-error',
			'cancelled': 'badge-error'
		};
		return classes[status] || 'badge-default';
	}

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
<nav id="breadcrumbs" class="breadcrumbs" aria-label="Breadcrumb">
	<div class="container-fluid">
		<ul>
			<li class="item-home">
				<a class="breadcrumb-link breadcrumb-home" href="/" title="Home">Home</a>
			</li>
			<li class="separator separator-home" aria-hidden="true">/</li>
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
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="64" height="64">
					<path fill-rule="evenodd" d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zm13.36-1.814a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd" />
				</svg>
			</div>
			<h1 class="typ-header__title">Thank You for Your Order!</h1>
			<p class="typ-header__subtitle">Order #{orderNumber} has been confirmed</p>
			{#if orderDetail?.status}
				<span class="typ-header__badge {getStatusClass(orderDetail.status)}">
					{orderDetail.status.charAt(0).toUpperCase() + orderDetail.status.slice(1)}
				</span>
			{/if}
		</section>

		<!-- Order Details Section -->
		{#if orderDetail}
			<section class="typ-order-details">
				<h2 class="typ-order-details__title">Order Details</h2>

				<div class="typ-order-details__grid">
					<!-- Order Info -->
					<div class="typ-order-info">
						<div class="typ-order-info__row">
							<span class="typ-order-info__label">Order Number</span>
							<span class="typ-order-info__value">{orderDetail.order_number}</span>
						</div>
						<div class="typ-order-info__row">
							<span class="typ-order-info__label">Date</span>
							<span class="typ-order-info__value">{formatDate(orderDetail.created_at)}</span>
						</div>
						{#if orderDetail.billing_email}
							<div class="typ-order-info__row">
								<span class="typ-order-info__label">Email</span>
								<span class="typ-order-info__value">{orderDetail.billing_email}</span>
							</div>
						{/if}
						{#if orderDetail.payment_provider}
							<div class="typ-order-info__row">
								<span class="typ-order-info__label">Payment Method</span>
								<span class="typ-order-info__value typ-order-info__value--capitalize">
									{orderDetail.payment_provider}
								</span>
							</div>
						{/if}
					</div>

					<!-- Order Summary -->
					<div class="typ-order-summary">
						<h3 class="typ-order-summary__title">Summary</h3>
						<div class="typ-order-summary__row">
							<span>Subtotal</span>
							<span>{formatCurrency(orderDetail.subtotal, orderDetail.currency)}</span>
						</div>
						{#if orderDetail.discount > 0}
							<div class="typ-order-summary__row typ-order-summary__row--discount">
								<span>Discount {orderDetail.coupon_code ? `(${orderDetail.coupon_code})` : ''}</span>
								<span>-{formatCurrency(orderDetail.discount, orderDetail.currency)}</span>
							</div>
						{/if}
						{#if orderDetail.tax > 0}
							<div class="typ-order-summary__row">
								<span>Tax</span>
								<span>{formatCurrency(orderDetail.tax, orderDetail.currency)}</span>
							</div>
						{/if}
						<div class="typ-order-summary__row typ-order-summary__row--total">
							<span>Total</span>
							<span>{formatCurrency(orderDetail.total, orderDetail.currency)}</span>
						</div>
					</div>
				</div>

				<!-- Order Items -->
				<div class="typ-order-items">
					<h3 class="typ-order-items__title">Items Purchased</h3>
					<div class="typ-order-items__list">
						{#each orderDetail.items as item}
							<div class="typ-order-item">
								<div class="typ-order-item__image">
									{#if item.thumbnail}
										<img src={item.thumbnail} alt={item.name} />
									{:else}
										<div class="typ-order-item__placeholder">
											<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="32" height="32">
												<path d="M5.566 4.657A4.505 4.505 0 016.75 4.5h10.5c.41 0 .806.055 1.183.157A3 3 0 0015.75 3h-7.5a3 3 0 00-2.684 1.657zM2.25 12a3 3 0 013-3h13.5a3 3 0 013 3v6a3 3 0 01-3 3H5.25a3 3 0 01-3-3v-6zM5.25 7.5c-.41 0-.806.055-1.184.157A3 3 0 016.75 6h10.5a3 3 0 012.683 1.657A4.505 4.505 0 0018.75 7.5H5.25z" />
											</svg>
										</div>
									{/if}
								</div>
								<div class="typ-order-item__details">
									<h4 class="typ-order-item__name">{item.name}</h4>
									{#if item.product_type}
										<span class="typ-order-item__type">{getProductTypeLabel(item.product_type)}</span>
									{/if}
									<div class="typ-order-item__meta">
										<span>Qty: {item.quantity}</span>
										<span>{formatCurrency(item.unit_price, orderDetail.currency)} each</span>
									</div>
								</div>
								<div class="typ-order-item__actions">
									<span class="typ-order-item__price">
										{formatCurrency(item.total, orderDetail.currency)}
									</span>
									<a href={getAccessUrl(item)} class="typ-order-item__access">
										Access Now
									</a>
								</div>
							</div>
						{/each}
					</div>
				</div>
			</section>
		{:else if fetchError}
			<section class="typ-error-notice">
				<p>{fetchError}</p>
			</section>
		{/if}

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
						<span class="typ-benefit-card__icon">
							<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="40" height="40">
								<path d="M11.7 2.805a.75.75 0 01.6 0A60.65 60.65 0 0122.83 8.72a.75.75 0 01-.231 1.337 49.949 49.949 0 00-9.902 3.912l-.003.002-.34.18a.75.75 0 01-.707 0A50.009 50.009 0 007.5 12.174v-.224c0-.131.067-.248.172-.311a54.614 54.614 0 014.653-2.52.75.75 0 00-.65-1.352 56.129 56.129 0 00-4.78 2.589 1.858 1.858 0 00-.859 1.228 49.803 49.803 0 00-4.634-1.527.75.75 0 01-.231-1.337A60.653 60.653 0 0111.7 2.805z" />
								<path d="M13.06 15.473a48.45 48.45 0 017.666-3.282c.134 1.414.22 2.843.255 4.285a.75.75 0 01-.46.71 47.878 47.878 0 00-8.105 4.342.75.75 0 01-.832 0 47.877 47.877 0 00-8.104-4.342.75.75 0 01-.461-.71c.035-1.442.121-2.87.255-4.286A48.4 48.4 0 016 13.18v1.27a1.5 1.5 0 00-.14 2.508c-.09.38-.222.753-.397 1.11.452.213.901.434 1.346.661a6.729 6.729 0 00.551-1.608 1.5 1.5 0 00.14-2.67v-.645a48.549 48.549 0 013.44 1.668 2.25 2.25 0 002.12 0z" />
								<path d="M4.462 19.462c.42-.419.753-.89 1-1.394.453.213.902.434 1.347.661a6.743 6.743 0 01-1.286 1.794.75.75 0 11-1.06-1.06z" />
							</svg>
						</span>
						<h3 class="typ-benefit-card__title">Learning Center</h3>
						<p class="typ-benefit-card__text">Complete video courses and tutorials</p>
					</div>
				</div>
				<div class="col-sm-6 col-lg-3">
					<div class="typ-benefit-card">
						<span class="typ-benefit-card__icon">
							<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="40" height="40">
								<path fill-rule="evenodd" d="M2.25 6a3 3 0 013-3h13.5a3 3 0 013 3v12a3 3 0 01-3 3H5.25a3 3 0 01-3-3V6zm3.97.97a.75.75 0 011.06 0l2.25 2.25a.75.75 0 010 1.06l-2.25 2.25a.75.75 0 01-1.06-1.06l1.72-1.72-1.72-1.72a.75.75 0 010-1.06zm4.28 4.28a.75.75 0 000 1.5h3a.75.75 0 000-1.5h-3z" clip-rule="evenodd" />
							</svg>
						</span>
						<h3 class="typ-benefit-card__title">Live Trading Room</h3>
						<p class="typ-benefit-card__text">Trade alongside our experts daily</p>
					</div>
				</div>
				<div class="col-sm-6 col-lg-3">
					<div class="typ-benefit-card">
						<span class="typ-benefit-card__icon">
							<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="40" height="40">
								<path fill-rule="evenodd" d="M1.5 5.625c0-1.036.84-1.875 1.875-1.875h17.25c1.035 0 1.875.84 1.875 1.875v12.75c0 1.035-.84 1.875-1.875 1.875H3.375A1.875 1.875 0 011.5 18.375V5.625zm1.5 0v1.5c0 .207.168.375.375.375h1.5a.375.375 0 00.375-.375v-1.5a.375.375 0 00-.375-.375h-1.5A.375.375 0 003 5.625zm16.125-.375a.375.375 0 00-.375.375v1.5c0 .207.168.375.375.375h1.5A.375.375 0 0021 7.125v-1.5a.375.375 0 00-.375-.375h-1.5zM21 9.375A.375.375 0 0020.625 9h-1.5a.375.375 0 00-.375.375v1.5c0 .207.168.375.375.375h1.5a.375.375 0 00.375-.375v-1.5zm0 3.75a.375.375 0 00-.375-.375h-1.5a.375.375 0 00-.375.375v1.5c0 .207.168.375.375.375h1.5a.375.375 0 00.375-.375v-1.5zm0 3.75a.375.375 0 00-.375-.375h-1.5a.375.375 0 00-.375.375v1.5c0 .207.168.375.375.375h1.5a.375.375 0 00.375-.375v-1.5zM4.875 18.75a.375.375 0 00.375-.375v-1.5a.375.375 0 00-.375-.375h-1.5a.375.375 0 00-.375.375v1.5c0 .207.168.375.375.375h1.5zM3.375 15h1.5a.375.375 0 00.375-.375v-1.5a.375.375 0 00-.375-.375h-1.5a.375.375 0 00-.375.375v1.5c0 .207.168.375.375.375zm0-3.75h1.5a.375.375 0 00.375-.375v-1.5A.375.375 0 004.875 9h-1.5A.375.375 0 003 9.375v1.5c0 .207.168.375.375.375zm4.125 0a.75.75 0 000 1.5h9a.75.75 0 000-1.5h-9z" clip-rule="evenodd" />
							</svg>
						</span>
						<h3 class="typ-benefit-card__title">Video Archives</h3>
						<p class="typ-benefit-card__text">Access to all past trading sessions</p>
					</div>
				</div>
				<div class="col-sm-6 col-lg-3">
					<div class="typ-benefit-card">
						<span class="typ-benefit-card__icon">
							<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="40" height="40">
								<path fill-rule="evenodd" d="M8.25 6.75a3.75 3.75 0 117.5 0 3.75 3.75 0 01-7.5 0zM15.75 9.75a3 3 0 116 0 3 3 0 01-6 0zM2.25 9.75a3 3 0 116 0 3 3 0 01-6 0zM6.31 15.117A6.745 6.745 0 0112 12a6.745 6.745 0 016.709 7.498.75.75 0 01-.372.568A12.696 12.696 0 0112 21.75c-2.305 0-4.47-.612-6.337-1.684a.75.75 0 01-.372-.568 6.787 6.787 0 011.019-4.38z" clip-rule="evenodd" />
								<path d="M5.082 14.254a8.287 8.287 0 00-1.308 5.135 9.687 9.687 0 01-1.764-.44l-.115-.04a.563.563 0 01-.373-.487l-.01-.121a3.75 3.75 0 013.57-4.047zM20.226 19.389a8.287 8.287 0 00-1.308-5.135 3.75 3.75 0 013.57 4.047l-.01.121a.563.563 0 01-.373.486l-.115.04c-.567.2-1.156.349-1.764.441z" />
							</svg>
						</span>
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
						<p class="typ-step__text">
							Click the button below to go to your member dashboard and explore your new membership.
						</p>
					</div>
				</div>
				<div class="typ-step">
					<span class="typ-step__number">2</span>
					<div class="typ-step__content">
						<h3 class="typ-step__title">Watch the Getting Started Video</h3>
						<p class="typ-step__text">
							We've prepared a quick orientation video to help you make the most of your membership.
						</p>
					</div>
				</div>
				<div class="typ-step">
					<span class="typ-step__number">3</span>
					<div class="typ-step__content">
						<h3 class="typ-step__title">Join the Trading Room</h3>
						<p class="typ-step__text">
							Our live trading room opens at 9:00 AM ET. Join us to see our strategies in action!
						</p>
					</div>
				</div>
			</div>
			<div class="typ-next-steps__actions">
				<a href="/account" class="btn btn-orange btn-lg"> Go to My Account </a>
				<a href="/courses" class="btn btn-outline-light btn-lg"> Browse Courses </a>
			</div>
		</section>

		<!-- Upsell Section -->
		<section class="typ-upsell">
			<div class="typ-upsell__header">
				<h2 class="typ-upsell__title">Exclusive Member Offers</h2>
				<p class="typ-upsell__subtitle">
					Special pricing available only for new members - limited time!
				</p>
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
					<a href="/support" class="btn btn-outline-orange">Contact Support</a>
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
		color: var(--st-color-green, #22c55e);
		margin-bottom: 1.5rem;
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
		margin-bottom: 1rem;
	}

	.typ-header__badge {
		display: inline-block;
		padding: 0.375rem 1rem;
		border-radius: 9999px;
		font-size: 0.875rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.badge-success {
		background: rgba(34, 197, 94, 0.2);
		color: #22c55e;
	}

	.badge-warning {
		background: rgba(245, 158, 11, 0.2);
		color: #f59e0b;
	}

	.badge-info {
		background: rgba(59, 130, 246, 0.2);
		color: #3b82f6;
	}

	.badge-error {
		background: rgba(239, 68, 68, 0.2);
		color: #ef4444;
	}

	/* Order Details Section */
	.typ-order-details {
		background: var(--st-color-gray-800, #1f2937);
		border-radius: 1rem;
		padding: 2rem;
		margin-bottom: 3rem;
	}

	.typ-order-details__title {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--st-color-white, #fff);
		margin-bottom: 1.5rem;
	}

	.typ-order-details__grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 2rem;
		margin-bottom: 2rem;
	}

	/* Order Info */
	.typ-order-info {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.typ-order-info__row {
		display: flex;
		justify-content: space-between;
		padding: 0.5rem 0;
		border-bottom: 1px solid var(--st-color-gray-700, #374151);
	}

	.typ-order-info__label {
		color: var(--st-color-gray-400, #9ca3af);
		font-size: 0.875rem;
	}

	.typ-order-info__value {
		color: var(--st-color-white, #fff);
		font-weight: 500;
	}

	.typ-order-info__value--capitalize {
		text-transform: capitalize;
	}

	/* Order Summary */
	.typ-order-summary {
		background: var(--st-color-gray-700, #374151);
		border-radius: 0.75rem;
		padding: 1.5rem;
	}

	.typ-order-summary__title {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--st-color-white, #fff);
		margin-bottom: 1rem;
	}

	.typ-order-summary__row {
		display: flex;
		justify-content: space-between;
		padding: 0.5rem 0;
		color: var(--st-color-gray-300, #d1d5db);
		font-size: 0.9375rem;
	}

	.typ-order-summary__row--discount {
		color: var(--st-color-green, #22c55e);
	}

	.typ-order-summary__row--total {
		border-top: 1px solid var(--st-color-gray-600, #4b5563);
		margin-top: 0.5rem;
		padding-top: 1rem;
		font-size: 1.125rem;
		font-weight: 700;
		color: var(--st-color-white, #fff);
	}

	/* Order Items */
	.typ-order-items {
		border-top: 1px solid var(--st-color-gray-700, #374151);
		padding-top: 2rem;
	}

	.typ-order-items__title {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--st-color-white, #fff);
		margin-bottom: 1rem;
	}

	.typ-order-items__list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.typ-order-item {
		display: flex;
		gap: 1rem;
		padding: 1rem;
		background: var(--st-color-gray-700, #374151);
		border-radius: 0.75rem;
		align-items: center;
	}

	.typ-order-item__image {
		width: 80px;
		height: 80px;
		flex-shrink: 0;
		border-radius: 0.5rem;
		overflow: hidden;
		background: var(--st-color-gray-600, #4b5563);
	}

	.typ-order-item__image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.typ-order-item__placeholder {
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--st-color-gray-400, #9ca3af);
	}

	.typ-order-item__details {
		flex: 1;
	}

	.typ-order-item__name {
		font-size: 1rem;
		font-weight: 600;
		color: var(--st-color-white, #fff);
		margin: 0 0 0.25rem;
	}

	.typ-order-item__type {
		display: inline-block;
		padding: 0.125rem 0.5rem;
		background: var(--st-color-orange, #f97316);
		color: var(--st-color-white, #fff);
		font-size: 0.75rem;
		font-weight: 600;
		border-radius: 9999px;
		margin-bottom: 0.5rem;
	}

	.typ-order-item__meta {
		display: flex;
		gap: 1rem;
		font-size: 0.875rem;
		color: var(--st-color-gray-400, #9ca3af);
	}

	.typ-order-item__actions {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		gap: 0.5rem;
	}

	.typ-order-item__price {
		font-size: 1.125rem;
		font-weight: 700;
		color: var(--st-color-white, #fff);
	}

	.typ-order-item__access {
		font-size: 0.875rem;
		color: var(--st-color-orange, #f97316);
		text-decoration: none;
		font-weight: 500;
	}

	.typ-order-item__access:hover {
		text-decoration: underline;
	}

	/* Error Notice */
	.typ-error-notice {
		background: rgba(245, 158, 11, 0.1);
		border: 1px solid rgba(245, 158, 11, 0.3);
		border-radius: 0.75rem;
		padding: 1rem 1.5rem;
		margin-bottom: 2rem;
		text-align: center;
	}

	.typ-error-notice p {
		color: #f59e0b;
		margin: 0;
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
		transition:
			transform 0.2s ease,
			box-shadow 0.2s ease;
	}

	.typ-benefit-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 20px 40px rgba(0, 0, 0, 0.3);
	}

	.typ-benefit-card__icon {
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

		.typ-order-details__grid {
			grid-template-columns: 1fr;
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

		.col-sm-6,
		.col-lg-3,
		.col-md-6 {
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

		.typ-order-item {
			flex-direction: column;
			text-align: center;
		}

		.typ-order-item__actions {
			align-items: center;
			width: 100%;
		}
	}
</style>

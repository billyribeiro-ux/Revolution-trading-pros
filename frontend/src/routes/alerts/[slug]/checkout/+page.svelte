<script lang="ts">
	/**
	 * Dynamic Room Checkout Page
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Apple ICT 7+ Principal Engineer Grade - January 2026
	 *
	 * Fetches subscription plans for a trading room and allows selection of
	 * monthly/quarterly/annual variants before proceeding to payment checkout.
	 *
	 * URL: /alerts/{room-slug}/checkout
	 * Example: /alerts/explosive-swings/checkout
	 */

	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { getPlansByRoom, planToCartItem, type SubscriptionPlan } from '$lib/api/plans';
	import { cartStore } from '$lib/stores/cart.svelte';
	import { isAuthenticated } from '$lib/stores/auth.svelte';
	import PricingSelector from '$lib/components/checkout/PricingSelector.svelte';

	// Get room slug from URL
	const roomSlug = page.params.slug;

	// State
	let loading = $state(true);
	let error = $state('');
	let roomName = $state('');
	let plans = $state<SubscriptionPlan[]>([]);
	let selectedPlanId = $state<number | undefined>(undefined);
	let selectedPlan = $derived(plans.find((p) => p.id === selectedPlanId));

	onMount(async () => {
		await loadPlans();
	});

	async function loadPlans() {
		loading = true;
		error = '';

		try {
			const response = await getPlansByRoom(roomSlug);
			plans = response.plans;
			roomName = response.room_name;

			// Auto-select the popular/quarterly plan
			const popular = plans.find((p) => p.is_popular);
			const quarterly = plans.find((p) => p.billing_cycle === 'quarterly');
			selectedPlanId = popular?.id || quarterly?.id || plans[0]?.id;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load pricing';
			console.error('[Checkout] Load error:', err);
		} finally {
			loading = false;
		}
	}

	function handlePlanSelect(plan: SubscriptionPlan) {
		selectedPlanId = plan.id;
	}

	function proceedToCheckout() {
		if (!selectedPlan) return;

		// Check if plan has Stripe Price ID
		if (!selectedPlan.stripe_price_id) {
			error = 'This plan is not yet available for purchase. Please contact support.';
			return;
		}

		// Clear cart and add selected plan
		cartStore.clearCart();
		const cartItem = planToCartItem(selectedPlan);
		cartStore.addItem({
			...cartItem,
			quantity: 1
		});

		// Check authentication
		if (!$isAuthenticated) {
			goto(`/login?redirect=/checkout`);
			return;
		}

		// Go to main checkout
		goto('/checkout');
	}

	function formatPrice(price: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0
		}).format(price);
	}
</script>

<svelte:head>
	<title>Subscribe to {roomName || 'Trading Room'} | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<main class="checkout-page">
	<div class="checkout-container">
		<!-- Header -->
		<header class="checkout-header">
			<a href="/alerts/{roomSlug}" class="back-link">
				<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M15 19l-7-7 7-7"
					/>
				</svg>
				Back to {roomName || 'Room'}
			</a>

			<h1>Choose Your Plan</h1>
			<p class="subtitle">Select a subscription for {roomName || 'this trading room'}</p>
		</header>

		<!-- Error Banner -->
		{#if error}
			<div class="error-banner">
				<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
					<path
						fill-rule="evenodd"
						d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
						clip-rule="evenodd"
					/>
				</svg>
				<span>{error}</span>
				<button onclick={() => (error = '')} class="dismiss">×</button>
			</div>
		{/if}

		<!-- Pricing Section -->
		<section class="pricing-section">
			<PricingSelector
				{plans}
				bind:selectedPlanId
				onSelect={handlePlanSelect}
				{loading}
			/>
		</section>

		<!-- Selected Plan Summary -->
		{#if selectedPlan && !loading}
			<section class="summary-section">
				<div class="summary-card">
					<h3>Order Summary</h3>

					<div class="summary-row">
						<span class="label">{selectedPlan.display_name || selectedPlan.name}</span>
						<span class="value">{formatPrice(selectedPlan.price)}</span>
					</div>

					{#if selectedPlan.trial_days && selectedPlan.trial_days > 0}
						<div class="trial-notice">
							<svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
								<path
									fill-rule="evenodd"
									d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
									clip-rule="evenodd"
								/>
							</svg>
							Includes {selectedPlan.trial_days}-day free trial
						</div>
					{/if}

					<div class="summary-total">
						<span>Total Today</span>
						<span class="total-price">{formatPrice(selectedPlan.price)}</span>
					</div>

					<button class="checkout-button" onclick={proceedToCheckout} disabled={!selectedPlan}>
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
							/>
						</svg>
						Continue to Checkout
					</button>

					<p class="secure-notice">
						<svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
							<path
								fill-rule="evenodd"
								d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z"
								clip-rule="evenodd"
							/>
						</svg>
						Secure checkout powered by Stripe
					</p>
				</div>
			</section>
		{/if}

		<!-- Features -->
		<section class="features-section">
			<h3>What's Included</h3>
			<ul class="features-list">
				<li>
					<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
						<path
							fill-rule="evenodd"
							d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
							clip-rule="evenodd"
						/>
					</svg>
					Full access to all trading alerts
				</li>
				<li>
					<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
						<path
							fill-rule="evenodd"
							d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
							clip-rule="evenodd"
						/>
					</svg>
					Real-time SMS & email notifications
				</li>
				<li>
					<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
						<path
							fill-rule="evenodd"
							d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
							clip-rule="evenodd"
						/>
					</svg>
					Private community access
				</li>
				<li>
					<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
						<path
							fill-rule="evenodd"
							d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
							clip-rule="evenodd"
						/>
					</svg>
					Educational resources & strategy guides
				</li>
				<li>
					<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
						<path
							fill-rule="evenodd"
							d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
							clip-rule="evenodd"
						/>
					</svg>
					Cancel anytime - no long-term commitment
				</li>
			</ul>
		</section>

		<!-- Guarantee -->
		<section class="guarantee-section">
			<div class="guarantee-badge">
				<svg class="w-8 h-8" fill="currentColor" viewBox="0 0 20 20">
					<path
						fill-rule="evenodd"
						d="M2.166 4.999A11.954 11.954 0 0010 1.944 11.954 11.954 0 0017.834 5c.11.65.166 1.32.166 2.001 0 5.225-3.34 9.67-8 11.317C5.34 16.67 2 12.225 2 7c0-.682.057-1.35.166-2.001zm11.541 3.708a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
						clip-rule="evenodd"
					/>
				</svg>
			</div>
			<div class="guarantee-content">
				<h4>30-Day Money-Back Guarantee</h4>
				<p>Not satisfied? Get a full refund within 30 days, no questions asked.</p>
			</div>
		</section>
	</div>
</main>

<style>
	.checkout-page {
		min-height: 100vh;
		background: linear-gradient(180deg, #f8fafc 0%, #f1f5f9 100%);
		padding: 2rem 1rem 4rem;
	}

	.checkout-container {
		max-width: 600px;
		margin: 0 auto;
	}

	/* Header */
	.checkout-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		color: #0984ae;
		font-size: 0.875rem;
		font-weight: 600;
		text-decoration: none;
		margin-bottom: 1.5rem;
		transition: color 0.2s;
	}

	.back-link:hover {
		color: #076787;
	}

	.checkout-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #1e293b;
		margin: 0 0 0.5rem;
	}

	.subtitle {
		color: #64748b;
		font-size: 1rem;
		margin: 0;
	}

	/* Error */
	.error-banner {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 10px;
		color: #dc2626;
		font-size: 0.9rem;
		margin-bottom: 1.5rem;
	}

	.error-banner .dismiss {
		margin-left: auto;
		background: none;
		border: none;
		color: inherit;
		font-size: 1.25rem;
		cursor: pointer;
		padding: 0 0.25rem;
	}

	/* Pricing */
	.pricing-section {
		margin-bottom: 1.5rem;
	}

	/* Summary */
	.summary-section {
		margin-bottom: 2rem;
	}

	.summary-card {
		background: white;
		border-radius: 16px;
		padding: 1.5rem;
		box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
	}

	.summary-card h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #374151;
		margin: 0 0 1rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.summary-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 0;
	}

	.summary-row .label {
		color: #4b5563;
	}

	.summary-row .value {
		font-weight: 600;
		color: #1e293b;
	}

	.trial-notice {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background: #ecfdf5;
		border-radius: 8px;
		color: #059669;
		font-size: 0.875rem;
		font-weight: 500;
		margin: 0.5rem 0;
	}

	.summary-total {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 0;
		margin-top: 0.5rem;
		border-top: 2px solid #e5e7eb;
		font-weight: 600;
	}

	.total-price {
		font-size: 1.5rem;
		color: #1e293b;
	}

	.checkout-button {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 1rem 1.5rem;
		background: linear-gradient(135deg, #0984ae, #076787);
		color: white;
		font-size: 1rem;
		font-weight: 600;
		border: none;
		border-radius: 10px;
		cursor: pointer;
		transition: all 0.2s;
		margin-top: 1rem;
	}

	.checkout-button:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(9, 132, 174, 0.4);
	}

	.checkout-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.secure-notice {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		margin-top: 1rem;
		color: #64748b;
		font-size: 0.8rem;
	}

	/* Features */
	.features-section {
		background: white;
		border-radius: 16px;
		padding: 1.5rem;
		margin-bottom: 1.5rem;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.features-section h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #374151;
		margin: 0 0 1rem;
	}

	.features-list {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.features-list li {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		color: #4b5563;
		font-size: 0.9rem;
	}

	.features-list svg {
		color: #10b981;
		flex-shrink: 0;
	}

	/* Guarantee */
	.guarantee-section {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: linear-gradient(135deg, rgba(16, 185, 129, 0.1), rgba(16, 185, 129, 0.05));
		border: 1px solid rgba(16, 185, 129, 0.2);
		border-radius: 12px;
	}

	.guarantee-badge {
		width: 48px;
		height: 48px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #10b981;
		border-radius: 50%;
		color: white;
		flex-shrink: 0;
	}

	.guarantee-content h4 {
		font-size: 0.9rem;
		font-weight: 600;
		color: #065f46;
		margin: 0 0 0.25rem;
	}

	.guarantee-content p {
		font-size: 0.8rem;
		color: #047857;
		margin: 0;
		line-height: 1.4;
	}

	/* Responsive */
	@media (max-width: 480px) {
		.checkout-header h1 {
			font-size: 1.5rem;
		}

		.summary-card,
		.features-section {
			padding: 1.25rem;
		}
	}
</style>

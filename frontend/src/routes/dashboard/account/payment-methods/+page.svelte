<script lang="ts">
	/**
	 * Payment Methods - Account Section
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Apple ICT 11+ Principal Engineer Grade - December 2025
	 * Stripe Customer Portal integration for payment management
	 *
	 * @version 3.0.0 - Production Ready
	 */

	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { authStore } from '$lib/stores/auth';

	// API Configuration
	const isDev = import.meta.env.DEV;
	const PRODUCTION_API_URL = 'https://revolution-trading-pros-api.fly.dev/api';
	const API_BASE = browser
		? isDev
			? '/api'
			: (import.meta.env['VITE_API_URL'] || PRODUCTION_API_URL)
		: '/api';

	// State
	let isLoading = $state(true);
	let error = $state<string | null>(null);
	let hasSubscription = $state(false);
	let isRedirecting = $state(false);

	// Check if user has any active subscriptions (required for Stripe portal)
	async function checkSubscriptionStatus(): Promise<void> {
		if (!browser) return;

		isLoading = true;
		error = null;

		try {
			const token = authStore.getToken();

			const response = await fetch(`${API_BASE}/subscriptions/my/active`, {
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					Accept: 'application/json',
					...(token ? { Authorization: `Bearer ${token}` } : {})
				},
				credentials: 'include'
			});

			if (response.ok) {
				const data = await response.json();
				hasSubscription = data.subscription !== null || data.has_subscription === true;
			} else if (response.status === 401) {
				error = 'Please log in to manage payment methods';
			} else {
				// No subscription found, but that's okay
				hasSubscription = false;
			}
		} catch (e) {
			console.error('Error checking subscription status:', e);
			hasSubscription = false;
		} finally {
			isLoading = false;
		}
	}

	// Open Stripe Customer Portal
	async function openStripePortal(): Promise<void> {
		if (!browser) return;

		isRedirecting = true;
		error = null;

		try {
			const token = authStore.getToken();
			const returnUrl = `${window.location.origin}/dashboard/account/payment-methods`;

			const response = await fetch(`${API_BASE}/api/payments/portal`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Accept: 'application/json',
					...(token ? { Authorization: `Bearer ${token}` } : {})
				},
				credentials: 'include',
				body: JSON.stringify({ return_url: returnUrl })
			});

			if (!response.ok) {
				const err = await response.json();
				throw new Error(err.error || 'Failed to open payment portal');
			}

			const data = await response.json();

			if (data.url) {
				window.location.href = data.url;
			} else {
				throw new Error('No portal URL returned');
			}
		} catch (e) {
			console.error('Error opening Stripe portal:', e);
			error = e instanceof Error ? e.message : 'Failed to open payment portal';
			isRedirecting = false;
		}
	}

	onMount(() => {
		checkSubscriptionStatus();
	});
</script>

<svelte:head>
	<title>Payment Methods - Account | Revolution Trading Pros</title>
</svelte:head>

<!-- Payment Methods Page - Stripe Portal Integration -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">Payment Methods</h2>

			{#if isLoading}
				<div class="loading-state">
					<div class="spinner"></div>
					<p>Loading...</p>
				</div>
			{:else if error}
				<div class="error-state">
					<p>{error}</p>
					<button class="btn btn-primary" onclick={() => checkSubscriptionStatus()}>
						Try Again
					</button>
				</div>
			{:else if hasSubscription}
				<div class="payment-portal-section">
					<div class="portal-info">
						<div class="portal-icon">💳</div>
						<h3>Manage Your Payment Methods</h3>
						<p>
							Use the secure Stripe Customer Portal to add, update, or remove payment methods.
							You can also view your billing history and download invoices.
						</p>
					</div>

					<div class="portal-features">
						<div class="feature">
							<span class="feature-icon">✓</span>
							<span>Add new credit/debit cards</span>
						</div>
						<div class="feature">
							<span class="feature-icon">✓</span>
							<span>Update card expiration dates</span>
						</div>
						<div class="feature">
							<span class="feature-icon">✓</span>
							<span>Set default payment method</span>
						</div>
						<div class="feature">
							<span class="feature-icon">✓</span>
							<span>View billing history</span>
						</div>
						<div class="feature">
							<span class="feature-icon">✓</span>
							<span>Download invoices</span>
						</div>
					</div>

					<button
						class="btn btn-primary btn-large"
						onclick={openStripePortal}
						disabled={isRedirecting}
					>
						{#if isRedirecting}
							<span class="btn-spinner"></span>
							Opening Stripe Portal...
						{:else}
							Open Payment Portal
						{/if}
					</button>

					<p class="portal-note">
						You will be redirected to Stripe's secure portal. After making changes, you'll be returned here.
					</p>
				</div>
			{:else}
				<div class="no-subscription-state">
					<div class="empty-icon">💳</div>
					<h3>No Active Subscriptions</h3>
					<p>
						Payment methods are managed through your active subscriptions.
						Once you subscribe to a plan, you can manage your payment methods here.
					</p>
					<a href="/pricing" class="btn btn-primary">
						Browse Membership Plans
					</a>
				</div>
			{/if}
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   Apple ICT 11+ Grade Styles - Production Ready
	   ═══════════════════════════════════════════════════════════════════════════ */

	.section-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
	}

	/* Loading State */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		color: #666;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e0e0e0;
		border-top-color: #0984ae;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		margin-bottom: 16px;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Error State */
	.error-state {
		text-align: center;
		padding: 40px 20px;
		color: #d32f2f;
		background: #ffebee;
		border-radius: 8px;
	}

	.error-state p {
		margin-bottom: 16px;
	}

	/* No Subscription State */
	.no-subscription-state {
		text-align: center;
		padding: 60px 20px;
		background: #f9f9f9;
		border-radius: 8px;
	}

	.empty-icon {
		font-size: 48px;
		margin-bottom: 16px;
	}

	.no-subscription-state h3 {
		font-size: 20px;
		font-weight: 600;
		color: #333;
		margin: 0 0 12px;
	}

	.no-subscription-state p {
		color: #666;
		margin-bottom: 24px;
		max-width: 400px;
		margin-left: auto;
		margin-right: auto;
	}

	/* Payment Portal Section */
	.payment-portal-section {
		background: #fff;
		border-radius: 8px;
		padding: 40px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		text-align: center;
	}

	.portal-info {
		margin-bottom: 32px;
	}

	.portal-icon {
		font-size: 48px;
		margin-bottom: 16px;
	}

	.portal-info h3 {
		font-size: 22px;
		font-weight: 700;
		color: #333;
		margin: 0 0 12px;
	}

	.portal-info p {
		color: #666;
		font-size: 15px;
		max-width: 500px;
		margin: 0 auto;
		line-height: 1.6;
	}

	/* Features List */
	.portal-features {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 16px;
		margin-bottom: 32px;
	}

	.feature {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 14px;
		color: #444;
		background: #f5f5f5;
		padding: 8px 16px;
		border-radius: 20px;
	}

	.feature-icon {
		color: #28a745;
		font-weight: bold;
	}

	/* Buttons */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 12px 24px;
		font-size: 15px;
		font-weight: 600;
		border-radius: 6px;
		border: none;
		cursor: pointer;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.btn-primary {
		background: #0984ae;
		color: #fff;
	}

	.btn-primary:hover:not(:disabled) {
		background: #077a9e;
	}

	.btn-primary:disabled {
		background: #85c4d6;
		cursor: not-allowed;
	}

	.btn-large {
		padding: 16px 40px;
		font-size: 16px;
	}

	.btn-spinner {
		width: 18px;
		height: 18px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: #fff;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	.portal-note {
		margin-top: 20px;
		font-size: 13px;
		color: #888;
	}

	@media (max-width: 768px) {
		.payment-portal-section {
			padding: 30px 20px;
		}

		.portal-features {
			flex-direction: column;
			align-items: center;
		}

		.feature {
			width: 100%;
			max-width: 280px;
			justify-content: center;
		}
	}
</style>

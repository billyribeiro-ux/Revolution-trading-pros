<script lang="ts">
	/**
	 * Dashboard - Payment Methods Page - Simpler Trading EXACT
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /dashboard/account/payment-methods
	 * Shows saved payment methods with ability to add/delete
	 *
	 * @version 1.0.0 (Simpler Trading Exact / December 2025)
	 */

	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { authStore, isAuthenticated } from '$lib/stores/auth';
	import Footer from '$lib/components/sections/Footer.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (browser && !$isAuthenticated && !$authStore.isInitializing) {
			goto('/login?redirect=/dashboard/account/payment-methods', { replaceState: true });
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// DATA (would come from API)
	// ═══════════════════════════════════════════════════════════════════════════

	interface PaymentMethod {
		id: string;
		type: 'visa' | 'mastercard' | 'amex' | 'discover' | 'paypal';
		lastFour: string;
		details?: string;
		expiresMonth: string;
		expiresYear: string;
		isDefault: boolean;
		subscriptions?: string[];
	}

	// Sample payment methods data
	const paymentMethods: PaymentMethod[] = [
		{
			id: 'pm_1',
			type: 'visa',
			lastFour: '9396',
			expiresMonth: '09',
			expiresYear: '29',
			isDefault: true
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function getMethodLabel(method: PaymentMethod): string {
		const typeLabels: Record<string, string> = {
			visa: 'Visa',
			mastercard: 'Mastercard',
			amex: 'American Express',
			discover: 'Discover',
			paypal: 'PayPal'
		};
		return `${typeLabels[method.type] || method.type} ending in ${method.lastFour}`;
	}

	function formatExpiry(method: PaymentMethod): string {
		return `${method.expiresMonth}/${method.expiresYear}`;
	}

	function handleDelete(methodId: string): void {
		// TODO: Implement delete functionality
		console.log('Delete payment method:', methodId);
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>Payment Methods | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEADER
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="dashboard__header">
	<h1 class="dashboard__page-title">My Account</h1>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     MAIN CONTENT
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Add Payment Method Link -->
		<div class="add-method-wrapper">
			<a href="/dashboard/account/add-payment-method" class="add-method-link">
				Add payment method
			</a>
		</div>

		<!-- Payment Methods Table -->
		<div class="payment-methods-table-wrapper">
			<table class="payment-methods-table">
				<thead>
					<tr>
						<th>Method</th>
						<th>Details</th>
						<th>Expires</th>
						<th>Default?</th>
						<th>Subscriptions</th>
						<th></th>
					</tr>
				</thead>
				<tbody>
					{#each paymentMethods as method (method.id)}
						<tr>
							<td class="col-method">{getMethodLabel(method)}</td>
							<td class="col-details">{method.details || ''}</td>
							<td class="col-expires">{formatExpiry(method)}</td>
							<td class="col-default">
								{#if method.isDefault}
									<span class="default-badge">DEFAULT</span>
								{/if}
							</td>
							<td class="col-subscriptions">
								{#if method.subscriptions && method.subscriptions.length > 0}
									{method.subscriptions.join(', ')}
								{/if}
							</td>
							<td class="col-actions">
								<button
									type="button"
									class="btn-delete"
									onclick={() => handleDelete(method.id)}
								>
									Delete
								</button>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>

		<!-- Empty State -->
		{#if paymentMethods.length === 0}
			<div class="empty-state">
				<p>No payment methods saved.</p>
			</div>
		{/if}
	</div>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     FOOTER
     ═══════════════════════════════════════════════════════════════════════════ -->

<Footer />

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - Simpler Trading EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background: #fff;
		border-bottom: 1px solid #e9ebed;
		padding: 20px 30px;
	}

	.dashboard__page-title {
		color: #333;
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
		line-height: 1.2;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		padding: 30px;
		background: #fff;
		min-height: 400px;
	}

	.dashboard__content-main {
		max-width: 100%;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ADD PAYMENT METHOD LINK
	   ═══════════════════════════════════════════════════════════════════════════ */

	.add-method-wrapper {
		margin-bottom: 20px;
	}

	.add-method-link {
		color: #1e73be;
		font-size: 14px;
		text-decoration: none;
	}

	.add-method-link:hover {
		text-decoration: underline;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PAYMENT METHODS TABLE - Simpler Trading EXACT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.payment-methods-table-wrapper {
		border: 1px solid #e9ebed;
		border-radius: 4px;
		overflow: hidden;
	}

	.payment-methods-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 14px;
	}

	.payment-methods-table thead {
		background: #f8f9fa;
	}

	.payment-methods-table th {
		padding: 12px 16px;
		text-align: left;
		font-weight: 600;
		color: #333;
		border-bottom: 1px solid #e9ebed;
	}

	.payment-methods-table td {
		padding: 16px;
		border-bottom: 1px solid #e9ebed;
		color: #333;
		vertical-align: middle;
	}

	.payment-methods-table tr:last-child td {
		border-bottom: none;
	}

	/* Column styles */
	.col-method {
		width: 25%;
	}

	.col-details {
		width: 15%;
	}

	.col-expires {
		width: 12%;
	}

	.col-default {
		width: 12%;
	}

	.col-subscriptions {
		width: 20%;
	}

	.col-actions {
		width: 16%;
		text-align: right;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DEFAULT BADGE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.default-badge {
		display: inline-block;
		padding: 3px 8px;
		font-size: 11px;
		font-weight: 600;
		color: #333;
		background: #e9ebed;
		border: 1px solid #ddd;
		border-radius: 3px;
		text-transform: uppercase;
		letter-spacing: 0.3px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DELETE BUTTON
	   ═══════════════════════════════════════════════════════════════════════════ */

	.btn-delete {
		display: inline-block;
		padding: 6px 14px;
		background: #fff;
		border: 1px solid #ddd;
		color: #333;
		font-size: 12px;
		font-weight: 600;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.btn-delete:hover {
		background: #f5f5f5;
		border-color: #ccc;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   EMPTY STATE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.empty-state {
		padding: 40px;
		text-align: center;
		color: #666;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 1024px) {
		.payment-methods-table-wrapper {
			overflow-x: auto;
		}

		.payment-methods-table {
			min-width: 650px;
		}
	}

	@media screen and (max-width: 768px) {
		.dashboard__header {
			padding: 16px 20px;
		}

		.dashboard__page-title {
			font-size: 28px;
		}

		.dashboard__content {
			padding: 20px;
		}

		.payment-methods-table th,
		.payment-methods-table td {
			padding: 12px;
		}
	}

	@media screen and (max-width: 480px) {
		.payment-methods-table {
			font-size: 13px;
		}

		.payment-methods-table th,
		.payment-methods-table td {
			padding: 10px 8px;
		}
	}
</style>

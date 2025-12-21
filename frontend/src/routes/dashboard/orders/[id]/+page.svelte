<script lang="ts">
	/**
	 * Dashboard - View Order Page - Simpler Trading EXACT
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /dashboard/orders/[id]
	 * Shows detailed order information including products, totals, and customer info
	 *
	 * @version 1.0.0 (Simpler Trading Exact / December 2025)
	 */

	import { page } from '$app/stores';
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { authStore, isAuthenticated } from '$lib/stores/auth';
	import { IconPhone, IconMail } from '$lib/icons';
	import Footer from '$lib/components/sections/Footer.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// ROUTE PARAMS
	// ═══════════════════════════════════════════════════════════════════════════

	const orderId = $derived($page.params.id);

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (browser && !$isAuthenticated && !$authStore.isInitializing) {
			goto('/login?redirect=/dashboard/orders', { replaceState: true });
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// DATA (would come from API based on orderId)
	// ═══════════════════════════════════════════════════════════════════════════

	interface OrderProduct {
		name: string;
		quantity: number;
		total: number;
	}

	interface OrderSubscription {
		id: string;
		status: string;
		nextPayment: string | null;
		total: number;
		viewUrl: string;
	}

	interface CustomerInfo {
		name: string;
		address: string[];
		phone: string;
		email: string;
	}

	interface OrderDetails {
		id: string;
		orderNumber: string;
		date: string;
		status: string;
		products: OrderProduct[];
		subtotal: number;
		discount: number;
		tax: number;
		paymentMethod: string;
		total: number;
		subscriptions: OrderSubscription[];
		customer: CustomerInfo;
	}

	// Sample order data (would come from API)
	const order: OrderDetails = {
		id: '2176654',
		orderNumber: '#2176654',
		date: 'December 3, 2025',
		status: 'Completed',
		products: [
			{ name: 'Mastering the Trade Room (1 Month Trial)', quantity: 1, total: 247.00 }
		],
		subtotal: 247.00,
		discount: -240.00,
		tax: 0.00,
		paymentMethod: 'Credit Card (Stripe)',
		total: 7.00,
		subscriptions: [
			{
				id: '2176655',
				status: 'Pending Cancellation',
				nextPayment: null,
				total: 197.00,
				viewUrl: '/dashboard/subscriptions/2176655'
			}
		],
		customer: {
			name: 'Zack Stambowski',
			address: ['2417 S KIHEI RD', 'KIHEI, HI 96753-8624'],
			phone: '801-721-0940',
			email: 'welberribeirodrums@gmail.com'
		}
	};

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function formatCurrency(amount: number): string {
		const prefix = amount < 0 ? '-' : '';
		const absAmount = Math.abs(amount);
		return `${prefix}$${absAmount.toFixed(2)}`;
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>Order {order.orderNumber} | Revolution Trading Pros</title>
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
		<!-- Order Summary -->
		<div class="order-summary">
			<p>
				Order # <a href="/dashboard/orders/{order.id}" class="order-link">{order.id}</a>
				was placed on
				<a href="/dashboard/orders/{order.id}" class="order-link">{order.date}</a>
				and is currently
				<span class="order-status order-status--{order.status.toLowerCase()}">{order.status}</span>.
			</p>
		</div>

		<!-- Products Table -->
		<div class="order-table-wrapper">
			<table class="order-table">
				<thead>
					<tr>
						<th class="col-product">Product</th>
						<th class="col-total">Total</th>
					</tr>
				</thead>
				<tbody>
					{#each order.products as product}
						<tr>
							<td class="col-product">{product.name} &times; {product.quantity}</td>
							<td class="col-total">{formatCurrency(product.total)}</td>
						</tr>
					{/each}
					<tr class="summary-row">
						<td class="col-product"><strong>Subtotal:</strong></td>
						<td class="col-total">{formatCurrency(order.subtotal)}</td>
					</tr>
					<tr class="summary-row">
						<td class="col-product"><strong>Discount:</strong></td>
						<td class="col-total">{formatCurrency(order.discount)}</td>
					</tr>
					<tr class="summary-row">
						<td class="col-product"><strong>Tax:</strong></td>
						<td class="col-total">{formatCurrency(order.tax)}</td>
					</tr>
					<tr class="summary-row">
						<td class="col-product"><strong>Payment method:</strong></td>
						<td class="col-total">{order.paymentMethod}</td>
					</tr>
					<tr class="summary-row total-row">
						<td class="col-product"><strong>Total:</strong></td>
						<td class="col-total"><strong>{formatCurrency(order.total)}</strong></td>
					</tr>
				</tbody>
			</table>
		</div>

		<!-- Subscriptions Table -->
		{#if order.subscriptions.length > 0}
			<div class="order-table-wrapper">
				<table class="order-table subscriptions-table">
					<thead>
						<tr>
							<th>Subscription</th>
							<th>Status</th>
							<th>Next payment</th>
							<th>Total</th>
							<th></th>
						</tr>
					</thead>
					<tbody>
						{#each order.subscriptions as sub}
							<tr>
								<td>
									<a href={sub.viewUrl} class="order-link">#{sub.id}</a>
								</td>
								<td>{sub.status}</td>
								<td>{sub.nextPayment || '-'}</td>
								<td>{formatCurrency(sub.total)}</td>
								<td>
									<a href={sub.viewUrl} class="btn-view">View</a>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}

		<!-- Customer Details -->
		<div class="customer-details">
			<p class="customer-name">{order.customer.name}</p>
			{#each order.customer.address as line}
				<p class="customer-address">{line}</p>
			{/each}
			<p class="customer-contact">
				<IconPhone size={14} />
				<span>{order.customer.phone}</span>
			</p>
			<p class="customer-contact">
				<IconMail size={14} />
				<span>{order.customer.email}</span>
			</p>
		</div>
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
		max-width: 800px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ORDER SUMMARY
	   ═══════════════════════════════════════════════════════════════════════════ */

	.order-summary {
		margin-bottom: 24px;
		padding: 16px 20px;
		background: #f8f9fa;
		border: 1px solid #e9ebed;
		border-radius: 4px;
	}

	.order-summary p {
		margin: 0;
		font-size: 14px;
		color: #333;
	}

	.order-link {
		color: #1e73be;
		text-decoration: none;
	}

	.order-link:hover {
		text-decoration: underline;
	}

	.order-status {
		font-weight: 600;
	}

	.order-status--completed {
		color: #166534;
	}

	.order-status--processing {
		color: #92400e;
	}

	.order-status--pending {
		color: #3730a3;
	}

	.order-status--cancelled,
	.order-status--refunded {
		color: #991b1b;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ORDER TABLE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.order-table-wrapper {
		margin-bottom: 24px;
		border: 1px solid #e9ebed;
		border-radius: 4px;
		overflow: hidden;
	}

	.order-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 14px;
	}

	.order-table thead {
		background: #f8f9fa;
	}

	.order-table th {
		padding: 12px 16px;
		text-align: left;
		font-weight: 600;
		color: #333;
		border-bottom: 1px solid #e9ebed;
	}

	.order-table td {
		padding: 12px 16px;
		border-bottom: 1px solid #e9ebed;
		color: #333;
	}

	.order-table tr:last-child td {
		border-bottom: none;
	}

	.col-product {
		width: 70%;
	}

	.col-total {
		width: 30%;
		text-align: right !important;
	}

	.order-table th.col-total {
		text-align: right;
	}

	.summary-row td {
		background: #fafafa;
	}

	.total-row td {
		background: #f0f0f0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SUBSCRIPTIONS TABLE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.subscriptions-table th,
	.subscriptions-table td {
		text-align: left;
	}

	.subscriptions-table th:last-child,
	.subscriptions-table td:last-child {
		text-align: right;
		width: 80px;
	}

	.btn-view {
		display: inline-block;
		padding: 6px 12px;
		background: #6b7280;
		color: #fff;
		font-size: 12px;
		font-weight: 600;
		border-radius: 4px;
		text-decoration: none;
		transition: background 0.15s ease;
	}

	.btn-view:hover {
		background: #4b5563;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CUSTOMER DETAILS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.customer-details {
		padding: 20px;
		background: #f8f9fa;
		border: 1px solid #e9ebed;
		border-radius: 4px;
	}

	.customer-name {
		font-weight: 600;
		color: #333;
		margin: 0 0 8px;
		font-size: 14px;
	}

	.customer-address {
		color: #333;
		margin: 0 0 4px;
		font-size: 14px;
	}

	.customer-contact {
		display: flex;
		align-items: center;
		gap: 8px;
		color: #333;
		margin: 8px 0 0;
		font-size: 14px;
	}

	.customer-contact:first-of-type {
		margin-top: 12px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

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

		.order-table th,
		.order-table td {
			padding: 10px 12px;
		}

		.col-product {
			width: 60%;
		}

		.col-total {
			width: 40%;
		}
	}

	@media screen and (max-width: 480px) {
		.order-table {
			font-size: 13px;
		}

		.subscriptions-table {
			display: block;
			overflow-x: auto;
		}
	}
</style>

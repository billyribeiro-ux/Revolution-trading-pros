<script lang="ts">
	interface OrderItem {
		id: number;
		name: string;
		quantity: number;
		total: string;
	}

	interface Subscription {
		id: number;
		status: string;
		nextPayment: string;
		total: string;
	}

	interface Order {
		id: number;
		number: string;
		date: string;
		status: string;
		items: OrderItem[];
		subtotal: string;
		discount: string;
		tax: string;
		total: string;
		paymentMethod: string;
		billingAddress: {
			name: string;
			address: string;
			phone?: string;
			email?: string;
		};
		subscriptions?: Subscription[];
	}

	interface PageData {
		order: Order;
	}

	let { data }: { data: PageData } = $props();

	const order = $derived(data.order);

	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}

	function getStatusClass(status: string): string {
		const statusMap: Record<string, string> = {
			completed: 'status-completed',
			processing: 'status-processing',
			pending: 'status-pending',
			cancelled: 'status-cancelled',
			refunded: 'status-refunded',
			failed: 'status-failed'
		};
		return statusMap[status.toLowerCase()] || 'status-default';
	}
</script>

<svelte:head>
	<title>Order #{order.number} - Revolution Trading Pros</title>
</svelte:head>

<!-- Dashboard Header -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">My Account</h1>
	</div>
</header>

<!-- Dashboard Content -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<div class="order-view-card">
			<div class="woocommerce">
				<div class="woocommerce-MyAccount-content">
					<div class="woocommerce-notices-wrapper"></div>
															
															<p>
																Order <mark class="order-number">#{order.number}</mark> was placed on 
																<mark class="order-date">{formatDate(order.date)}</mark> and is currently 
																<mark class="order-status {getStatusClass(order.status)}">{order.status}</mark>.
															</p>

															<section class="woocommerce-order-details">
																<h2 class="woocommerce-order-details__title">Order details</h2>

																<table class="woocommerce-table woocommerce-table--order-details shop_table order_details">
																	<thead>
																		<tr>
																			<th class="woocommerce-table__product-name product-name">Product</th>
																			<th class="woocommerce-table__product-table product-total">Total</th>
																		</tr>
																	</thead>

																	<tbody>
																		{#each order.items as item (item.id)}
																			<tr class="woocommerce-table__line-item order_item">
																				<td class="woocommerce-table__product-name product-name">
																					{item.name} <strong class="product-quantity">&times;&nbsp;{item.quantity}</strong>
																				</td>
																				<td class="woocommerce-table__product-total product-total">
																					<span class="woocommerce-Price-amount amount">
																						<bdi><span class="woocommerce-Price-currencySymbol">$</span>{item.total}</bdi>
																					</span>
																				</td>
																			</tr>
																		{/each}
																	</tbody>

																	<tfoot>
																		<tr>
																			<th scope="row">Subtotal:</th>
																			<td>
																				<span class="woocommerce-Price-amount amount">
																					<span class="woocommerce-Price-currencySymbol">$</span>{order.subtotal}
																				</span>
																			</td>
																		</tr>
																		{#if order.discount && parseFloat(order.discount) > 0}
																			<tr>
																				<th scope="row">Discount:</th>
																				<td>
																					-<span class="woocommerce-Price-amount amount">
																						<span class="woocommerce-Price-currencySymbol">$</span>{order.discount}
																					</span>
																				</td>
																			</tr>
																		{/if}
																		<tr>
																			<th scope="row">Tax:</th>
																			<td>
																				<span class="woocommerce-Price-amount amount">
																					<span class="woocommerce-Price-currencySymbol">$</span>{order.tax}
																				</span>
																			</td>
																		</tr>
																		<tr>
																			<th scope="row">Payment method:</th>
																			<td>{order.paymentMethod}</td>
																		</tr>
																		<tr>
																			<th scope="row">Total:</th>
																			<td>
																				<span class="woocommerce-Price-amount amount">
																					<span class="woocommerce-Price-currencySymbol">$</span>{order.total}
																				</span>
																			</td>
																		</tr>
																	</tfoot>
																</table>

																{#if order.subscriptions && order.subscriptions.length > 0}
																	<header>
																		<h2>Related subscriptions</h2>
																	</header>

																	<table class="shop_table shop_table_responsive my_account_orders woocommerce-orders-table woocommerce-MyAccount-subscriptions woocommerce-orders-table--subscriptions">
																		<thead>
																			<tr>
																				<th class="subscription-id order-number woocommerce-orders-table__header woocommerce-orders-table__header-order-number woocommerce-orders-table__header-subscription-id">
																					<span class="nobr">Subscription</span>
																				</th>
																				<th class="subscription-status order-status woocommerce-orders-table__header woocommerce-orders-table__header-order-status woocommerce-orders-table__header-subscription-status">
																					<span class="nobr">Status</span>
																				</th>
																				<th class="subscription-next-payment order-date woocommerce-orders-table__header woocommerce-orders-table__header-order-date woocommerce-orders-table__header-subscription-next-payment">
																					<span class="nobr">Next payment</span>
																				</th>
																				<th class="subscription-total order-total woocommerce-orders-table__header woocommerce-orders-table__header-order-total woocommerce-orders-table__header-subscription-total">
																					<span class="nobr">Total</span>
																				</th>
																				<th class="subscription-actions order-actions woocommerce-orders-table__header woocommerce-orders-table__header-order-actions woocommerce-orders-table__header-subscription-actions">
																					&nbsp;
																				</th>
																			</tr>
																		</thead>
																		<tbody>
																			{#each order.subscriptions as subscription (subscription.id)}
																				<tr class="order woocommerce-orders-table__row woocommerce-orders-table__row--status-{subscription.status.toLowerCase()}">
																					<td class="subscription-id order-number woocommerce-orders-table__cell woocommerce-orders-table__cell-subscription-id woocommerce-orders-table__cell-order-number" data-title="ID">
																						<a href="/dashboard/account/view-subscription/{subscription.id}">
																							#{subscription.id}
																						</a>
																					</td>
																					<td class="subscription-status order-status woocommerce-orders-table__cell woocommerce-orders-table__cell-subscription-status woocommerce-orders-table__cell-order-status" style="white-space:nowrap;" data-title="Status">
																						{subscription.status}
																					</td>
																					<td class="subscription-next-payment order-date woocommerce-orders-table__cell woocommerce-orders-table__cell-subscription-next-payment woocommerce-orders-table__cell-order-date" data-title="Next payment">
																						{formatDate(subscription.nextPayment)}
																					</td>
																					<td class="subscription-total order-total woocommerce-orders-table__cell woocommerce-orders-table__cell-subscription-total woocommerce-orders-table__cell-order-total" data-title="Total">
																						<span class="woocommerce-Price-amount amount">
																							<span class="woocommerce-Price-currencySymbol">$</span>{subscription.total}
																						</span> / month
																					</td>
																					<td class="subscription-actions order-actions woocommerce-orders-table__cell woocommerce-orders-table__cell-subscription-actions woocommerce-orders-table__cell-order-actions">
																						<a href="/dashboard/account/view-subscription/{subscription.id}" class="woocommerce-button button view">View</a>
																					</td>
																				</tr>
																			{/each}
																		</tbody>
																	</table>
																{/if}
															</section>

															<section class="woocommerce-customer-details">
																<h2 class="woocommerce-column__title">Billing address</h2>

																<address>
																	{order.billingAddress.name}<br />
																	{@html order.billingAddress.address.replace(/\n/g, '<br />')}
																	{#if order.billingAddress.phone}
																		<p class="woocommerce-customer-details--phone">{order.billingAddress.phone}</p>
																	{/if}
																	{#if order.billingAddress.email}
																		<p class="woocommerce-customer-details--email">
																			<a href="mailto:{order.billingAddress.email}">{order.billingAddress.email}</a>
																		</p>
																	{/if}
																</address>
															</section>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	/* Dashboard Header */
	.dashboard__header {
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
		border-right: 1px solid #dbdbdb;
		padding: 20px;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	@media (min-width: 1024px) {
		.dashboard__header {
			padding: 30px;
		}
	}

	@media (min-width: 1440px) {
		.dashboard__header {
			padding: 30px 40px;
		}
	}

	.dashboard__header-left {
		flex: 1;
	}

	.dashboard__page-title {
		font-family: 'Montserrat', sans-serif;
		font-size: 24px;
		font-weight: 600;
		color: #0a2335;
		margin: 0;
		line-height: 1.2;
	}

	/* Dashboard Content */
	.dashboard__content {
		background: #f5f5f5;
		min-height: calc(100vh - 60px);
		padding: 40px 30px;
	}

	.dashboard__content-main {
		max-width: 1100px;
		margin: 0 auto;
	}

	/* Professional Card Container */
	.order-view-card {
		background: #ffffff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		padding: 40px;
		margin-bottom: 30px;
	}

	/* WooCommerce Content */
	.woocommerce {
		background: transparent;
		font-family: 'Open Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
	}

	.woocommerce-MyAccount-content {
		padding: 0;
	}

	.woocommerce-MyAccount-content p {
		font-size: 15px;
		line-height: 1.8;
		color: #515151;
		margin-bottom: 30px;
		padding: 0;
	}

	mark {
		background: transparent;
		font-weight: 600;
		padding: 0;
	}

	.order-number {
		color: #0984ae;
	}

	.order-date {
		color: #333;
	}

	.order-status {
		text-transform: capitalize;
	}

	.status-completed {
		color: #28a745;
	}

	.status-processing {
		color: #ffc107;
	}

	.status-pending {
		color: #6c757d;
	}

	.status-cancelled,
	.status-refunded,
	.status-failed {
		color: #dc3545;
	}

	.woocommerce-order-details {
		margin-top: 40px;
		padding-top: 30px;
		border-top: 1px solid #e9ecef;
	}

	.woocommerce-order-details__title {
		font-size: 20px;
		font-weight: 600;
		margin-bottom: 24px;
		color: #333;
		font-family: 'Open Sans', sans-serif;
	}

	.woocommerce-table {
		width: 100%;
		border-collapse: collapse;
		margin-bottom: 40px;
		border: 1px solid #e9ecef;
		border-radius: 6px;
		overflow: hidden;
	}

	.woocommerce-table thead {
		background: #f8f9fa;
		border-bottom: 2px solid #e9ecef;
	}

	.woocommerce-table thead th {
		padding: 16px 20px;
		font-size: 13px;
		font-weight: 600;
		text-align: left;
		color: #495057;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.woocommerce-table tbody tr {
		border-bottom: 1px solid #e9ecef;
		transition: background-color 0.15s ease;
	}

	.woocommerce-table tbody tr:hover {
		background-color: #f8f9fa;
	}

	.woocommerce-table tbody tr:last-child {
		border-bottom: none;
	}

	.woocommerce-table tbody td {
		padding: 16px 20px;
		font-size: 14px;
		color: #495057;
	}

	.product-quantity {
		color: #6c757d;
		font-weight: 400;
	}

	.woocommerce-table tfoot {
		background: #fafbfc;
	}

	.woocommerce-table tfoot th,
	.woocommerce-table tfoot td {
		padding: 14px 20px;
		font-size: 14px;
	}

	.woocommerce-table tfoot th {
		text-align: left;
		font-weight: 600;
		color: #495057;
	}

	.woocommerce-table tfoot td {
		text-align: right;
		color: #333;
	}

	.woocommerce-table tfoot tr:last-child {
		background: #f1f3f5;
	}

	.woocommerce-table tfoot tr:last-child th,
	.woocommerce-table tfoot tr:last-child td {
		font-size: 17px;
		font-weight: 700;
		padding: 18px 20px;
		border-top: 2px solid #dee2e6;
		color: #0984ae;
	}

	.woocommerce-Price-amount {
		font-weight: 600;
	}

	.woocommerce-customer-details {
		margin-top: 40px;
		padding: 24px;
		background: #f8f9fa;
		border-radius: 6px;
		border-left: 4px solid #0984ae;
	}

	.woocommerce-column__title {
		font-size: 18px;
		font-weight: 600;
		margin-bottom: 20px;
		color: #333;
		font-family: 'Open Sans', sans-serif;
	}

	address {
		font-style: normal;
		line-height: 1.9;
		color: #495057;
		font-size: 14px;
	}

	.woocommerce-customer-details--phone,
	.woocommerce-customer-details--email {
		margin-top: 12px;
		margin-bottom: 0;
	}

	.woocommerce-customer-details--email a {
		color: #0984ae;
		text-decoration: none;
	}

	.woocommerce-customer-details--email a:hover {
		text-decoration: underline;
	}

	header h2 {
		font-size: 18px;
		font-weight: 600;
		margin: 40px 0 24px;
		color: #333;
		font-family: 'Open Sans', sans-serif;
	}

	.shop_table_responsive {
		width: 100%;
		border-collapse: collapse;
	}

	.shop_table_responsive thead th {
		padding: 12px 16px;
		font-size: 13px;
		font-weight: 600;
		text-align: left;
		background: #f8f9fa;
		border-bottom: 2px solid #e9ecef;
	}

	.shop_table_responsive tbody td {
		padding: 12px 16px;
		font-size: 14px;
		border-bottom: 1px solid #e9ecef;
	}

	.shop_table_responsive tbody td a {
		color: #0984ae;
		text-decoration: none;
		font-weight: 600;
	}

	.shop_table_responsive tbody td a:hover {
		text-decoration: underline;
	}

	.woocommerce-button.button {
		display: inline-block;
		padding: 8px 16px;
		background: #0984ae;
		color: #fff;
		text-decoration: none;
		border-radius: 4px;
		font-size: 13px;
		font-weight: 600;
		transition: background 0.2s ease;
	}

	.woocommerce-button.button:hover {
		background: #076a8a;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.dashboard__content {
			padding: 20px 15px;
		}

		.order-view-card {
			padding: 24px 20px;
			border-radius: 6px;
		}

		.woocommerce-MyAccount-content p {
			font-size: 14px;
			margin-bottom: 20px;
		}

		.woocommerce-order-details {
			margin-top: 30px;
			padding-top: 20px;
		}

		.woocommerce-customer-details {
			padding: 20px;
			margin-top: 30px;
		}


		.woocommerce-table thead {
			display: none;
		}

		.woocommerce-table tbody tr {
			display: block;
			margin-bottom: 20px;
			border: 1px solid #e9ecef;
			border-radius: 8px;
			padding: 16px;
		}

		.woocommerce-table tbody td {
			display: flex;
			justify-content: space-between;
			padding: 8px 0;
			border: none;
		}

		.woocommerce-table tbody td::before {
			content: attr(data-title);
			font-weight: 600;
			color: #6c757d;
		}

		.woocommerce-table tfoot th,
		.woocommerce-table tfoot td {
			display: block;
			text-align: left;
		}

		.shop_table_responsive thead {
			display: none;
		}

		.shop_table_responsive tbody tr {
			display: block;
			margin-bottom: 16px;
			border: 1px solid #e9ecef;
			border-radius: 8px;
			padding: 12px;
		}

		.shop_table_responsive tbody td {
			display: flex;
			justify-content: space-between;
			padding: 8px 0;
			border: none;
		}

		.shop_table_responsive tbody td::before {
			content: attr(data-title);
			font-weight: 600;
			color: #6c757d;
		}
	}
</style>

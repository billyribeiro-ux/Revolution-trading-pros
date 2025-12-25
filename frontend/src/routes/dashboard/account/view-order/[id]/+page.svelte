<script lang="ts">
	/**
	 * View Order - Account Section
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Order details view page
	 *
	 * @version 2.0.0 - 100% Pixel Perfect
	 */
	import { page } from '$app/stores';

	const orderId = $page.params.id;

	// Sample order data
	const order = {
		id: orderId,
		date: 'December 3, 2025',
		status: 'Completed',
		statusClass: 'label--success',
		total: '$197.00',
		paymentMethod: 'Visa card ending in 9396',
		items: [
			{
				name: 'Mastering the Trade Room (1 Month Trial)',
				quantity: 1,
				price: '$197.00'
			}
		],
		billingAddress: {
			name: 'John Doe',
			address: '123 Main St',
			city: 'Austin',
			state: 'TX',
			zip: '78759',
			country: 'United States'
		}
	};
</script>

<svelte:head>
	<title>Order #{orderId} - Account | Revolution Trading Pros</title>
</svelte:head>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">Order #{orderId}</h2>
			<p class="order-meta">Order placed on <time>{order.date}</time></p>

			<div class="order-status">
				<span class="label {order.statusClass}">{order.status}</span>
			</div>

			<div class="order-details">
				<h3>Order Details</h3>
				<table class="table">
					<thead>
						<tr>
							<th>Product</th>
							<th>Quantity</th>
							<th class="text-right">Total</th>
						</tr>
					</thead>
					<tbody>
						{#each order.items as item}
							<tr>
								<td>{item.name}</td>
								<td>{item.quantity}</td>
								<td class="text-right">{item.price}</td>
							</tr>
						{/each}
					</tbody>
					<tfoot>
						<tr class="order-total">
							<th colspan="2">Total:</th>
							<td class="text-right"><strong>{order.total}</strong></td>
						</tr>
					</tfoot>
				</table>
			</div>

			<div class="order-addresses">
				<div class="address-block">
					<h3>Billing Address</h3>
					<address>
						{order.billingAddress.name}<br>
						{order.billingAddress.address}<br>
						{order.billingAddress.city}, {order.billingAddress.state} {order.billingAddress.zip}<br>
						{order.billingAddress.country}
					</address>
				</div>
				<div class="address-block">
					<h3>Payment Method</h3>
					<p>{order.paymentMethod}</p>
				</div>
			</div>

			<p class="back-link">
				<a href="/dashboard/account/orders" class="btn btn-default">← Back to Orders</a>
			</p>
		</section>
	</div>
</div>

<style>
	.section-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 10px;
	}

	.order-meta {
		font-size: 14px;
		color: #666;
		margin-bottom: 20px;
	}

	.order-status {
		margin-bottom: 30px;
	}

	.label {
		display: inline-block;
		padding: 6px 14px;
		font-size: 12px;
		font-weight: 700;
		text-transform: uppercase;
		border-radius: 25px;
	}

	.label--success {
		background: #d4edda;
		color: #155724;
	}

	.order-details {
		margin-bottom: 30px;
	}

	.order-details h3 {
		font-size: 18px;
		font-weight: 700;
		margin-bottom: 15px;
	}

	.table {
		width: 100%;
		border-collapse: collapse;
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.table th,
	.table td {
		padding: 15px 20px;
		text-align: left;
		border-bottom: 1px solid #ededed;
	}

	.table thead {
		background: #f9f9f9;
	}

	.table th {
		font-size: 13px;
		font-weight: 700;
		text-transform: uppercase;
	}

	.table tfoot {
		background: #f9f9f9;
	}

	.text-right {
		text-align: right;
	}

	.order-addresses {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 30px;
		margin-bottom: 30px;
	}

	.address-block h3 {
		font-size: 16px;
		font-weight: 700;
		margin-bottom: 10px;
	}

	.address-block address,
	.address-block p {
		font-size: 14px;
		color: #666;
		line-height: 1.6;
		font-style: normal;
	}

	.btn {
		display: inline-block;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		border-radius: 4px;
		text-decoration: none;
		transition: all 0.15s ease;
	}

	.btn-default {
		background: #f4f4f4;
		color: #333;
		border: 1px solid #dbdbdb;
	}

	.btn-default:hover {
		background: #e9e9e9;
	}
</style>

<script lang="ts">
	/**
	 * View Subscription - Account Section
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Subscription details view page
	 *
	 * @version 2.0.0 - 100% Pixel Perfect
	 */
	import { page } from '$app/stores';

	const subscriptionId = $page.params.id;

	// Sample subscription data
	const subscription = {
		id: subscriptionId,
		status: 'Active',
		statusClass: 'label--success',
		startDate: 'December 3, 2025',
		nextPayment: 'January 3, 2026',
		product: 'Mastering the Trade Room (1 Month Trial)',
		total: '$197.00',
		interval: '/ month',
		paymentMethod: 'Visa card ending in 9396',
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
	<title>Subscription #{subscriptionId} - Account | Revolution Trading Pros</title>
</svelte:head>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">Subscription #{subscriptionId}</h2>

			<div class="subscription-header">
				<span class="label {subscription.statusClass}">{subscription.status}</span>
			</div>

			<div class="subscription-details">
				<table class="table">
					<tbody>
						<tr>
							<th>Product</th>
							<td>{subscription.product}</td>
						</tr>
						<tr>
							<th>Amount</th>
							<td>{subscription.total} {subscription.interval}</td>
						</tr>
						<tr>
							<th>Start Date</th>
							<td>{subscription.startDate}</td>
						</tr>
						<tr>
							<th>Next Payment</th>
							<td>{subscription.nextPayment}</td>
						</tr>
						<tr>
							<th>Payment Method</th>
							<td>{subscription.paymentMethod}</td>
						</tr>
					</tbody>
				</table>
			</div>

			<div class="subscription-actions">
				<h3>Subscription Actions</h3>
				<div class="action-buttons">
					<button class="btn btn-default">Update Payment Method</button>
					<button class="btn btn-warning">Cancel Subscription</button>
				</div>
			</div>

			<div class="billing-address">
				<h3>Billing Address</h3>
				<address>
					{subscription.billingAddress.name}<br>
					{subscription.billingAddress.address}<br>
					{subscription.billingAddress.city}, {subscription.billingAddress.state} {subscription.billingAddress.zip}<br>
					{subscription.billingAddress.country}
				</address>
			</div>

			<p class="back-link">
				<a href="/dashboard/account/subscriptions" class="btn btn-default">← Back to Subscriptions</a>
			</p>
		</section>
	</div>
</div>

<style>
	.section-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
	}

	.subscription-header {
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

	.subscription-details {
		margin-bottom: 30px;
	}

	.table {
		width: 100%;
		max-width: 500px;
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

	.table th {
		font-size: 14px;
		font-weight: 600;
		color: #666;
		width: 40%;
	}

	.table td {
		font-size: 14px;
		color: #333;
		font-weight: 600;
	}

	.table tbody tr:last-child th,
	.table tbody tr:last-child td {
		border-bottom: none;
	}

	.subscription-actions {
		margin-bottom: 30px;
	}

	.subscription-actions h3,
	.billing-address h3 {
		font-size: 16px;
		font-weight: 700;
		margin-bottom: 15px;
	}

	.action-buttons {
		display: flex;
		gap: 12px;
		flex-wrap: wrap;
	}

	.billing-address {
		margin-bottom: 30px;
	}

	.billing-address address {
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
		border: none;
		cursor: pointer;
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

	.btn-warning {
		background: #fff3cd;
		color: #856404;
		border: 1px solid #ffc107;
	}

	.btn-warning:hover {
		background: #ffe69c;
	}
</style>

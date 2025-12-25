<script lang="ts">
	/**
	 * My Subscriptions - Account Section
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * 100% PIXEL-PERFECT match to SimplerMySubscriptions reference
	 * Subscriptions table with status labels
	 *
	 * @version 2.0.0 - 100% Pixel Perfect
	 */

	// Sample subscriptions data matching SimplerMySubscriptions reference
	const subscriptions = [
		{
			id: '2176655',
			status: 'Active',
			statusClass: 'label--success',
			product: 'Mastering the Trade Room (1 Month Trial)',
			nextPayment: 'January 3, 2026',
			paymentMethod: 'visa card ending in 9396',
			total: '$197.00',
			interval: '/ month'
		},
		{
			id: '2173015',
			status: 'Pending Cancellation',
			statusClass: 'label--info',
			product: 'Moxie Indicator™ Mastery Monthly (Trial)',
			nextPayment: '-',
			paymentMethod: null,
			total: '$247.00',
			interval: ''
		},
		{
			id: '2170842',
			status: 'Cancelled',
			statusClass: 'label--error',
			product: 'Day Trading Room Monthly',
			nextPayment: '-',
			paymentMethod: null,
			total: '$297.00',
			interval: ''
		}
	];
</script>

<svelte:head>
	<title>My Subscriptions - Account | Revolution Trading Pros</title>
</svelte:head>

<!-- 100% EXACT structure from SimplerMySubscriptions reference -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">My Subscriptions</h2>

			<div class="woocommerce_account_subscriptions">
				<table class="table">
					<thead>
						<tr>
							<th class="col-xs-3"><span class="nobr">Subscription</span></th>
							<th class="col-xs-3"><span class="nobr">Status</span></th>
							<th class="col-xs-2"><span class="nobr">Product</span></th>
							<th class="col-xs-2"><span class="nobr">Next Payment</span></th>
							<th class="col-xs-2"><span class="nobr">Total</span></th>
							<th class="col-xs-2">&nbsp;</th>
						</tr>
					</thead>
					<tbody class="u--font-size-sm">
						{#each subscriptions as sub}
							<tr class="order">
								<td class="col-xs-3" data-title="ID">
									<a href="/dashboard/account/view-subscription/{sub.id}">
										#{sub.id}
									</a>
								</td>
								<td class="col-xs-3" data-title="Status">
									<span class="label {sub.statusClass}">
										{sub.status}
									</span>
								</td>
								<td class="col-xs-2" data-title="Product">
									<span>{sub.product}</span>
								</td>
								<td class="col-xs-2" data-title="Next Payment">
									{sub.nextPayment}
									{#if sub.paymentMethod}
										<br/><small>Via {sub.paymentMethod}</small>
									{/if}
								</td>
								<td class="col-xs-2" data-title="Total">
									<span class="woocommerce-Price-amount amount">{sub.total}</span>{sub.interval}
								</td>
								<td class="col-xs-2 text-right">
									<a href="/dashboard/account/view-subscription/{sub.id}" class="btn btn-default btn-xs">View</a>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>

			{#if subscriptions.length === 0}
				<div class="empty-state">
					<p>No subscriptions found.</p>
				</div>
			{/if}
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   100% PIXEL-PERFECT STYLES - Matching SimplerMySubscriptions reference
	   ═══════════════════════════════════════════════════════════════════════════ */

	.section-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
	}

	/* Table - Exact from reference */
	.table {
		width: 100%;
		border-collapse: collapse;
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.table thead {
		background: #f9f9f9;
		border-bottom: 1px solid #dbdbdb;
	}

	.table th {
		padding: 15px 15px;
		font-size: 12px;
		font-weight: 700;
		color: #333;
		text-align: left;
		font-family: 'Open Sans', sans-serif;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.table td {
		padding: 15px 15px;
		font-size: 14px;
		color: #333;
		border-bottom: 1px solid #ededed;
		vertical-align: middle;
	}

	.table tbody tr:last-child td {
		border-bottom: none;
	}

	.table tbody tr:hover {
		background: #f9f9f9;
	}

	/* Links */
	.table td a:not(.btn) {
		color: #0984ae;
		text-decoration: none;
		font-weight: 600;
	}

	.table td a:not(.btn):hover {
		color: #0e6ac4;
		text-decoration: underline;
	}

	/* Status Labels - Exact from reference */
	.label {
		display: inline-block;
		padding: 4px 10px;
		font-size: 11px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		border-radius: 25px;
	}

	.label--success {
		background-color: #d4edda;
		color: #155724;
	}

	.label--info {
		background-color: #d1ecf1;
		color: #0c5460;
	}

	.label--warning {
		background-color: #fff3cd;
		color: #856404;
	}

	.label--danger {
		background-color: #f8d7da;
		color: #721c24;
	}

	.label--error {
		background-color: #f8d7da;
		color: #721c24;
	}

	/* Small text */
	small {
		font-size: 12px;
		color: #666;
	}

	/* Price */
	.woocommerce-Price-amount {
		font-weight: 600;
	}

	/* View button */
	.btn {
		display: inline-block;
		padding: 8px 16px;
		font-size: 13px;
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
		border-color: #999;
	}

	.btn-xs {
		padding: 6px 12px;
		font-size: 12px;
	}

	.text-right {
		text-align: right;
	}

	/* Empty state */
	.empty-state {
		text-align: center;
		padding: 40px 20px;
		color: #666;
	}

	@media (max-width: 992px) {
		.table {
			display: block;
			overflow-x: auto;
		}
	}
</style>

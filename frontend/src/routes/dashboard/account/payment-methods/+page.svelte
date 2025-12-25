<script lang="ts">
	/**
	 * Payment Methods - Account Section
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * 100% PIXEL-PERFECT match to SimplerPaymentMethods reference
	 * Saved payment methods management
	 *
	 * @version 2.0.0 - 100% Pixel Perfect
	 */

	// Sample payment methods
	const paymentMethods = [
		{
			id: 1,
			type: 'Visa',
			last4: '9396',
			expires: '12/2027',
			isDefault: true
		}
	];
</script>

<svelte:head>
	<title>Payment Methods - Account | Revolution Trading Pros</title>
</svelte:head>

<!-- 100% EXACT structure matching reference -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">Payment Methods</h2>

			{#if paymentMethods.length > 0}
				<table class="table woocommerce-MyAccount-paymentMethods">
					<thead>
						<tr>
							<th class="woocommerce-PaymentMethod">Method</th>
							<th class="woocommerce-PaymentMethod-expires">Expires</th>
							<th class="woocommerce-PaymentMethod-actions">&nbsp;</th>
						</tr>
					</thead>
					<tbody>
						{#each paymentMethods as method}
							<tr class="woocommerce-PaymentMethod">
								<td class="woocommerce-PaymentMethod-method" data-title="Method">
									<span class="card-type">{method.type}</span> ending in <strong>{method.last4}</strong>
									{#if method.isDefault}
										<span class="label label--default">Default</span>
									{/if}
								</td>
								<td class="woocommerce-PaymentMethod-expires" data-title="Expires">
									{method.expires}
								</td>
								<td class="woocommerce-PaymentMethod-actions" data-title="Actions">
									{#if !method.isDefault}
										<button class="btn btn-xs btn-default">Make Default</button>
									{/if}
									<button class="btn btn-xs btn-danger">Delete</button>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			{:else}
				<div class="woocommerce-Message woocommerce-Message--info">
					No saved methods found.
				</div>
			{/if}

			<p class="add-payment-method">
				<a href="/dashboard/account/add-payment-method" class="btn btn-primary">Add payment method</a>
			</p>
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   100% PIXEL-PERFECT STYLES - Matching reference
	   ═══════════════════════════════════════════════════════════════════════════ */

	.section-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
	}

	/* Table */
	.table {
		width: 100%;
		border-collapse: collapse;
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		margin-bottom: 20px;
	}

	.table thead {
		background: #f9f9f9;
		border-bottom: 1px solid #dbdbdb;
	}

	.table th {
		padding: 15px 20px;
		font-size: 13px;
		font-weight: 700;
		color: #333;
		text-align: left;
		font-family: 'Open Sans', sans-serif;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.table td {
		padding: 15px 20px;
		font-size: 14px;
		color: #333;
		border-bottom: 1px solid #ededed;
		vertical-align: middle;
	}

	.table tbody tr:last-child td {
		border-bottom: none;
	}

	.card-type {
		font-weight: 600;
	}

	/* Labels */
	.label {
		display: inline-block;
		padding: 3px 8px;
		font-size: 10px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		border-radius: 3px;
		margin-left: 8px;
	}

	.label--default {
		background: #e9f5f9;
		color: #0984ae;
	}

	/* Buttons */
	.btn {
		display: inline-block;
		padding: 8px 16px;
		font-size: 13px;
		font-weight: 600;
		border-radius: 4px;
		border: none;
		cursor: pointer;
		text-decoration: none;
		transition: all 0.15s ease;
	}

	.btn-xs {
		padding: 6px 12px;
		font-size: 12px;
	}

	.btn-default {
		background: #f4f4f4;
		color: #333;
		border: 1px solid #dbdbdb;
	}

	.btn-default:hover {
		background: #e9e9e9;
	}

	.btn-danger {
		background: #dc3545;
		color: #fff;
	}

	.btn-danger:hover {
		background: #c82333;
	}

	.btn-primary {
		background: #0984ae;
		color: #fff;
	}

	.btn-primary:hover {
		background: #076787;
	}

	.woocommerce-PaymentMethod-actions {
		text-align: right;
	}

	.woocommerce-PaymentMethod-actions .btn {
		margin-left: 8px;
	}

	/* Message */
	.woocommerce-Message {
		padding: 20px 25px;
		margin-bottom: 20px;
		border-radius: 8px;
		font-size: 14px;
		background: #f0f8ff;
		border: 1px solid #b3d7ff;
		color: #31708f;
	}

	.add-payment-method {
		margin-top: 20px;
	}
</style>

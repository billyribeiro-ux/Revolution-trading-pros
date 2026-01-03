<script lang="ts">
	interface PaymentMethod {
		id: number;
		method: string;
		details: string;
		expires: string;
		isDefault: boolean;
		subscriptions: string[];
	}

	interface PageData {
		paymentMethods: PaymentMethod[];
	}

	let { data }: { data: PageData } = $props();

	const paymentMethods = $derived(data.paymentMethods || []);
</script>

<svelte:head>
	<title>Payment Methods - Revolution Trading Pros</title>
</svelte:head>

<div class="woocommerce">
	<div class="woocommerce-MyAccount-content">
		<div class="woocommerce-notices-wrapper"></div>
		
		<div class="row">
			<div class="col-md-6">
				<h2 class="section-title">Payment Methods</h2>
			</div>
			<div class="col-md-6 text-right hidden-xs hidden-sm">
				<a class="btn btn-white btn-xs" href="/dashboard/account/add-payment-method">
					Add payment method
				</a>
			</div>
		</div>

		<p class="visible-xs visible-sm">
			<a class="btn btn-white btn-xs" href="/dashboard/account/add-payment-method">
				Add payment method
			</a>
		</p>

		{#if paymentMethods.length === 0}
			<div class="woocommerce-message woocommerce-message--info">
				<p>No saved payment methods.</p>
			</div>
		{:else}
			<table class="table woocommerce-MyAccount-paymentMethods">
				<thead>
					<tr>
						<th class="woocommerce-PaymentMethod woocommerce-PaymentMethod--method payment-method-method">
							<span class="nobr">Method</span>
						</th>
						<th class="woocommerce-PaymentMethod woocommerce-PaymentMethod--details payment-method-details">
							<span class="nobr">Details</span>
						</th>
						<th class="woocommerce-PaymentMethod woocommerce-PaymentMethod--expires payment-method-expires">
							<span class="nobr">Expires</span>
						</th>
						<th class="woocommerce-PaymentMethod woocommerce-PaymentMethod--default payment-method-default">
							<span class="nobr">Default?</span>
						</th>
						<th class="woocommerce-PaymentMethod woocommerce-PaymentMethod--subscriptions payment-method-subscriptions">
							<span class="nobr">Subscriptions</span>
						</th>
						<th class="woocommerce-PaymentMethod woocommerce-PaymentMethod--actions payment-method-actions">
							<span class="nobr">&nbsp;</span>
						</th>
					</tr>
				</thead>
				<tbody>
					{#each paymentMethods as method (method.id)}
						<tr class:default-payment-method={method.isDefault}>
							<td class="woocommerce-PaymentMethod woocommerce-PaymentMethod--method payment-method-method" data-title="Method">
								{method.method}
							</td>
							<td class="woocommerce-PaymentMethod woocommerce-PaymentMethod--details payment-method-details" data-title="Details">
								{method.details || ''}
							</td>
							<td class="woocommerce-PaymentMethod woocommerce-PaymentMethod--expires payment-method-expires" data-title="Expires">
								{method.expires}
							</td>
							<td class="woocommerce-PaymentMethod woocommerce-PaymentMethod--default payment-method-default" data-title="Default?">
								{#if method.isDefault}
									<mark class="default">Default</mark>
								{/if}
							</td>
							<td class="woocommerce-PaymentMethod woocommerce-PaymentMethod--subscriptions payment-method-subscriptions" data-title="Subscriptions">
								{#if method.subscriptions && method.subscriptions.length > 0}
									{method.subscriptions.join(', ')}
								{/if}
							</td>
							<td class="woocommerce-PaymentMethod woocommerce-PaymentMethod--actions payment-method-actions" data-title="&nbsp;">
								{#if !method.isDefault}
									<a href="/dashboard/account/payment-methods/delete/{method.id}" class="button">Delete</a>
								{/if}
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		{/if}
	</div>
</div>

<style>
	.section-title {
		font-size: 24px;
		font-weight: 700;
		margin-bottom: 20px;
		color: #333;
	}

	.row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 20px;
	}

	.col-md-6 {
		flex: 0 0 50%;
	}

	.text-right {
		text-align: right;
	}

	.btn {
		display: inline-block;
		padding: 8px 16px;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.15s ease;
		border: 1px solid #dee2e6;
	}

	.btn-white {
		background: #fff;
		color: #0984ae;
		border-color: #0984ae;
	}

	.btn-white:hover {
		background: #0984ae;
		color: #fff;
	}

	.btn-xs {
		padding: 6px 12px;
		font-size: 13px;
	}

	.table {
		width: 100%;
		border-collapse: collapse;
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
		margin-top: 20px;
	}

	.table thead {
		background: #f8f9fa;
		border-bottom: 2px solid #e9ecef;
	}

	.table thead th {
		padding: 16px 20px;
		font-size: 14px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: #495057;
		text-align: left;
	}

	.table tbody tr {
		border-bottom: 1px solid #e9ecef;
		transition: background-color 0.15s ease;
	}

	.table tbody tr:hover {
		background-color: #f8f9fa;
	}

	.table tbody tr:last-child {
		border-bottom: none;
	}

	.table tbody tr.default-payment-method {
		background-color: #f0f8ff;
	}

	.table tbody td {
		padding: 16px 20px;
		font-size: 14px;
		color: #495057;
		vertical-align: middle;
	}

	.table tbody td mark.default {
		background: #28a745;
		color: #fff;
		padding: 4px 8px;
		border-radius: 4px;
		font-size: 12px;
		font-weight: 600;
	}

	.table tbody td .button {
		display: inline-block;
		padding: 6px 12px;
		font-size: 13px;
		font-weight: 600;
		color: #dc3545;
		text-decoration: none;
		border: 1px solid #dc3545;
		border-radius: 4px;
		transition: all 0.15s ease;
	}

	.table tbody td .button:hover {
		background: #dc3545;
		color: #fff;
	}

	.woocommerce-message {
		padding: 16px 20px;
		background: #e7f3ff;
		border-left: 4px solid #0984ae;
		border-radius: 4px;
		margin-bottom: 20px;
	}

	.woocommerce-message p {
		margin: 0;
		color: #495057;
		font-size: 14px;
	}

	.visible-xs,
	.visible-sm {
		display: none;
	}

	.hidden-xs,
	.hidden-sm {
		display: block;
	}

	@media (max-width: 768px) {
		.row {
			flex-direction: column;
			align-items: flex-start;
		}

		.col-md-6 {
			flex: 0 0 100%;
			width: 100%;
		}

		.text-right {
			text-align: left;
		}

		.hidden-xs,
		.hidden-sm {
			display: none;
		}

		.visible-xs,
		.visible-sm {
			display: block;
			margin-bottom: 20px;
		}

		.table {
			display: block;
			overflow-x: auto;
		}

		.table thead {
			display: none;
		}

		.table tbody tr {
			display: block;
			margin-bottom: 16px;
			border: 1px solid #e9ecef;
			border-radius: 8px;
			padding: 16px;
		}

		.table tbody td {
			display: flex;
			justify-content: space-between;
			align-items: center;
			padding: 8px 0;
			border: none;
		}

		.table tbody td::before {
			content: attr(data-title);
			font-weight: 600;
			text-transform: uppercase;
			font-size: 12px;
			color: #6c757d;
		}
	}
</style>

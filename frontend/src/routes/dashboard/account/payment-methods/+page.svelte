<script lang="ts">
	import { enhance } from '$app/forms';

	interface PaymentMethod {
		id: string;
		type: string;
		brand?: string;
		last4?: string;
		expiryMonth?: number;
		expiryYear?: number;
		isDefault: boolean;
		subscriptions?: string[];
	}

	interface PageData {
		paymentMethods: PaymentMethod[];
	}

	interface ActionData {
		success?: boolean;
		message?: string;
		error?: string;
	}

	let { data, form }: { data: PageData; form?: ActionData } = $props();

	const paymentMethods = $derived(data.paymentMethods || []);
	let isSubmitting = $state(false);

	function confirmDelete(event: Event, method: PaymentMethod): boolean {
		if (method.subscriptions && method.subscriptions.length > 0) {
			event.preventDefault();
			alert('That payment method cannot be deleted because it is linked to an automatic subscription. Please add a payment method or choose a default payment method, before trying again.');
			return false;
		}
		
		const confirmed = confirm('Are you sure you want to delete this payment method?');
		if (!confirmed) {
			event.preventDefault();
			return false;
		}
		return true;
	}

	function formatExpiry(method: PaymentMethod): string {
		if (method.expiryMonth && method.expiryYear) {
			return `${String(method.expiryMonth).padStart(2, '0')}/${method.expiryYear}`;
		}
		return 'N/A';
	}

	function formatDetails(method: PaymentMethod): string {
		if (method.brand && method.last4) {
			return `${method.brand} ending in ${method.last4}`;
		}
		return method.type || 'Card';
	}
</script>

<svelte:head>
	<title>Payment Methods - Revolution Trading Pros</title>
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
		<div class="payment-methods-card">
			<div class="woocommerce-notices-wrapper">
			{#if form?.success}
				<div class="woocommerce-message" role="alert">
					{form.message}
				</div>
			{/if}

			{#if form?.error}
				<div class="woocommerce-error" role="alert">
					{form.error}
				</div>
			{/if}
		</div>
		
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
								{method.type || 'Card'}
							</td>
							<td class="woocommerce-PaymentMethod woocommerce-PaymentMethod--details payment-method-details" data-title="Details">
								{formatDetails(method)}
							</td>
							<td class="woocommerce-PaymentMethod woocommerce-PaymentMethod--expires payment-method-expires" data-title="Expires">
								{formatExpiry(method)}
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
									<form 
										method="post" 
										action="?/delete"
										use:enhance={() => {
											isSubmitting = true;
											return async ({ update }) => {
												await update();
												isSubmitting = false;
											};
										}}
										onsubmit={(e) => confirmDelete(e, method)}
									>
										<input type="hidden" name="payment_method_id" value={method.id} />
										<button 
											type="submit" 
											class="button"
											disabled={isSubmitting}
										>
											Delete
										</button>
									</form>
								{/if}
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
			{/if}
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
		font-family: 'Open Sans', sans-serif;
		font-size: 28px;
		font-weight: 400;
		font-style: italic;
		color: #333333;
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
	.payment-methods-card {
		background: #ffffff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		padding: 40px;
		margin-bottom: 30px;
	}

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

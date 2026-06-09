<script lang="ts">
	import { enhance } from '$app/forms';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';

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

	let props: { data: PageData; form?: ActionData } = $props();
	let data = $derived(props.data);
	let form = $derived(props.form);

	const paymentMethods = $derived(data.paymentMethods || []);
	let isSubmitting = $state(false);

	// Delete confirmation modal state
	let showDeleteModal = $state(false);
	let showLinkedWarningModal = $state(false);
	let _pendingDeleteMethod = $state<PaymentMethod | null>(null);
	let deleteFormElement = $state<HTMLFormElement | null>(null);

	function confirmDelete(event: Event, method: PaymentMethod): boolean {
		event.preventDefault();

		if (method.subscriptions && method.subscriptions.length > 0) {
			showLinkedWarningModal = true;
			return false;
		}

		_pendingDeleteMethod = method;
		deleteFormElement = (event.target as HTMLButtonElement).closest('form');
		showDeleteModal = true;
		return false;
	}

	function handleConfirmDelete() {
		showDeleteModal = false;
		if (deleteFormElement) {
			deleteFormElement.submit();
		}
		_pendingDeleteMethod = null;
		deleteFormElement = null;
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

			<div class="payment-methods-card-header">
				<div class="payment-methods-title-column">
					<h2 class="section-title">Payment Methods</h2>
				</div>
				<div class="payment-methods-desktop-action">
					<a class="secondary-action-link" href="/dashboard/account/add-payment-method">
						Add payment method
					</a>
				</div>
			</div>

			<p class="payment-methods-mobile-action">
				<a class="secondary-action-link" href="/dashboard/account/add-payment-method">
					Add payment method
				</a>
			</p>

			{#if paymentMethods.length === 0}
				<div class="woocommerce-message woocommerce-message--info">
					<p>No saved payment methods.</p>
				</div>
			{:else}
				<table class="payment-methods-table woocommerce-MyAccount-paymentMethods">
					<thead>
						<tr>
							<th
								class="woocommerce-PaymentMethod woocommerce-PaymentMethod--method payment-method-method"
							>
								<span class="nobr">Method</span>
							</th>
							<th
								class="woocommerce-PaymentMethod woocommerce-PaymentMethod--details payment-method-details"
							>
								<span class="nobr">Details</span>
							</th>
							<th
								class="woocommerce-PaymentMethod woocommerce-PaymentMethod--expires payment-method-expires"
							>
								<span class="nobr">Expires</span>
							</th>
							<th
								class="woocommerce-PaymentMethod woocommerce-PaymentMethod--default payment-method-default"
							>
								<span class="nobr">Default?</span>
							</th>
							<th
								class="woocommerce-PaymentMethod woocommerce-PaymentMethod--subscriptions payment-method-subscriptions"
							>
								<span class="nobr">Subscriptions</span>
							</th>
							<th
								class="woocommerce-PaymentMethod woocommerce-PaymentMethod--actions payment-method-actions"
							>
								<span class="nobr">&nbsp;</span>
							</th>
						</tr>
					</thead>
					<tbody>
						{#each paymentMethods as method (method.id)}
							<tr class={{ 'default-payment-method': method.isDefault }}>
								<td
									class="woocommerce-PaymentMethod woocommerce-PaymentMethod--method payment-method-method"
									data-title="Method"
								>
									{method.type || 'Card'}
								</td>
								<td
									class="woocommerce-PaymentMethod woocommerce-PaymentMethod--details payment-method-details"
									data-title="Details"
								>
									{formatDetails(method)}
								</td>
								<td
									class="woocommerce-PaymentMethod woocommerce-PaymentMethod--expires payment-method-expires"
									data-title="Expires"
								>
									{formatExpiry(method)}
								</td>
								<td
									class="woocommerce-PaymentMethod woocommerce-PaymentMethod--default payment-method-default"
									data-title="Default?"
								>
									{#if method.isDefault}
										<mark class="default">Default</mark>
									{/if}
								</td>
								<td
									class="woocommerce-PaymentMethod woocommerce-PaymentMethod--subscriptions payment-method-subscriptions"
									data-title="Subscriptions"
								>
									{#if method.subscriptions && method.subscriptions.length > 0}
										{method.subscriptions.join(', ')}
									{/if}
								</td>
								<td
									class="woocommerce-PaymentMethod woocommerce-PaymentMethod--actions payment-method-actions"
									data-title="&nbsp;"
								>
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
											<button type="submit" class="button" disabled={isSubmitting}> Delete </button>
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

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete Payment Method"
	message="Are you sure you want to delete this payment method?"
	confirmText="Delete"
	variant="danger"
	onConfirm={handleConfirmDelete}
	onCancel={() => {
		showDeleteModal = false;
		_pendingDeleteMethod = null;
		deleteFormElement = null;
	}}
/>

<ConfirmationModal
	isOpen={showLinkedWarningModal}
	title="Cannot Delete Payment Method"
	message="That payment method cannot be deleted because it is linked to an automatic subscription. Please add a payment method or choose a default payment method, before trying again."
	confirmText="OK"
	variant="warning"
	onConfirm={() => {
		showLinkedWarningModal = false;
	}}
	onCancel={() => {
		showLinkedWarningModal = false;
	}}
/>

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
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 28px;
		font-weight: 700;
		font-style: italic;
		color: #333333;
		margin: 0;
		line-height: 1.2;
	}

	/* Dashboard Content */
	/* ICT11+ Fix: Removed min-height: calc(100vh - 60px) - let parent flex container handle height */
	.dashboard__content {
		background: #f5f5f5;
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

	.payment-methods-card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 20px;
	}

	.payment-methods-title-column,
	.payment-methods-desktop-action {
		flex: 0 0 50%;
	}

	.payment-methods-desktop-action {
		text-align: right;
	}

	.secondary-action-link {
		display: inline-block;
		padding: 8px 16px;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.15s ease;
		border: 1px solid #dee2e6;
	}

	.secondary-action-link {
		background: #fff;
		color: #0984ae;
		border-color: #0984ae;
		padding: 6px 12px;
		font-size: 13px;
	}

	.secondary-action-link:hover {
		background: #0984ae;
		color: #fff;
	}

	.payment-methods-table {
		width: 100%;
		border-collapse: collapse;
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
		margin-top: 20px;
	}

	.payment-methods-table thead {
		background: #f8f9fa;
		border-bottom: 2px solid #e9ecef;
	}

	.payment-methods-table thead th {
		padding: 16px 20px;
		font-size: 14px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: #495057;
		text-align: left;
	}

	.payment-methods-table tbody tr {
		border-bottom: 1px solid #e9ecef;
		transition: background-color 0.15s ease;
	}

	.payment-methods-table tbody tr:hover {
		background-color: #f8f9fa;
	}

	.payment-methods-table tbody tr:last-child {
		border-bottom: none;
	}

	.payment-methods-table tbody tr.default-payment-method {
		background-color: #f0f8ff;
	}

	.payment-methods-table tbody td {
		padding: 16px 20px;
		font-size: 14px;
		color: #495057;
		vertical-align: middle;
	}

	.payment-methods-table tbody td mark.default {
		background: #28a745;
		color: #fff;
		padding: 4px 8px;
		border-radius: 4px;
		font-size: 12px;
		font-weight: 600;
	}

	.payment-methods-table tbody td .button {
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

	.payment-methods-table tbody td .button:hover {
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

	.payment-methods-mobile-action {
		display: none;
	}

	.payment-methods-desktop-action {
		display: block;
	}

	@media (max-width: 767.98px) {
		.payment-methods-card-header {
			flex-direction: column;
			align-items: flex-start;
		}

		.payment-methods-title-column,
		.payment-methods-desktop-action {
			flex: 0 0 100%;
			width: 100%;
		}

		.payment-methods-desktop-action {
			text-align: left;
		}

		.payment-methods-desktop-action {
			display: none;
		}

		.payment-methods-mobile-action {
			display: block;
			margin-bottom: 20px;
		}

		.payment-methods-table {
			display: block;
			overflow-x: auto;
		}

		.payment-methods-table thead {
			display: none;
		}

		.payment-methods-table tbody tr {
			display: block;
			margin-bottom: 16px;
			border: 1px solid #e9ecef;
			border-radius: 8px;
			padding: 16px;
		}

		.payment-methods-table tbody td {
			display: flex;
			justify-content: space-between;
			align-items: center;
			padding: 8px 0;
			border: none;
		}

		.payment-methods-table tbody td::before {
			content: attr(data-title);
			font-weight: 600;
			text-transform: uppercase;
			font-size: 12px;
			color: #6c757d;
		}
	}
</style>

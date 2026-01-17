<script lang="ts">
	import { enhance } from '$app/forms';

	interface PaymentHistory {
		id: string;
		amount: number;
		status: string;
		paymentDate: string;
		dueDate: string;
		paymentMethod: string;
	}

	interface Subscription {
		id: string;
		userId: string;
		productId: string;
		productName: string;
		planId: string;
		status: string;
		interval: string;
		price: number;
		currency: string;
		startDate: string;
		nextPaymentDate?: string;
		lastPaymentDate?: string;
		endDate?: string;
		cancelledAt?: string;
		pausedAt?: string;
		totalPaid: number;
		failedPayments: number;
		successfulPayments: number;
		paymentHistory: PaymentHistory[];
		pauseReason?: string;
		cancellationReason?: string;
		renewalCount: number;
		autoRenew: boolean;
		trialEndDate?: string;
		isTrialing: boolean;
		paymentMethod: {
			type: string;
			last4?: string;
			brand?: string;
		};
		notes?: string;
		mrr: number;
		arr: number;
		ltv: number;
	}

	interface PageData {
		subscription: Subscription;
	}

	interface ActionData {
		success?: boolean;
		message?: string;
		error?: string;
	}

	let { data, form }: { data: PageData; form?: ActionData } = $props();

	let subscription = $derived(data.subscription);
	let isSubmitting = $state(false);

	function formatDate(dateString: string | undefined): string {
		if (!dateString) return 'N/A';
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}

	function getStatusClass(status: string): string {
		const statusMap: Record<string, string> = {
			active: 'status-active',
			trial: 'status-trial',
			'on-hold': 'status-on-hold',
			paused: 'status-paused',
			cancelled: 'status-cancelled',
			expired: 'status-expired',
			'pending-cancel': 'status-pending-cancel',
			'past-due': 'status-past-due'
		};
		return statusMap[status.toLowerCase()] || 'status-default';
	}

	function canCancel(sub: Subscription): boolean {
		return ['active', 'trial', 'on-hold', 'paused'].includes(sub.status.toLowerCase());
	}

	function canPause(sub: Subscription): boolean {
		return ['active', 'trial'].includes(sub.status.toLowerCase());
	}

	function formatCurrency(amount: number, currency: string = 'USD'): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: currency
		}).format(amount);
	}
</script>

<svelte:head>
	<title>Subscription #{subscription.id} - Revolution Trading Pros</title>
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
		<section class="dashboard__content-section">
			<div class="subscription-container">
				<div class="subscription-content">
					<div class="woocommerce">
						<div class="woocommerce-MyAccount-content">
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

							<h2 class="section-title">Subscription #{subscription.id}</h2>

							<div class="content-box u--margin-bottom-20">
								<div class="content-box__section">
									<table class="shop_table subscription_details u--margin-bottom-0">
										<tbody>
											<tr>
												<td>Status</td>
												<td class={getStatusClass(subscription.status)}>
													{subscription.status}
												</td>
											</tr>
											<tr>
												<td>Start Date</td>
												<td>{formatDate(subscription.startDate)}</td>
											</tr>
											{#if subscription.lastPaymentDate}
												<tr>
													<td>Last Payment Date</td>
													<td>{formatDate(subscription.lastPaymentDate)}</td>
												</tr>
											{/if}
											{#if subscription.nextPaymentDate}
												<tr>
													<td>Next Payment Date</td>
													<td>{formatDate(subscription.nextPaymentDate)}</td>
												</tr>
											{/if}
											{#if subscription.trialEndDate}
												<tr>
													<td>Trial End Date</td>
													<td>{formatDate(subscription.trialEndDate)}</td>
												</tr>
											{/if}
											<tr>
												<td>Price</td>
												<td
													>{formatCurrency(subscription.price, subscription.currency)} / {subscription.interval}</td
												>
											</tr>
											<tr>
												<td>Total Paid</td>
												<td>{formatCurrency(subscription.totalPaid, subscription.currency)}</td>
											</tr>
											<tr>
												<td>Actions</td>
												<td class="subscription-actions">
													{#if canCancel(subscription)}
														<form
															method="post"
															action="?/cancel"
															use:enhance={() => {
																isSubmitting = true;
																return async ({ update }) => {
																	await update();
																	isSubmitting = false;
																};
															}}
															style="display: inline;"
														>
															<button
																type="submit"
																class="btn btn-xs btn-default cancel"
																disabled={isSubmitting}
															>
																Cancel
															</button>
														</form>
													{/if}
													{#if canPause(subscription)}
														<form
															method="post"
															action="?/pause"
															use:enhance={() => {
																isSubmitting = true;
																return async ({ update }) => {
																	await update();
																	isSubmitting = false;
																};
															}}
															style="display: inline;"
														>
															<button
																type="submit"
																class="btn btn-xs btn-default suspend"
																disabled={isSubmitting}
															>
																Pause
															</button>
														</form>
													{/if}
													<a
														href="/dashboard/account/payment-methods"
														class="btn btn-xs btn-default change_payment_method"
													>
														Change payment
													</a>
												</td>
											</tr>
										</tbody>
									</table>
								</div>
							</div>

							{#if subscription.paymentHistory && subscription.paymentHistory.length > 0}
								<div class="content-box u--margin-bottom-20">
									<div class="content-box__section">
										<header>
											<h2>Payment History</h2>
										</header>

										<table class="shop_table shop_table_responsive">
											<thead>
												<tr>
													<th>Payment Date</th>
													<th>Amount</th>
													<th>Status</th>
													<th>Method</th>
												</tr>
											</thead>

											<tbody>
												{#each subscription.paymentHistory as payment (payment.id)}
													<tr>
														<td data-title="Payment Date">
															{formatDate(payment.paymentDate)}
														</td>
														<td data-title="Amount">
															{formatCurrency(payment.amount, subscription.currency)}
														</td>
														<td data-title="Status" style="text-transform: capitalize;">
															{payment.status}
														</td>
														<td data-title="Method" style="text-transform: capitalize;">
															{payment.paymentMethod}
														</td>
													</tr>
												{/each}
											</tbody>
										</table>
									</div>
								</div>
							{/if}
						</div>
					</div>
				</div>
			</div>
		</section>
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
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 28px;
		font-weight: 700;
		color: #0a2335;
		margin: 0;
		line-height: 1.2;
	}

	/* Dashboard Content */
	.dashboard__content {
		background: #f5f5f5;
		min-height: calc(100vh - 60px);
		padding: 30px;
	}

	.dashboard__content-main {
		max-width: 1200px;
		margin: 0 auto;
	}

	.dashboard__content-section {
		padding: 0;
	}

	/* Subscription Container - Semantic naming */
	.subscription-container {
		width: 100%;
		max-width: 1100px;
		margin: 0 auto;
		background-color: #ffffff;
		font-family: 'Montserrat', sans-serif;
	}

	.subscription-content {
		line-height: 1.6;
	}

	/* Section Title */
	.section-title {
		font-family: 'Montserrat', sans-serif;
		font-size: 24px;
		font-weight: 700;
		margin-bottom: 20px;
		color: #333;
	}

	/* WooCommerce Content */
	.woocommerce {
		background: #fff;
		font-family: 'Montserrat', sans-serif;
	}

	.woocommerce-MyAccount-content {
		padding: 0;
	}

	.woocommerce-notices-wrapper {
		margin-bottom: 20px;
	}

	.woocommerce-message,
	.woocommerce-error {
		padding: 16px 20px;
		border-radius: 4px;
		margin-bottom: 20px;
		font-size: 14px;
	}

	.woocommerce-message {
		background: #d4edda;
		color: #155724;
		border: 1px solid #c3e6cb;
	}

	.woocommerce-error {
		background: #f8d7da;
		color: #721c24;
		border: 1px solid #f5c6cb;
	}

	.content-box {
		background: #fff;
		border: 1px solid #e9ecef;
		border-radius: 8px;
		margin-bottom: 20px;
	}

	.content-box__section {
		padding: 24px;
	}

	.u--margin-bottom-20 {
		margin-bottom: 20px;
	}

	.u--margin-bottom-0 {
		margin-bottom: 0;
	}

	.shop_table {
		width: 100%;
		border-collapse: collapse;
	}

	.subscription_details td {
		padding: 12px 16px;
		border-bottom: 1px solid #e9ecef;
		font-size: 14px;
	}

	.subscription_details td:first-child {
		font-weight: 600;
		color: #495057;
		width: 200px;
	}

	.subscription_details td:last-child {
		color: #333;
	}

	.subscription_details tr:last-child td {
		border-bottom: none;
	}

	.status-active {
		color: #28a745;
		font-weight: 600;
		text-transform: capitalize;
	}

	.status-on-hold {
		color: #ffc107;
		font-weight: 600;
		text-transform: capitalize;
	}

	.status-cancelled,
	.status-expired,
	.status-pending-cancel {
		color: #dc3545;
		font-weight: 600;
		text-transform: capitalize;
	}

	.subscription-actions {
		display: flex;
		gap: 8px;
		flex-wrap: wrap;
	}

	.btn {
		display: inline-block;
		padding: 6px 12px;
		font-size: 13px;
		font-weight: 600;
		border-radius: 4px;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s ease;
		border: 1px solid #ddd;
		background: #fff;
		color: #333;
	}

	.btn:hover:not(:disabled) {
		background: #f8f9fa;
		border-color: #adb5bd;
	}

	.btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn.cancel {
		color: #dc3545;
		border-color: #dc3545;
	}

	.btn.cancel:hover:not(:disabled) {
		background: #dc3545;
		color: #fff;
	}

	.btn.suspend {
		color: #ffc107;
		border-color: #ffc107;
	}

	.btn.suspend:hover:not(:disabled) {
		background: #ffc107;
		color: #333;
	}

	header h2 {
		font-size: 18px;
		font-weight: 600;
		margin: 0 0 20px 0;
		color: #333;
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

	@media (max-width: 768px) {
		.subscription_details td:first-child {
			width: auto;
		}

		.subscription-actions {
			flex-direction: column;
		}

		.btn {
			width: 100%;
			text-align: center;
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

<script lang="ts">
	/**
	 * View Subscription - Account Section
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Subscription details view page with cancel functionality
	 *
	 * @version 2.1.0 - API Integration + 100% Pixel Perfect
	 */
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { cancelSubscription } from '$lib/api/user-memberships';

	const subscriptionId = $page.params.id;

	// State for cancel flow
	let showCancelModal = $state(false);
	let cancelReason = $state('');
	let cancelling = $state(false);
	let cancelError = $state('');

	// Sample subscription data (will be replaced with API fetch)
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

	// Handle cancel subscription
	async function handleCancelSubscription() {
		cancelling = true;
		cancelError = '';

		try {
			await cancelSubscription(subscriptionId, {
				cancel_immediately: false,
				reason: cancelReason || undefined
			});

			// Success - redirect back to subscriptions
			showCancelModal = false;
			goto('/dashboard/account/subscriptions?cancelled=true');
		} catch (err) {
			cancelError = err instanceof Error ? err.message : 'Failed to cancel subscription';
		} finally {
			cancelling = false;
		}
	}
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
					<button class="btn btn-warning" onclick={() => showCancelModal = true}>Cancel Subscription</button>
				</div>
			</div>

			<!-- Cancel Confirmation Modal -->
			{#if showCancelModal}
				<div class="modal-overlay" onclick={() => showCancelModal = false} role="presentation">
					<div class="modal" onclick={(e: MouseEvent) => e.stopPropagation()} role="dialog" aria-modal="true" aria-labelledby="cancel-title" tabindex="-1">
						<h3 id="cancel-title">Cancel Subscription</h3>
						<p>Are you sure you want to cancel your subscription to <strong>{subscription.product}</strong>?</p>
						<p class="modal-note">Your subscription will remain active until the end of your current billing period.</p>

						<div class="form-group">
							<label for="cancel-reason">Reason for cancellation (optional)</label>
							<textarea id="cancel-reason" bind:value={cancelReason} rows="3" placeholder="Please let us know why you're canceling..."></textarea>
						</div>

						{#if cancelError}
							<div class="error-message">{cancelError}</div>
						{/if}

						<div class="modal-actions">
							<button class="btn btn-default" onclick={() => showCancelModal = false} disabled={cancelling}>Keep Subscription</button>
							<button class="btn btn-danger" onclick={handleCancelSubscription} disabled={cancelling}>
								{cancelling ? 'Canceling...' : 'Confirm Cancellation'}
							</button>
						</div>
					</div>
				</div>
			{/if}

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

	.btn-danger {
		background: #dc3545;
		color: #fff;
		border: 1px solid #dc3545;
	}

	.btn-danger:hover {
		background: #c82333;
	}

	.btn-danger:disabled,
	.btn-default:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* Modal styles */
	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal {
		background: #fff;
		border-radius: 8px;
		padding: 30px;
		max-width: 500px;
		width: 90%;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
	}

	.modal h3 {
		font-size: 20px;
		font-weight: 700;
		margin: 0 0 15px;
		color: #333;
	}

	.modal p {
		font-size: 14px;
		color: #666;
		margin: 0 0 10px;
	}

	.modal-note {
		font-size: 13px;
		color: #999;
		margin-bottom: 20px !important;
	}

	.form-group {
		margin-bottom: 20px;
	}

	.form-group label {
		display: block;
		font-size: 14px;
		font-weight: 600;
		color: #333;
		margin-bottom: 8px;
	}

	.form-group textarea {
		width: 100%;
		padding: 10px 12px;
		font-size: 14px;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		resize: vertical;
		font-family: inherit;
	}

	.form-group textarea:focus {
		outline: none;
		border-color: #0984ae;
	}

	.error-message {
		background: #f8d7da;
		color: #721c24;
		padding: 10px 15px;
		border-radius: 4px;
		font-size: 14px;
		margin-bottom: 20px;
	}

	.modal-actions {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
	}
</style>

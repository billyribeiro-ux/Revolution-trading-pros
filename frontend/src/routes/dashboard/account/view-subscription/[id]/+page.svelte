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

	const subscriptionId = $page.params.id!;

	// State for cancel survey flow
	let showCancelModal = $state(false);
	let cancelStep = $state(1);
	let cancelling = $state(false);
	let cancelError = $state('');
	let stepError = $state('');

	// Cancel survey answers
	let cancelReason = $state('');
	let cancelReasonOther = $state('');
	let rejoinLikelihood = $state(0);
	let additionalFeedback = $state('');

	// Pause modal state
	let showPauseModal = $state(false);
	let pauseDuration = $state(7);
	let pausing = $state(false);

	const cancelReasons = [
		'Trading styles and strategies are too advanced',
		'Subscription is out of my budget',
		"I don't have time to use the service",
		"Trading styles don't align with my trading style",
		"I'm not making money",
		'I want to see how I fare on my own',
		'Other'
	];

	function openCancelModal() {
		showCancelModal = true;
		cancelStep = 1;
		cancelReason = '';
		cancelReasonOther = '';
		rejoinLikelihood = 0;
		additionalFeedback = '';
		stepError = '';
	}

	function closeCancelModal() {
		showCancelModal = false;
		cancelStep = 1;
	}

	function nextStep() {
		stepError = '';

		// Validate current step
		if (cancelStep === 1 && !cancelReason) {
			stepError = 'Please select an option';
			return;
		}
		if (cancelStep === 4 && rejoinLikelihood === 0) {
			stepError = 'Please select an option';
			return;
		}

		if (cancelStep < 5) {
			cancelStep++;
		} else {
			handleCancelSubscription();
		}
	}

	function openPauseModal() {
		showCancelModal = false;
		showPauseModal = true;
	}

	async function handlePauseSubscription() {
		pausing = true;
		try {
			// Simulate API call
			await new Promise((resolve) => setTimeout(resolve, 1000));
			showPauseModal = false;
			goto('/dashboard/account/subscriptions?paused=true');
		} catch (err) {
			// Handle error
		} finally {
			pausing = false;
		}
	}

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

	// State for reactivation
	let showReactivateModal = $state(false);
	let reactivating = $state(false);
	let reactivateError = $state('');

	// Handle cancel subscription
	async function handleCancelSubscription() {
		cancelling = true;
		cancelError = '';

		try {
			const reason = cancelReason === 'Other' ? cancelReasonOther : cancelReason;
			await cancelSubscription(subscriptionId, {
				cancel_immediately: false,
				reason: reason || undefined,
				survey: {
					cancel_reason: reason,
					rejoin_likelihood: rejoinLikelihood,
					additional_feedback: additionalFeedback
				}
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

	// Handle reactivate subscription
	async function handleReactivateSubscription() {
		reactivating = true;
		reactivateError = '';

		try {
			// Simulate API call - in production this would call the backend
			await new Promise((resolve) => setTimeout(resolve, 1000));

			// Success - redirect back to subscriptions
			showReactivateModal = false;
			goto('/dashboard/account/subscriptions?reactivated=true');
		} catch (err) {
			reactivateError = err instanceof Error ? err.message : 'Failed to reactivate subscription';
		} finally {
			reactivating = false;
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
					{#if subscription.status === 'Active'}
						<button class="btn btn-warning" onclick={openCancelModal}>Cancel Subscription</button>
					{:else if subscription.status === 'Cancelled' || subscription.status === 'Pending Cancellation'}
						<button class="btn btn-success" onclick={() => showReactivateModal = true}>Reactivate Subscription</button>
					{/if}
				</div>
			</div>

			<!-- Cancel Survey Modal -->
			{#if showCancelModal}
				<div class="modal-overlay" onclick={closeCancelModal} role="presentation">
					<div class="survey-modal" onclick={(e: MouseEvent) => e.stopPropagation()} role="dialog" aria-modal="true" aria-labelledby="cancel-title" tabindex="-1">
						<button class="modal-close" onclick={closeCancelModal} aria-label="Close">
							<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 16 16" fill="currentColor">
								<path d="M2.146 2.854a.5.5 0 1 1 .708-.708L8 7.293l5.146-5.147a.5.5 0 0 1 .708.708L8.707 8l5.147 5.146a.5.5 0 0 1-.708.708L8 8.707l-5.146 5.147a.5.5 0 0 1-.708-.708L7.293 8 2.146 2.854Z"/>
							</svg>
						</button>

						<!-- Step 1: Why canceling? -->
						{#if cancelStep === 1}
							<div class="survey-step">
								<p class="survey-question"><strong>Please tell us why you are choosing to cancel your subscription</strong></p>
								<select class="survey-select" bind:value={cancelReason}>
									<option value="">Select</option>
									{#each cancelReasons as reason}
										<option value={reason}>{reason}</option>
									{/each}
								</select>

								{#if cancelReason === 'Other'}
									<div class="form-group mt-3">
										<textarea bind:value={cancelReasonOther} rows="3" placeholder="Please tell us more..."></textarea>
									</div>
								{/if}

								{#if stepError}
									<div class="step-error">{stepError}</div>
								{/if}

								<div class="survey-actions">
									<button class="btn btn-white" onclick={nextStep}>Continue</button>
								</div>
							</div>
						{/if}

						<!-- Step 2: Are you sure? -->
						{#if cancelStep === 2}
							<div class="survey-step">
								<p class="survey-question"><strong>Are you sure you want to cancel?</strong></p>
								<p class="survey-warning">By cancelling your subscription, you'll lose your grandfathered subscription discount.</p>
								<p class="survey-note">If you cancel and rejoin, you may pay a higher rate.</p>

								<div class="survey-buttons-row">
									<button class="btn btn-orange" onclick={closeCancelModal}>Nevermind, I don't want to cancel</button>
									<button class="btn btn-white" onclick={nextStep}>Yes, please cancel</button>
								</div>
							</div>
						{/if}

						<!-- Step 3: Pause option -->
						{#if cancelStep === 3}
							<div class="survey-step">
								<p class="survey-question"><strong>Did you know you could pause your subscription?</strong></p>
								<p>Keep your grandfathered subscription rates by pausing your subscription.</p>
								<p class="survey-note">You can pause your subscription up to 30 days.</p>

								<div class="survey-buttons-row">
									<button class="btn btn-orange" onclick={openPauseModal}>Pause my subscription</button>
									<button class="btn btn-white" onclick={nextStep}>Cancel my subscription</button>
								</div>
							</div>
						{/if}

						<!-- Step 4: Rejoin likelihood -->
						{#if cancelStep === 4}
							<div class="survey-step">
								<p class="survey-question"><strong>How likely are you to rejoin? 1-5 likely scale</strong></p>
								<p class="survey-note">1 = not at all, 2 = slightly, 3 = maybe, 4 = very, 5 = extremely</p>

								<div class="rating-buttons">
									{#each [1, 2, 3, 4, 5] as rating}
										<label class="rating-option">
											<input type="radio" name="rejoin-rating" value={rating} bind:group={rejoinLikelihood} />
											<span class="rating-label">{rating}</span>
										</label>
									{/each}
								</div>

								{#if stepError}
									<div class="step-error">{stepError}</div>
								{/if}

								<div class="survey-actions">
									<button class="btn btn-white" onclick={nextStep}>Continue</button>
								</div>
							</div>
						{/if}

						<!-- Step 5: Additional feedback -->
						{#if cancelStep === 5}
							<div class="survey-step">
								<p class="survey-question"><strong>Is there any other feedback?</strong></p>

								<div class="form-group">
									<textarea bind:value={additionalFeedback} rows="4" placeholder="Any additional comments..."></textarea>
								</div>

								{#if cancelError}
									<div class="error-message">{cancelError}</div>
								{/if}

								<div class="survey-actions">
									<button class="btn btn-white" onclick={nextStep} disabled={cancelling}>
										{cancelling ? 'Processing...' : 'Continue'}
									</button>
								</div>
							</div>
						{/if}
					</div>
				</div>
			{/if}

			<!-- Pause Subscription Modal -->
			{#if showPauseModal}
				<div class="modal-overlay" onclick={() => showPauseModal = false} role="presentation">
					<div class="modal" onclick={(e: MouseEvent) => e.stopPropagation()} role="dialog" aria-modal="true" aria-labelledby="pause-title" tabindex="-1">
						<h3 id="pause-title">Pause Subscription</h3>
						<p>How long would you like to pause your subscription?</p>

						<div class="form-group">
							<label for="pause-duration">Pause duration</label>
							<select id="pause-duration" class="survey-select" bind:value={pauseDuration}>
								<option value={7}>1 week</option>
								<option value={14}>2 weeks</option>
								<option value={21}>3 weeks</option>
								<option value={30}>30 days</option>
							</select>
						</div>

						<div class="modal-actions">
							<button class="btn btn-default" onclick={() => showPauseModal = false} disabled={pausing}>Cancel</button>
							<button class="btn btn-orange" onclick={handlePauseSubscription} disabled={pausing}>
								{pausing ? 'Pausing...' : 'Pause Subscription'}
							</button>
						</div>
					</div>
				</div>
			{/if}

			<!-- Reactivate Confirmation Modal -->
			{#if showReactivateModal}
				<div class="modal-overlay" onclick={() => showReactivateModal = false} role="presentation">
					<div class="modal" onclick={(e: MouseEvent) => e.stopPropagation()} role="dialog" aria-modal="true" aria-labelledby="reactivate-title" tabindex="-1">
						<h3 id="reactivate-title">Reactivate Subscription</h3>
						<p>Would you like to reactivate your subscription to <strong>{subscription.product}</strong>?</p>
						<p class="modal-note">Your subscription will resume and you will be billed on the next billing cycle.</p>

						{#if reactivateError}
							<div class="error-message">{reactivateError}</div>
						{/if}

						<div class="modal-actions">
							<button class="btn btn-default" onclick={() => showReactivateModal = false} disabled={reactivating}>Cancel</button>
							<button class="btn btn-success" onclick={handleReactivateSubscription} disabled={reactivating}>
								{reactivating ? 'Reactivating...' : 'Confirm Reactivation'}
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

	.btn-success {
		background: #28a745;
		color: #fff;
		border: 1px solid #28a745;
	}

	.btn-success:hover {
		background: #218838;
	}

	.btn-danger:disabled,
	.btn-default:disabled,
	.btn-success:disabled {
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

	/* Survey Modal Styles */
	.survey-modal {
		background: #fff;
		border-radius: 12px;
		padding: 40px;
		max-width: 480px;
		width: 90%;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
		position: relative;
		text-align: center;
	}

	.modal-close {
		position: absolute;
		top: 15px;
		right: 15px;
		background: none;
		border: none;
		color: #0984ae;
		cursor: pointer;
		padding: 5px;
	}

	.modal-close:hover {
		color: #076787;
	}

	.survey-step {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.survey-question {
		font-size: 18px;
		color: #333;
		margin-bottom: 20px;
	}

	.survey-warning {
		font-size: 14px;
		color: #666;
		margin-bottom: 10px;
	}

	.survey-note {
		font-size: 13px;
		color: #999;
		margin-bottom: 25px;
	}

	.survey-select {
		width: 100%;
		max-width: 300px;
		padding: 12px 14px;
		font-size: 14px;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		background: #fff;
		margin-bottom: 20px;
	}

	.survey-select:focus {
		outline: none;
		border-color: #0984ae;
	}

	.survey-actions {
		margin-top: 10px;
	}

	.survey-buttons-row {
		display: flex;
		flex-direction: column;
		gap: 12px;
		width: 100%;
		max-width: 300px;
	}

	.btn-orange {
		background: #f7931a;
		color: #fff;
		border: 1px solid #f7931a;
	}

	.btn-orange:hover {
		background: #e8850f;
	}

	.btn-orange:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-white {
		background: #fff;
		color: #333;
		border: 1px solid #dbdbdb;
	}

	.btn-white:hover {
		background: #f5f5f5;
	}

	.btn-white:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.rating-buttons {
		display: flex;
		justify-content: center;
		gap: 15px;
		margin-bottom: 25px;
	}

	.rating-option {
		display: flex;
		flex-direction: column;
		align-items: center;
		cursor: pointer;
	}

	.rating-option input[type="radio"] {
		appearance: none;
		width: 40px;
		height: 40px;
		border: 2px solid #dbdbdb;
		border-radius: 50%;
		margin-bottom: 5px;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.rating-option input[type="radio"]:checked {
		background: #0984ae;
		border-color: #0984ae;
	}

	.rating-option input[type="radio"]:hover {
		border-color: #0984ae;
	}

	.rating-label {
		font-size: 14px;
		color: #666;
	}

	.step-error {
		background: #f8d7da;
		color: #721c24;
		padding: 10px 20px;
		border-radius: 4px;
		font-size: 14px;
		margin: 15px 0;
		width: 100%;
		max-width: 300px;
	}

	.mt-3 {
		margin-top: 15px;
	}
</style>

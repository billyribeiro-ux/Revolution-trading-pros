<script lang="ts">
	/**
	 * SubscriptionDetailDrawer - Full Subscription Profile Drawer
	 * Revolution Trading Pros - Apple ICT 11+ Principal Engineer Grade
	 *
	 * Enterprise-grade slide-out drawer showing complete subscription details
	 * with tabs for billing, usage, and history.
	 */
	import type {
		Subscription,
		SubscriptionStatus,
		SubscriptionPayment
	} from '$lib/stores/subscriptions.svelte';
	import {
		pauseSubscription,
		resumeSubscription,
		cancelSubscription,
		reactivateSubscription,
		retryPayment,
		getPaymentHistory
	} from '$lib/api/subscriptions';
	import {
		IconX,
		IconCreditCard,
		IconCalendar,
		IconEdit,
		IconPlayerPause,
		IconPlayerPlay,
		IconBan,
		IconRefresh,
		IconReceipt,
		IconChartBar,
		IconClock,
		IconCheck,
		IconAlertTriangle,
		IconMail,
		IconCurrencyDollar
	} from '$lib/icons';
	import ConfirmationModal from './ConfirmationModal.svelte';

	interface Props {
		isOpen: boolean;
		subscription: Subscription | null;
		onClose: () => void;
		onEdit?: (subscription: Subscription) => void;
		onRefresh?: () => void;
	}

	let {
		isOpen,
		subscription,
		onClose,
		onEdit,
		onRefresh
	}: Props = $props();

	// State
	let isLoading = $state(false);
	let error = $state('');
	let activeTab = $state<'billing' | 'usage' | 'history'>('billing');
	let paymentHistory = $state<SubscriptionPayment[]>([]);
	let historyLoading = $state(false);

	// Action modals
	let showPauseModal = $state(false);
	let showCancelModal = $state(false);
	let showReactivateModal = $state(false);
	let actionReason = $state('');
	let cancelImmediate = $state(false);
	let isProcessingAction = $state(false);

	// Load payment history when switching to history tab
	$effect(() => {
		if (isOpen && subscription && activeTab === 'history') {
			loadPaymentHistory();
		}
	});

	$effect(() => {
		if (isOpen) {
			document.body.style.overflow = 'hidden';
			activeTab = 'billing';
			error = '';
		} else {
			document.body.style.overflow = '';
		}
		return () => {
			document.body.style.overflow = '';
		};
	});

	async function loadPaymentHistory() {
		if (!subscription) return;

		historyLoading = true;
		try {
			const history = await getPaymentHistory(subscription.id);
			paymentHistory = history.map(h => ({
				id: h.id,
				amount: h.amount,
				status: h.status === 'succeeded' ? 'paid' : h.status === 'failed' ? 'failed' : 'pending',
				paymentDate: h.createdAt,
				dueDate: h.createdAt,
				paymentMethod: typeof h.method === 'string' ? h.method : h.method?.type || 'card'
			}));
		} catch (err) {
			// Use existing payment history from subscription if API fails
			paymentHistory = subscription.paymentHistory || [];
		} finally {
			historyLoading = false;
		}
	}

	async function handlePause() {
		if (!subscription) return;

		isProcessingAction = true;
		try {
			await pauseSubscription(subscription.id, actionReason);
			showPauseModal = false;
			actionReason = '';
			onRefresh?.();
			onClose();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to pause subscription';
		} finally {
			isProcessingAction = false;
		}
	}

	async function handleResume() {
		if (!subscription) return;

		isProcessingAction = true;
		try {
			await resumeSubscription(subscription.id);
			onRefresh?.();
			onClose();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to resume subscription';
		} finally {
			isProcessingAction = false;
		}
	}

	async function handleCancel() {
		if (!subscription) return;

		isProcessingAction = true;
		try {
			await cancelSubscription(subscription.id, actionReason, cancelImmediate);
			showCancelModal = false;
			actionReason = '';
			cancelImmediate = false;
			onRefresh?.();
			onClose();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to cancel subscription';
		} finally {
			isProcessingAction = false;
		}
	}

	async function handleReactivate() {
		if (!subscription) return;

		isProcessingAction = true;
		try {
			await reactivateSubscription(subscription.id);
			showReactivateModal = false;
			onRefresh?.();
			onClose();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to reactivate subscription';
		} finally {
			isProcessingAction = false;
		}
	}

	async function handleRetryPayment(paymentId: string) {
		if (!subscription) return;

		isLoading = true;
		try {
			await retryPayment(subscription.id, paymentId);
			await loadPaymentHistory();
			onRefresh?.();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to retry payment';
		} finally {
			isLoading = false;
		}
	}

	function formatDate(dateStr: string | null | undefined): string {
		if (!dateStr) return 'N/A';
		return new Date(dateStr).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function formatCurrency(amount: number | null | undefined): string {
		if (amount === null || amount === undefined) return '$0.00';
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD'
		}).format(amount);
	}

	function getStatusColor(status: SubscriptionStatus | string): string {
		switch (status) {
			case 'active':
				return 'var(--admin-success)';
			case 'trial':
				return 'var(--admin-accent-primary)';
			case 'pending':
				return 'var(--admin-warning)';
			case 'on-hold':
				return 'var(--admin-warning)';
			case 'cancelled':
			case 'expired':
				return 'var(--admin-error)';
			case 'pending-cancel':
				return 'var(--admin-error)';
			default:
				return 'var(--admin-text-muted)';
		}
	}

	function getPaymentStatusColor(status: string): string {
		switch (status) {
			case 'paid':
			case 'succeeded':
				return 'var(--admin-success)';
			case 'pending':
			case 'processing':
				return 'var(--admin-warning)';
			case 'failed':
				return 'var(--admin-error)';
			case 'refunded':
			case 'partially-refunded':
				return 'var(--admin-accent-primary)';
			default:
				return 'var(--admin-text-muted)';
		}
	}

	function getIntervalLabel(interval: string): string {
		switch (interval) {
			case 'monthly': return 'Monthly';
			case 'quarterly': return 'Quarterly';
			case 'yearly': return 'Yearly';
			default: return interval.charAt(0).toUpperCase() + interval.slice(1);
		}
	}

	function handleBackdropClick(e: MouseEvent | KeyboardEvent) {
		if (e.target === e.currentTarget) {
			onClose();
		}
	}
</script>

{#if isOpen && subscription}
	<div class="drawer-backdrop" role="presentation" onclick={handleBackdropClick} onkeydown={(e) => { if (e.key === 'Escape') handleBackdropClick(e); }}>
		<aside class="drawer" class:open={isOpen}>
			<!-- Header -->
			<header class="drawer-header">
				<div class="subscription-icon">
					<IconCreditCard size={24} />
				</div>
				<div class="subscription-info">
					<h2 class="subscription-name">{subscription.productName}</h2>
					<p class="subscription-id">ID: {subscription.id.slice(0, 12)}...</p>
					<div class="subscription-badges">
						<span
							class="status-badge"
							style="--badge-color: {getStatusColor(subscription.status)}"
						>
							{subscription.status}
						</span>
						<span class="interval-badge">{getIntervalLabel(subscription.interval)}</span>
					</div>
				</div>
				<button type="button" class="btn-close" onclick={onClose} aria-label="Close">
					<IconX size={24} />
				</button>
			</header>

			{#if error}
				<div class="error-banner">
					<IconAlertTriangle size={16} />
					{error}
				</div>
			{/if}

			<!-- Quick Stats -->
			<div class="quick-stats">
				<div class="stat-item">
					<span class="stat-value">{formatCurrency(subscription.price)}</span>
					<span class="stat-label">Price/{subscription.interval?.slice(0, 2) || 'mo'}</span>
				</div>
				<div class="stat-item">
					<span class="stat-value">{formatCurrency(subscription.totalPaid)}</span>
					<span class="stat-label">Total Paid</span>
				</div>
				<div class="stat-item">
					<span class="stat-value">{subscription.renewalCount || 0}</span>
					<span class="stat-label">Renewals</span>
				</div>
				<div class="stat-item">
					<span class="stat-value">{subscription.successfulPayments || 0}</span>
					<span class="stat-label">Payments</span>
				</div>
			</div>

			<!-- Action Buttons -->
			<div class="action-buttons">
				<button type="button" class="btn-action" onclick={() => onEdit?.(subscription!)}>
					<IconEdit size={16} />
					Edit
				</button>
				{#if subscription.status === 'active'}
					<button type="button" class="btn-action warning" onclick={() => showPauseModal = true}>
						<IconPlayerPause size={16} />
						Pause
					</button>
					<button type="button" class="btn-action danger" onclick={() => showCancelModal = true}>
						<IconBan size={16} />
						Cancel
					</button>
				{:else if subscription.status === 'on-hold'}
					<button type="button" class="btn-action success" onclick={handleResume} disabled={isProcessingAction}>
						<IconPlayerPlay size={16} />
						Resume
					</button>
					<button type="button" class="btn-action danger" onclick={() => showCancelModal = true}>
						<IconBan size={16} />
						Cancel
					</button>
				{:else if subscription.status === 'cancelled' || subscription.status === 'expired'}
					<button type="button" class="btn-action success" onclick={() => showReactivateModal = true}>
						<IconRefresh size={16} />
						Reactivate
					</button>
				{:else if subscription.status === 'pending-cancel'}
					<button type="button" class="btn-action success" onclick={() => showReactivateModal = true}>
						<IconRefresh size={16} />
						Keep Active
					</button>
				{/if}
			</div>

			<!-- Tabs -->
			<nav class="drawer-tabs">
				<button
					type="button"
					class="tab"
					class:active={activeTab === 'billing'}
					onclick={() => activeTab = 'billing'}
				>
					<IconCurrencyDollar size={16} />
					Billing
				</button>
				<button
					type="button"
					class="tab"
					class:active={activeTab === 'usage'}
					onclick={() => activeTab = 'usage'}
				>
					<IconChartBar size={16} />
					Usage
				</button>
				<button
					type="button"
					class="tab"
					class:active={activeTab === 'history'}
					onclick={() => activeTab = 'history'}
				>
					<IconReceipt size={16} />
					History
				</button>
			</nav>

			<!-- Tab Content -->
			<div class="drawer-content">
				{#if activeTab === 'billing'}
					<div class="tab-content">
						<section class="info-section">
							<h3 class="section-title">Billing Details</h3>
							<div class="info-grid">
								<div class="info-item">
									<IconCurrencyDollar size={16} />
									<div>
										<span class="info-label">Amount</span>
										<span class="info-value">{formatCurrency(subscription.price)}</span>
									</div>
								</div>
								<div class="info-item">
									<IconCalendar size={16} />
									<div>
										<span class="info-label">Billing Cycle</span>
										<span class="info-value">{getIntervalLabel(subscription.interval)}</span>
									</div>
								</div>
								<div class="info-item">
									<IconCalendar size={16} />
									<div>
										<span class="info-label">Start Date</span>
										<span class="info-value">{formatDate(subscription.startDate)}</span>
									</div>
								</div>
								<div class="info-item">
									<IconClock size={16} />
									<div>
										<span class="info-label">Next Payment</span>
										<span class="info-value">{formatDate(subscription.nextPaymentDate)}</span>
									</div>
								</div>
							</div>
						</section>

						<section class="info-section">
							<h3 class="section-title">Payment Method</h3>
							<div class="payment-method-card">
								{#if subscription.paymentMethod}
									<div class="payment-method-icon">
										<IconCreditCard size={24} />
									</div>
									<div class="payment-method-details">
										<span class="payment-method-type">
											{subscription.paymentMethod.brand || subscription.paymentMethod.type || 'Card'}
										</span>
										{#if subscription.paymentMethod.last4}
											<span class="payment-method-info">•••• {subscription.paymentMethod.last4}</span>
										{/if}
										{#if subscription.paymentMethod.expiryMonth && subscription.paymentMethod.expiryYear}
											<span class="payment-method-expiry">
												Expires {subscription.paymentMethod.expiryMonth}/{subscription.paymentMethod.expiryYear}
											</span>
										{/if}
									</div>
								{:else}
									<span class="no-payment-method">No payment method on file</span>
								{/if}
							</div>
						</section>

						{#if subscription.isTrialing && subscription.trialEndDate}
							<section class="info-section trial-section">
								<h3 class="section-title">
									<IconClock size={16} />
									Trial Period
								</h3>
								<div class="trial-info">
									<p>Trial ends on <strong>{formatDate(subscription.trialEndDate)}</strong></p>
								</div>
							</section>
						{/if}

						{#if subscription.status === 'cancelled' || subscription.status === 'pending-cancel'}
							<section class="info-section warning-section">
								<h3 class="section-title">
									<IconAlertTriangle size={16} />
									Cancellation Details
								</h3>
								<div class="info-grid">
									{#if subscription.cancellationReason}
										<div class="info-item full-width">
											<span class="info-label">Reason</span>
											<span class="info-value">{subscription.cancellationReason}</span>
										</div>
									{/if}
									{#if subscription.cancelledAt}
										<div class="info-item">
											<span class="info-label">Cancelled At</span>
											<span class="info-value">{formatDate(subscription.cancelledAt)}</span>
										</div>
									{/if}
									{#if subscription.endDate}
										<div class="info-item">
											<span class="info-label">Access Until</span>
											<span class="info-value">{formatDate(subscription.endDate)}</span>
										</div>
									{/if}
								</div>
							</section>
						{/if}
					</div>

				{:else if activeTab === 'usage'}
					<div class="tab-content">
						<section class="info-section">
							<h3 class="section-title">Subscription Metrics</h3>
							<div class="metrics-grid">
								<div class="metric-card">
									<div class="metric-icon success">
										<IconCheck size={20} />
									</div>
									<div class="metric-content">
										<span class="metric-value">{subscription.successfulPayments || 0}</span>
										<span class="metric-label">Successful Payments</span>
									</div>
								</div>
								<div class="metric-card">
									<div class="metric-icon error">
										<IconAlertTriangle size={20} />
									</div>
									<div class="metric-content">
										<span class="metric-value">{subscription.failedPayments || 0}</span>
										<span class="metric-label">Failed Payments</span>
									</div>
								</div>
								<div class="metric-card">
									<div class="metric-icon primary">
										<IconRefresh size={20} />
									</div>
									<div class="metric-content">
										<span class="metric-value">{subscription.renewalCount || 0}</span>
										<span class="metric-label">Renewals</span>
									</div>
								</div>
								<div class="metric-card">
									<div class="metric-icon accent">
										<IconCurrencyDollar size={20} />
									</div>
									<div class="metric-content">
										<span class="metric-value">{formatCurrency(subscription.totalPaid)}</span>
										<span class="metric-label">Lifetime Value</span>
									</div>
								</div>
							</div>
						</section>

						<section class="info-section">
							<h3 class="section-title">Activity</h3>
							<div class="activity-summary">
								<div class="activity-item">
									<span class="activity-label">Auto-Renew</span>
									<span class="activity-value" class:enabled={subscription.autoRenew}>
										{subscription.autoRenew ? 'Enabled' : 'Disabled'}
									</span>
								</div>
								<div class="activity-item">
									<span class="activity-label">Last Payment</span>
									<span class="activity-value">{formatDate(subscription.lastPaymentDate)}</span>
								</div>
								<div class="activity-item">
									<span class="activity-label">Created</span>
									<span class="activity-value">{formatDate(subscription.createdAt)}</span>
								</div>
								<div class="activity-item">
									<span class="activity-label">Updated</span>
									<span class="activity-value">{formatDate(subscription.updatedAt)}</span>
								</div>
							</div>
						</section>

						{#if subscription.emailsSent && subscription.emailsSent.length > 0}
							<section class="info-section">
								<h3 class="section-title">
									<IconMail size={16} />
									Notifications Sent
								</h3>
								<div class="emails-list">
									{#each subscription.emailsSent.slice(0, 5) as email}
										<div class="email-item">
											<span class="email-type">{email.type.replace('-', ' ')}</span>
											<span class="email-subject">{email.subject}</span>
											<span class="email-date">{formatDate(email.sentAt)}</span>
										</div>
									{/each}
								</div>
							</section>
						{/if}
					</div>

				{:else if activeTab === 'history'}
					<div class="tab-content">
						{#if historyLoading}
							<div class="loading-state">
								<div class="spinner"></div>
								<p>Loading payment history...</p>
							</div>
						{:else if paymentHistory.length === 0 && (!subscription.paymentHistory || subscription.paymentHistory.length === 0)}
							<div class="empty-state">
								<IconReceipt size={48} />
								<p>No payment history found</p>
							</div>
						{:else}
							<div class="payments-list">
								{#each (paymentHistory.length > 0 ? paymentHistory : subscription.paymentHistory || []) as payment}
									<div class="payment-card">
										<div class="payment-header">
											<span class="payment-amount">{formatCurrency(payment.amount)}</span>
											<span
												class="payment-status"
												style="--status-color: {getPaymentStatusColor(payment.status)}"
											>
												{payment.status}
											</span>
										</div>
										<div class="payment-details">
											<div class="payment-detail">
												<span class="detail-label">Date</span>
												<span class="detail-value">{formatDate(payment.paymentDate)}</span>
											</div>
											<div class="payment-detail">
												<span class="detail-label">Method</span>
												<span class="detail-value">{payment.paymentMethod}</span>
											</div>
											{#if payment.failureReason}
												<div class="payment-detail full-width">
													<span class="detail-label">Failure Reason</span>
													<span class="detail-value error">{payment.failureReason}</span>
												</div>
											{/if}
										</div>
										{#if payment.status === 'failed'}
											<button
												type="button"
												class="btn-retry-payment"
												onclick={() => handleRetryPayment(payment.id)}
												disabled={isLoading}
											>
												<IconRefresh size={14} />
												Retry Payment
											</button>
										{/if}
									</div>
								{/each}
							</div>
						{/if}
					</div>
				{/if}
			</div>
		</aside>
	</div>
{/if}

<!-- Pause Modal -->
<ConfirmationModal
	isOpen={showPauseModal}
	title="Pause Subscription"
	message="This will temporarily pause billing for this subscription. The customer will retain access until the current period ends."
	confirmText="Pause Subscription"
	variant="warning"
	isLoading={isProcessingAction}
	showInput={true}
	inputLabel="Reason (optional)"
	inputPlaceholder="Enter pause reason..."
	bind:inputValue={actionReason}
	onConfirm={handlePause}
	onCancel={() => { showPauseModal = false; actionReason = ''; }}
/>

<!-- Cancel Modal -->
<ConfirmationModal
	isOpen={showCancelModal}
	title="Cancel Subscription"
	message="This will cancel the subscription at the end of the current billing period. Enter a reason for cancellation."
	confirmText="Cancel Subscription"
	variant="danger"
	isLoading={isProcessingAction}
	showInput={true}
	inputLabel="Cancellation Reason"
	inputPlaceholder="Enter cancellation reason..."
	bind:inputValue={actionReason}
	onConfirm={handleCancel}
	onCancel={() => { showCancelModal = false; actionReason = ''; cancelImmediate = false; }}
/>

<!-- Reactivate Modal -->
<ConfirmationModal
	isOpen={showReactivateModal}
	title="Reactivate Subscription"
	message="This will reactivate the subscription and resume billing."
	confirmText="Reactivate"
	variant="success"
	isLoading={isProcessingAction}
	onConfirm={handleReactivate}
	onCancel={() => showReactivateModal = false}
/>

<style>
	.drawer-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		z-index: var(--z-modal, 1000);
		animation: fadeIn 0.2s ease;
	}

	@keyframes fadeIn {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	.drawer {
		position: fixed;
		top: 0;
		right: 0;
		width: 520px;
		max-width: 100%;
		height: 100vh;
		background: var(--admin-surface-primary);
		border-left: 1px solid var(--admin-border-subtle);
		display: flex;
		flex-direction: column;
		transform: translateX(100%);
		transition: transform 0.3s cubic-bezier(0.16, 1, 0.3, 1);
		z-index: 1001;
	}

	.drawer.open {
		transform: translateX(0);
	}

	.error-banner {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: var(--admin-error-bg);
		border: 1px solid var(--admin-error-border);
		color: var(--admin-error);
		padding: 0.75rem 1rem;
		margin: 0 1rem;
		border-radius: var(--radius-md, 0.5rem);
		font-size: 0.875rem;
	}

	/* Header */
	.drawer-header {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
		padding: 1.5rem;
		border-bottom: 1px solid var(--admin-border-subtle);
	}

	.subscription-icon {
		width: 56px;
		height: 56px;
		border-radius: var(--radius-lg, 0.75rem);
		background: linear-gradient(135deg, var(--admin-accent-primary), var(--admin-widget-purple-icon));
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		flex-shrink: 0;
	}

	.subscription-info {
		flex: 1;
		min-width: 0;
	}

	.subscription-name {
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--admin-text-primary);
		margin: 0 0 0.25rem;
	}

	.subscription-id {
		font-size: 0.8125rem;
		color: var(--admin-text-muted);
		font-family: 'JetBrains Mono', monospace;
		margin: 0 0 0.5rem;
	}

	.subscription-badges {
		display: flex;
		flex-wrap: wrap;
		gap: 0.375rem;
	}

	.status-badge {
		display: inline-flex;
		align-items: center;
		padding: 0.25rem 0.625rem;
		background: color-mix(in srgb, var(--badge-color) 15%, transparent);
		color: var(--badge-color);
		border-radius: 999px;
		font-size: 0.75rem;
		font-weight: 500;
		text-transform: capitalize;
	}

	.interval-badge {
		display: inline-flex;
		align-items: center;
		padding: 0.25rem 0.625rem;
		background: var(--admin-surface-hover);
		color: var(--admin-text-secondary);
		border-radius: 999px;
		font-size: 0.75rem;
		font-weight: 500;
	}

	.btn-close {
		background: transparent;
		border: none;
		color: var(--admin-text-muted);
		cursor: pointer;
		padding: 0.25rem;
		border-radius: var(--radius-sm, 0.25rem);
		transition: all 0.2s ease;
	}

	.btn-close:hover {
		background: var(--admin-surface-hover);
		color: var(--admin-text-primary);
	}

	/* Quick Stats */
	.quick-stats {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 0.5rem;
		padding: 1rem 1.5rem;
		background: var(--admin-surface-sunken);
	}

	.stat-item {
		text-align: center;
	}

	.stat-value {
		display: block;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 1rem;
		font-weight: 600;
		color: var(--admin-text-primary);
	}

	.stat-label {
		font-size: 0.6875rem;
		color: var(--admin-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	/* Action Buttons */
	.action-buttons {
		display: flex;
		gap: 0.5rem;
		padding: 1rem 1.5rem;
		border-bottom: 1px solid var(--admin-border-subtle);
	}

	.btn-action {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.5rem;
		background: var(--admin-surface-hover);
		border: 1px solid var(--admin-border-subtle);
		color: var(--admin-text-secondary);
		border-radius: var(--radius-md, 0.5rem);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-action:hover:not(:disabled) {
		background: var(--admin-surface-primary);
		border-color: var(--admin-border-light);
	}

	.btn-action:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-action.danger {
		color: var(--admin-error);
	}

	.btn-action.danger:hover:not(:disabled) {
		background: var(--admin-error-bg);
		border-color: var(--admin-error-border);
	}

	.btn-action.warning {
		color: var(--admin-warning);
	}

	.btn-action.warning:hover:not(:disabled) {
		background: var(--admin-warning-bg);
		border-color: var(--admin-warning);
	}

	.btn-action.success {
		color: var(--admin-success);
	}

	.btn-action.success:hover:not(:disabled) {
		background: var(--admin-success-bg);
		border-color: var(--admin-success);
	}

	/* Tabs */
	.drawer-tabs {
		display: flex;
		padding: 0 1rem;
		border-bottom: 1px solid var(--admin-border-subtle);
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.875rem 1rem;
		background: transparent;
		border: none;
		border-bottom: 2px solid transparent;
		color: var(--admin-text-muted);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		white-space: nowrap;
		transition: all 0.2s ease;
	}

	.tab:hover {
		color: var(--admin-text-secondary);
	}

	.tab.active {
		color: var(--admin-accent-primary);
		border-bottom-color: var(--admin-accent-primary);
	}

	/* Content */
	.drawer-content {
		flex: 1;
		overflow-y: auto;
		padding: 1.5rem;
	}

	.tab-content {
		animation: fadeIn 0.2s ease;
	}

	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 1rem;
		color: var(--admin-text-muted);
		text-align: center;
	}

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid var(--admin-border-subtle);
		border-top-color: var(--admin-accent-primary);
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.empty-state p,
	.loading-state p {
		margin-top: 0.75rem;
	}

	/* Info Sections */
	.info-section {
		margin-bottom: 1.5rem;
	}

	.info-section.warning-section,
	.info-section.trial-section {
		background: var(--admin-warning-bg);
		border: 1px solid var(--admin-warning);
		border-radius: var(--radius-md, 0.5rem);
		padding: 1rem;
	}

	.section-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 0.8125rem;
		font-weight: 600;
		color: var(--admin-text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin: 0 0 1rem;
	}

	.info-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
	}

	.info-item {
		display: flex;
		gap: 0.75rem;
		color: var(--admin-text-muted);
	}

	.info-item.full-width {
		grid-column: 1 / -1;
	}

	.info-item div {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.info-label {
		font-size: 0.75rem;
		color: var(--admin-text-muted);
	}

	.info-value {
		font-size: 0.875rem;
		color: var(--admin-text-primary);
	}

	/* Payment Method Card */
	.payment-method-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md, 0.5rem);
	}

	.payment-method-icon {
		width: 48px;
		height: 48px;
		border-radius: var(--radius-md, 0.5rem);
		background: var(--admin-surface-hover);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--admin-text-muted);
	}

	.payment-method-details {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.payment-method-type {
		font-weight: 500;
		color: var(--admin-text-primary);
		text-transform: capitalize;
	}

	.payment-method-info {
		font-size: 0.875rem;
		color: var(--admin-text-secondary);
		font-family: 'JetBrains Mono', monospace;
	}

	.payment-method-expiry {
		font-size: 0.75rem;
		color: var(--admin-text-muted);
	}

	.no-payment-method {
		color: var(--admin-text-muted);
		font-style: italic;
	}

	.trial-info p {
		margin: 0;
		color: var(--admin-text-secondary);
	}

	.trial-info strong {
		color: var(--admin-text-primary);
	}

	/* Metrics Grid */
	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
	}

	.metric-card {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md, 0.5rem);
	}

	.metric-icon {
		width: 40px;
		height: 40px;
		border-radius: var(--radius-md, 0.5rem);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.metric-icon.success {
		background: var(--admin-success-bg);
		color: var(--admin-success);
	}

	.metric-icon.error {
		background: var(--admin-error-bg);
		color: var(--admin-error);
	}

	.metric-icon.primary {
		background: var(--admin-accent-bg);
		color: var(--admin-accent-primary);
	}

	.metric-icon.accent {
		background: color-mix(in srgb, var(--admin-widget-purple-icon) 15%, transparent);
		color: var(--admin-widget-purple-icon);
	}

	.metric-content {
		display: flex;
		flex-direction: column;
	}

	.metric-value {
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--admin-text-primary);
	}

	.metric-label {
		font-size: 0.75rem;
		color: var(--admin-text-muted);
	}

	/* Activity Summary */
	.activity-summary {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.activity-summary .activity-item {
		display: flex;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		background: var(--admin-surface-sunken);
		border-radius: var(--radius-md, 0.5rem);
	}

	.activity-label {
		color: var(--admin-text-muted);
		font-size: 0.875rem;
	}

	.activity-value {
		color: var(--admin-text-primary);
		font-weight: 500;
		font-size: 0.875rem;
	}

	.activity-value.enabled {
		color: var(--admin-success);
	}

	/* Emails List */
	.emails-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.email-item {
		display: grid;
		grid-template-columns: auto 1fr auto;
		gap: 0.75rem;
		padding: 0.75rem;
		background: var(--admin-surface-sunken);
		border-radius: var(--radius-md, 0.5rem);
		align-items: center;
	}

	.email-type {
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--admin-accent-primary);
		text-transform: capitalize;
		background: var(--admin-accent-bg);
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
	}

	.email-subject {
		font-size: 0.875rem;
		color: var(--admin-text-primary);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.email-date {
		font-size: 0.75rem;
		color: var(--admin-text-muted);
	}

	/* Payments List */
	.payments-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.payment-card {
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md, 0.5rem);
		padding: 1rem;
	}

	.payment-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
	}

	.payment-amount {
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--admin-text-primary);
	}

	.payment-status {
		display: inline-flex;
		align-items: center;
		padding: 0.25rem 0.5rem;
		background: color-mix(in srgb, var(--status-color) 15%, transparent);
		color: var(--status-color);
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 500;
		text-transform: capitalize;
	}

	.payment-details {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0.5rem;
	}

	.payment-detail {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.payment-detail.full-width {
		grid-column: 1 / -1;
	}

	.detail-label {
		font-size: 0.75rem;
		color: var(--admin-text-muted);
	}

	.detail-value {
		font-size: 0.875rem;
		color: var(--admin-text-secondary);
	}

	.detail-value.error {
		color: var(--admin-error);
	}

	.btn-retry-payment {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		width: 100%;
		margin-top: 0.75rem;
		padding: 0.5rem;
		background: var(--admin-accent-bg);
		border: 1px solid var(--admin-accent-primary);
		color: var(--admin-accent-primary);
		border-radius: var(--radius-md, 0.5rem);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-retry-payment:hover:not(:disabled) {
		background: var(--admin-accent-primary);
		color: #0D1117;
	}

	.btn-retry-payment:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>

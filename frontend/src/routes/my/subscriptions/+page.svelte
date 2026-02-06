<script lang="ts">
	/**
	 * User Subscription Management Page
	 * =========================================================================
	 * Apple ICT 7+ Principal Engineer Grade - February 2026
	 *
	 * Allows users to view and manage their subscriptions:
	 * - View active subscriptions with plan details
	 * - Upgrade/downgrade plans with proration preview
	 * - Cancel subscriptions
	 * - Reactivate cancelled subscriptions
	 * - View billing history
	 */

	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { getAuthToken } from '$lib/stores/auth.svelte';

	// Types
	interface Subscription {
		id: number;
		userId: number;
		planId: number;
		status: string;
		startDate: string;
		nextPayment: string;
		expiresAt: string | null;
		cancelledAt: string | null;
		cancelAtPeriodEnd: boolean;
		inTrial: boolean;
		trialEndsAt: string | null;
		inGracePeriod: boolean;
		gracePeriodEnd: string | null;
		failedPaymentCount: number;
		productName: string;
		price: number;
		total: string;
		interval: string;
		features: string[];
		daysRemaining: number;
		currentPeriodStart: string;
		currentPeriodEnd: string;
	}

	interface Plan {
		id: number;
		name: string;
		slug: string;
		price: number;
		billing_cycle: string;
		is_active: boolean;
		features: string[];
		trial_days: number;
	}

	// State
	let subscriptions = $state<Subscription[]>([]);
	let availablePlans = $state<Plan[]>([]);
	let loading = $state(true);
	let error = $state('');
	let successMessage = $state('');

	// Modal states
	let showCancelModal = $state(false);
	let showUpgradeModal = $state(false);
	let selectedSubscription = $state<Subscription | null>(null);
	let cancelReason = $state('');
	let cancelImmediately = $state(false);
	let selectedNewPlanId = $state<number | null>(null);
	let prorationPreview = $state<any>(null);
	let processingAction = $state(false);

	onMount(async () => {
		await loadData();
	});

	async function loadData() {
		loading = true;
		error = '';

		try {
			const token = getAuthToken();
			if (!token) {
				goto('/login?redirect=/my/subscriptions');
				return;
			}

			// Load subscriptions and available plans in parallel
			const [subsRes, plansRes] = await Promise.all([
				fetch('/api/subscriptions/my', {
					headers: { Authorization: `Bearer ${token}` }
				}),
				fetch('/api/subscriptions/plans', {
					headers: { Authorization: `Bearer ${token}` }
				})
			]);

			if (subsRes.ok) {
				const data = await subsRes.json();
				subscriptions = data.subscriptions || [];
			}

			if (plansRes.ok) {
				availablePlans = await plansRes.json();
			}
		} catch (err) {
			error = 'Failed to load subscription data';
			console.error(err);
		} finally {
			loading = false;
		}
	}

	// Cancel subscription
	function openCancelModal(sub: Subscription) {
		selectedSubscription = sub;
		cancelReason = '';
		cancelImmediately = false;
		showCancelModal = true;
	}

	async function confirmCancel() {
		if (!selectedSubscription || !cancelReason.trim()) return;

		processingAction = true;
		try {
			const token = getAuthToken();
			const res = await fetch(`/api/subscriptions/${selectedSubscription.id}/cancel`, {
				method: 'POST',
				headers: {
					Authorization: `Bearer ${token}`,
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					cancel_immediately: cancelImmediately,
					reason: cancelReason
				})
			});

			if (!res.ok) {
				const data = await res.json();
				throw new Error(data.error || 'Failed to cancel subscription');
			}

			successMessage = cancelImmediately
				? 'Subscription cancelled successfully'
				: 'Subscription will be cancelled at the end of your billing period';
			showCancelModal = false;
			await loadData();

			setTimeout(() => (successMessage = ''), 5000);
		} catch (err: any) {
			error = err.message;
		} finally {
			processingAction = false;
		}
	}

	// Upgrade/Downgrade
	function openUpgradeModal(sub: Subscription) {
		selectedSubscription = sub;
		selectedNewPlanId = null;
		prorationPreview = null;
		showUpgradeModal = true;
	}

	async function previewPlanChange(newPlanId: number) {
		if (!selectedSubscription) return;

		selectedNewPlanId = newPlanId;
		try {
			const token = getAuthToken();
			const res = await fetch(`/api/subscriptions/${selectedSubscription.id}/preview-change`, {
				method: 'POST',
				headers: {
					Authorization: `Bearer ${token}`,
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ new_plan_id: newPlanId })
			});

			if (res.ok) {
				prorationPreview = await res.json();
			}
		} catch (err) {
			console.error('Failed to preview plan change:', err);
		}
	}

	async function confirmPlanChange() {
		if (!selectedSubscription || !selectedNewPlanId) return;

		processingAction = true;
		try {
			const token = getAuthToken();
			const res = await fetch(`/api/subscriptions/${selectedSubscription.id}/change-plan`, {
				method: 'POST',
				headers: {
					Authorization: `Bearer ${token}`,
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					new_plan_id: selectedNewPlanId,
					prorate: true
				})
			});

			if (!res.ok) {
				const data = await res.json();
				throw new Error(data.error || 'Failed to change plan');
			}

			const result = await res.json();
			successMessage = result.message || 'Plan changed successfully';
			showUpgradeModal = false;
			await loadData();

			setTimeout(() => (successMessage = ''), 5000);
		} catch (err: any) {
			error = err.message;
		} finally {
			processingAction = false;
		}
	}

	// Reactivate subscription
	async function reactivateSubscription(sub: Subscription) {
		processingAction = true;
		try {
			const token = getAuthToken();
			const res = await fetch(`/api/subscriptions/${sub.id}/reactivate`, {
				method: 'POST',
				headers: { Authorization: `Bearer ${token}` }
			});

			if (!res.ok) {
				const data = await res.json();
				throw new Error(data.error || 'Failed to reactivate');
			}

			successMessage = 'Subscription reactivated successfully';
			await loadData();

			setTimeout(() => (successMessage = ''), 5000);
		} catch (err: any) {
			error = err.message;
		} finally {
			processingAction = false;
		}
	}

	// Helper functions
	function getStatusColor(status: string): string {
		switch (status) {
			case 'active':
				return 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30';
			case 'trial':
				return 'bg-blue-500/20 text-blue-400 border-blue-500/30';
			case 'cancelled':
				return 'bg-red-500/20 text-red-400 border-red-500/30';
			case 'expired':
				return 'bg-slate-500/20 text-slate-400 border-slate-500/30';
			case 'past_due':
				return 'bg-orange-500/20 text-orange-400 border-orange-500/30';
			default:
				return 'bg-slate-500/20 text-slate-400 border-slate-500/30';
		}
	}

	function getStatusLabel(sub: Subscription): string {
		if (sub.inTrial) return 'Trial';
		if (sub.inGracePeriod) return 'Grace Period';
		if (sub.cancelAtPeriodEnd) return 'Cancelling';
		return sub.status.charAt(0).toUpperCase() + sub.status.slice(1);
	}

	function formatDate(dateStr: string | null): string {
		if (!dateStr) return 'N/A';
		return new Date(dateStr).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD'
		}).format(amount);
	}
</script>

<svelte:head>
	<title>My Subscriptions - Revolution Trading Pros</title>
</svelte:head>

<div class="subscriptions-page">
	<div class="container">
		<!-- Background Effects -->
		<div class="bg-effects">
			<div class="bg-blob bg-blob-1"></div>
			<div class="bg-blob bg-blob-2"></div>
		</div>

		<!-- Header -->
		<header class="page-header">
			<h1>My Subscriptions</h1>
			<p class="subtitle">Manage your subscription plans and billing</p>
		</header>

		<!-- Success Message -->
		{#if successMessage}
			<div class="success-banner">
				<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
					<path
						fill-rule="evenodd"
						d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
						clip-rule="evenodd"
					/>
				</svg>
				{successMessage}
			</div>
		{/if}

		<!-- Error Message -->
		{#if error}
			<div class="error-banner">
				<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
					<path
						fill-rule="evenodd"
						d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
						clip-rule="evenodd"
					/>
				</svg>
				{error}
				<button onclick={() => (error = '')} class="ml-auto">x</button>
			</div>
		{/if}

		<!-- Loading State -->
		{#if loading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading subscriptions...</p>
			</div>
		{:else if subscriptions.length === 0}
			<!-- Empty State -->
			<div class="empty-state">
				<svg class="w-16 h-16 text-slate-500 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"
					/>
				</svg>
				<h2 class="text-xl font-semibold text-white mb-2">No Active Subscriptions</h2>
				<p class="text-slate-400 mb-6">You don't have any subscriptions yet.</p>
				<a href="/pricing" class="btn-primary">View Available Plans</a>
			</div>
		{:else}
			<!-- Subscriptions List -->
			<div class="subscriptions-grid">
				{#each subscriptions as sub (sub.id)}
					<div class="subscription-card" class:cancelled={sub.status === 'cancelled'}>
						<!-- Card Header -->
						<div class="card-header">
							<div class="plan-info">
								<h2 class="plan-name">{sub.productName}</h2>
								<span class="status-badge {getStatusColor(sub.status)}">
									{getStatusLabel(sub)}
								</span>
							</div>
							<div class="plan-price">
								<span class="price">{sub.total}</span>
								<span class="interval">/{sub.interval}</span>
							</div>
						</div>

						<!-- Trial/Grace Period Alerts -->
						{#if sub.inTrial}
							<div class="alert alert-info">
								<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
									<path
										fill-rule="evenodd"
										d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
										clip-rule="evenodd"
									/>
								</svg>
								<span>Trial ends {formatDate(sub.trialEndsAt)}</span>
							</div>
						{/if}

						{#if sub.inGracePeriod}
							<div class="alert alert-warning">
								<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
									<path
										fill-rule="evenodd"
										d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
										clip-rule="evenodd"
									/>
								</svg>
								<span>Payment failed. Update by {formatDate(sub.gracePeriodEnd)}</span>
							</div>
						{/if}

						{#if sub.cancelAtPeriodEnd}
							<div class="alert alert-warning">
								<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
									<path
										fill-rule="evenodd"
										d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z"
										clip-rule="evenodd"
									/>
								</svg>
								<span>Cancels on {formatDate(sub.currentPeriodEnd)}</span>
							</div>
						{/if}

						<!-- Details -->
						<div class="card-details">
							<div class="detail-row">
								<span class="detail-label">Started</span>
								<span class="detail-value">{formatDate(sub.startDate)}</span>
							</div>
							{#if sub.status === 'active' && !sub.cancelAtPeriodEnd}
								<div class="detail-row">
									<span class="detail-label">Next billing</span>
									<span class="detail-value">{formatDate(sub.currentPeriodEnd)}</span>
								</div>
								<div class="detail-row">
									<span class="detail-label">Days remaining</span>
									<span class="detail-value">{sub.daysRemaining} days</span>
								</div>
							{/if}
							{#if sub.cancelledAt}
								<div class="detail-row">
									<span class="detail-label">Cancelled</span>
									<span class="detail-value">{formatDate(sub.cancelledAt)}</span>
								</div>
							{/if}
						</div>

						<!-- Features -->
						{#if sub.features && sub.features.length > 0}
							<div class="features-section">
								<h3 class="features-title">Included Features</h3>
								<ul class="features-list">
									{#each sub.features.slice(0, 4) as feature}
										<li>
											<svg class="w-4 h-4 text-emerald-400" fill="currentColor" viewBox="0 0 20 20">
												<path
													fill-rule="evenodd"
													d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
													clip-rule="evenodd"
												/>
											</svg>
											{feature}
										</li>
									{/each}
								</ul>
							</div>
						{/if}

						<!-- Actions -->
						<div class="card-actions">
							{#if sub.status === 'active' && !sub.cancelAtPeriodEnd}
								<button class="btn-secondary" onclick={() => openUpgradeModal(sub)}>
									Change Plan
								</button>
								<button class="btn-danger" onclick={() => openCancelModal(sub)}>
									Cancel
								</button>
							{:else if sub.cancelAtPeriodEnd || sub.status === 'cancelled'}
								<button
									class="btn-primary"
									onclick={() => reactivateSubscription(sub)}
									disabled={processingAction}
								>
									{processingAction ? 'Processing...' : 'Reactivate'}
								</button>
							{/if}
						</div>
					</div>
				{/each}
			</div>
		{/if}

		<!-- Browse Plans Link -->
		{#if !loading && subscriptions.length > 0}
			<div class="browse-plans">
				<a href="/pricing" class="browse-link">
					<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 6v6m0 0v6m0-6h6m-6 0H6"
						/>
					</svg>
					Browse All Plans
				</a>
			</div>
		{/if}
	</div>
</div>

<!-- Cancel Modal -->
{#if showCancelModal && selectedSubscription}
	<div
		class="modal-overlay"
		onclick={() => (showCancelModal = false)}
		onkeydown={(e) => e.key === 'Escape' && (showCancelModal = false)}
		role="button"
		tabindex="-1"
		aria-label="Close modal"
	>
		<div class="modal" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="dialog" aria-modal="true" tabindex="-1">
			<div class="modal-header">
				<h2>Cancel Subscription</h2>
				<button class="modal-close" onclick={() => (showCancelModal = false)}>x</button>
			</div>
			<div class="modal-body">
				<p class="text-slate-300 mb-4">
					Are you sure you want to cancel your <strong>{selectedSubscription.productName}</strong> subscription?
				</p>

				<div class="form-group">
					<label for="cancel-reason">Reason for cancellation</label>
					<textarea
						id="cancel-reason"
						bind:value={cancelReason}
						rows="3"
						placeholder="Please let us know why you're cancelling..."
						class="form-textarea"
					></textarea>
				</div>

				<div class="form-group">
					<label class="checkbox-label">
						<input type="checkbox" bind:checked={cancelImmediately} />
						<span>Cancel immediately (lose access now)</span>
					</label>
					<p class="form-hint">
						If unchecked, you'll keep access until {formatDate(selectedSubscription.currentPeriodEnd)}
					</p>
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showCancelModal = false)}>Keep Subscription</button>
				<button
					class="btn-danger"
					onclick={confirmCancel}
					disabled={!cancelReason.trim() || processingAction}
				>
					{processingAction ? 'Cancelling...' : 'Cancel Subscription'}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Upgrade Modal -->
{#if showUpgradeModal && selectedSubscription}
	<div
		class="modal-overlay"
		onclick={() => (showUpgradeModal = false)}
		onkeydown={(e) => e.key === 'Escape' && (showUpgradeModal = false)}
		role="button"
		tabindex="-1"
		aria-label="Close modal"
	>
		<div class="modal modal-lg" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="dialog" aria-modal="true" tabindex="-1">
			<div class="modal-header">
				<h2>Change Plan</h2>
				<button class="modal-close" onclick={() => (showUpgradeModal = false)}>x</button>
			</div>
			<div class="modal-body">
				<p class="text-slate-300 mb-6">
					Current plan: <strong>{selectedSubscription.productName}</strong> ({selectedSubscription.total}/{selectedSubscription.interval})
				</p>

				<div class="plans-grid">
					{#each availablePlans.filter((p) => p.id !== selectedSubscription?.planId && p.is_active) as plan}
						<button
							class="plan-option"
							class:selected={selectedNewPlanId === plan.id}
							onclick={() => previewPlanChange(plan.id)}
						>
							<div class="plan-option-header">
								<h3>{plan.name}</h3>
								<span class="plan-option-price">
									{formatCurrency(plan.price)}/{plan.billing_cycle}
								</span>
							</div>
							{#if plan.trial_days > 0}
								<p class="plan-option-trial">{plan.trial_days}-day free trial</p>
							{/if}
						</button>
					{/each}
				</div>

				{#if prorationPreview}
					<div class="proration-preview">
						<h3>Proration Preview</h3>
						<div class="proration-details">
							<div class="proration-row">
								<span>Credit from current plan:</span>
								<span class="text-emerald-400">-{formatCurrency(prorationPreview.proration.current_plan_credit)}</span>
							</div>
							<div class="proration-row">
								<span>New plan cost (prorated):</span>
								<span>{formatCurrency(prorationPreview.proration.new_plan_cost)}</span>
							</div>
							<div class="proration-row proration-total">
								<span>Amount due today:</span>
								<span class:text-emerald-400={prorationPreview.proration.proration_amount < 0}>
									{prorationPreview.proration.proration_amount >= 0 ? '' : '-'}
									{formatCurrency(Math.abs(prorationPreview.proration.proration_amount))}
								</span>
							</div>
						</div>
						<p class="proration-summary">{prorationPreview.summary}</p>
					</div>
				{/if}
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showUpgradeModal = false)}>Cancel</button>
				<button
					class="btn-primary"
					onclick={confirmPlanChange}
					disabled={!selectedNewPlanId || processingAction}
				>
					{processingAction ? 'Processing...' : 'Confirm Change'}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.subscriptions-page {
		background: linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #0f172a 100%);
		min-height: 100vh;
		color: white;
		position: relative;
	}

	.container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 3rem 1rem;
		position: relative;
		z-index: 10;
	}

	.bg-effects {
		position: fixed;
		inset: 0;
		pointer-events: none;
		overflow: hidden;
	}

	.bg-blob {
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.12;
	}

	.bg-blob-1 {
		width: 500px;
		height: 500px;
		top: -150px;
		right: -150px;
		background: linear-gradient(135deg, #10b981, #059669);
	}

	.bg-blob-2 {
		width: 400px;
		height: 400px;
		bottom: -100px;
		left: -100px;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
	}

	.page-header {
		text-align: center;
		margin-bottom: 3rem;
	}

	.page-header h1 {
		font-size: 2.5rem;
		font-weight: 700;
		margin: 0 0 0.5rem;
	}

	.subtitle {
		color: #94a3b8;
		font-size: 1.1rem;
		margin: 0;
	}

	/* Banners */
	.success-banner,
	.error-banner {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		border-radius: 12px;
		margin-bottom: 1.5rem;
		max-width: 800px;
		margin-left: auto;
		margin-right: auto;
	}

	.success-banner {
		background: rgba(16, 185, 129, 0.15);
		border: 1px solid rgba(16, 185, 129, 0.3);
		color: #34d399;
	}

	.error-banner {
		background: rgba(239, 68, 68, 0.15);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	/* Loading & Empty States */
	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(16, 185, 129, 0.2);
		border-top-color: #10b981;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		margin-bottom: 1rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Subscriptions Grid */
	.subscriptions-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
		gap: 1.5rem;
	}

	.subscription-card {
		background: rgba(30, 41, 59, 0.7);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 16px;
		padding: 1.5rem;
		backdrop-filter: blur(8px);
	}

	.subscription-card.cancelled {
		opacity: 0.7;
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1rem;
	}

	.plan-name {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0 0 0.5rem;
	}

	.status-badge {
		display: inline-flex;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		border: 1px solid;
	}

	.plan-price {
		text-align: right;
	}

	.price {
		font-size: 1.5rem;
		font-weight: 700;
		color: #10b981;
	}

	.interval {
		color: #64748b;
		font-size: 0.9rem;
	}

	/* Alerts */
	.alert {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		border-radius: 8px;
		margin-bottom: 1rem;
		font-size: 0.875rem;
	}

	.alert-info {
		background: rgba(59, 130, 246, 0.15);
		border: 1px solid rgba(59, 130, 246, 0.3);
		color: #60a5fa;
	}

	.alert-warning {
		background: rgba(245, 158, 11, 0.15);
		border: 1px solid rgba(245, 158, 11, 0.3);
		color: #fbbf24;
	}

	/* Details */
	.card-details {
		margin: 1rem 0;
		padding: 1rem 0;
		border-top: 1px solid rgba(51, 65, 85, 0.5);
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.detail-row {
		display: flex;
		justify-content: space-between;
		padding: 0.375rem 0;
	}

	.detail-label {
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.detail-value {
		color: white;
		font-size: 0.875rem;
		font-weight: 500;
	}

	/* Features */
	.features-section {
		margin: 1rem 0;
	}

	.features-title {
		font-size: 0.875rem;
		font-weight: 600;
		color: #94a3b8;
		margin: 0 0 0.75rem;
	}

	.features-list {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	.features-list li {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.25rem 0;
		font-size: 0.875rem;
		color: #e2e8f0;
	}

	/* Actions */
	.card-actions {
		display: flex;
		gap: 0.75rem;
		margin-top: 1.5rem;
	}

	.btn-primary,
	.btn-secondary,
	.btn-danger {
		flex: 1;
		padding: 0.75rem 1rem;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-primary {
		background: linear-gradient(135deg, #10b981, #059669);
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(16, 185, 129, 0.4);
	}

	.btn-secondary {
		background: rgba(71, 85, 105, 0.5);
		border: 1px solid rgba(100, 116, 139, 0.3);
		color: #cbd5e1;
	}

	.btn-secondary:hover:not(:disabled) {
		background: rgba(71, 85, 105, 0.7);
	}

	.btn-danger {
		background: rgba(239, 68, 68, 0.15);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.btn-danger:hover:not(:disabled) {
		background: rgba(239, 68, 68, 0.25);
	}

	.btn-primary:disabled,
	.btn-secondary:disabled,
	.btn-danger:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Browse Plans */
	.browse-plans {
		text-align: center;
		margin-top: 2rem;
	}

	.browse-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		color: #10b981;
		text-decoration: none;
		font-weight: 500;
	}

	.browse-link:hover {
		text-decoration: underline;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.8);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 100;
		padding: 1rem;
	}

	.modal {
		background: #1e293b;
		border: 1px solid rgba(51, 65, 85, 0.8);
		border-radius: 16px;
		width: 100%;
		max-width: 480px;
		max-height: 90vh;
		overflow-y: auto;
	}

	.modal-lg {
		max-width: 640px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0;
	}

	.modal-close {
		width: 32px;
		height: 32px;
		background: transparent;
		border: none;
		color: #64748b;
		font-size: 1.5rem;
		cursor: pointer;
		border-radius: 6px;
	}

	.modal-close:hover {
		background: rgba(71, 85, 105, 0.5);
		color: white;
	}

	.modal-body {
		padding: 1.5rem;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.25rem 1.5rem;
		border-top: 1px solid rgba(51, 65, 85, 0.5);
	}

	.modal-footer .btn-primary,
	.modal-footer .btn-secondary,
	.modal-footer .btn-danger {
		flex: none;
		padding: 0.75rem 1.5rem;
	}

	/* Form elements */
	.form-group {
		margin-bottom: 1.25rem;
	}

	.form-group label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: #e2e8f0;
		margin-bottom: 0.5rem;
	}

	.form-textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(71, 85, 105, 0.5);
		border-radius: 8px;
		color: white;
		font-size: 0.9rem;
		resize: vertical;
	}

	.form-textarea:focus {
		outline: none;
		border-color: #10b981;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
	}

	.checkbox-label input {
		width: 18px;
		height: 18px;
		accent-color: #10b981;
	}

	.form-hint {
		font-size: 0.8rem;
		color: #64748b;
		margin-top: 0.5rem;
	}

	/* Plans grid in modal */
	.plans-grid {
		display: grid;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
	}

	.plan-option {
		width: 100%;
		padding: 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 2px solid rgba(51, 65, 85, 0.5);
		border-radius: 10px;
		text-align: left;
		cursor: pointer;
		transition: all 0.2s;
	}

	.plan-option:hover {
		border-color: rgba(16, 185, 129, 0.5);
	}

	.plan-option.selected {
		border-color: #10b981;
		background: rgba(16, 185, 129, 0.1);
	}

	.plan-option-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.plan-option h3 {
		font-size: 1rem;
		font-weight: 600;
		color: white;
		margin: 0;
	}

	.plan-option-price {
		font-weight: 600;
		color: #10b981;
	}

	.plan-option-trial {
		font-size: 0.8rem;
		color: #60a5fa;
		margin: 0.5rem 0 0;
	}

	/* Proration preview */
	.proration-preview {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 12px;
		padding: 1.25rem;
	}

	.proration-preview h3 {
		font-size: 0.9rem;
		font-weight: 600;
		color: #94a3b8;
		margin: 0 0 1rem;
	}

	.proration-details {
		margin-bottom: 1rem;
	}

	.proration-row {
		display: flex;
		justify-content: space-between;
		padding: 0.5rem 0;
		font-size: 0.9rem;
		color: #cbd5e1;
	}

	.proration-total {
		border-top: 1px solid rgba(51, 65, 85, 0.5);
		margin-top: 0.5rem;
		padding-top: 0.75rem;
		font-weight: 600;
	}

	.proration-summary {
		font-size: 0.875rem;
		color: #94a3b8;
		margin: 0;
		font-style: italic;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.subscriptions-grid {
			grid-template-columns: 1fr;
		}

		.page-header h1 {
			font-size: 1.75rem;
		}

		.card-header {
			flex-direction: column;
			gap: 1rem;
		}

		.plan-price {
			text-align: left;
		}
	}
</style>

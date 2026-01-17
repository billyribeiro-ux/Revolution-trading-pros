<script lang="ts">
	/**
	 * SubscriptionFormModal - Create/Edit Subscription Modal
	 * Revolution Trading Pros - Apple ICT 11+ Principal Engineer Grade
	 *
	 * Enterprise-grade modal for subscription CRUD operations with plan management.
	 */
	import type {
		Subscription,
		SubscriptionInterval
	} from '$lib/stores/subscriptions.svelte';
	import { createSubscription, updateSubscription } from '$lib/api/subscriptions';
	import {
		IconX,
		IconCreditCard,
		IconPlus,
		IconEdit,
		IconCalendar,
		IconCurrencyDollar
	} from '$lib/icons';

	interface Props {
		isOpen: boolean;
		mode?: 'create' | 'edit';
		subscription?: Subscription | null;
		onSave?: (subscription: Subscription) => void;
		onSaved?: (subscription: Subscription) => void;
		onClose: () => void;
	}

	let {
		isOpen,
		mode: modeProp,
		subscription = null,
		onSave,
		onSaved,
		onClose
	}: Props = $props();

	// Derive mode from props
	let mode = $derived(modeProp ?? (subscription ? 'edit' : 'create'));

	const handleSaved = (sub: Subscription) => {
		onSave?.(sub);
		onSaved?.(sub);
	};

	// Form state
	let userId = $state('');
	let productId = $state('');
	let productName = $state('');
	let price = $state(0);
	let interval = $state<SubscriptionInterval>('monthly');
	let autoRenew = $state(true);
	let trialDays = $state(0);
	let notes = $state('');

	// Payment method (for create)
	let paymentMethodType = $state<'card' | 'paypal' | 'bank'>('card');

	// UI state
	let isLoading = $state(false);
	let error = $state('');
	let activeSection = $state<'details' | 'billing' | 'options'>('details');

	const intervals: { value: SubscriptionInterval; label: string; description: string }[] = [
		{ value: 'monthly', label: 'Monthly', description: 'Billed every month' },
		{ value: 'quarterly', label: 'Quarterly', description: 'Billed every 3 months' },
		{ value: 'yearly', label: 'Yearly', description: 'Billed annually' }
	];

	// Initialize form when subscription changes
	$effect(() => {
		if (isOpen && mode === 'edit' && subscription) {
			userId = subscription.userId || '';
			productId = subscription.productId || '';
			productName = subscription.productName || '';
			price = subscription.price || 0;
			interval = subscription.interval || 'monthly';
			autoRenew = subscription.autoRenew ?? true;
			notes = subscription.notes || '';
			trialDays = 0;
		} else if (isOpen && mode === 'create') {
			userId = '';
			productId = '';
			productName = '';
			price = 0;
			interval = 'monthly';
			autoRenew = true;
			trialDays = 0;
			notes = '';
			paymentMethodType = 'card';
		}
		error = '';
		activeSection = 'details';
	});

	$effect(() => {
		if (isOpen) {
			document.body.style.overflow = 'hidden';
		} else {
			document.body.style.overflow = '';
		}
		return () => {
			document.body.style.overflow = '';
		};
	});

	function validateForm(): boolean {
		if (mode === 'create') {
			if (!userId.trim()) {
				error = 'User ID is required';
				activeSection = 'details';
				return false;
			}
			if (!productName.trim()) {
				error = 'Product name is required';
				activeSection = 'details';
				return false;
			}
		}
		if (price <= 0) {
			error = 'Price must be greater than 0';
			activeSection = 'billing';
			return false;
		}
		return true;
	}

	async function handleSubmit() {
		if (!validateForm()) return;

		isLoading = true;
		error = '';

		try {
			if (mode === 'create') {
				const data: Partial<Subscription> = {
					userId: userId.trim(),
					productId: productId.trim() || `prod_${Date.now()}`,
					productName: productName.trim(),
					price,
					interval,
					autoRenew,
					notes: notes.trim() || undefined,
					paymentMethod: {
						type: paymentMethodType
					},
					status: trialDays > 0 ? 'trial' : 'pending'
				};

				if (trialDays > 0) {
					const trialEnd = new Date();
					trialEnd.setDate(trialEnd.getDate() + trialDays);
					data.trialEndDate = trialEnd.toISOString();
					data.isTrialing = true;
				}

				const result = await createSubscription(data as any);
				handleSaved(result as Subscription);
				onClose();
			} else if (mode === 'edit' && subscription) {
				const data: Partial<Subscription> = {};
				
				if (productName.trim() !== subscription.productName) {
					data.productName = productName.trim();
				}
				if (price !== subscription.price) {
					data.price = price;
				}
				if (interval !== subscription.interval) {
					data.interval = interval;
				}
				if (autoRenew !== subscription.autoRenew) {
					data.autoRenew = autoRenew;
				}
				if (notes.trim() !== (subscription.notes || '')) {
					data.notes = notes.trim() || undefined;
				}

				const result = await updateSubscription(subscription.id, data as any);
				handleSaved(result as Subscription);
				onClose();
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'An error occurred';
		} finally {
			isLoading = false;
		}
	}

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget && !isLoading) {
			onClose();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape' && !isLoading) {
			onClose();
		}
	}

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD'
		}).format(amount);
	}

	function calculateAnnualCost(): number {
		switch (interval) {
			case 'monthly': return price * 12;
			case 'quarterly': return price * 4;
			case 'yearly': return price;
		}
	}
</script>

{#if isOpen}
	<div
		class="modal-backdrop"
		role="dialog"
		aria-modal="true"
		aria-labelledby="modal-title"
		onclick={handleBackdropClick}
		onkeydown={handleKeydown}
		tabindex="-1"
	>
		<div class="modal-container">
			<!-- Header -->
			<div class="modal-header">
				<div class="header-icon">
					{#if mode === 'create'}
						<IconPlus size={24} />
					{:else}
						<IconEdit size={24} />
					{/if}
				</div>
				<h2 id="modal-title" class="modal-title">
					{mode === 'create' ? 'Create Subscription' : 'Edit Subscription'}
				</h2>
				<button type="button" class="btn-close" onclick={onClose} disabled={isLoading} aria-label="Close">
					<IconX size={20} />
				</button>
			</div>

			<!-- Section Tabs -->
			<nav class="section-tabs">
				<button
					type="button"
					class="section-tab"
					class:active={activeSection === 'details'}
					onclick={() => activeSection = 'details'}
				>
					<IconCreditCard size={16} />
					Details
				</button>
				<button
					type="button"
					class="section-tab"
					class:active={activeSection === 'billing'}
					onclick={() => activeSection = 'billing'}
				>
					<IconCurrencyDollar size={16} />
					Billing
				</button>
				<button
					type="button"
					class="section-tab"
					class:active={activeSection === 'options'}
					onclick={() => activeSection = 'options'}
				>
					<IconCalendar size={16} />
					Options
				</button>
			</nav>

			<!-- Form -->
			<form class="modal-form" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
				{#if error}
					<div class="error-banner">{error}</div>
				{/if}

				{#if activeSection === 'details'}
					<div class="form-section">
						{#if mode === 'create'}
							<div class="form-group">
								<label for="userId" class="form-label">User ID <span class="required">*</span></label>
								<input
									id="userId"
									type="text"
									class="form-input"
									placeholder="Enter user ID or email"
									bind:value={userId}
									disabled={isLoading}
								/>
								<span class="form-hint">The member who will own this subscription</span>
							</div>

							<div class="form-group">
								<label for="productId" class="form-label">Product ID</label>
								<input
									id="productId"
									type="text"
									class="form-input"
									placeholder="Auto-generated if empty"
									bind:value={productId}
									disabled={isLoading}
								/>
								<span class="form-hint">Optional - will be auto-generated</span>
							</div>
						{/if}

						<div class="form-group">
							<label for="productName" class="form-label">
								Product Name <span class="required">*</span>
							</label>
							<input
								id="productName"
								type="text"
								class="form-input"
								placeholder="e.g., Pro Trading Course"
								bind:value={productName}
								disabled={isLoading}
							/>
						</div>

						<div class="form-group">
							<label for="notes" class="form-label">Notes</label>
							<textarea
								id="notes"
								class="form-textarea"
								placeholder="Add any internal notes about this subscription..."
								bind:value={notes}
								disabled={isLoading}
								rows="3"
							></textarea>
						</div>
					</div>

				{:else if activeSection === 'billing'}
					<div class="form-section">
						<div class="form-group">
							<label for="price" class="form-label">Price <span class="required">*</span></label>
							<div class="price-input-wrapper">
								<span class="price-symbol">$</span>
								<input
									id="price"
									type="number"
									class="form-input price-input"
									placeholder="0.00"
									step="0.01"
									min="0"
									bind:value={price}
									disabled={isLoading}
								/>
							</div>
						</div>

						<div class="form-group" role="radiogroup" aria-labelledby="interval-label">
							<span id="interval-label" class="form-label">Billing Interval</span>
							<div class="interval-options">
								{#each intervals as int (int.value)}
									<label class="interval-option" class:selected={interval === int.value}>
										<input
											type="radio"
											name="interval"
											value={int.value}
											bind:group={interval}
											disabled={isLoading}
										/>
										<div class="interval-content">
											<span class="interval-label">{int.label}</span>
											<span class="interval-desc">{int.description}</span>
										</div>
									</label>
								{/each}
							</div>
						</div>

						{#if price > 0}
							<div class="pricing-summary">
								<div class="summary-row">
									<span>Per billing cycle</span>
									<span class="summary-value">{formatCurrency(price)}</span>
								</div>
								<div class="summary-row">
									<span>Annualized cost</span>
									<span class="summary-value">{formatCurrency(calculateAnnualCost())}</span>
								</div>
							</div>
						{/if}

						{#if mode === 'create'}
							<div class="form-group" role="radiogroup" aria-labelledby="payment-type-label">
								<span id="payment-type-label" class="form-label">Payment Method Type</span>
								<div class="payment-type-options">
									<label class="payment-type-option" class:selected={paymentMethodType === 'card'}>
										<input
											type="radio"
											name="paymentType"
											value="card"
											bind:group={paymentMethodType}
											disabled={isLoading}
										/>
										<IconCreditCard size={20} />
										<span>Card</span>
									</label>
									<label class="payment-type-option" class:selected={paymentMethodType === 'paypal'}>
										<input
											type="radio"
											name="paymentType"
											value="paypal"
											bind:group={paymentMethodType}
											disabled={isLoading}
										/>
										<span>PayPal</span>
									</label>
									<label class="payment-type-option" class:selected={paymentMethodType === 'bank'}>
										<input
											type="radio"
											name="paymentType"
											value="bank"
											bind:group={paymentMethodType}
											disabled={isLoading}
										/>
										<span>Bank</span>
									</label>
								</div>
							</div>
						{/if}
					</div>

				{:else if activeSection === 'options'}
					<div class="form-section">
						<div class="form-group">
							<label class="toggle-label">
								<input
									type="checkbox"
									bind:checked={autoRenew}
									disabled={isLoading}
								/>
								<span class="toggle-text">
									<strong>Auto-Renew</strong>
									<span>Automatically renew subscription at the end of each billing period</span>
								</span>
							</label>
						</div>

						{#if mode === 'create'}
							<div class="form-group">
								<label for="trialDays" class="form-label">Trial Period</label>
								<div class="trial-input-wrapper">
									<input
										id="trialDays"
										type="number"
										class="form-input trial-input"
										placeholder="0"
										min="0"
										max="365"
										bind:value={trialDays}
										disabled={isLoading}
									/>
									<span class="trial-suffix">days</span>
								</div>
								<span class="form-hint">Set to 0 for no trial period</span>
							</div>

							{#if trialDays > 0}
								<div class="trial-info">
									<IconCalendar size={16} />
									<span>
										Trial will end on <strong>{new Date(Date.now() + trialDays * 86400000).toLocaleDateString()}</strong>
									</span>
								</div>
							{/if}
						{/if}

						{#if mode === 'edit' && subscription}
							<div class="subscription-meta">
								<h4>Subscription Info</h4>
								<div class="meta-grid">
									<div class="meta-item">
										<span class="meta-label">Created</span>
										<span class="meta-value">{new Date(subscription.createdAt).toLocaleDateString()}</span>
									</div>
									<div class="meta-item">
										<span class="meta-label">Status</span>
										<span class="meta-value status">{subscription.status}</span>
									</div>
									<div class="meta-item">
										<span class="meta-label">Renewals</span>
										<span class="meta-value">{subscription.renewalCount || 0}</span>
									</div>
									<div class="meta-item">
										<span class="meta-label">Total Paid</span>
										<span class="meta-value">{formatCurrency(subscription.totalPaid || 0)}</span>
									</div>
								</div>
							</div>
						{/if}
					</div>
				{/if}

				<!-- Actions -->
				<div class="modal-actions">
					<button type="button" class="btn-cancel" onclick={onClose} disabled={isLoading}>
						Cancel
					</button>
					<button type="submit" class="btn-submit" disabled={isLoading}>
						{#if isLoading}
							<span class="spinner"></span>
							{mode === 'create' ? 'Creating...' : 'Saving...'}
						{:else}
							{mode === 'create' ? 'Create Subscription' : 'Save Changes'}
						{/if}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	.modal-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: var(--z-modal, 1000);
		padding: 1rem;
		animation: fadeIn 0.2s ease;
	}

	@keyframes fadeIn {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	.modal-container {
		background: var(--admin-surface-primary);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-xl, 1rem);
		max-width: 560px;
		width: 100%;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		animation: slideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
	}

	@keyframes slideUp {
		from {
			opacity: 0;
			transform: translateY(20px) scale(0.95);
		}
		to {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
	}

	.modal-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid var(--admin-border-subtle);
	}

	.header-icon {
		width: 40px;
		height: 40px;
		border-radius: var(--radius-md, 0.5rem);
		background: var(--admin-accent-bg);
		color: var(--admin-accent-primary);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.modal-title {
		flex: 1;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--admin-text-primary);
		margin: 0;
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

	.btn-close:hover:not(:disabled) {
		background: var(--admin-surface-hover);
		color: var(--admin-text-primary);
	}

	/* Section Tabs */
	.section-tabs {
		display: flex;
		padding: 0 1rem;
		border-bottom: 1px solid var(--admin-border-subtle);
		background: var(--admin-surface-sunken);
	}

	.section-tab {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.75rem 1rem;
		background: transparent;
		border: none;
		border-bottom: 2px solid transparent;
		color: var(--admin-text-muted);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.section-tab:hover {
		color: var(--admin-text-secondary);
	}

	.section-tab.active {
		color: var(--admin-accent-primary);
		border-bottom-color: var(--admin-accent-primary);
	}

	.modal-form {
		padding: 1.5rem;
	}

	.form-section {
		animation: fadeIn 0.2s ease;
	}

	.error-banner {
		background: var(--admin-error-bg);
		border: 1px solid var(--admin-error-border);
		color: var(--admin-error);
		padding: 0.75rem 1rem;
		border-radius: var(--radius-md, 0.5rem);
		margin-bottom: 1.25rem;
		font-size: 0.875rem;
	}

	.form-group {
		margin-bottom: 1.25rem;
	}

	.form-label {
		display: block;
		font-family: var(--font-body), 'Roboto', sans-serif;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--admin-text-secondary);
		margin-bottom: 0.5rem;
	}

	.required {
		color: var(--admin-error);
	}

	.form-hint {
		display: block;
		font-size: 0.75rem;
		color: var(--admin-text-muted);
		margin-top: 0.375rem;
	}

	.form-input,
	.form-textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md, 0.5rem);
		color: var(--admin-text-primary);
		font-family: var(--font-body), 'Roboto', sans-serif;
		font-size: 0.9375rem;
		transition: border-color 0.2s ease, box-shadow 0.2s ease;
	}

	.form-textarea {
		resize: vertical;
		min-height: 80px;
	}

	.form-input:focus,
	.form-textarea:focus {
		outline: none;
		border-color: var(--admin-accent-primary);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.15);
	}

	.form-input::placeholder,
	.form-textarea::placeholder {
		color: var(--admin-text-muted);
	}

	.form-input:disabled,
	.form-textarea:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* Price Input */
	.price-input-wrapper {
		position: relative;
		display: flex;
		align-items: center;
	}

	.price-symbol {
		position: absolute;
		left: 1rem;
		color: var(--admin-text-muted);
		font-size: 1rem;
	}

	.price-input {
		padding-left: 2rem;
	}

	/* Interval Options */
	.interval-options {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.interval-option {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.875rem 1rem;
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md, 0.5rem);
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.interval-option:hover {
		border-color: var(--admin-border-light);
	}

	.interval-option.selected {
		border-color: var(--admin-accent-primary);
		background: var(--admin-accent-bg);
	}

	.interval-option input {
		width: 18px;
		height: 18px;
		accent-color: var(--admin-accent-primary);
	}

	.interval-content {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.interval-label {
		font-weight: 500;
		color: var(--admin-text-primary);
	}

	.interval-desc {
		font-size: 0.75rem;
		color: var(--admin-text-muted);
	}

	/* Pricing Summary */
	.pricing-summary {
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md, 0.5rem);
		padding: 1rem;
		margin-bottom: 1.25rem;
	}

	.summary-row {
		display: flex;
		justify-content: space-between;
		padding: 0.375rem 0;
		font-size: 0.875rem;
		color: var(--admin-text-secondary);
	}

	.summary-value {
		font-weight: 600;
		color: var(--admin-text-primary);
	}

	/* Payment Type Options */
	.payment-type-options {
		display: flex;
		gap: 0.5rem;
	}

	.payment-type-option {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.375rem;
		padding: 0.75rem;
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md, 0.5rem);
		cursor: pointer;
		transition: all 0.2s ease;
		color: var(--admin-text-muted);
	}

	.payment-type-option:hover {
		border-color: var(--admin-border-light);
	}

	.payment-type-option.selected {
		border-color: var(--admin-accent-primary);
		background: var(--admin-accent-bg);
		color: var(--admin-accent-primary);
	}

	.payment-type-option input {
		position: absolute;
		opacity: 0;
		pointer-events: none;
	}

	.payment-type-option span {
		font-size: 0.8125rem;
		font-weight: 500;
	}

	/* Toggle Label */
	.toggle-label {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 1rem;
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md, 0.5rem);
		cursor: pointer;
	}

	.toggle-label input {
		width: 18px;
		height: 18px;
		accent-color: var(--admin-accent-primary);
		margin-top: 0.125rem;
	}

	.toggle-text {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.toggle-text strong {
		color: var(--admin-text-primary);
	}

	.toggle-text span:not(strong) {
		font-size: 0.8125rem;
		color: var(--admin-text-muted);
	}

	/* Trial Input */
	.trial-input-wrapper {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.trial-input {
		width: 100px;
	}

	.trial-suffix {
		color: var(--admin-text-muted);
		font-size: 0.875rem;
	}

	.trial-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: var(--admin-accent-bg);
		border: 1px solid var(--admin-accent-primary);
		border-radius: var(--radius-md, 0.5rem);
		color: var(--admin-text-secondary);
		font-size: 0.875rem;
	}

	.trial-info strong {
		color: var(--admin-accent-primary);
	}

	/* Subscription Meta */
	.subscription-meta {
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md, 0.5rem);
		padding: 1rem;
	}

	.subscription-meta h4 {
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--admin-text-secondary);
		margin: 0 0 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.meta-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0.75rem;
	}

	.meta-item {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.meta-label {
		font-size: 0.75rem;
		color: var(--admin-text-muted);
	}

	.meta-value {
		font-size: 0.875rem;
		color: var(--admin-text-primary);
	}

	.meta-value.status {
		text-transform: capitalize;
		color: var(--admin-accent-primary);
	}

	/* Actions */
	.modal-actions {
		display: flex;
		gap: 0.75rem;
		padding-top: 1rem;
		border-top: 1px solid var(--admin-border-subtle);
		margin-top: 1.5rem;
	}

	.btn-cancel,
	.btn-submit {
		flex: 1;
		padding: 0.75rem 1.5rem;
		border-radius: var(--radius-md, 0.5rem);
		font-family: var(--font-body), 'Roboto', sans-serif;
		font-size: 0.9375rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-cancel {
		background: transparent;
		border: 1px solid var(--admin-border-subtle);
		color: var(--admin-text-secondary);
	}

	.btn-cancel:hover:not(:disabled) {
		background: var(--admin-surface-hover);
		border-color: var(--admin-border-light);
	}

	.btn-submit {
		background: var(--admin-accent-primary);
		border: none;
		color: #0D1117;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
	}

	.btn-submit:hover:not(:disabled) {
		background: var(--admin-accent-primary-hover);
	}

	.btn-cancel:disabled,
	.btn-submit:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.spinner {
		display: inline-block;
		width: 16px;
		height: 16px;
		border: 2px solid transparent;
		border-top-color: currentColor;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}
</style>

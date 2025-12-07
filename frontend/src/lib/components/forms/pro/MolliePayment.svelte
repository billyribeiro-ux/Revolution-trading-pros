<script lang="ts">
	/**
	 * MolliePayment Component (FluentForms Pro 6.1.8)
	 *
	 * Mollie payment integration for European payments.
	 * Supports:
	 * - iDEAL (Netherlands)
	 * - Bancontact (Belgium)
	 * - SOFORT (Germany)
	 * - Credit Cards
	 * - Klarna
	 * - Bank Transfer
	 */

	interface Props {
		profileId: string;
		amount: number;
		currency?: string;
		description?: string;
		redirectUrl?: string;
		webhookUrl?: string;
		locale?: string;
		methods?: string[];
		testMode?: boolean;
		label?: string;
		disabled?: boolean;
		error?: string;
		onpayment?: (result: MolliePaymentResult) => void;
		onerror?: (error: string) => void;
	}

	interface MolliePaymentResult {
		paymentId: string;
		checkoutUrl: string;
		method: string;
		status: string;
	}

	interface PaymentMethod {
		id: string;
		name: string;
		icon: string;
		description?: string;
	}

	let {
		profileId,
		amount,
		currency = 'EUR',
		description = 'Payment',
		redirectUrl = '',
		webhookUrl = '',
		locale = 'en_US',
		methods = ['ideal', 'creditcard', 'bancontact', 'sofort', 'klarna'],
		testMode = false,
		label = 'Select Payment Method',
		disabled = false,
		error = '',
		onpayment,
		onerror
	}: Props = $props();

	let selectedMethod = $state('');
	let selectedIssuer = $state('');
	let processing = $state(false);
	let methodError = $state('');
	let issuers = $state<{ id: string; name: string }[]>([]);
	let loadingIssuers = $state(false);

	const availableMethods: PaymentMethod[] = [
		{ id: 'ideal', name: 'iDEAL', icon: '🏦', description: 'Dutch bank transfer' },
		{ id: 'creditcard', name: 'Credit Card', icon: '💳', description: 'Visa, Mastercard, Amex' },
		{ id: 'bancontact', name: 'Bancontact', icon: '💳', description: 'Belgian debit cards' },
		{ id: 'sofort', name: 'SOFORT', icon: '🏦', description: 'German bank transfer' },
		{ id: 'klarna', name: 'Klarna', icon: '🛒', description: 'Pay later or slice it' },
		{ id: 'banktransfer', name: 'Bank Transfer', icon: '🏛️', description: 'Manual bank transfer' },
		{ id: 'paypal', name: 'PayPal', icon: '💰', description: 'PayPal account' },
		{ id: 'applepay', name: 'Apple Pay', icon: '🍎', description: 'Apple devices' },
		{ id: 'eps', name: 'EPS', icon: '🏦', description: 'Austrian bank transfer' },
		{ id: 'giropay', name: 'Giropay', icon: '🏦', description: 'German online banking' }
	];

	const filteredMethods = $derived(
		availableMethods.filter((m) => methods.includes(m.id))
	);

	async function selectMethod(methodId: string) {
		selectedMethod = methodId;
		selectedIssuer = '';
		issuers = [];

		// Load issuers for iDEAL
		if (methodId === 'ideal') {
			loadingIssuers = true;
			try {
				const response = await fetch(`/api/payments/mollie/issuers?method=${methodId}`);
				const data = await response.json();
				issuers = data.issuers || [];
			} catch {
				// Failed to load issuers
			} finally {
				loadingIssuers = false;
			}
		}
	}

	async function handleSubmit() {
		if (!selectedMethod || processing) return;

		if (selectedMethod === 'ideal' && !selectedIssuer) {
			methodError = 'Please select your bank';
			return;
		}

		processing = true;
		methodError = '';

		try {
			const response = await fetch('/api/payments/mollie/create', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					profile_id: profileId,
					amount: amount.toFixed(2),
					currency,
					description,
					method: selectedMethod,
					issuer: selectedIssuer || undefined,
					redirect_url: redirectUrl,
					webhook_url: webhookUrl,
					locale,
					test_mode: testMode
				})
			});

			const data = await response.json();

			if (data.success && data.checkout_url) {
				const result: MolliePaymentResult = {
					paymentId: data.payment_id,
					checkoutUrl: data.checkout_url,
					method: selectedMethod,
					status: 'pending'
				};

				if (onpayment) onpayment(result);

				// Redirect to Mollie checkout
				window.location.href = data.checkout_url;
			} else {
				methodError = data.message || 'Failed to create payment';
				if (onerror) onerror(methodError);
			}
		} catch (err) {
			methodError = err instanceof Error ? err.message : 'Payment failed';
			if (onerror) onerror(methodError);
		} finally {
			processing = false;
		}
	}

	function formatAmount(amount: number, currency: string): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: currency
		}).format(amount);
	}
</script>

<div class="mollie-payment" class:disabled class:has-error={error || methodError}>
	{#if label}
		<label class="field-label" for="mollie-method-select">{label}</label>
	{/if}

	{#if testMode}
		<div class="test-mode-banner">
			<span>⚠️ Mollie Test Mode</span>
		</div>
	{/if}

	<div class="amount-display">
		Total: <strong>{formatAmount(amount, currency)}</strong>
	</div>

	<div class="methods-grid">
		{#each filteredMethods as method}
			<button
				type="button"
				class="method-btn"
				class:selected={selectedMethod === method.id}
				aria-label="Select {method.name}"
				onclick={() => selectMethod(method.id)}
				{disabled}
			>
				<span class="method-icon">{method.icon}</span>
				<span class="method-name">{method.name}</span>
				{#if method.description}
					<span class="method-desc">{method.description}</span>
				{/if}
			</button>
		{/each}
	</div>

	{#if selectedMethod === 'ideal' && (loadingIssuers || issuers.length > 0)}
		<div class="issuer-select">
			<label class="issuer-label" for="ideal-issuer-select">Select your bank:</label>
			{#if loadingIssuers}
				<div class="loading-issuers">Loading banks...</div>
			{:else}
				<select
					id="ideal-issuer-select"
					bind:value={selectedIssuer}
					class="issuer-dropdown"
					{disabled}
				>
					<option value="">-- Select your bank --</option>
					{#each issuers as issuer}
						<option value={issuer.id}>{issuer.name}</option>
					{/each}
				</select>
			{/if}
		</div>
	{/if}

	{#if methodError}
		<p class="method-error">{methodError}</p>
	{/if}

	<button
		type="button"
		class="pay-button"
		onclick={handleSubmit}
		disabled={disabled || processing || !selectedMethod}
	>
		{#if processing}
			<svg class="button-spinner" viewBox="0 0 24 24">
				<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" fill="none" />
			</svg>
			Processing...
		{:else}
			Continue to Payment
		{/if}
	</button>

	<div class="secure-badge">
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
			<path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
		</svg>
		<span>Secured by Mollie</span>
	</div>

	{#if error}
		<p class="error-text">{error}</p>
	{/if}
</div>

<style>
	.mollie-payment {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.field-label {
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
	}

	.test-mode-banner {
		padding: 0.5rem 0.75rem;
		background-color: #fef3c7;
		border: 1px solid #f59e0b;
		border-radius: 0.375rem;
		font-size: 0.75rem;
		color: #92400e;
	}

	.amount-display {
		padding: 0.75rem 1rem;
		background-color: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		text-align: center;
		color: #374151;
	}

	.amount-display strong {
		color: #111827;
		font-size: 1.125rem;
	}

	.methods-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
		gap: 0.75rem;
	}

	.method-btn {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.25rem;
		padding: 1rem;
		border: 2px solid #e5e7eb;
		border-radius: 0.5rem;
		background-color: white;
		cursor: pointer;
		transition: all 0.15s;
	}

	.method-btn:hover:not(:disabled) {
		border-color: #3b82f6;
		background-color: #eff6ff;
	}

	.method-btn.selected {
		border-color: #3b82f6;
		background-color: #dbeafe;
	}

	.method-icon {
		font-size: 1.5rem;
	}

	.method-name {
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
	}

	.method-desc {
		font-size: 0.6875rem;
		color: #6b7280;
		text-align: center;
	}

	.issuer-select {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.issuer-label {
		font-size: 0.875rem;
		color: #374151;
	}

	.issuer-dropdown {
		padding: 0.625rem 0.875rem;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		font-size: 0.9375rem;
	}

	.issuer-dropdown:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.loading-issuers {
		padding: 0.625rem;
		color: #6b7280;
		font-style: italic;
	}

	.method-error {
		font-size: 0.75rem;
		color: #ef4444;
		margin: 0;
	}

	.pay-button {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.875rem 1.5rem;
		background-color: #000000;
		color: white;
		border: none;
		border-radius: 0.5rem;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		transition: background-color 0.15s;
	}

	.pay-button:hover:not(:disabled) {
		background-color: #1f2937;
	}

	.pay-button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.button-spinner {
		width: 20px;
		height: 20px;
		animation: spin 1s linear infinite;
	}

	.button-spinner circle {
		stroke-dasharray: 60;
		stroke-dashoffset: 45;
		stroke-linecap: round;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	.secure-badge {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		font-size: 0.75rem;
		color: #6b7280;
	}

	.secure-badge svg {
		width: 14px;
		height: 14px;
	}

	.error-text {
		font-size: 0.75rem;
		color: #ef4444;
		margin: 0;
	}

	.disabled {
		opacity: 0.6;
		pointer-events: none;
	}
</style>

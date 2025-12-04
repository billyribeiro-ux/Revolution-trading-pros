<script lang="ts">
	/**
	 * PaystackPayment Component (FluentForms Pro 6.1.8)
	 *
	 * Paystack payment integration for African payments.
	 * Supports:
	 * - Card payments
	 * - Bank transfers
	 * - USSD
	 * - Mobile Money
	 */

	interface Props {
		publicKey: string;
		amount: number;
		currency?: string;
		email: string;
		firstName?: string;
		lastName?: string;
		phone?: string;
		reference?: string;
		channels?: string[];
		testMode?: boolean;
		label?: string;
		disabled?: boolean;
		error?: string;
		onpayment?: (result: PaystackPaymentResult) => void;
		onerror?: (error: string) => void;
	}

	interface PaystackPaymentResult {
		reference: string;
		transactionId: string;
		status: string;
		channel?: string;
		paidAt?: string;
	}

	let {
		publicKey,
		amount,
		currency = 'NGN',
		email,
		firstName = '',
		lastName = '',
		phone = '',
		reference = '',
		channels = ['card', 'bank', 'ussd', 'mobile_money'],
		testMode = false,
		label = 'Pay Now',
		disabled = false,
		error = '',
		onpayment,
		onerror
	}: Props = $props();

	let loading = $state(false);
	let paymentError = $state('');

	async function loadPaystack(): Promise<boolean> {
		return new Promise((resolve) => {
			if (window.PaystackPop) {
				resolve(true);
				return;
			}

			const script = document.createElement('script');
			script.src = 'https://js.paystack.co/v1/inline.js';
			script.onload = () => resolve(true);
			script.onerror = () => resolve(false);
			document.head.appendChild(script);
		});
	}

	function generateReference(): string {
		return 'PS_' + Date.now() + '_' + Math.random().toString(36).substr(2, 9);
	}

	async function handlePayment() {
		if (loading || disabled) return;

		loading = true;
		paymentError = '';

		try {
			const loaded = await loadPaystack();
			if (!loaded) {
				throw new Error('Failed to load Paystack');
			}

			const paymentRef = reference || generateReference();

			const handler = window.PaystackPop.setup({
				key: publicKey,
				email,
				amount: Math.round(amount * 100), // Paystack expects amount in kobo
				currency,
				ref: paymentRef,
				firstname: firstName,
				lastname: lastName,
				phone,
				channels,
				callback: (response: any) => {
					const result: PaystackPaymentResult = {
						reference: response.reference,
						transactionId: response.transaction,
						status: response.status,
						channel: response.channel
					};

					if (onpayment) onpayment(result);
				},
				onClose: () => {
					paymentError = 'Payment was cancelled';
				}
			});

			handler.openIframe();
		} catch (err) {
			paymentError = err instanceof Error ? err.message : 'Payment failed';
			if (onerror) onerror(paymentError);
		} finally {
			loading = false;
		}
	}

	function formatAmount(amount: number, currency: string): string {
		const formatter = new Intl.NumberFormat('en-NG', {
			style: 'currency',
			currency: currency
		});
		return formatter.format(amount);
	}

	declare global {
		interface Window {
			PaystackPop: any;
		}
	}
</script>

<div class="paystack-payment" class:disabled class:has-error={error || paymentError}>
	{#if testMode}
		<div class="test-mode-banner">
			<span>⚠️ Paystack Test Mode</span>
		</div>
	{/if}

	<div class="amount-display">
		Total: <strong>{formatAmount(amount, currency)}</strong>
	</div>

	{#if paymentError}
		<p class="payment-error">{paymentError}</p>
	{/if}

	<button
		type="button"
		class="pay-button"
		onclick={handlePayment}
		disabled={disabled || loading}
	>
		{#if loading}
			<svg class="button-spinner" viewBox="0 0 24 24">
				<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" fill="none" />
			</svg>
			Loading...
		{:else}
			{label}
		{/if}
	</button>

	<div class="payment-methods">
		{#if channels.includes('card')}
			<span class="payment-badge">💳 Card</span>
		{/if}
		{#if channels.includes('bank')}
			<span class="payment-badge">🏦 Bank</span>
		{/if}
		{#if channels.includes('ussd')}
			<span class="payment-badge">📱 USSD</span>
		{/if}
		{#if channels.includes('mobile_money')}
			<span class="payment-badge">💰 Mobile Money</span>
		{/if}
	</div>

	<div class="secure-badge">
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
			<path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
		</svg>
		<span>Secured by Paystack</span>
	</div>

	{#if error}
		<p class="error-text">{error}</p>
	{/if}
</div>

<style>
	.paystack-payment {
		display: flex;
		flex-direction: column;
		gap: 1rem;
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

	.payment-error {
		font-size: 0.75rem;
		color: #ef4444;
		margin: 0;
		text-align: center;
	}

	.pay-button {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.875rem 1.5rem;
		background-color: #00c3f7;
		color: white;
		border: none;
		border-radius: 0.5rem;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		transition: background-color 0.15s;
	}

	.pay-button:hover:not(:disabled) {
		background-color: #00a8d4;
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

	.payment-methods {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 0.5rem;
	}

	.payment-badge {
		padding: 0.25rem 0.5rem;
		background-color: #f3f4f6;
		border-radius: 0.25rem;
		font-size: 0.6875rem;
		color: #6b7280;
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

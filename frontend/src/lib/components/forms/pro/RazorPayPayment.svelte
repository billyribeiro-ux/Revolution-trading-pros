<script lang="ts">
	/**
	 * RazorPayPayment Component (FluentForms Pro 6.1.8)
	 *
	 * Razorpay payment integration for Indian payments.
	 * Supports:
	 * - Credit/Debit Cards
	 * - UPI
	 * - Net Banking
	 * - Wallets (PayTM, PhonePe, etc.)
	 * - EMI
	 */

	interface Props {
		keyId: string;
		amount: number;
		currency?: string;
		name?: string;
		description?: string;
		orderId?: string;
		customerEmail?: string;
		customerPhone?: string;
		customerName?: string;
		prefill?: boolean;
		theme?: { color: string };
		testMode?: boolean;
		label?: string;
		disabled?: boolean;
		error?: string;
		onpayment?: (result: RazorPayPaymentResult) => void;
		onerror?: (error: string) => void;
	}

	interface RazorPayPaymentResult {
		paymentId: string;
		orderId?: string;
		signature?: string;
		method?: string;
		bank?: string;
		wallet?: string;
		vpa?: string;
	}

	let {
		keyId,
		amount,
		currency = 'INR',
		name = 'Payment',
		description = '',
		orderId = '',
		customerEmail = '',
		customerPhone = '',
		customerName = '',
		prefill = true,
		theme = { color: '#3b82f6' },
		testMode = false,
		label = 'Pay Now',
		disabled = false,
		error = '',
		onpayment,
		onerror
	}: Props = $props();

	let loading = $state(false);
	let paymentError = $state('');

	async function loadRazorpay(): Promise<boolean> {
		return new Promise((resolve) => {
			if (window.Razorpay) {
				resolve(true);
				return;
			}

			const script = document.createElement('script');
			script.src = 'https://checkout.razorpay.com/v1/checkout.js';
			script.onload = () => resolve(true);
			script.onerror = () => resolve(false);
			document.head.appendChild(script);
		});
	}

	async function handlePayment() {
		if (loading || disabled) return;

		loading = true;
		paymentError = '';

		try {
			const loaded = await loadRazorpay();
			if (!loaded) {
				throw new Error('Failed to load Razorpay');
			}

			const options = {
				key: keyId,
				amount: Math.round(amount * 100), // Razorpay expects amount in paise
				currency,
				name,
				description,
				order_id: orderId || undefined,
				prefill: prefill
					? {
							name: customerName,
							email: customerEmail,
							contact: customerPhone
						}
					: undefined,
				theme,
				handler: (response: any) => {
					const result: RazorPayPaymentResult = {
						paymentId: response.razorpay_payment_id,
						orderId: response.razorpay_order_id,
						signature: response.razorpay_signature
					};

					if (onpayment) onpayment(result);
				},
				modal: {
					ondismiss: () => {
						paymentError = 'Payment was cancelled';
					}
				}
			};

			const rzp = new window.Razorpay(options);

			rzp.on('payment.failed', (response: any) => {
				paymentError = response.error?.description || 'Payment failed';
				if (onerror) onerror(paymentError);
			});

			rzp.open();
		} catch (err) {
			paymentError = err instanceof Error ? err.message : 'Payment failed';
			if (onerror) onerror(paymentError);
		} finally {
			loading = false;
		}
	}

	function formatAmount(amount: number, currency: string): string {
		return new Intl.NumberFormat('en-IN', {
			style: 'currency',
			currency: currency
		}).format(amount);
	}
</script>

<div class="razorpay-payment" class:disabled class:has-error={error || paymentError}>
	{#if testMode}
		<div class="test-mode-banner">
			<span>⚠️ Razorpay Test Mode</span>
		</div>
	{/if}

	<div class="amount-display">
		Total: <strong>{formatAmount(amount, currency)}</strong>
	</div>

	{#if paymentError}
		<p class="payment-error">{paymentError}</p>
	{/if}

	<button type="button" class="pay-button" onclick={handlePayment} disabled={disabled || loading}>
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
		<span class="payment-badge">💳 Cards</span>
		<span class="payment-badge">🏦 UPI</span>
		<span class="payment-badge">🏛️ Net Banking</span>
		<span class="payment-badge">💰 Wallets</span>
	</div>

	<div class="secure-badge">
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
			<path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
		</svg>
		<span>Secured by Razorpay</span>
	</div>

	{#if error}
		<p class="error-text">{error}</p>
	{/if}
</div>

<style>
	.razorpay-payment {
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
		background-color: #528ff0;
		color: white;
		border: none;
		border-radius: 0.5rem;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		transition: background-color 0.15s;
	}

	.pay-button:hover:not(:disabled) {
		background-color: #3d7dd8;
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
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
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

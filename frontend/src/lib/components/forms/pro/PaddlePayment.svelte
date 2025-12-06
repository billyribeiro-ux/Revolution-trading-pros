<script lang="ts">
	/**
	 * PaddlePayment Component (FluentForms Pro 6.1.8)
	 *
	 * Paddle payment integration for SaaS and software sales.
	 * Supports:
	 * - Credit/Debit Cards
	 * - PayPal
	 * - Wire Transfer
	 * - Automatic tax calculation
	 * - Subscription management
	 */

	interface Props {
		vendorId: number;
		productId?: number;
		planId?: number;
		amount?: number;
		currency?: string;
		customerEmail?: string;
		customerCountry?: string;
		passthrough?: Record<string, unknown>;
		successCallback?: string;
		closeCallback?: string;
		testMode?: boolean;
		label?: string;
		disabled?: boolean;
		error?: string;
		onpayment?: (result: PaddlePaymentResult) => void;
		onerror?: (error: string) => void;
	}

	interface PaddlePaymentResult {
		checkoutId: string;
		productId?: number;
		planId?: number;
		email: string;
		country: string;
		coupon?: string;
		status: string;
	}

	let {
		vendorId,
		productId,
		planId,
		amount,
		currency = 'USD',
		customerEmail = '',
		customerCountry = '',
		passthrough = {},
		testMode = false,
		label = 'Complete Purchase',
		disabled = false,
		error = '',
		onpayment,
		onerror
	}: Props = $props();

	let loading = $state(true);
	let processing = $state(false);
	let paymentError = $state('');

	$effect(() => {
		if (typeof window !== 'undefined') {
			loadPaddle();
		}
	});

	async function loadPaddle() {
		try {
			if (!window.Paddle) {
				const script = document.createElement('script');
				script.src = 'https://cdn.paddle.com/paddle/paddle.js';
				script.async = true;
				await new Promise((resolve, reject) => {
					script.onload = resolve;
					script.onerror = reject;
					document.head.appendChild(script);
				});
			}

			// Initialize Paddle
			if (testMode) {
				window.Paddle.Environment.set('sandbox');
			}
			window.Paddle.Setup({
				vendor: vendorId,
				eventCallback: (data: any) => {
					handlePaddleEvent(data);
				}
			});

			loading = false;
		} catch (err) {
			loading = false;
			paymentError = 'Failed to load payment system';
			if (onerror) onerror('Failed to initialize Paddle');
		}
	}

	function handlePaddleEvent(data: any) {
		switch (data.event) {
			case 'Checkout.Complete':
				const result: PaddlePaymentResult = {
					checkoutId: data.checkoutData?.checkout?.id,
					productId: data.checkoutData?.product?.id,
					planId: data.checkoutData?.subscription?.plan_id,
					email: data.checkoutData?.user?.email,
					country: data.checkoutData?.user?.country,
					coupon: data.checkoutData?.checkout?.coupon?.coupon_code,
					status: 'complete'
				};
				if (onpayment) onpayment(result);
				break;

			case 'Checkout.Close':
				if (!processing) {
					paymentError = 'Checkout was closed';
				}
				break;

			case 'Checkout.Error':
				paymentError = data.eventData?.error || 'Payment failed';
				if (onerror) onerror(paymentError);
				break;
		}
	}

	function openCheckout() {
		if (!window.Paddle || loading || disabled) return;

		processing = true;
		paymentError = '';

		const checkoutOptions: Record<string, unknown> = {
			email: customerEmail || undefined,
			country: customerCountry || undefined,
			passthrough: JSON.stringify(passthrough),
			successCallback: (data: any) => {
				processing = false;
			},
			closeCallback: () => {
				processing = false;
			}
		};

		if (productId) {
			checkoutOptions.product = productId;
			if (amount) {
				checkoutOptions.override = amount;
			}
		} else if (planId) {
			window.Paddle.Checkout.open({
				...checkoutOptions,
				product: planId
			});
			return;
		}

		window.Paddle.Checkout.open(checkoutOptions);
	}

	function formatAmount(amount: number, currency: string): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: currency
		}).format(amount);
	}

</script>

<div class="paddle-payment" class:disabled class:has-error={error || paymentError}>
	{#if testMode}
		<div class="test-mode-banner">
			<span>⚠️ Paddle Sandbox Mode</span>
		</div>
	{/if}

	{#if amount}
		<div class="amount-display">
			Total: <strong>{formatAmount(amount, currency)}</strong>
			<span class="tax-note">* Tax calculated at checkout</span>
		</div>
	{/if}

	{#if loading}
		<div class="loading-state">
			<svg class="spinner" viewBox="0 0 24 24">
				<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" fill="none" />
			</svg>
			<span>Loading checkout...</span>
		</div>
	{:else}
		{#if paymentError}
			<p class="payment-error">{paymentError}</p>
		{/if}

		<button
			type="button"
			class="pay-button"
			onclick={openCheckout}
			disabled={disabled || processing}
		>
			{#if processing}
				<svg class="button-spinner" viewBox="0 0 24 24">
					<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" fill="none" />
				</svg>
				Processing...
			{:else}
				{label}
			{/if}
		</button>

		<div class="features">
			<div class="feature">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<circle cx="12" cy="12" r="10"></circle>
					<path d="M12 6v6l4 2"></path>
				</svg>
				<span>Instant access</span>
			</div>
			<div class="feature">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<rect x="1" y="4" width="22" height="16" rx="2" ry="2"></rect>
					<line x1="1" y1="10" x2="23" y2="10"></line>
				</svg>
				<span>Multiple payment options</span>
			</div>
			<div class="feature">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path>
				</svg>
				<span>Secure checkout</span>
			</div>
		</div>

		<div class="secure-badge">
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
				<path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
			</svg>
			<span>Powered by Paddle</span>
		</div>
	{/if}

	{#if error}
		<p class="error-text">{error}</p>
	{/if}
</div>

<style>
	.paddle-payment {
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

	.tax-note {
		display: block;
		font-size: 0.6875rem;
		color: #6b7280;
		margin-top: 0.25rem;
	}

	.loading-state {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		padding: 2rem;
		background-color: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		color: #6b7280;
	}

	.spinner,
	.button-spinner {
		width: 20px;
		height: 20px;
		animation: spin 1s linear infinite;
	}

	.spinner circle,
	.button-spinner circle {
		stroke-dasharray: 60;
		stroke-dashoffset: 45;
		stroke-linecap: round;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
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
		background-color: #ffcc00;
		color: #1a1a1a;
		border: none;
		border-radius: 0.5rem;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		transition: background-color 0.15s;
	}

	.pay-button:hover:not(:disabled) {
		background-color: #e6b800;
	}

	.pay-button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.features {
		display: flex;
		justify-content: center;
		gap: 1.5rem;
	}

	.feature {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.75rem;
		color: #6b7280;
	}

	.feature svg {
		width: 16px;
		height: 16px;
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

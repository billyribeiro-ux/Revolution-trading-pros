<script lang="ts">
	/**
	 * PayPalPayment Component (FluentForms Pro 6.1.8)
	 *
	 * PayPal payment integration with Smart Payment Buttons.
	 * Supports:
	 * - PayPal Checkout
	 * - Credit/Debit Cards via PayPal
	 * - Pay Later options
	 * - Subscriptions
	 */

	interface Props {
		clientId: string;
		amount: number;
		currency?: string;
		intent?: 'capture' | 'authorize';
		description?: string;
		enablePayLater?: boolean;
		enableCard?: boolean;
		testMode?: boolean;
		label?: string;
		disabled?: boolean;
		error?: string;
		onpayment?: (result: PayPalPaymentResult) => void;
		onerror?: (error: string) => void;
	}

	interface PayPalPaymentResult {
		orderId: string;
		payerId: string;
		status: string;
		captureId?: string;
		payerEmail?: string;
		payerName?: string;
	}

	let {
		clientId,
		amount,
		currency = 'USD',
		intent = 'capture',
		description = 'Payment',
		enablePayLater = true,
		enableCard = true,
		testMode = false,
		label = 'Pay with PayPal',
		disabled = false,
		error = '',
		onpayment,
		onerror
	}: Props = $props();

	let loading = $state(true);
	let buttonsRendered = $state(false);
	let paypalError = $state('');
	let buttonContainerRef: HTMLDivElement;

	$effect(() => {
		if (typeof window !== 'undefined' && buttonContainerRef && !buttonsRendered) {
			loadPayPal();
		}
	});

	async function loadPayPal() {
		try {
			// Construct funding sources
			let fundingSources = 'paypal';
			if (enablePayLater) fundingSources += ',paylater';
			if (enableCard) fundingSources += ',card';

			// Load PayPal SDK
			if (!window.paypal) {
				const script = document.createElement('script');
				script.src = `https://www.paypal.com/sdk/js?client-id=${clientId}&currency=${currency}&intent=${intent}&enable-funding=${fundingSources}`;
				script.async = true;
				await new Promise((resolve, reject) => {
					script.onload = resolve;
					script.onerror = reject;
					document.head.appendChild(script);
				});
			}

			loading = false;

			// Render PayPal buttons
			setTimeout(() => {
				if (buttonContainerRef && window.paypal) {
					renderButtons();
				}
			}, 100);
		} catch (err) {
			loading = false;
			paypalError = 'Failed to load PayPal';
			if (onerror) onerror('Failed to initialize PayPal');
		}
	}

	function renderButtons() {
		if (buttonsRendered || !window.paypal) return;

		window.paypal.Buttons({
			style: {
				layout: 'vertical',
				color: 'gold',
				shape: 'rect',
				label: 'paypal',
				height: 45
			},
			createOrder: (_data: any, actions: any) => {
				return actions.order.create({
					purchase_units: [{
						description: description,
						amount: {
							currency_code: currency,
							value: amount.toFixed(2)
						}
					}]
				});
			},
			onApprove: async (data: any, actions: any) => {
				try {
					const order = await actions.order.capture();

					const result: PayPalPaymentResult = {
						orderId: data.orderID,
						payerId: data.payerID,
						status: order.status,
						captureId: order.purchase_units?.[0]?.payments?.captures?.[0]?.id,
						payerEmail: order.payer?.email_address,
						payerName: order.payer?.name?.given_name
					};

					if (onpayment) onpayment(result);
				} catch (err) {
					paypalError = 'Payment failed to complete';
					if (onerror) onerror('Payment capture failed');
				}
			},
			onError: (err: any) => {
				paypalError = 'Payment was cancelled or failed';
				if (onerror) onerror(err.message || 'PayPal error');
			},
			onCancel: () => {
				paypalError = 'Payment was cancelled';
			}
		}).render(buttonContainerRef);

		buttonsRendered = true;
	}

	function formatAmount(amount: number, currency: string): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: currency
		}).format(amount);
	}

	declare global {
		interface Window {
			paypal: any;
		}
	}
</script>

<div class="paypal-payment" class:disabled class:has-error={error || paypalError}>
	{#if label}
		<label class="field-label">{label}</label>
	{/if}

	{#if testMode}
		<div class="test-mode-banner">
			<span>⚠️ PayPal Sandbox Mode - Use test accounts</span>
		</div>
	{/if}

	<div class="amount-display">
		Total: <strong>{formatAmount(amount, currency)}</strong>
	</div>

	{#if loading}
		<div class="loading-state">
			<svg class="spinner" viewBox="0 0 24 24">
				<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" fill="none" />
			</svg>
			<span>Loading PayPal...</span>
		</div>
	{/if}

	<div bind:this={buttonContainerRef} class="paypal-buttons" class:hidden={loading}></div>

	{#if paypalError}
		<p class="paypal-error">{paypalError}</p>
	{/if}

	<div class="secure-badge">
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
			<path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
		</svg>
		<span>Secured by PayPal</span>
	</div>

	{#if error}
		<p class="error-text">{error}</p>
	{/if}
</div>

<style>
	.paypal-payment {
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

	.spinner {
		width: 20px;
		height: 20px;
		animation: spin 1s linear infinite;
	}

	.spinner circle {
		stroke-dasharray: 60;
		stroke-dashoffset: 45;
		stroke-linecap: round;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	.paypal-buttons {
		min-height: 45px;
	}

	.paypal-buttons.hidden {
		display: none;
	}

	.paypal-error {
		font-size: 0.75rem;
		color: #ef4444;
		margin: 0;
		text-align: center;
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

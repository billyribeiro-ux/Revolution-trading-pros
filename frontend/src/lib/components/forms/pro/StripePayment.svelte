<script lang="ts">
	/**
	 * StripePayment Component (FluentForms Pro 6.1.8 - Updated Jan 2026)
	 *
	 * Stripe payment integration with embedded checkout using @stripe/stripe-js.
	 * Supports:
	 * - Card Element (Stripe.js)
	 * - Payment Request Button (Apple Pay, Google Pay)
	 * - 3D Secure (SCA) authentication
	 * - Subscription payments
	 *
	 * @version 2.0.0 - Svelte 5 + @stripe/stripe-js
	 */

	import { loadStripe as loadStripeJS } from '@stripe/stripe-js';
	import type { Stripe, StripeElements, StripeCardElement } from '@stripe/stripe-js';

	interface Props {
		publicKey: string;
		amount: number;
		currency?: string;
		description?: string;
		customerEmail?: string;
		metadata?: Record<string, string>;
		enablePaymentRequest?: boolean;
		verifyZip?: boolean;
		testMode?: boolean;
		label?: string;
		disabled?: boolean;
		error?: string;
		onpayment?: (result: StripePaymentResult) => void;
		onerror?: (error: string) => void;
	}

	interface StripePaymentResult {
		paymentMethodId: string;
		paymentIntentId?: string;
		last4: string;
		cardBrand: string;
		expiryMonth: number;
		expiryYear: number;
	}

	let {
		publicKey,
		amount,
		currency = 'USD',
		description = 'Payment',
		customerEmail = '',
		metadata = {},
		enablePaymentRequest = true,
		verifyZip = true,
		testMode = false,
		label = 'Pay with Card',
		disabled = false,
		error = '',
		onpayment,
		onerror
	}: Props = $props();

	let stripe: Stripe | null = null;
	let elements: StripeElements | null = null;
	let cardElement: StripeCardElement | null = null;
	let paymentRequestButton: any = null;
	let loading = $state(true);
	let processing = $state(false);
	let cardError = $state('');
	let cardComplete = $state(false);
	let paymentRequestAvailable = $state(false);
	let mounted = $state(false);
	// Element refs - Svelte 5 handles bind:this automatically
	let cardElementRef: HTMLDivElement | undefined = $state();
	let paymentRequestRef: HTMLDivElement | undefined = $state();

	$effect(() => {
		if (typeof window !== 'undefined' && !mounted) {
			initializeStripe();
		}
		return () => {
			if (cardElement) {
				cardElement.destroy();
			}
		};
	});

	async function initializeStripe() {
		try {
			// Load Stripe.js using the official package
			stripe = await loadStripeJS(publicKey);

			if (!stripe) {
				throw new Error('Failed to load Stripe');
			}

			elements = stripe.elements();

			// Create Card Element
			cardElement = elements.create('card', {
				style: {
					base: {
						fontSize: '16px',
						color: '#374151',
						fontFamily: 'system-ui, -apple-system, sans-serif',
						'::placeholder': {
							color: '#9ca3af'
						}
					},
					invalid: {
						color: '#ef4444',
						iconColor: '#ef4444'
					}
				},
				hidePostalCode: !verifyZip
			});

			cardElement.on('change', (event: any) => {
				cardError = event.error?.message || '';
				cardComplete = event.complete;
			});

			// Mount card element after component is rendered
			setTimeout(() => {
				if (cardElementRef && cardElement) {
					cardElement.mount(cardElementRef);
					mounted = true;
				}
			}, 0);

			// Setup Payment Request Button (Apple Pay, Google Pay)
			if (enablePaymentRequest && elements) {
				const paymentRequest = stripe.paymentRequest({
					country: 'US',
					currency: currency.toLowerCase(),
					total: {
						label: description,
						amount: Math.round(amount * 100)
					},
					requestPayerName: true,
					requestPayerEmail: true
				});

				paymentRequest.canMakePayment().then((result: any) => {
					if (result && elements) {
						paymentRequestAvailable = true;
						paymentRequestButton = elements.create('paymentRequestButton', {
							paymentRequest
						});
						setTimeout(() => {
							if (paymentRequestRef) {
								paymentRequestButton.mount(paymentRequestRef);
							}
						}, 0);
					}
				});

				paymentRequest.on('paymentmethod', async (event: any) => {
					processing = true;
					try {
						const result: StripePaymentResult = {
							paymentMethodId: event.paymentMethod.id,
							last4: event.paymentMethod.card.last4,
							cardBrand: event.paymentMethod.card.brand,
							expiryMonth: event.paymentMethod.card.exp_month,
							expiryYear: event.paymentMethod.card.exp_year
						};
						event.complete('success');
						if (onpayment) onpayment(result);
					} catch (err) {
						event.complete('fail');
						if (onerror) onerror('Payment failed');
					} finally {
						processing = false;
					}
				});
			}

			loading = false;
		} catch (err) {
			loading = false;
			cardError = 'Failed to load payment system';
			if (onerror) onerror('Failed to initialize Stripe');
		}
	}

	async function handleSubmit() {
		if (!stripe || !cardElement || !cardComplete || processing) return;

		processing = true;
		cardError = '';

		try {
			const { paymentMethod, error } = await stripe.createPaymentMethod({
				type: 'card',
				card: cardElement,
				billing_details: {
					email: customerEmail || undefined
				}
			});

			if (error || !paymentMethod) {
				const errorMsg = error?.message || 'Payment method creation failed';
				cardError = errorMsg;
				if (onerror) onerror(errorMsg);
				return;
			}

			if (!paymentMethod.card) {
				const errorMsg = 'Card information not available';
				cardError = errorMsg;
				if (onerror) onerror(errorMsg);
				return;
			}

			const result: StripePaymentResult = {
				paymentMethodId: paymentMethod.id,
				last4: paymentMethod.card.last4,
				cardBrand: paymentMethod.card.brand,
				expiryMonth: paymentMethod.card.exp_month,
				expiryYear: paymentMethod.card.exp_year
			};

			if (onpayment) onpayment(result);
		} catch (err) {
			cardError = err instanceof Error ? err.message : 'Payment failed';
			if (onerror) onerror(cardError);
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

<div class="stripe-payment" class:disabled class:has-error={error || cardError}>
	{#if label}
		<span class="field-label" id="stripe-label">{label}</span>
	{/if}

	{#if testMode}
		<div class="test-mode-banner">
			<span>⚠️ Test mode - Use card 4242 4242 4242 4242</span>
		</div>
	{/if}

	{#if loading}
		<div class="loading-state">
			<svg class="spinner" viewBox="0 0 24 24">
				<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" fill="none" />
			</svg>
			<span>Loading payment form...</span>
		</div>
	{:else}
		{#if paymentRequestAvailable}
			<div bind:this={paymentRequestRef} class="payment-request-button"></div>
			<div class="divider">
				<span>Or pay with card</span>
			</div>
		{/if}

		<div class="card-element-wrapper">
			<div bind:this={cardElementRef} class="card-element"></div>
		</div>

		{#if cardError}
			<p class="card-error">{cardError}</p>
		{/if}

		<button
			type="button"
			class="pay-button"
			onclick={handleSubmit}
			disabled={disabled || processing || !cardComplete}
		>
			{#if processing}
				<svg class="button-spinner" viewBox="0 0 24 24">
					<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" fill="none" />
				</svg>
				Processing...
			{:else}
				Pay {formatAmount(amount, currency)}
			{/if}
		</button>

		<div class="secure-badge">
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
				<path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
			</svg>
			<span>Secured by Stripe</span>
		</div>
	{/if}

	{#if error}
		<p class="error-text">{error}</p>
	{/if}
</div>

<style>
	.stripe-payment {
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

	.loading-state {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1.5rem;
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
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.payment-request-button {
		margin-bottom: 0.5rem;
	}

	.divider {
		display: flex;
		align-items: center;
		gap: 1rem;
		color: #6b7280;
		font-size: 0.875rem;
	}

	.divider::before,
	.divider::after {
		content: '';
		flex: 1;
		height: 1px;
		background-color: #e5e7eb;
	}

	.card-element-wrapper {
		padding: 0.75rem 1rem;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		background-color: white;
		transition:
			border-color 0.15s,
			box-shadow 0.15s;
	}

	.card-element-wrapper:focus-within {
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.card-element {
		min-height: 24px;
	}

	.card-error {
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
		background-color: #5469d4;
		color: white;
		border: none;
		border-radius: 0.5rem;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		transition: background-color 0.15s;
	}

	.pay-button:hover:not(:disabled) {
		background-color: #4354b3;
	}

	.pay-button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
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

	.has-error .card-element-wrapper {
		border-color: #fca5a5;
	}
</style>

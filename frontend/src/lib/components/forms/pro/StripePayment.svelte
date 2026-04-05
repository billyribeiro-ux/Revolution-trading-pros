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
	 * ICT 7 Fix: Added explicit 3D Secure/SCA handling with confirmPaymentIntent
	 *
	 * @version 2.2.0 - Svelte 5 + @stripe/stripe-js + 3D Secure + Responsive Design
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
		clientSecret?: string; // ICT 7: For 3D Secure confirmation
		onpayment?: (result: StripePaymentResult) => void;
		onerror?: (error: string) => void;
		onrequiresaction?: (clientSecret: string) => void; // ICT 7: For 3D Secure callback
	}

	interface StripePaymentResult {
		paymentMethodId: string;
		paymentIntentId?: string;
		last4: string;
		cardBrand: string;
		expiryMonth: number;
		expiryYear: number;
		status?: string; // ICT 7: Payment status for 3D Secure flow
	}

	// ICT 7: Export confirmation function type for parent components
	export interface StripePaymentHandle {
		confirmPaymentIntent: (clientSecret: string) => Promise<StripePaymentResult | null>;
		getStripeInstance: () => Stripe | null;
	}

	let {
		publicKey,
		amount,
		currency = 'USD',
		description = 'Payment',
		customerEmail = '',
		metadata: _metadata = {},
		enablePaymentRequest = true,
		verifyZip = true,
		testMode = false,
		label = 'Pay with Card',
		disabled = false,
		error = '',
		clientSecret = '', // ICT 7: For 3D Secure confirmation
		onpayment,
		onerror,
		onrequiresaction // ICT 7: For 3D Secure callback
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
					} catch (_err) {
						event.complete('fail');
						if (onerror) onerror('Payment failed');
					} finally {
						processing = false;
					}
				});
			}

			loading = false;
		} catch (_err) {
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

	/**
	 * ICT 7 Fix: Confirm a PaymentIntent that requires 3D Secure authentication
	 * Called by parent component when server returns requires_action status
	 *
	 * @param secret - The client_secret from the PaymentIntent
	 * @returns Payment result with status, or null if failed
	 */
	export async function confirmPaymentIntent(secret: string): Promise<StripePaymentResult | null> {
		if (!stripe || !cardElement) {
			if (onerror) onerror('Payment system not initialized');
			return null;
		}

		processing = true;
		cardError = '';

		try {
			// Confirm the PaymentIntent with 3D Secure authentication
			const { paymentIntent, error: confirmError } = await stripe.confirmCardPayment(secret, {
				payment_method: {
					card: cardElement,
					billing_details: {
						email: customerEmail || undefined
					}
				}
			});

			if (confirmError) {
				const errorMsg = confirmError.message || 'Payment confirmation failed';
				cardError = errorMsg;
				if (onerror) onerror(errorMsg);
				return null;
			}

			if (!paymentIntent) {
				const errorMsg = 'No payment intent returned';
				cardError = errorMsg;
				if (onerror) onerror(errorMsg);
				return null;
			}

			// Check payment status
			if (paymentIntent.status === 'succeeded') {
				// Get payment method details
				const paymentMethod = paymentIntent.payment_method;
				let last4 = '****';
				let cardBrand = 'unknown';
				let expiryMonth = 0;
				let expiryYear = 0;

				// Payment method might be a string ID or expanded object
				if (typeof paymentMethod === 'object' && paymentMethod?.card) {
					last4 = paymentMethod.card.last4 || '****';
					cardBrand = paymentMethod.card.brand || 'unknown';
					expiryMonth = paymentMethod.card.exp_month || 0;
					expiryYear = paymentMethod.card.exp_year || 0;
				}

				const result: StripePaymentResult = {
					paymentMethodId:
						typeof paymentMethod === 'string' ? paymentMethod : paymentMethod?.id || '',
					paymentIntentId: paymentIntent.id,
					last4,
					cardBrand,
					expiryMonth,
					expiryYear,
					status: paymentIntent.status
				};

				if (onpayment) onpayment(result);
				return result;
			} else if (paymentIntent.status === 'requires_action') {
				// Still requires action (e.g., redirects)
				if (onrequiresaction) onrequiresaction(secret);
				return null;
			} else if (paymentIntent.status === 'requires_payment_method') {
				const errorMsg = 'Your card was declined. Please try a different payment method.';
				cardError = errorMsg;
				if (onerror) onerror(errorMsg);
				return null;
			} else {
				const errorMsg = `Payment status: ${paymentIntent.status}`;
				cardError = errorMsg;
				if (onerror) onerror(errorMsg);
				return null;
			}
		} catch (err) {
			cardError = err instanceof Error ? err.message : 'Payment confirmation failed';
			if (onerror) onerror(cardError);
			return null;
		} finally {
			processing = false;
		}
	}

	/**
	 * ICT 7: Get the Stripe instance for advanced operations
	 */
	export function getStripeInstance(): Stripe | null {
		return stripe;
	}

	/**
	 * ICT 7: Handle payment with 3D Secure flow
	 * If clientSecret is provided, confirms the payment intent
	 * Otherwise, creates a payment method for checkout
	 */
	$effect(() => {
		if (clientSecret && stripe && cardElement && !processing) {
			confirmPaymentIntent(clientSecret);
		}
	});
</script>

<div class="sp-wrap" class:sp-disabled={disabled}>
	{#if label}
		<span class="sp-label" id="stripe-label">
			{label}
		</span>
	{/if}

	{#if testMode}
		<div class="sp-test-notice">
			<span>Test mode - Use card 4242 4242 4242 4242</span>
		</div>
	{/if}

	{#if loading}
		<div class="sp-loading">
			<svg class="sp-spinner" viewBox="0 0 24 24">
				<circle
					cx="12"
					cy="12"
					r="10"
					stroke="currentColor"
					stroke-width="3"
					fill="none"
					stroke-dasharray="60"
					stroke-dashoffset="45"
					stroke-linecap="round"
				/>
			</svg>
			<span>Loading payment form...</span>
		</div>
	{:else}
		{#if paymentRequestAvailable}
			<div bind:this={paymentRequestRef} class="sp-pr-button"></div>
			<div class="sp-divider">
				<span class="sp-divider-line"></span>
				<span>Or pay with card</span>
				<span class="sp-divider-line"></span>
			</div>
		{/if}

		<div class="sp-card-wrap" class:sp-card-error={error || cardError}>
			<div bind:this={cardElementRef} class="sp-card-element"></div>
		</div>

		{#if cardError}
			<p class="sp-error-text">{cardError}</p>
		{/if}

		<button
			type="button"
			class="sp-pay-btn"
			onclick={handleSubmit}
			disabled={disabled || processing || !cardComplete}
		>
			{#if processing}
				<svg class="sp-spinner" viewBox="0 0 24 24">
					<circle
						cx="12"
						cy="12"
						r="10"
						stroke="currentColor"
						stroke-width="3"
						fill="none"
						stroke-dasharray="60"
						stroke-dashoffset="45"
						stroke-linecap="round"
					/>
				</svg>
				<span>Processing...</span>
			{:else}
				<span>Pay {formatAmount(amount, currency)}</span>
			{/if}
		</button>

		<div class="sp-secure">
			<svg
				class="sp-secure-icon"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
				<path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
			</svg>
			<span>Secured by Stripe</span>
		</div>
	{/if}

	{#if error}
		<p class="sp-error-text">{error}</p>
	{/if}
</div>

<style>
	.sp-wrap {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		inline-size: 100%;
		max-inline-size: 100%;
		margin-inline: auto;
		padding: 0.75rem;
		padding-block-end: env(safe-area-inset-bottom);

		@media (min-width: 640px) {
			gap: 1rem;
			max-inline-size: 32rem;
			padding: 1rem;
		}
		@media (min-width: 768px) {
			max-inline-size: 36rem;
			padding: 1.5rem;
		}
		@media (min-width: 1024px) {
			max-inline-size: 42rem;
		}
	}

	.sp-disabled {
		opacity: 0.6;
		pointer-events: none;
	}

	.sp-label {
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(0.35 0.01 265);

		@media (min-width: 640px) {
			font-size: var(--text-base);
		}
	}

	.sp-test-notice {
		padding: 0.5rem 0.75rem;
		background-color: oklch(0.95 0.03 85);
		border: 1px solid oklch(0.8 0.08 85);
		border-radius: var(--radius-md);
		font-size: var(--text-xs);
		color: oklch(0.35 0.08 85);

		@media (min-width: 640px) {
			padding: 0.75rem 1rem;
			font-size: var(--text-sm);
		}
	}

	.sp-loading {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background-color: oklch(0.97 0.005 265);
		border: 1px solid oklch(0.9 0.005 265);
		border-radius: var(--radius-lg);
		color: oklch(0.45 0.005 265);
		font-size: var(--text-sm);

		@media (min-width: 640px) {
			gap: 1rem;
			padding: 1.25rem;
			font-size: var(--text-base);
		}
	}

	.sp-spinner {
		inline-size: 1.25rem;
		block-size: 1.25rem;
		flex-shrink: 0;
		animation: sp-spin 1s linear infinite;

		@media (min-width: 640px) {
			inline-size: 1.5rem;
			block-size: 1.5rem;
		}
	}

	@keyframes sp-spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.sp-pr-button {
		margin-block-end: 0.5rem;
		min-block-size: 2.75rem;

		@media (min-width: 640px) {
			min-block-size: 3rem;
		}
	}

	.sp-divider {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		color: oklch(0.45 0.005 265);
		font-size: var(--text-xs);

		@media (min-width: 640px) {
			gap: 1rem;
			font-size: var(--text-sm);
		}
	}

	.sp-divider-line {
		flex: 1;
		block-size: 1px;
		background-color: oklch(0.9 0.005 265);
	}

	.sp-card-wrap {
		padding: 0.75rem;
		border: 1px solid oklch(0.75 0.005 265);
		border-radius: var(--radius-lg);
		background-color: oklch(1 0 0);
		transition:
			border-color 150ms,
			box-shadow 150ms;
		min-block-size: 3rem;

		@media (min-width: 640px) {
			padding: 1rem;
			min-block-size: 3.25rem;
		}

		&:focus-within {
			border-color: oklch(0.5 0.18 260);
			box-shadow: 0 0 0 3px oklch(0.5 0.18 260 / 0.1);
		}
	}

	.sp-card-error {
		border-color: oklch(0.65 0.15 25);
	}

	.sp-card-element {
		min-block-size: 1.5rem;

		@media (min-width: 640px) {
			min-block-size: 1.75rem;
		}
	}

	.sp-error-text {
		font-size: var(--text-xs);
		color: oklch(0.5 0.2 25);
		margin: 0;
		padding-inline: 0.25rem;

		@media (min-width: 640px) {
			font-size: var(--text-sm);
		}
	}

	.sp-pay-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		inline-size: 100%;
		min-block-size: 3rem;
		padding: 0.75rem 1rem;
		background-color: oklch(0.4 0.15 260);
		color: oklch(1 0 0);
		border: none;
		border-radius: var(--radius-lg);
		font-size: var(--text-base);
		font-weight: var(--weight-semibold);
		cursor: pointer;
		transition:
			background-color 150ms,
			transform 100ms;
		touch-action: manipulation;

		@media (min-width: 640px) {
			min-block-size: 3.25rem;
			padding: 1rem 1.5rem;
			font-size: var(--text-lg);
		}
		@media (min-width: 768px) {
			min-block-size: 3.5rem;
		}

		&:hover:not(:disabled) {
			background-color: oklch(0.35 0.12 260);
		}
		&:active:not(:disabled) {
			transform: scale(0.98);
		}
		&:disabled {
			opacity: 0.6;
			cursor: not-allowed;
		}
	}

	.sp-secure {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		font-size: var(--text-xs);
		color: oklch(0.45 0.005 265);
		padding-block: 0.5rem;

		@media (min-width: 640px) {
			font-size: var(--text-sm);
			gap: 0.5rem;
		}
	}

	.sp-secure-icon {
		inline-size: 0.875rem;
		block-size: 0.875rem;
		flex-shrink: 0;

		@media (min-width: 640px) {
			inline-size: 1rem;
			block-size: 1rem;
		}
	}
</style>

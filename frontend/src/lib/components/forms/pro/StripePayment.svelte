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

	import { onMount } from 'svelte';
	import { loadStripe as loadStripeJS } from '@stripe/stripe-js';
	import Icon from '$lib/components/Icon.svelte';
	import type { Attachment } from 'svelte/attachments';
	import type {
		Stripe,
		StripeElements,
		StripeCardElement,
		StripeCardElementChangeEvent,
		StripePaymentRequestButtonElement,
		PaymentRequestPaymentMethodEvent,
		CanMakePaymentResult
	} from '@stripe/stripe-js';

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
	let paymentRequestButton: StripePaymentRequestButtonElement | null = null;
	let loading = $state(true);
	let processing = $state(false);
	let cardError = $state('');
	let cardComplete = $state(false);
	let paymentRequestAvailable = $state(false);
	let mounted = $state(false);
	let cardElementNode: HTMLDivElement | null = null;
	let paymentRequestNode: HTMLDivElement | null = null;
	let autoConfirmedClientSecret = '';

	onMount(() => {
		void initializeStripe();

		return () => {
			if (cardElement) {
				cardElement.destroy();
			}
			if (paymentRequestButton) {
				paymentRequestButton.destroy();
			}
		};
	});

	function mountCardElement() {
		if (cardElementNode && cardElement && !mounted) {
			cardElement.mount(cardElementNode);
			mounted = true;
		}
	}

	function mountPaymentRequestButton() {
		if (paymentRequestNode && paymentRequestButton) {
			paymentRequestButton.mount(paymentRequestNode);
		}
	}

	const attachCardElement: Attachment<HTMLDivElement> = (node) => {
		cardElementNode = node;
		mountCardElement();

		return () => {
			cardElementNode = null;
		};
	};

	const attachPaymentRequest: Attachment<HTMLDivElement> = (node) => {
		paymentRequestNode = node;
		mountPaymentRequestButton();

		return () => {
			paymentRequestNode = null;
		};
	};

	function attachClientSecretConfirmation(secret: string): Attachment<HTMLDivElement> {
		return () => {
			// Track readiness state so the attachment re-runs after Stripe mounts.
			mounted;

			if (
				!secret ||
				!stripe ||
				!cardElement ||
				processing ||
				secret === autoConfirmedClientSecret
			) {
				return;
			}

			autoConfirmedClientSecret = secret;
			void confirmPaymentIntent(secret);
		};
	}

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

			cardElement.on('change', (event: StripeCardElementChangeEvent) => {
				cardError = event.error?.message || '';
				cardComplete = event.complete;
			});

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

				paymentRequest.canMakePayment().then((result: CanMakePaymentResult | null) => {
					if (result && elements) {
						paymentRequestAvailable = true;
						paymentRequestButton = elements.create('paymentRequestButton', {
							paymentRequest
						});
						mountPaymentRequestButton();
					}
				});

				paymentRequest.on('paymentmethod', async (event: PaymentRequestPaymentMethodEvent) => {
					processing = true;
					try {
						const card = event.paymentMethod.card;
						if (!card) {
							event.complete('fail');
							if (onerror) onerror('Payment failed');
							return;
						}
						const result: StripePaymentResult = {
							paymentMethodId: event.paymentMethod.id,
							last4: card.last4,
							cardBrand: card.brand,
							expiryMonth: card.exp_month,
							expiryYear: card.exp_year
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
			mountCardElement();
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
</script>

<!-- Responsive Stripe Payment Component - Mobile-first design -->
<div
	{@attach clientSecret ? attachClientSecretConfirmation(clientSecret) : false}
	class={['stripe-payment', disabled && 'stripe-payment--disabled']}
>
	{#if label}
		<span class="stripe-label" id="stripe-label">{label}</span>
	{/if}

	{#if testMode}
		<div class="test-banner">
			<span>Test mode - Use card 4242 4242 4242 4242</span>
		</div>
	{/if}

	{#if loading}
		<div class="loading-panel">
			<span class="spinner-icon">
				<Icon name="IconLoader2" />
			</span>
			<span>Loading payment form...</span>
		</div>
	{:else}
		{#if paymentRequestAvailable}
			<!-- Payment Request Button (Apple Pay, Google Pay) - Touch-friendly -->
			<div {@attach attachPaymentRequest} class="payment-request-slot"></div>
			<div class="payment-divider">
				<span class="payment-divider__line"></span>
				<span>Or pay with card</span>
				<span class="payment-divider__line"></span>
			</div>
		{/if}

		<!-- Card Element Wrapper - Larger touch area on mobile -->
		<div class={['card-element-shell', (error || cardError) && 'card-element-shell--invalid']}>
			<div {@attach attachCardElement} class="card-element-target"></div>
		</div>

		{#if cardError}
			<p class="payment-error">{cardError}</p>
		{/if}

		<!-- Pay Button - Full width mobile, touch-friendly 44px+ height -->
		<button
			type="button"
			onclick={handleSubmit}
			disabled={disabled || processing || !cardComplete}
			class="pay-button"
		>
			{#if processing}
				<span class="spinner-icon spinner-icon--button">
					<Icon name="IconLoader2" />
				</span>
				<span>Processing...</span>
			{:else}
				<span>Pay {formatAmount(amount, currency)}</span>
			{/if}
		</button>

		<!-- Secure Badge -->
		<div class="secure-badge">
			<span class="secure-badge__icon">
				<Icon name="IconLock" />
			</span>
			<span>Secured by Stripe</span>
		</div>
	{/if}

	{#if error}
		<p class="payment-error">{error}</p>
	{/if}
</div>

<style>
	.stripe-payment {
		display: flex;
		width: 100%;
		max-width: 100%;
		flex-direction: column;
		gap: 0.75rem;
		margin-inline: auto;
		padding: 0.75rem;
		padding-bottom: calc(0.75rem + env(safe-area-inset-bottom));
	}

	.stripe-payment--disabled {
		opacity: 0.6;
		pointer-events: none;
	}

	.stripe-label {
		color: #374151;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.test-banner {
		padding: 0.5rem;
		border: 1px solid #f59e0b;
		border-radius: 0.375rem;
		background: #fffbeb;
		color: #92400e;
		font-size: 0.75rem;
	}

	.loading-panel {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		background: #f9fafb;
		color: #6b7280;
		font-size: 0.875rem;
	}

	.spinner-icon {
		display: inline-flex;
		width: 1.25rem;
		height: 1.25rem;
		flex: 0 0 auto;
		animation: stripe-spin 0.8s linear infinite;
	}

	.payment-request-slot {
		min-height: 44px;
		margin-bottom: 0.5rem;
	}

	.payment-divider {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		color: #6b7280;
		font-size: 0.75rem;
	}

	.payment-divider__line {
		height: 1px;
		flex: 1 1 auto;
		background: #e5e7eb;
	}

	.card-element-shell {
		min-height: 48px;
		padding: 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		background: #fff;
		transition:
			border-color 0.15s ease,
			box-shadow 0.15s ease;
	}

	.card-element-shell:focus-within {
		border-color: #3b82f6;
		box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
	}

	.card-element-shell--invalid {
		border-color: #fca5a5;
	}

	.card-element-target {
		min-height: 24px;
	}

	.payment-error {
		margin: 0;
		padding-inline: 0.25rem;
		color: #ef4444;
		font-size: 0.75rem;
	}

	.pay-button {
		display: flex;
		width: 100%;
		min-height: 48px;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		border: 0;
		border-radius: 0.5rem;
		background: #4f46e5;
		color: #fff;
		cursor: pointer;
		font-size: 1rem;
		font-weight: 600;
		touch-action: manipulation;
		transition:
			background-color 0.15s ease,
			opacity 0.15s ease,
			transform 0.15s ease;
	}

	.pay-button:hover:not(:disabled) {
		background: #4338ca;
	}

	.pay-button:active:not(:disabled) {
		transform: scale(0.98);
	}

	.pay-button:disabled {
		cursor: not-allowed;
		opacity: 0.6;
	}

	.spinner-icon--button {
		width: 1.25rem;
		height: 1.25rem;
	}

	.secure-badge {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding-block: 0.5rem;
		color: #6b7280;
		font-size: 0.75rem;
	}

	.secure-badge__icon {
		display: inline-flex;
		width: 0.875rem;
		height: 0.875rem;
		flex: 0 0 auto;
	}

	@keyframes stripe-spin {
		to {
			transform: rotate(360deg);
		}
	}

	@media (min-width: 640px) {
		.stripe-payment {
			max-width: 32rem;
			gap: 1rem;
			padding: 1rem;
			padding-bottom: calc(1rem + env(safe-area-inset-bottom));
		}

		.stripe-label,
		.loading-panel {
			font-size: 1rem;
		}

		.test-banner,
		.payment-divider,
		.payment-error,
		.secure-badge {
			font-size: 0.875rem;
		}

		.test-banner {
			padding: 0.75rem;
		}

		.loading-panel {
			gap: 1rem;
			padding: 1.25rem;
		}

		.spinner-icon {
			width: 1.5rem;
			height: 1.5rem;
		}

		.payment-request-slot {
			min-height: 48px;
			margin-bottom: 0.75rem;
		}

		.payment-divider {
			gap: 1rem;
		}

		.card-element-shell {
			min-height: 52px;
			padding: 1rem;
		}

		.card-element-target {
			min-height: 28px;
		}

		.pay-button {
			min-height: 52px;
			gap: 0.75rem;
			padding: 1rem 1.5rem;
			font-size: 1.125rem;
		}

		.spinner-icon--button {
			width: 1.5rem;
			height: 1.5rem;
		}

		.secure-badge {
			gap: 0.5rem;
		}

		.secure-badge__icon {
			width: 1rem;
			height: 1rem;
		}
	}

	@media (min-width: 768px) {
		.stripe-payment {
			max-width: 36rem;
			padding: 1.5rem;
			padding-bottom: calc(1.5rem + env(safe-area-inset-bottom));
		}

		.loading-panel {
			padding: 1.5rem;
		}

		.pay-button {
			min-height: 56px;
		}
	}

	@media (min-width: 1024px) {
		.stripe-payment {
			max-width: 42rem;
		}
	}
</style>

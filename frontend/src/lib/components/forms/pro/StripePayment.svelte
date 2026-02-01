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
	 * @version 2.1.0 - Svelte 5 + @stripe/stripe-js + Responsive Design
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

<!-- Responsive Stripe Payment Component - Mobile-first design -->
<div
	class="flex flex-col gap-3 sm:gap-4 w-full max-w-full sm:max-w-lg md:max-w-xl lg:max-w-2xl mx-auto p-3 sm:p-4 md:p-6 pb-[env(safe-area-inset-bottom)]"
	class:opacity-60={disabled}
	class:pointer-events-none={disabled}
>
	{#if label}
		<span
			class="text-sm sm:text-base font-medium text-gray-700"
			id="stripe-label"
		>
			{label}
		</span>
	{/if}

	{#if testMode}
		<div class="p-2 sm:p-3 bg-amber-50 border border-amber-500 rounded-md text-xs sm:text-sm text-amber-800">
			<span>Test mode - Use card 4242 4242 4242 4242</span>
		</div>
	{/if}

	{#if loading}
		<div class="flex items-center gap-3 sm:gap-4 p-4 sm:p-5 md:p-6 bg-gray-50 border border-gray-200 rounded-lg text-gray-500 text-sm sm:text-base">
			<svg class="w-5 h-5 sm:w-6 sm:h-6 animate-spin flex-shrink-0" viewBox="0 0 24 24">
				<circle
					cx="12" cy="12" r="10"
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
			<!-- Payment Request Button (Apple Pay, Google Pay) - Touch-friendly -->
			<div
				bind:this={paymentRequestRef}
				class="mb-2 sm:mb-3 min-h-[44px] sm:min-h-[48px]"
			></div>
			<div class="flex items-center gap-3 sm:gap-4 text-gray-500 text-xs sm:text-sm">
				<span class="flex-1 h-px bg-gray-200"></span>
				<span>Or pay with card</span>
				<span class="flex-1 h-px bg-gray-200"></span>
			</div>
		{/if}

		<!-- Card Element Wrapper - Larger touch area on mobile -->
		<div
			class="p-3 sm:p-4 border border-gray-300 rounded-lg bg-white transition-all duration-150 min-h-[48px] sm:min-h-[52px] focus-within:border-blue-500 focus-within:ring-2 focus-within:ring-blue-500/10"
			class:border-red-300={error || cardError}
		>
			<div
				bind:this={cardElementRef}
				class="min-h-[24px] sm:min-h-[28px]"
			></div>
		</div>

		{#if cardError}
			<p class="text-xs sm:text-sm text-red-500 m-0 px-1">{cardError}</p>
		{/if}

		<!-- Pay Button - Full width mobile, touch-friendly 44px+ height -->
		<button
			type="button"
			class="flex items-center justify-center gap-2 sm:gap-3 w-full min-h-[48px] sm:min-h-[52px] md:min-h-[56px] px-4 sm:px-6 py-3 sm:py-4 bg-indigo-600 hover:bg-indigo-700 text-white border-none rounded-lg text-base sm:text-lg font-semibold cursor-pointer transition-colors duration-150 disabled:opacity-60 disabled:cursor-not-allowed active:scale-[0.98] touch-manipulation"
			onclick={handleSubmit}
			disabled={disabled || processing || !cardComplete}
		>
			{#if processing}
				<svg class="w-5 h-5 sm:w-6 sm:h-6 animate-spin flex-shrink-0" viewBox="0 0 24 24">
					<circle
						cx="12" cy="12" r="10"
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

		<!-- Secure Badge -->
		<div class="flex items-center justify-center gap-1.5 sm:gap-2 text-xs sm:text-sm text-gray-500 py-2">
			<svg class="w-3.5 h-3.5 sm:w-4 sm:h-4 flex-shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
				<path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
			</svg>
			<span>Secured by Stripe</span>
		</div>
	{/if}

	{#if error}
		<p class="text-xs sm:text-sm text-red-500 m-0 px-1">{error}</p>
	{/if}
</div>

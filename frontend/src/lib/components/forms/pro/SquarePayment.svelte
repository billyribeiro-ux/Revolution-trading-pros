<script lang="ts">
	/**
	 * SquarePayment Component (FluentForms Pro 6.1.8)
	 *
	 * Square payment integration with Web Payments SDK.
	 * Supports:
	 * - Card payments
	 * - Apple Pay / Google Pay
	 * - ACH bank transfers
	 *
	 * NOTE: This component is not yet wired into any route/form (FluentForms Pro
	 * port). The Apple/Google Pay flow follows the current Web Payments SDK API
	 * (paymentRequest → applePay/googlePay → tokenize) but has NOT been verified
	 * against a live Square sandbox — do that before enabling it in production.
	 */

	interface Props {
		applicationId: string;
		locationId: string;
		amount: number;
		currency?: string;
		testMode?: boolean;
		enableApplePay?: boolean;
		enableGooglePay?: boolean;
		label?: string;
		disabled?: boolean;
		error?: string;
		onpayment?: (result: SquarePaymentResult) => void;
		onerror?: (error: string) => void;
	}

	interface SquarePaymentResult {
		token: string;
		last4?: string;
		cardBrand?: string;
		expirationMonth?: string;
		expirationYear?: string;
	}

	import Icon from '$lib/components/Icon.svelte';
	import type {
		Payments,
		Card,
		ApplePay,
		GooglePay,
		PaymentRequest
	} from '@square/web-payments-sdk-types';

	let {
		applicationId,
		locationId,
		amount,
		currency = 'USD',
		testMode = false,
		enableApplePay = true,
		enableGooglePay = true,
		label = 'Pay with Card',
		disabled = false,
		error = '',
		onpayment,
		onerror
	}: Props = $props();

	let card: Card | null = null;
	let applePay: ApplePay | null = null;
	let googlePay: GooglePay | null = null;
	let loading = $state(true);
	let processing = $state(false);
	let cardError = $state('');
	let cardContainerRef = $state<HTMLDivElement>();
	let googlePayContainerRef = $state<HTMLDivElement>();
	let applePayAvailable = $state(false);
	let googlePayAvailable = $state(false);

	$effect(() => {
		if (typeof window !== 'undefined' && cardContainerRef) {
			loadSquare();
		}
		return () => {
			void card?.destroy();
			void applePay?.destroy();
			void googlePay?.destroy();
		};
	});

	// Digital wallets (Apple Pay / Google Pay) require a PaymentRequest. `amount`
	// is a decimal string in major units (e.g. "10.00"), not cents.
	function buildPaymentRequest(sdk: Payments): PaymentRequest {
		return sdk.paymentRequest({
			countryCode: 'US',
			currencyCode: currency,
			total: { amount: amount.toFixed(2), label: 'Total' }
		});
	}

	async function loadSquare() {
		try {
			// Load Square Web Payments SDK
			if (!window.Square) {
				const script = document.createElement('script');
				script.src = testMode
					? 'https://sandbox.web.squarecdn.com/v1/square.js'
					: 'https://web.squarecdn.com/v1/square.js';
				script.async = true;
				await new Promise((resolve, reject) => {
					script.onload = resolve;
					script.onerror = reject;
					document.head.appendChild(script);
				});
			}

			const sdk = window.Square;
			if (!sdk) throw new Error('Square Web Payments SDK unavailable');
			const pay = sdk.payments(applicationId, locationId);

			// Initialize Card
			const cardInstance = await pay.card();
			card = cardInstance;
			if (cardContainerRef) await cardInstance.attach(cardContainerRef);

			// Apple Pay: built from a PaymentRequest. `ApplePay` has no `attach()`
			// — we render our own button and call `tokenize()` on click.
			if (enableApplePay) {
				try {
					applePay = await pay.applePay(buildPaymentRequest(pay));
					applePayAvailable = true;
				} catch {
					// Apple Pay unavailable on this device/browser
				}
			}

			// Google Pay: `attach()` renders the wallet button; `tokenize()` on click.
			if (enableGooglePay && googlePayContainerRef) {
				try {
					const gp = await pay.googlePay(buildPaymentRequest(pay));
					googlePay = gp;
					await gp.attach(googlePayContainerRef);
					googlePayAvailable = true;
				} catch {
					// Google Pay unavailable
				}
			}

			loading = false;
		} catch (_err) {
			loading = false;
			cardError = 'Failed to load payment form';
			if (onerror) onerror('Failed to initialize Square');
		}
	}

	async function payWithWallet(wallet: ApplePay | GooglePay, brand: string) {
		if (processing) return;
		processing = true;
		cardError = '';
		try {
			const result = await wallet.tokenize();
			if (result.status === 'OK' && result.token) {
				if (onpayment) onpayment({ token: result.token, cardBrand: brand });
			} else {
				const msg =
					'errors' in result ? result.errors?.map((e) => e.message).join(', ') : undefined;
				cardError = msg || `${brand} payment failed`;
				if (onerror) onerror(cardError);
			}
		} catch (err) {
			cardError = err instanceof Error ? err.message : `${brand} payment failed`;
			if (onerror) onerror(cardError);
		} finally {
			processing = false;
		}
	}

	async function handleSubmit() {
		if (!card || processing) return;

		processing = true;
		cardError = '';

		try {
			const result = await card.tokenize();

			if (result.status === 'OK' && result.token) {
				const paymentResult: SquarePaymentResult = {
					token: result.token,
					last4: result.details?.card?.last4,
					cardBrand: result.details?.card?.brand,
					expirationMonth: result.details?.card?.expMonth?.toString(),
					expirationYear: result.details?.card?.expYear?.toString()
				};

				if (onpayment) onpayment(paymentResult);
			} else {
				const errorMessages =
					'errors' in result ? result.errors?.map((e) => e.message).join(', ') : undefined;
				cardError = errorMessages || 'Card tokenization failed';
				if (onerror) onerror(cardError);
			}
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

<div class="square-payment" class:disabled class:has-error={error || cardError}>
	{#if label}
		<label class="field-label" for="square-card-container">{label}</label>
	{/if}

	{#if testMode}
		<div class="test-mode-banner">
			<span>⚠️ Square Sandbox Mode</span>
		</div>
	{/if}

	{#if loading}
		<div class="loading-state">
			<Icon name="IconLoader2" size={20} class="spinner" />
			<span>Loading payment form...</span>
		</div>
	{:else}
		{#if applePayAvailable || googlePayAvailable}
			<div class="digital-wallets">
				{#if applePayAvailable}
					<button
						type="button"
						class="wallet-button apple-pay-button"
						aria-label="Pay with Apple Pay"
						disabled={processing}
						onclick={() => applePay && payWithWallet(applePay, 'Apple Pay')}
					></button>
				{/if}
				{#if googlePayAvailable}
					<!-- GooglePay.attach() renders the wallet button into this element;
					     clicking it triggers tokenize() (Square Web Payments SDK). -->
					<div
						bind:this={googlePayContainerRef}
						class="wallet-button"
						role="button"
						tabindex="0"
						aria-label="Pay with Google Pay"
						onclick={() => googlePay && payWithWallet(googlePay, 'Google Pay')}
						onkeydown={(e) => {
							if (e.key === 'Enter' || e.key === ' ') {
								e.preventDefault();
								if (googlePay) payWithWallet(googlePay, 'Google Pay');
							}
						}}
					></div>
				{/if}
			</div>
			<div class="divider">
				<span>Or pay with card</span>
			</div>
		{/if}

		<div id="square-card-container" bind:this={cardContainerRef} class="card-container"></div>

		{#if cardError}
			<p class="card-error">{cardError}</p>
		{/if}

		<button
			type="button"
			class="pay-button"
			onclick={handleSubmit}
			disabled={disabled || processing}
		>
			{#if processing}
				<Icon name="IconLoader2" size={20} class="button-spinner" />
				Processing...
			{:else}
				Pay {formatAmount(amount, currency)}
			{/if}
		</button>

		<div class="secure-badge">
			<Icon name="IconLock" size={14} />
			<span>Secured by Square</span>
		</div>
	{/if}

	{#if error}
		<p class="error-text">{error}</p>
	{/if}
</div>

<style>
	.square-payment {
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
		justify-content: center;
		gap: 0.75rem;
		padding: 2rem;
		background-color: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		color: #6b7280;
	}

	:global(.spinner),
	:global(.button-spinner) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.digital-wallets {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.wallet-button {
		min-height: 48px;
		cursor: pointer;
	}

	.apple-pay-button {
		-webkit-appearance: -apple-pay-button;
		appearance: -apple-pay-button;
		-apple-pay-button-type: plain;
		-apple-pay-button-style: black;
		border: none;
		border-radius: 0.375rem;
		width: 100%;
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

	.card-container {
		min-height: 90px;
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
		background-color: #006aff;
		color: white;
		border: none;
		border-radius: 0.5rem;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		transition: background-color 0.15s;
	}

	.pay-button:hover:not(:disabled) {
		background-color: #0058d4;
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

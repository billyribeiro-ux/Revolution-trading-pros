<script lang="ts">
	/**
	 * SquarePayment Component (FluentForms Pro 6.1.8)
	 *
	 * Square payment integration with Web Payments SDK.
	 * Supports:
	 * - Card payments
	 * - Apple Pay / Google Pay
	 * - ACH bank transfers
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

	let payments: any = null;
	let card: any = null;
	let loading = $state(true);
	let processing = $state(false);
	let cardError = $state('');
	let cardContainerRef: HTMLDivElement;
	let applePayContainerRef: HTMLDivElement;
	let googlePayContainerRef: HTMLDivElement;
	let applePayAvailable = $state(false);
	let googlePayAvailable = $state(false);

	$effect(() => {
		if (typeof window !== 'undefined' && cardContainerRef) {
			loadSquare();
		}
		return () => {
			if (card) card.destroy();
		};
	});

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

			payments = window.Square.payments(applicationId, locationId);

			// Initialize Card
			card = await payments.card();
			await card.attach(cardContainerRef);

			// Check Apple Pay availability
			if (enableApplePay && applePayContainerRef) {
				try {
					const applePay = await payments.applePay({
						countryCode: 'US',
						currencyCode: currency,
						total: {
							amount: (amount * 100).toString(),
							label: 'Total'
						}
					});
					applePayAvailable = true;
					await applePay.attach(applePayContainerRef);

					applePay.addEventListener('payment', async (event: any) => {
						processing = true;
						try {
							const result = await applePay.tokenize();
							if (result.status === 'OK') {
								if (onpayment) {
									onpayment({
										token: result.token,
										cardBrand: 'Apple Pay'
									});
								}
							}
						} finally {
							processing = false;
						}
					});
				} catch {
					// Apple Pay not available
				}
			}

			// Check Google Pay availability
			if (enableGooglePay && googlePayContainerRef) {
				try {
					const googlePay = await payments.googlePay({
						countryCode: 'US',
						currencyCode: currency,
						totalPrice: amount.toString(),
						totalPriceStatus: 'FINAL'
					});
					googlePayAvailable = true;
					await googlePay.attach(googlePayContainerRef);

					googlePay.addEventListener('payment', async (event: any) => {
						processing = true;
						try {
							const result = await googlePay.tokenize();
							if (result.status === 'OK') {
								if (onpayment) {
									onpayment({
										token: result.token,
										cardBrand: 'Google Pay'
									});
								}
							}
						} finally {
							processing = false;
						}
					});
				} catch {
					// Google Pay not available
				}
			}

			loading = false;
		} catch (err) {
			loading = false;
			cardError = 'Failed to load payment form';
			if (onerror) onerror('Failed to initialize Square');
		}
	}

	async function handleSubmit() {
		if (!card || processing) return;

		processing = true;
		cardError = '';

		try {
			const result = await card.tokenize();

			if (result.status === 'OK') {
				const paymentResult: SquarePaymentResult = {
					token: result.token,
					last4: result.details?.card?.last4,
					cardBrand: result.details?.card?.brand,
					expirationMonth: result.details?.card?.expMonth?.toString(),
					expirationYear: result.details?.card?.expYear?.toString()
				};

				if (onpayment) onpayment(paymentResult);
			} else {
				const errorMessages = result.errors?.map((e: any) => e.message).join(', ');
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
		<label class="field-label">{label}</label>
	{/if}

	{#if testMode}
		<div class="test-mode-banner">
			<span>⚠️ Square Sandbox Mode</span>
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
		{#if applePayAvailable || googlePayAvailable}
			<div class="digital-wallets">
				{#if applePayAvailable}
					<div bind:this={applePayContainerRef} class="wallet-button"></div>
				{/if}
				{#if googlePayAvailable}
					<div bind:this={googlePayContainerRef} class="wallet-button"></div>
				{/if}
			</div>
			<div class="divider">
				<span>Or pay with card</span>
			</div>
		{/if}

		<div bind:this={cardContainerRef} class="card-container"></div>

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

	.digital-wallets {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.wallet-button {
		min-height: 48px;
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

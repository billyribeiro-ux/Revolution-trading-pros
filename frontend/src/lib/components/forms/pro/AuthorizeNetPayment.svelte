<script lang="ts">
	/**
	 * AuthorizeNetPayment Component (FluentForms 6.1.5 - November 2025)
	 *
	 * Authorize.net payment gateway integration for secure payment processing.
	 * Supports Accept.js for PCI-compliant tokenization.
	 */

	interface Props {
		name: string;
		amount: number;
		currency?: string;
		apiLoginId: string;
		clientKey: string;
		testMode?: boolean;
		label?: string;
		description?: string;
		billingAddress?: BillingAddress;
		disabled?: boolean;
		error?: string;
		onPaymentReady?: (paymentData: AuthorizeNetPaymentData) => void;
		onError?: (error: string) => void;
	}

	interface BillingAddress {
		firstName?: string;
		lastName?: string;
		address?: string;
		city?: string;
		state?: string;
		zip?: string;
		country?: string;
	}

	interface AuthorizeNetPaymentData {
		opaqueData: {
			dataDescriptor: string;
			dataValue: string;
		};
		cardInfo: {
			lastFour: string;
			cardType: string;
			expirationDate: string;
		};
	}

	let {
		name,
		amount,
		currency = 'USD',
		apiLoginId,
		clientKey,
		testMode = false,
		label = 'Payment Details',
		description = '',
		billingAddress = {},
		disabled = false,
		error = '',
		onPaymentReady,
		onError
	}: Props = $props();

	let cardNumber = $state('');
	let expirationMonth = $state('');
	let expirationYear = $state('');
	let cvv = $state('');
	let cardholderName = $state('');
	let isProcessing = $state(false);
	let validationError = $state('');
	let paymentToken = $state<AuthorizeNetPaymentData | null>(null);
	let scriptLoaded = $state(false);

	// Load Accept.js script
	$effect(() => {
		if (typeof window !== 'undefined' && !scriptLoaded) {
			const existingScript = document.querySelector('script[src*="Accept.js"]');
			if (existingScript) {
				scriptLoaded = true;
				return;
			}

			const script = document.createElement('script');
			script.src = testMode
				? 'https://jstest.authorize.net/v1/Accept.js'
				: 'https://js.authorize.net/v1/Accept.js';
			script.async = true;
			script.onload = () => {
				scriptLoaded = true;
			};
			script.onerror = () => {
				validationError = 'Failed to load payment gateway. Please refresh and try again.';
			};
			document.head.appendChild(script);
		}
	});

	function formatCardNumber(value: string): string {
		const cleaned = value.replace(/\D/g, '');
		const groups = cleaned.match(/.{1,4}/g);
		return groups ? groups.join(' ') : cleaned;
	}

	function handleCardNumberInput(e: Event) {
		const target = e.target as HTMLInputElement;
		const cleaned = target.value.replace(/\D/g, '').slice(0, 16);
		cardNumber = formatCardNumber(cleaned);
		validateCard();
	}

	function handleExpirationMonthInput(e: Event) {
		const target = e.target as HTMLInputElement;
		expirationMonth = target.value.replace(/\D/g, '').slice(0, 2);
		validateCard();
	}

	function handleExpirationYearInput(e: Event) {
		const target = e.target as HTMLInputElement;
		expirationYear = target.value.replace(/\D/g, '').slice(0, 4);
		validateCard();
	}

	function handleCvvInput(e: Event) {
		const target = e.target as HTMLInputElement;
		cvv = target.value.replace(/\D/g, '').slice(0, 4);
		validateCard();
	}

	function validateCard(): boolean {
		validationError = '';

		const cardNum = cardNumber.replace(/\s/g, '');
		if (cardNum.length < 13 || cardNum.length > 19) {
			return false;
		}

		// Luhn algorithm validation
		let sum = 0;
		let isEven = false;
		for (let i = cardNum.length - 1; i >= 0; i--) {
			let digit = parseInt(cardNum[i], 10);
			if (isEven) {
				digit *= 2;
				if (digit > 9) {
					digit -= 9;
				}
			}
			sum += digit;
			isEven = !isEven;
		}

		if (sum % 10 !== 0) {
			validationError = 'Invalid card number';
			return false;
		}

		const month = parseInt(expirationMonth, 10);
		if (month < 1 || month > 12) {
			validationError = 'Invalid expiration month';
			return false;
		}

		const year = parseInt(expirationYear, 10);
		const currentYear = new Date().getFullYear();
		const fullYear = year < 100 ? 2000 + year : year;
		if (fullYear < currentYear || fullYear > currentYear + 20) {
			validationError = 'Invalid expiration year';
			return false;
		}

		if (cvv.length < 3) {
			validationError = 'Invalid CVV';
			return false;
		}

		return true;
	}

	function getCardType(number: string): string {
		const cleaned = number.replace(/\s/g, '');
		if (/^4/.test(cleaned)) return 'Visa';
		if (/^5[1-5]/.test(cleaned)) return 'Mastercard';
		if (/^3[47]/.test(cleaned)) return 'American Express';
		if (/^6(?:011|5)/.test(cleaned)) return 'Discover';
		if (/^35(?:2[89]|[3-8])/.test(cleaned)) return 'JCB';
		return 'Unknown';
	}

	async function tokenizeCard() {
		if (!validateCard() || disabled || isProcessing) return;

		isProcessing = true;
		validationError = '';

		try {
			// @ts-expect-error Accept.js is loaded dynamically
			const Accept = window.Accept;
			if (!Accept) {
				throw new Error('Payment gateway not loaded');
			}

			const secureData = {
				authData: {
					apiLoginID: apiLoginId,
					clientKey: clientKey
				},
				cardData: {
					cardNumber: cardNumber.replace(/\s/g, ''),
					month: expirationMonth.padStart(2, '0'),
					year: expirationYear.length === 2 ? '20' + expirationYear : expirationYear,
					cardCode: cvv,
					fullName: cardholderName || undefined
				}
			};

			Accept.dispatchData(secureData, (response: {
				opaqueData?: { dataDescriptor: string; dataValue: string };
				messages: { resultCode: string; message: { code: string; text: string }[] };
			}) => {
				isProcessing = false;

				if (response.messages.resultCode === 'Error') {
					const errorMessage = response.messages.message
						.map((m: { text: string }) => m.text)
						.join(', ');
					validationError = errorMessage;
					if (onError) onError(errorMessage);
					return;
				}

				if (response.opaqueData) {
					const cardNum = cardNumber.replace(/\s/g, '');
					paymentToken = {
						opaqueData: response.opaqueData,
						cardInfo: {
							lastFour: cardNum.slice(-4),
							cardType: getCardType(cardNum),
							expirationDate: `${expirationMonth}/${expirationYear}`
						}
					};

					if (onPaymentReady) {
						onPaymentReady(paymentToken);
					}
				}
			});
		} catch (err) {
			isProcessing = false;
			const errorMessage = err instanceof Error ? err.message : 'Payment processing failed';
			validationError = errorMessage;
			if (onError) onError(errorMessage);
		}
	}

	function formatCurrency(value: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: currency
		}).format(value);
	}

	const cardType = $derived(getCardType(cardNumber));
	const isFormValid = $derived(
		cardNumber.replace(/\s/g, '').length >= 13 &&
		expirationMonth.length === 2 &&
		expirationYear.length >= 2 &&
		cvv.length >= 3
	);
</script>

<div class="authorize-net-payment" class:disabled class:has-error={error || validationError}>
	<div class="payment-header">
		<h3 class="payment-label">{label}</h3>
		{#if description}
			<p class="payment-description">{description}</p>
		{/if}
	</div>

	<div class="payment-amount">
		<span class="amount-label">Amount to Pay:</span>
		<span class="amount-value">{formatCurrency(amount)}</span>
	</div>

	{#if paymentToken}
		<div class="payment-success">
			<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
				<polyline points="22 4 12 14.01 9 11.01"></polyline>
			</svg>
			<div class="success-details">
				<span class="success-title">Payment Method Saved</span>
				<span class="card-details">{paymentToken.cardInfo.cardType} ending in {paymentToken.cardInfo.lastFour}</span>
			</div>
			<button type="button" class="change-card-btn" onclick={() => { paymentToken = null; }}>
				Change
			</button>
		</div>
	{:else}
		<div class="card-form">
			<div class="form-row">
				<label for="{name}_cardholder">Cardholder Name</label>
				<input
					type="text"
					id="{name}_cardholder"
					bind:value={cardholderName}
					placeholder="Name on card"
					autocomplete="cc-name"
					{disabled}
				/>
			</div>

			<div class="form-row">
				<label for="{name}_card_number">Card Number</label>
				<div class="card-input-wrapper">
					<input
						type="text"
						id="{name}_card_number"
						value={cardNumber}
						oninput={handleCardNumberInput}
						placeholder="1234 5678 9012 3456"
						autocomplete="cc-number"
						inputmode="numeric"
						{disabled}
					/>
					<span class="card-type-badge" class:visible={cardType !== 'Unknown'}>
						{cardType}
					</span>
				</div>
			</div>

			<div class="form-row-group">
				<div class="form-row expiry">
					<label for="{name}_exp_month">Expiration</label>
					<div class="expiry-inputs">
						<input
							type="text"
							id="{name}_exp_month"
							value={expirationMonth}
							oninput={handleExpirationMonthInput}
							placeholder="MM"
							autocomplete="cc-exp-month"
							inputmode="numeric"
							maxlength="2"
							{disabled}
						/>
						<span class="expiry-separator">/</span>
						<input
							type="text"
							id="{name}_exp_year"
							value={expirationYear}
							oninput={handleExpirationYearInput}
							placeholder="YYYY"
							autocomplete="cc-exp-year"
							inputmode="numeric"
							maxlength="4"
							{disabled}
						/>
					</div>
				</div>

				<div class="form-row cvv">
					<label for="{name}_cvv">CVV</label>
					<input
						type="text"
						id="{name}_cvv"
						value={cvv}
						oninput={handleCvvInput}
						placeholder="123"
						autocomplete="cc-csc"
						inputmode="numeric"
						maxlength="4"
						{disabled}
					/>
				</div>
			</div>

			<button
				type="button"
				class="tokenize-btn"
				onclick={tokenizeCard}
				disabled={disabled || isProcessing || !isFormValid || !scriptLoaded}
			>
				{#if isProcessing}
					<span class="spinner"></span>
					Processing...
				{:else if !scriptLoaded}
					Loading...
				{:else}
					Verify Card
				{/if}
			</button>
		</div>
	{/if}

	<!-- Hidden inputs for form submission -->
	{#if paymentToken}
		<input type="hidden" name="{name}[data_descriptor]" value={paymentToken.opaqueData.dataDescriptor} />
		<input type="hidden" name="{name}[data_value]" value={paymentToken.opaqueData.dataValue} />
		<input type="hidden" name="{name}[card_last_four]" value={paymentToken.cardInfo.lastFour} />
		<input type="hidden" name="{name}[card_type]" value={paymentToken.cardInfo.cardType} />
	{/if}

	{#if validationError || error}
		<div class="error-message">
			<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<circle cx="12" cy="12" r="10"></circle>
				<line x1="12" y1="8" x2="12" y2="12"></line>
				<line x1="12" y1="16" x2="12.01" y2="16"></line>
			</svg>
			{validationError || error}
		</div>
	{/if}

	<div class="security-notice">
		<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
			<path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
		</svg>
		<span>Your payment information is encrypted and secure</span>
	</div>
</div>

<style>
	.authorize-net-payment {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		padding: 1.5rem;
		background-color: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
	}

	.payment-header {
		margin-bottom: 0.5rem;
	}

	.payment-label {
		font-size: 1.125rem;
		font-weight: 600;
		color: #111827;
		margin: 0;
	}

	.payment-description {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0.25rem 0 0;
	}

	.payment-amount {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		background-color: #eff6ff;
		border: 1px solid #bfdbfe;
		border-radius: 0.5rem;
	}

	.amount-label {
		font-size: 0.875rem;
		color: #1e40af;
	}

	.amount-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: #1e40af;
	}

	.payment-success {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background-color: #f0fdf4;
		border: 1px solid #86efac;
		border-radius: 0.5rem;
		color: #166534;
	}

	.success-details {
		display: flex;
		flex-direction: column;
		flex: 1;
	}

	.success-title {
		font-weight: 600;
		font-size: 0.9375rem;
	}

	.card-details {
		font-size: 0.8125rem;
		color: #15803d;
	}

	.change-card-btn {
		padding: 0.375rem 0.75rem;
		background: none;
		border: 1px solid #86efac;
		border-radius: 0.375rem;
		color: #166534;
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.change-card-btn:hover {
		background-color: #dcfce7;
	}

	.card-form {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.form-row {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.form-row label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #374151;
	}

	.form-row input {
		padding: 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		font-size: 1rem;
		color: #111827;
		background-color: white;
		transition: all 0.15s;
	}

	.form-row input:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.form-row input::placeholder {
		color: #9ca3af;
	}

	.card-input-wrapper {
		position: relative;
	}

	.card-input-wrapper input {
		width: 100%;
		padding-right: 5rem;
	}

	.card-type-badge {
		position: absolute;
		right: 0.75rem;
		top: 50%;
		transform: translateY(-50%);
		font-size: 0.75rem;
		font-weight: 600;
		color: #6b7280;
		background-color: #f3f4f6;
		padding: 0.25rem 0.5rem;
		border-radius: 0.25rem;
		opacity: 0;
		transition: opacity 0.15s;
	}

	.card-type-badge.visible {
		opacity: 1;
	}

	.form-row-group {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.expiry-inputs {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.expiry-inputs input {
		flex: 1;
		text-align: center;
	}

	.expiry-separator {
		color: #9ca3af;
		font-size: 1.25rem;
	}

	.tokenize-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.875rem 1.5rem;
		background-color: #3b82f6;
		color: white;
		border: none;
		border-radius: 0.5rem;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		margin-top: 0.5rem;
	}

	.tokenize-btn:hover:not(:disabled) {
		background-color: #2563eb;
	}

	.tokenize-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.spinner {
		width: 18px;
		height: 18px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.error-message {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background-color: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 0.5rem;
		color: #dc2626;
		font-size: 0.875rem;
	}

	.security-notice {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.5rem;
		color: #6b7280;
		font-size: 0.75rem;
	}

	/* Disabled State */
	.disabled {
		opacity: 0.6;
		pointer-events: none;
	}

	/* Error State */
	.has-error .form-row input {
		border-color: #fca5a5;
	}

	/* Responsive */
	@media (max-width: 480px) {
		.authorize-net-payment {
			padding: 1rem;
		}

		.form-row-group {
			grid-template-columns: 1fr;
		}
	}
</style>

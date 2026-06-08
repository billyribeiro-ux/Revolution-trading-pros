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

	import type { Attachment } from 'svelte/attachments';
	import Icon from '$lib/components/Icon.svelte';

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
	let buttonContainerRef: HTMLDivElement | undefined;

	const buttonContainerAttachment: Attachment<HTMLDivElement> = (element) => {
		buttonContainerRef = element;
		void loadPayPal();

		return () => {
			if (buttonContainerRef === element) {
				buttonContainerRef = undefined;
			}
		};
	};

	async function loadPayPal() {
		if (buttonsRendered) return;

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

			if (buttonContainerRef && window.paypal) {
				renderButtons();
			}
		} catch (_err) {
			loading = false;
			paypalError = 'Failed to load PayPal';
			if (onerror) onerror('Failed to initialize PayPal');
		}
	}

	function renderButtons() {
		const container = buttonContainerRef;
		if (buttonsRendered || !container || !window.paypal?.Buttons) return;

		window.paypal
			.Buttons({
				style: {
					layout: 'vertical',
					color: 'gold',
					shape: 'rect',
					label: 'paypal',
					height: 45
				},
				createOrder: (_data, actions) => {
					return actions.order.create({
						intent: 'CAPTURE',
						purchase_units: [
							{
								description: description,
								amount: {
									currency_code: currency,
									value: amount.toFixed(2)
								}
							}
						]
					});
				},
				onApprove: async (data, actions) => {
					try {
						const order = await actions.order?.capture();
						if (!order) throw new Error('Order capture returned no result');

						const result: PayPalPaymentResult = {
							orderId: data.orderID ?? '',
							payerId: data.payerID ?? '',
							status: order.status ?? '',
							captureId: order.purchase_units?.[0]?.payments?.captures?.[0]?.id,
							payerEmail: order.payer?.email_address,
							payerName: order.payer?.name?.given_name
						};

						if (onpayment) onpayment(result);
					} catch (_err) {
						paypalError = 'Payment failed to complete';
						if (onerror) onerror('Payment capture failed');
					}
				},
				onError: (err) => {
					paypalError = 'Payment was cancelled or failed';
					if (onerror) onerror(typeof err.message === 'string' ? err.message : 'PayPal error');
				},
				onCancel: () => {
					paypalError = 'Payment was cancelled';
				}
			})
			.render(container);

		buttonsRendered = true;
	}

	function formatAmount(amount: number, currency: string): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: currency
		}).format(amount);
	}
</script>

<div class={['paypal-payment', { disabled, 'has-error': error || paypalError }]}>
	{#if label}
		<label class="field-label" for="paypal-buttons-container">{label}</label>
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
			<Icon name="IconLoader" size={20} class="spinner" />
			<span>Loading PayPal...</span>
		</div>
	{/if}

	<div
		id="paypal-buttons-container"
		class={['paypal-buttons', { hidden: loading }]}
		{@attach buttonContainerAttachment}
	></div>

	{#if paypalError}
		<p class="paypal-error">{paypalError}</p>
	{/if}

	<div class="secure-badge">
		<Icon name="IconLock" size={14} />
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

	:global(.spinner) {
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

	.secure-badge :global(svg) {
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

<script lang="ts">
	import { enhance } from '$app/forms';
	import { onMount } from 'svelte';

	interface PageData {
		user: {
			email: string;
		};
	}

	interface ActionData {
		error?: string;
	}

	let { data, form }: { data: PageData; form?: ActionData } = $props();

	let stripe: any = $state(null);
	let elements: any = $state(null);
	let cardElement: any = $state(null);
	let isLoading = $state(false);
	let errorMessage = $state('');
	let setAsDefault = $state(true);
	let stripeLoaded = $state(false);

	onMount(async () => {
		// Load Stripe.js
		const script = document.createElement('script');
		script.src = 'https://js.stripe.com/v3/';
		script.async = true;
		script.onload = initializeStripe;
		document.head.appendChild(script);
	});

	function initializeStripe() {
		// Get Stripe publishable key from environment or config
		// For now, we'll need to fetch it from the backend
		fetch('/api/stripe/config')
			.then(res => res.json())
			.then(config => {
				stripe = (window as any).Stripe(config.publishableKey);
				elements = stripe.elements();
				
				// Create card element
				cardElement = elements.create('card', {
					style: {
						base: {
							fontSize: '15px',
							color: '#32325d',
							fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
							'::placeholder': {
								color: '#aab7c4'
							}
						},
						invalid: {
							color: '#fa755a',
							iconColor: '#fa755a'
						}
					}
				});
				
				cardElement.mount('#card-element');
				
				cardElement.on('change', (event: any) => {
					if (event.error) {
						errorMessage = event.error.message;
					} else {
						errorMessage = '';
					}
				});

				stripeLoaded = true;
			})
			.catch(err => {
				console.error('Error loading Stripe:', err);
				errorMessage = 'Failed to load payment form. Please refresh the page.';
			});
	}

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		
		if (!stripe || !cardElement) {
			errorMessage = 'Payment form not loaded. Please refresh the page.';
			return;
		}

		isLoading = true;
		errorMessage = '';

		try {
			// Create payment method
			const { error, paymentMethod } = await stripe.createPaymentMethod({
				type: 'card',
				card: cardElement,
				billing_details: {
					email: data.user.email
				}
			});

			if (error) {
				errorMessage = error.message;
				isLoading = false;
				return;
			}

			// Submit the form with the payment method ID
			const form = event.target as HTMLFormElement;
			const paymentMethodInput = form.querySelector('#payment_method_id') as HTMLInputElement;
			if (paymentMethodInput) {
				paymentMethodInput.value = paymentMethod.id;
			}

			// Now submit the form
			form.submit();
		} catch (err) {
			console.error('Error creating payment method:', err);
			errorMessage = 'An error occurred. Please try again.';
			isLoading = false;
		}
	}
</script>

<svelte:head>
	<title>Add Payment Method - Revolution Trading Pros</title>
</svelte:head>

<!-- Dashboard Header -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">My Account</h1>
	</div>
</header>

<!-- Dashboard Content -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<div class="add-payment-card">
		<div class="woocommerce-notices-wrapper">
			{#if form?.error}
				<div class="woocommerce-error" role="alert">
					{form.error}
				</div>
			{/if}

			{#if errorMessage}
				<div class="woocommerce-error" role="alert">
					{errorMessage}
				</div>
			{/if}
		</div>

		<h2 class="section-title">Add Payment Method</h2>

		<form 
			method="post" 
			id="add_payment_method"
			onsubmit={handleSubmit}
		>
			<input type="hidden" id="payment_method_id" name="payment_method_id" value="" />
			<input type="hidden" name="set_as_default" value={setAsDefault.toString()} />

			<div class="woocommerce-PaymentMethods payment_methods methods">
				<div class="woocommerce-PaymentMethod woocommerce-PaymentMethod--stripe payment_method_stripe">
					<label for="payment_method_stripe">
						<input 
							id="payment_method_stripe" 
							type="radio" 
							class="input-radio" 
							name="payment_method" 
							value="stripe" 
							checked 
						/>
						Credit Card (Stripe)
					</label>

					<div class="woocommerce-PaymentBox woocommerce-PaymentBox--stripe payment_box payment_method_stripe">
						<fieldset id="wc-stripe-cc-form" class="wc-credit-card-form wc-payment-form">
							<div class="form-row form-row-wide">
								<label for="card-element">
									Card Number <span class="required">*</span>
								</label>
								<div id="card-element" class="wc-stripe-elements-field"></div>
							</div>

							<div class="form-row form-row-wide">
								<label for="set_as_default_checkbox">
									<input 
										id="set_as_default_checkbox"
										type="checkbox" 
										bind:checked={setAsDefault}
									/>
									Set as default payment method
								</label>
							</div>
						</fieldset>
					</div>
				</div>
			</div>

			<div class="form-row">
				<button 
					type="submit" 
					class="button alt" 
					disabled={isLoading || !stripeLoaded}
				>
					{#if isLoading}
						Processing...
					{:else}
						Add Payment Method
					{/if}
				</button>

				<a href="/dashboard/account/payment-methods" class="button">Cancel</a>
			</div>
		</form>
		</div>
	</div>
</div>

<style>
	/* Dashboard Header */
	.dashboard__header {
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
		border-right: 1px solid #dbdbdb;
		padding: 20px;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	@media (min-width: 1024px) {
		.dashboard__header {
			padding: 30px;
		}
	}

	@media (min-width: 1440px) {
		.dashboard__header {
			padding: 30px 40px;
		}
	}

	.dashboard__header-left {
		flex: 1;
	}

	.dashboard__page-title {
		font-family: 'Open Sans', sans-serif;
		font-size: 28px;
		font-weight: 400;
		font-style: italic;
		color: #333333;
		margin: 0;
		line-height: 1.2;
	}

	/* Dashboard Content */
	.dashboard__content {
		background: #f5f5f5;
		min-height: calc(100vh - 60px);
		padding: 40px 30px;
	}

	.dashboard__content-main {
		max-width: 900px;
		margin: 0 auto;
	}

	/* Professional Card Container */
	.add-payment-card {
		background: #ffffff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		padding: 40px;
		margin-bottom: 30px;
	}

	.section-title {
		font-size: 24px;
		font-weight: 600;
		margin-bottom: 20px;
		color: #333;
	}

	.woocommerce-notices-wrapper {
		margin-bottom: 20px;
	}

	.woocommerce-error {
		background-color: #e2401c;
		color: #fff;
		padding: 15px;
		margin-bottom: 15px;
		border-radius: 4px;
	}

	.woocommerce-PaymentMethods {
		list-style: none;
		padding: 0;
		margin: 0 0 20px;
	}

	.woocommerce-PaymentMethod {
		margin-bottom: 15px;
	}

	.woocommerce-PaymentMethod label {
		display: block;
		font-weight: 600;
		margin-bottom: 10px;
		cursor: pointer;
	}

	.input-radio {
		margin-right: 8px;
	}

	.woocommerce-PaymentBox {
		padding: 20px;
		background: #f8f9fa;
		border: 1px solid #e9ecef;
		border-radius: 4px;
		margin-top: 10px;
	}

	.wc-credit-card-form {
		border: none;
		padding: 0;
		margin: 0;
	}

	.form-row {
		margin-bottom: 15px;
	}

	.form-row label {
		display: block;
		font-weight: 600;
		margin-bottom: 8px;
		color: #333;
	}

	.required {
		color: #e2401c;
	}

	#card-element {
		padding: 12px;
		border: 1px solid #ddd;
		border-radius: 4px;
		background: #fff;
	}

	.button {
		display: inline-block;
		padding: 12px 24px;
		background: #f8f9fa;
		color: #333;
		text-decoration: none;
		border: 1px solid #ddd;
		border-radius: 4px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		margin-right: 10px;
		transition: all 0.2s ease;
	}

	.button:hover {
		background: #e9ecef;
	}

	.button.alt {
		background: #143E59;
		color: #fff;
		border-color: #143E59;
	}

	.button.alt:hover {
		background: #0f2f43;
		border-color: #0f2f43;
	}

	.button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.button:disabled:hover {
		background: #143E59;
		border-color: #143E59;
	}
</style>

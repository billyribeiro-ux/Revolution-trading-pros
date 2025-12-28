<script lang="ts">
	/**
	 * Edit Billing Address - Account Section
	 * Reference: frontend/Do's/Billing-Address-Edit
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';

	// Form state - pre-populated with mock data
	let firstName = $state('John');
	let lastName = $state('Doe');
	let company = $state('');
	let country = $state('US');
	let address1 = $state('123 Main Street');
	let address2 = $state('');
	let city = $state('Austin');
	let billingState = $state('TX');
	let postcode = $state('78701');
	let phone = $state('(555) 123-4567');
	let email = $state('john.doe@example.com');
	let isSaving = $state(false);
	let message = $state<{ type: 'success' | 'error'; text: string } | null>(null);

	const countries = [
		{ code: 'US', name: 'United States (US)' },
		{ code: 'CA', name: 'Canada' },
		{ code: 'GB', name: 'United Kingdom (UK)' },
		{ code: 'AU', name: 'Australia' },
		{ code: 'DE', name: 'Germany' },
		{ code: 'FR', name: 'France' }
	];

	const usStates = [
		{ code: 'AL', name: 'Alabama' },
		{ code: 'AK', name: 'Alaska' },
		{ code: 'AZ', name: 'Arizona' },
		{ code: 'AR', name: 'Arkansas' },
		{ code: 'CA', name: 'California' },
		{ code: 'CO', name: 'Colorado' },
		{ code: 'CT', name: 'Connecticut' },
		{ code: 'DE', name: 'Delaware' },
		{ code: 'FL', name: 'Florida' },
		{ code: 'GA', name: 'Georgia' },
		{ code: 'HI', name: 'Hawaii' },
		{ code: 'ID', name: 'Idaho' },
		{ code: 'IL', name: 'Illinois' },
		{ code: 'IN', name: 'Indiana' },
		{ code: 'IA', name: 'Iowa' },
		{ code: 'KS', name: 'Kansas' },
		{ code: 'KY', name: 'Kentucky' },
		{ code: 'LA', name: 'Louisiana' },
		{ code: 'ME', name: 'Maine' },
		{ code: 'MD', name: 'Maryland' },
		{ code: 'MA', name: 'Massachusetts' },
		{ code: 'MI', name: 'Michigan' },
		{ code: 'MN', name: 'Minnesota' },
		{ code: 'MS', name: 'Mississippi' },
		{ code: 'MO', name: 'Missouri' },
		{ code: 'MT', name: 'Montana' },
		{ code: 'NE', name: 'Nebraska' },
		{ code: 'NV', name: 'Nevada' },
		{ code: 'NH', name: 'New Hampshire' },
		{ code: 'NJ', name: 'New Jersey' },
		{ code: 'NM', name: 'New Mexico' },
		{ code: 'NY', name: 'New York' },
		{ code: 'NC', name: 'North Carolina' },
		{ code: 'ND', name: 'North Dakota' },
		{ code: 'OH', name: 'Ohio' },
		{ code: 'OK', name: 'Oklahoma' },
		{ code: 'OR', name: 'Oregon' },
		{ code: 'PA', name: 'Pennsylvania' },
		{ code: 'RI', name: 'Rhode Island' },
		{ code: 'SC', name: 'South Carolina' },
		{ code: 'SD', name: 'South Dakota' },
		{ code: 'TN', name: 'Tennessee' },
		{ code: 'TX', name: 'Texas' },
		{ code: 'UT', name: 'Utah' },
		{ code: 'VT', name: 'Vermont' },
		{ code: 'VA', name: 'Virginia' },
		{ code: 'WA', name: 'Washington' },
		{ code: 'WV', name: 'West Virginia' },
		{ code: 'WI', name: 'Wisconsin' },
		{ code: 'WY', name: 'Wyoming' }
	];

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		isSaving = true;
		message = null;

		// Simulate API call
		await new Promise((resolve) => setTimeout(resolve, 1000));

		message = { type: 'success', text: 'Billing address updated successfully.' };
		isSaving = false;
	}
</script>

<svelte:head>
	<title>Edit Billing Address | Revolution Trading Pros</title>
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<nav class="breadcrumb">
			<a href="/dashboard">Dashboard</a>
			<span>/</span>
			<a href="/dashboard/account">My Account</a>
			<span>/</span>
			<a href="/dashboard/account/billing-address">Billing Address</a>
			<span>/</span>
			<span class="current">Edit</span>
		</nav>
		<h1 class="dashboard__page-title">Edit Billing Address</h1>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			{#if message}
				<div class="woocommerce-message woocommerce-message--{message.type}">
					{message.text}
				</div>
			{/if}

			<div class="content-box">
				<form
					class="woocommerce-address-fields"
					onsubmit={handleSubmit}
				>
					<div class="woocommerce-address-fields__field-wrapper">
						<div class="form-row form-row-first">
							<label for="billing_first_name"
								>First name <span class="required">*</span></label
							>
							<input
								type="text"
								class="input-text"
								name="billing_first_name"
								id="billing_first_name"
								bind:value={firstName}
								required
							/>
						</div>
						<div class="form-row form-row-last">
							<label for="billing_last_name"
								>Last name <span class="required">*</span></label
							>
							<input
								type="text"
								class="input-text"
								name="billing_last_name"
								id="billing_last_name"
								bind:value={lastName}
								required
							/>
						</div>
						<div class="clear"></div>

						<div class="form-row form-row-wide">
							<label for="billing_company">Company name (optional)</label>
							<input
								type="text"
								class="input-text"
								name="billing_company"
								id="billing_company"
								bind:value={company}
							/>
						</div>

						<div class="form-row form-row-wide address-field">
							<label for="billing_country"
								>Country / Region <span class="required">*</span></label
							>
							<select
								name="billing_country"
								id="billing_country"
								class="country_select"
								bind:value={country}
							>
								{#each countries as c (c.code)}
									<option value={c.code}>{c.name}</option>
								{/each}
							</select>
						</div>

						<div class="form-row form-row-wide address-field">
							<label for="billing_address_1"
								>Street address <span class="required">*</span></label
							>
							<input
								type="text"
								class="input-text"
								name="billing_address_1"
								id="billing_address_1"
								placeholder="House number and street name"
								bind:value={address1}
								required
							/>
						</div>

						<div class="form-row form-row-wide address-field">
							<input
								type="text"
								class="input-text"
								name="billing_address_2"
								id="billing_address_2"
								placeholder="Apartment, suite, unit, etc. (optional)"
								bind:value={address2}
							/>
						</div>

						<div class="form-row form-row-wide address-field">
							<label for="billing_city">Town / City <span class="required">*</span></label
							>
							<input
								type="text"
								class="input-text"
								name="billing_city"
								id="billing_city"
								bind:value={city}
								required
							/>
						</div>

						<div class="form-row form-row-first address-field">
							<label for="billing_state">State <span class="required">*</span></label>
							{#if country === 'US'}
								<select
									name="billing_state"
									id="billing_state"
									class="state_select"
									bind:value={billingState}
								>
									{#each usStates as s (s.code)}
										<option value={s.code}>{s.name}</option>
									{/each}
								</select>
							{:else}
								<input
									type="text"
									class="input-text"
									name="billing_state"
									id="billing_state"
									bind:value={billingState}
									required
								/>
							{/if}
						</div>

						<div class="form-row form-row-last address-field">
							<label for="billing_postcode"
								>ZIP Code <span class="required">*</span></label
							>
							<input
								type="text"
								class="input-text"
								name="billing_postcode"
								id="billing_postcode"
								bind:value={postcode}
								required
							/>
						</div>
						<div class="clear"></div>

						<div class="form-row form-row-wide">
							<label for="billing_phone">Phone <span class="required">*</span></label>
							<input
								type="tel"
								class="input-text"
								name="billing_phone"
								id="billing_phone"
								bind:value={phone}
								required
							/>
						</div>

						<div class="form-row form-row-wide">
							<label for="billing_email"
								>Email address <span class="required">*</span></label
							>
							<input
								type="email"
								class="input-text"
								name="billing_email"
								id="billing_email"
								bind:value={email}
								required
							/>
						</div>
					</div>

					<div class="form-actions">
						<button type="submit" class="btn btn-primary" disabled={isSaving}>
							{isSaving ? 'Saving...' : 'Save address'}
						</button>
						<a href="/dashboard/account/billing-address" class="btn btn-secondary">Cancel</a>
					</div>
				</form>
			</div>
		</section>
	</div>
</div>

<style>
	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		padding: 20px 40px;
	}

	.breadcrumb {
		font-size: 14px;
		color: #666;
		margin-bottom: 10px;
	}

	.breadcrumb a {
		color: #0984ae;
		text-decoration: none;
	}

	.breadcrumb a:hover {
		text-decoration: underline;
	}

	.breadcrumb span {
		margin: 0 8px;
		color: #999;
	}

	.breadcrumb .current {
		color: #333;
	}

	h1.dashboard__page-title {
		margin: 0;
		color: #333;
		font-size: 32px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
	}

	.dashboard__content {
		display: flex;
	}

	.dashboard__content-main {
		flex: 1;
		background-color: #efefef;
	}

	.dashboard__content-section {
		padding: 30px 40px;
		background-color: #fff;
		margin-bottom: 20px;
	}

	.content-box {
		background: #fff;
		border-radius: 8px;
		padding: 30px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		max-width: 600px;
	}

	.woocommerce-address-fields__field-wrapper {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -10px;
	}

	.form-row {
		padding: 0 10px;
		margin-bottom: 20px;
	}

	.form-row-wide {
		width: 100%;
	}

	.form-row-first,
	.form-row-last {
		width: 50%;
	}

	.clear {
		clear: both;
	}

	label {
		display: block;
		margin-bottom: 8px;
		font-size: 14px;
		font-weight: 600;
		color: #333;
	}

	.required {
		color: #dc3545;
	}

	.input-text,
	.country_select,
	.state_select {
		width: 100%;
		padding: 12px 14px;
		font-size: 14px;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		transition: border-color 0.15s ease;
		font-family: 'Open Sans', sans-serif;
	}

	.input-text:focus,
	.country_select:focus,
	.state_select:focus {
		outline: none;
		border-color: #0984ae;
	}

	.woocommerce-message {
		padding: 15px 20px;
		border-radius: 4px;
		margin-bottom: 20px;
		font-size: 14px;
	}

	.woocommerce-message--success {
		background: #d4edda;
		color: #155724;
		border: 1px solid #c3e6cb;
	}

	.woocommerce-message--error {
		background: #f8d7da;
		color: #721c24;
		border: 1px solid #f5c6cb;
	}

	.form-actions {
		display: flex;
		gap: 15px;
		margin-top: 10px;
	}

	.btn {
		display: inline-block;
		padding: 12px 24px;
		font-size: 14px;
		font-weight: 600;
		border-radius: 4px;
		border: none;
		cursor: pointer;
		transition: all 0.15s ease;
		text-decoration: none;
	}

	.btn-primary {
		background: #0984ae;
		color: #fff;
	}

	.btn-primary:hover:not(:disabled) {
		background: #076787;
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: #f5f5f5;
		color: #333;
		border: 1px solid #dbdbdb;
	}

	.btn-secondary:hover {
		background: #e9e9e9;
	}

	@media (max-width: 600px) {
		.form-row-first,
		.form-row-last {
			width: 100%;
		}

		.content-box {
			padding: 20px;
		}

		.form-actions {
			flex-direction: column;
		}
	}
</style>

<script lang="ts">
	import { enhance } from '$app/forms';

	interface Address {
		firstName?: string;
		lastName?: string;
		company?: string;
		address1?: string;
		address2?: string;
		city?: string;
		state?: string;
		postcode?: string;
		country?: string;
		email?: string;
		phone?: string;
	}

	interface PageData {
		address: Address;
	}

	interface ActionData {
		success?: boolean;
		message?: string;
	}

	let props: { data: PageData; form?: ActionData } = $props();
	let data = $derived(props.data);
	let form = $derived(props.form);

	const address = $derived(data.address || {});
</script>

<svelte:head>
	<title>Billing Address - Revolution Trading Pros</title>
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
		<div class="billing-address-card">
			<div class="woocommerce-notices-wrapper">
				{#if form?.success}
					<div class="woocommerce-message woocommerce-message--success">
						<p>{form.message}</p>
					</div>
				{:else if form?.message}
					<div class="woocommerce-error">
						<p>{form.message}</p>
					</div>
				{/if}
			</div>

			<h2 class="section-title">Billing Address</h2>

			<form method="POST" use:enhance>
				<div class="form-row">
					<div class="form-group col-md-6">
						<label for="firstName">First Name <span class="required">*</span></label>
						<input
							type="text"
							id="firstName"
							name="firstName"
							class="form-control"
							value={address.firstName || ''}
							required
						/>
					</div>
					<div class="form-group col-md-6">
						<label for="lastName">Last Name <span class="required">*</span></label>
						<input
							type="text"
							id="lastName"
							name="lastName"
							class="form-control"
							value={address.lastName || ''}
							required
						/>
					</div>
				</div>

				<div class="form-group">
					<label for="company">Company Name (optional)</label>
					<input
						type="text"
						id="company"
						name="company"
						class="form-control"
						value={address.company || ''}
					/>
				</div>

				<div class="form-group">
					<label for="country">Country / Region <span class="required">*</span></label>
					<select id="country" name="country" class="form-control" required>
						<option value="US" selected={address.country === 'US'}>United States</option>
						<option value="CA" selected={address.country === 'CA'}>Canada</option>
						<option value="GB" selected={address.country === 'GB'}>United Kingdom</option>
					</select>
				</div>

				<div class="form-group">
					<label for="address1">Street Address <span class="required">*</span></label>
					<input
						type="text"
						id="address1"
						name="address1"
						class="form-control"
						placeholder="House number and street name"
						value={address.address1 || ''}
						required
					/>
				</div>

				<div class="form-group">
					<input
						type="text"
						id="address2"
						name="address2"
						class="form-control"
						placeholder="Apartment, suite, unit, etc. (optional)"
						value={address.address2 || ''}
					/>
				</div>

				<div class="form-group">
					<label for="city">Town / City <span class="required">*</span></label>
					<input
						type="text"
						id="city"
						name="city"
						class="form-control"
						value={address.city || ''}
						required
					/>
				</div>

				<div class="form-row">
					<div class="form-group col-md-6">
						<label for="state">State <span class="required">*</span></label>
						<input
							type="text"
							id="state"
							name="state"
							class="form-control"
							value={address.state || ''}
							required
						/>
					</div>
					<div class="form-group col-md-6">
						<label for="postcode">ZIP Code <span class="required">*</span></label>
						<input
							type="text"
							id="postcode"
							name="postcode"
							class="form-control"
							value={address.postcode || ''}
							required
						/>
					</div>
				</div>

				<div class="form-group">
					<label for="phone">Phone <span class="required">*</span></label>
					<input
						type="tel"
						id="phone"
						autocomplete="tel"
						name="phone"
						class="form-control"
						value={address.phone || ''}
						required
					/>
				</div>

				<div class="form-group">
					<label for="email">Email Address <span class="required">*</span></label>
					<input
						type="email"
						id="email"
						autocomplete="email"
						name="email"
						class="form-control"
						value={address.email || ''}
						required
					/>
				</div>

				<button type="submit" class="btn btn-primary">Save Address</button>
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
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 28px;
		font-weight: 700;
		font-style: italic;
		color: #333333;
		margin: 0;
		line-height: 1.2;
	}

	/* Dashboard Content */
	/* ICT11+ Fix: Removed min-height: calc(100vh - 60px) - let parent flex container handle height */
	.dashboard__content {
		background: #f5f5f5;
		padding: 40px 30px;
	}

	.dashboard__content-main {
		max-width: 900px;
		margin: 0 auto;
	}

	/* Professional Card Container */
	.billing-address-card {
		background: #ffffff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		padding: 40px;
		margin-bottom: 30px;
	}

	.section-title {
		font-family: 'Open Sans', sans-serif;
		font-size: 22px;
		font-weight: 600;
		margin-bottom: 32px;
		color: #333;
		padding-bottom: 16px;
		border-bottom: 2px solid #e9ecef;
	}

	.form-row {
		display: flex;
		gap: 24px;
		margin-bottom: 0;
	}

	.form-group {
		margin-bottom: 24px;
	}

	.form-group.col-md-6 {
		flex: 1;
		margin-bottom: 24px;
	}

	.form-group label {
		display: block;
		font-size: 14px;
		font-weight: 600;
		color: #495057;
		margin-bottom: 10px;
		font-family: 'Open Sans', sans-serif;
	}

	.form-group label .required {
		color: #dc3545;
	}

	.form-control {
		width: 100%;
		padding: 13px 16px;
		font-size: 14px;
		line-height: 1.5;
		color: #495057;
		background-color: #fff;
		border: 1px solid #d4d4d4;
		border-radius: 5px;
		transition:
			border-color 0.15s ease-in-out,
			box-shadow 0.15s ease-in-out;
		font-family: 'Open Sans', sans-serif;
	}

	.form-control::placeholder {
		color: #999;
	}

	.form-control:focus {
		outline: none;
		border-color: #0984ae;
		box-shadow: 0 0 0 3px rgba(9, 132, 174, 0.1);
	}

	.btn {
		display: inline-block;
		padding: 14px 32px;
		font-size: 15px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border: none;
		border-radius: 5px;
		cursor: pointer;
		transition: all 0.2s ease;
		font-family: 'Open Sans', sans-serif;
		margin-top: 8px;
	}

	.btn-primary {
		background: #0984ae;
		color: #fff;
		box-shadow: 0 2px 4px rgba(9, 132, 174, 0.2);
	}

	.btn-primary:hover {
		background: #076a8a;
		box-shadow: 0 4px 8px rgba(9, 132, 174, 0.3);
		transform: translateY(-1px);
	}

	.woocommerce-notices-wrapper {
		margin-bottom: 24px;
	}

	.woocommerce-message {
		padding: 16px 20px;
		border-radius: 6px;
		margin-bottom: 24px;
	}

	.woocommerce-message--success {
		background: #d4edda;
		border-left: 4px solid #28a745;
		color: #155724;
	}

	.woocommerce-error {
		padding: 16px 20px;
		background: #f8d7da;
		border-left: 4px solid #dc3545;
		border-radius: 6px;
		margin-bottom: 24px;
		color: #721c24;
	}

	.woocommerce-message p,
	.woocommerce-error p {
		margin: 0;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.dashboard__content {
			padding: 20px 15px;
		}

		.billing-address-card {
			padding: 24px 20px;
			border-radius: 6px;
		}

		.section-title {
			font-size: 20px;
			margin-bottom: 24px;
		}

		.form-row {
			flex-direction: column;
			gap: 0;
		}

		.form-group.col-md-6 {
			flex: none;
		}

		.btn {
			width: 100%;
			padding: 12px 24px;
		}
	}
</style>

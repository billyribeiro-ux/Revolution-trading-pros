<script lang="ts">
	import { enhance } from '$app/forms';
	import type { PageData, ActionData } from './$types';

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

	let { data, form }: { data: PageData; form: ActionData } = $props();

	const address = $derived(data.address || {});
</script>

<svelte:head>
	<title>Billing Address - Revolution Trading Pros</title>
</svelte:head>

<div class="woocommerce">
	<div class="woocommerce-MyAccount-content">
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

<style>
	.section-title {
		font-size: 24px;
		font-weight: 700;
		margin-bottom: 30px;
		color: #333;
	}

	.form-row {
		display: flex;
		gap: 20px;
		margin-bottom: 20px;
	}

	.form-group {
		margin-bottom: 20px;
	}

	.form-group.col-md-6 {
		flex: 1;
	}

	.form-group label {
		display: block;
		font-size: 14px;
		font-weight: 600;
		color: #495057;
		margin-bottom: 8px;
	}

	.form-group label .required {
		color: #dc3545;
	}

	.form-control {
		width: 100%;
		padding: 12px 16px;
		font-size: 14px;
		line-height: 1.5;
		color: #495057;
		background-color: #fff;
		border: 1px solid #ced4da;
		border-radius: 4px;
		transition: border-color 0.15s ease-in-out, box-shadow 0.15s ease-in-out;
	}

	.form-control:focus {
		outline: none;
		border-color: #143E59;
		box-shadow: 0 0 0 3px rgba(20, 62, 89, 0.1);
	}

	.btn {
		display: inline-block;
		padding: 12px 24px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.btn-primary {
		background: #143E59;
		color: #fff;
	}

	.btn-primary:hover {
		background: #0f2f43;
	}

	.woocommerce-message {
		padding: 16px 20px;
		border-radius: 4px;
		margin-bottom: 20px;
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
		border-radius: 4px;
		margin-bottom: 20px;
		color: #721c24;
	}

	.woocommerce-message p,
	.woocommerce-error p {
		margin: 0;
		font-size: 14px;
	}

	@media (max-width: 768px) {
		.form-row {
			flex-direction: column;
			gap: 0;
		}

		.form-group.col-md-6 {
			flex: none;
		}
	}
</style>

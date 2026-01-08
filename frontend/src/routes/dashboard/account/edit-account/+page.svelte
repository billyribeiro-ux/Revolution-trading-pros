<script lang="ts">
	import { enhance } from '$app/forms';

	interface Profile {
		firstName: string;
		lastName: string;
		displayName: string;
		email: string;
	}

	interface PageData {
		profile: Profile;
	}

	interface ActionData {
		success?: boolean;
		message?: string;
		error?: string;
	}

	let { data, form }: { data: PageData; form?: ActionData } = $props();

	let isSubmitting = $state(false);
	let profile = $derived(data.profile);
</script>

<svelte:head>
	<title>Account Details - Revolution Trading Pros</title>
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
		<div class="edit-account-card">
			<div class="woocommerce-notices-wrapper">
			{#if form?.success}
				<div class="woocommerce-message" role="alert">
					{form.message}
				</div>
			{/if}

			{#if form?.error}
				<div class="woocommerce-error" role="alert">
					{form.error}
				</div>
			{/if}
		</div>

		<form 
			class="woocommerce-EditAccountForm edit-account" 
			method="post"
			use:enhance={() => {
				isSubmitting = true;
				return async ({ update }) => {
					await update();
					isSubmitting = false;
				};
			}}
		>
			<p class="woocommerce-form-row woocommerce-form-row--first form-row form-row-first">
				<label for="account_first_name">
					First name&nbsp;<span class="required">*</span>
				</label>
				<input 
					type="text" 
					class="woocommerce-Input woocommerce-Input--text input-text" 
					name="account_first_name" 
					id="account_first_name" 
					autocomplete="given-name" 
					value={profile.firstName}
					required
				/>
			</p>

			<p class="woocommerce-form-row woocommerce-form-row--last form-row form-row-last">
				<label for="account_last_name">
					Last name&nbsp;<span class="required">*</span>
				</label>
				<input 
					type="text" 
					class="woocommerce-Input woocommerce-Input--text input-text" 
					name="account_last_name" 
					id="account_last_name" 
					autocomplete="family-name" 
					value={profile.lastName}
					required
				/>
			</p>
			<div class="clear"></div>

			<p class="woocommerce-form-row woocommerce-form-row--wide form-row form-row-wide">
				<label for="account_display_name">
					Display name&nbsp;<span class="required">*</span>
				</label>
				<input 
					type="text" 
					class="woocommerce-Input woocommerce-Input--text input-text" 
					name="account_display_name" 
					id="account_display_name" 
					value={profile.displayName}
					required
				/>
				<span><em>This will be how your name will be displayed in the account section and in reviews</em></span>
			</p>
			<div class="clear"></div>

			<p class="woocommerce-form-row woocommerce-form-row--wide form-row form-row-wide">
				<label for="account_email">
					Email address&nbsp;<span class="required">*</span>
				</label>
				<input 
					type="email" 
					class="woocommerce-Input woocommerce-Input--email input-text" 
					name="account_email" 
					id="account_email" 
					autocomplete="email" 
					value={profile.email}
					required
				/>
			</p>

			<fieldset>
				<legend>Password change</legend>

				<p class="woocommerce-form-row woocommerce-form-row--wide form-row form-row-wide">
					<label for="password_current">Current password (leave blank to leave unchanged)</label>
					<input 
						type="password" 
						class="woocommerce-Input woocommerce-Input--password input-text" 
						name="password_current" 
						id="password_current" 
						autocomplete="off"
					/>
				</p>

				<p class="woocommerce-form-row woocommerce-form-row--wide form-row form-row-wide">
					<label for="password_1">New password (leave blank to leave unchanged)</label>
					<input 
						type="password" 
						class="woocommerce-Input woocommerce-Input--password input-text" 
						name="password_1" 
						id="password_1" 
						autocomplete="off"
					/>
				</p>

				<p class="woocommerce-form-row woocommerce-form-row--wide form-row form-row-wide">
					<label for="password_2">Confirm new password</label>
					<input 
						type="password" 
						class="woocommerce-Input woocommerce-Input--password input-text" 
						name="password_2" 
						id="password_2" 
						autocomplete="off"
					/>
				</p>
			</fieldset>
			<div class="clear"></div>

			<p>
				<button 
					type="submit" 
					class="woocommerce-Button button" 
					name="save_account_details" 
					value="Save changes"
					disabled={isSubmitting}
				>
					{isSubmitting ? 'Saving...' : 'Save changes'}
				</button>
				<input type="hidden" name="action" value="save_account_details" />
			</p>
			</form>
		</div>
	</div>
</div>

<style>
	/* Dashboard Header */
	.dashboard__header {
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
		padding: 20px 30px;
		display: flex;
		align-items: center;
		justify-content: space-between;
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
	.edit-account-card {
		background: #ffffff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		padding: 40px;
		margin-bottom: 30px;
	}


	.woocommerce {
		background: #fff;
	}

	.woocommerce-MyAccount-content {
		padding: 0;
	}

	.woocommerce-notices-wrapper {
		margin-bottom: 20px;
	}

	.woocommerce-message,
	.woocommerce-error {
		padding: 16px 20px;
		border-radius: 4px;
		margin-bottom: 20px;
		font-size: 14px;
	}

	.woocommerce-message {
		background: #d4edda;
		color: #155724;
		border: 1px solid #c3e6cb;
	}

	.woocommerce-error {
		background: #f8d7da;
		color: #721c24;
		border: 1px solid #f5c6cb;
	}

	.woocommerce-EditAccountForm {
		max-width: 600px;
	}

	.form-row {
		margin-bottom: 20px;
	}

	.form-row-first {
		width: 48%;
		float: left;
	}

	.form-row-last {
		width: 48%;
		float: right;
	}

	.form-row-wide {
		clear: both;
		width: 100%;
	}

	.clear {
		clear: both;
	}

	label {
		display: block;
		margin-bottom: 8px;
		font-weight: 600;
		font-size: 14px;
		color: #333;
	}

	.required {
		color: #dc3545;
		font-weight: 700;
	}

	.woocommerce-Input {
		width: 100%;
		padding: 12px 16px;
		border: 1px solid #ddd;
		border-radius: 4px;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
		transition: border-color 0.2s ease, box-shadow 0.2s ease;
	}

	.woocommerce-Input:focus {
		outline: none;
		border-color: #0984ae;
		box-shadow: 0 0 0 3px rgba(9, 132, 174, 0.1);
	}

	.form-row span {
		display: block;
		margin-top: 8px;
		font-size: 13px;
		color: #6c757d;
	}

	.form-row span em {
		font-style: italic;
	}

	fieldset {
		border: 1px solid #e9ecef;
		border-radius: 8px;
		padding: 24px;
		margin: 30px 0;
	}

	legend {
		font-size: 16px;
		font-weight: 600;
		color: #333;
		padding: 0 10px;
	}

	.woocommerce-Button {
		display: inline-block;
		padding: 12px 32px;
		background: #0984ae;
		color: #fff;
		border: none;
		border-radius: 4px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.2s ease;
		font-family: 'Open Sans', sans-serif;
	}

	.woocommerce-Button:hover:not(:disabled) {
		background: #076a8a;
	}

	.woocommerce-Button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	@media (max-width: 768px) {
		.form-row-first,
		.form-row-last {
			width: 100%;
			float: none;
		}

		fieldset {
			padding: 16px;
		}

		.woocommerce-Button {
			width: 100%;
		}
	}
</style>

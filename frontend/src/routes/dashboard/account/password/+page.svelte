<script lang="ts">
	/**
	 * Edit Your Password - Account Section
	 * Reference: frontend/Do's/Edit-Your-Password
	 */
	let currentPassword = $state('');
	let newPassword = $state('');
	let confirmPassword = $state('');
	let isSaving = $state(false);
	let message = $state<{ type: 'success' | 'error'; text: string } | null>(null);

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		message = null;

		// Validation
		if (!currentPassword) {
			message = { type: 'error', text: 'Please enter your current password.' };
			return;
		}

		if (!newPassword) {
			message = { type: 'error', text: 'Please enter a new password.' };
			return;
		}

		if (newPassword.length < 8) {
			message = { type: 'error', text: 'Password must be at least 8 characters long.' };
			return;
		}

		if (newPassword !== confirmPassword) {
			message = { type: 'error', text: 'New passwords do not match.' };
			return;
		}

		isSaving = true;

		// Simulate API call
		await new Promise((resolve) => setTimeout(resolve, 1000));

		message = { type: 'success', text: 'Password changed successfully.' };
		isSaving = false;
		currentPassword = '';
		newPassword = '';
		confirmPassword = '';
	}
</script>

<svelte:head>
	<title>Change Password | Revolution Trading Pros</title>
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<nav class="breadcrumb">
			<a href="/dashboard">Dashboard</a>
			<span>/</span>
			<a href="/dashboard/account">My Account</a>
			<span>/</span>
			<span class="current">Change Password</span>
		</nav>
		<h1 class="dashboard__page-title">Change Password</h1>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<div class="content-box">
				<div class="content-box__section">
					{#if message}
						<div class="woocommerce-message woocommerce-message--{message.type}">
							{message.text}
						</div>
					{/if}

					<form class="edit-password-form" onsubmit={handleSubmit}>
						<fieldset>
							<legend>Password change</legend>

							<p class="form-row form-row-wide">
								<label for="password_current"
									>Current password <span class="required">*</span></label
								>
								<input
									type="password"
									class="input-text"
									name="password_current"
									id="password_current"
									autocomplete="current-password"
									bind:value={currentPassword}
								/>
							</p>

							<p class="form-row form-row-wide">
								<label for="password_1"
									>New password <span class="required">*</span></label
								>
								<input
									type="password"
									class="input-text"
									name="password_1"
									id="password_1"
									autocomplete="new-password"
									bind:value={newPassword}
								/>
								<span class="description"
									>Password must be at least 8 characters long</span
								>
							</p>

							<p class="form-row form-row-wide">
								<label for="password_2"
									>Confirm new password <span class="required">*</span></label
								>
								<input
									type="password"
									class="input-text"
									name="password_2"
									id="password_2"
									autocomplete="new-password"
									bind:value={confirmPassword}
								/>
							</p>
						</fieldset>

						<div class="clear"></div>

						<div class="form-actions">
							<button type="submit" class="btn btn-primary" disabled={isSaving}>
								{isSaving ? 'Saving...' : 'Change Password'}
							</button>
							<a href="/dashboard/account" class="btn btn-secondary">Cancel</a>
						</div>
					</form>
				</div>
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
		border-radius: 4px;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
		max-width: 500px;
	}

	.content-box__section {
		padding: 30px;
	}

	fieldset {
		border: none;
		padding: 0;
		margin: 0;
	}

	fieldset legend {
		font-size: 20px;
		font-weight: 700;
		color: #333;
		margin-bottom: 25px;
		padding-bottom: 15px;
		border-bottom: 1px solid #ededed;
		display: block;
		width: 100%;
	}

	.form-row {
		margin-bottom: 20px;
	}

	.form-row-wide {
		width: 100%;
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

	.input-text {
		width: 100%;
		padding: 12px 14px;
		font-size: 14px;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		transition: border-color 0.15s ease;
		font-family: 'Open Sans', sans-serif;
	}

	.input-text:focus {
		outline: none;
		border-color: #0984ae;
	}

	.description {
		display: block;
		margin-top: 6px;
		font-size: 13px;
		color: #666;
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

	.clear {
		clear: both;
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
		.content-box__section {
			padding: 20px;
		}

		.form-actions {
			flex-direction: column;
		}
	}
</style>

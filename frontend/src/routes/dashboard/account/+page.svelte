<script lang="ts">
	/**
	 * Dashboard - My Account Page - Simpler Trading EXACT
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Exact match of Simpler Trading's WooCommerce Account Details page.
	 * Shows personal details form and password change section.
	 *
	 * @version 4.0.0 (Simpler Trading Exact / December 2025)
	 */

	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { authStore, isAuthenticated, user } from '$lib/stores/auth';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let firstName = $state('');
	let lastName = $state('');
	let displayName = $state('');
	let email = $state('');
	let currentPassword = $state('');
	let newPassword = $state('');
	let confirmPassword = $state('');
	let isSaving = $state(false);
	let message = $state<{ type: 'success' | 'error'; text: string } | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (browser && !$isAuthenticated && !$authStore.isInitializing) {
			goto('/login?redirect=/dashboard/account', { replaceState: true });
		}
	});

	// Initialize form with user data
	$effect(() => {
		if ($user) {
			const nameParts = ($user.name || '').split(' ');
			firstName = nameParts[0] || '';
			lastName = nameParts.slice(1).join(' ') || '';
			displayName = $user.name || '';
			email = $user.email || '';
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function handleSubmit(e: Event): Promise<void> {
		e.preventDefault();
		isSaving = true;
		message = null;

		try {
			// TODO: Implement actual API call
			await new Promise((resolve) => setTimeout(resolve, 1000));
			message = { type: 'success', text: 'Account details updated successfully.' };
		} catch (err) {
			message = { type: 'error', text: 'Failed to update account details.' };
		} finally {
			isSaving = false;
		}
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>My Account | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     DASHBOARD HEADER - WordPress: .dashboard__header
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">My Account</h1>
	</div>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     DASHBOARD CONTENT - WordPress: .dashboard__content
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<!-- WooCommerce: Account Details Form -->
			<div class="account-details-form">
				<h2 class="form-title">Account Details</h2>

				{#if message}
					<div class="message message--{message.type}">
						{message.text}
					</div>
				{/if}

				<form onsubmit={handleSubmit}>
					<!-- Personal Details Section -->
					<div class="form-section">
						<h3 class="form-section-title">PERSONAL DETAILS</h3>

						<div class="form-row form-row--two-columns">
							<div class="form-group">
								<label for="firstName" class="form-label">
									First name <span class="required">*</span>
								</label>
								<input
									type="text"
									id="firstName"
									class="form-input"
									bind:value={firstName}
									required
								/>
							</div>
							<div class="form-group">
								<label for="lastName" class="form-label">
									Last name <span class="required">*</span>
								</label>
								<input
									type="text"
									id="lastName"
									class="form-input"
									bind:value={lastName}
									required
								/>
							</div>
						</div>

						<div class="form-group">
							<label for="displayName" class="form-label">
								Display name <span class="required">*</span>
							</label>
							<input
								type="text"
								id="displayName"
								class="form-input"
								bind:value={displayName}
								required
							/>
							<p class="form-help">
								<em>This will be how your name will be displayed in the account section and in reviews</em>
							</p>
						</div>

						<div class="form-group">
							<label for="email" class="form-label">
								Email address <span class="required">*</span>
							</label>
							<input
								type="email"
								id="email"
								class="form-input"
								bind:value={email}
								required
							/>
						</div>
					</div>

					<!-- Password Change Section -->
					<div class="form-section">
						<h3 class="form-section-title">PASSWORD CHANGE</h3>

						<div class="form-group">
							<label for="currentPassword" class="form-label">
								Current password (leave blank to leave unchanged)
							</label>
							<div class="password-input-wrapper">
								<input
									type="password"
									id="currentPassword"
									class="form-input"
									bind:value={currentPassword}
								/>
								<button type="button" class="password-toggle" aria-label="Toggle password visibility">
									<span class="password-toggle-icon">👁</span>
								</button>
							</div>
						</div>

						<div class="form-group">
							<label for="newPassword" class="form-label">
								New password (leave blank to leave unchanged)
							</label>
							<div class="password-input-wrapper">
								<input
									type="password"
									id="newPassword"
									class="form-input"
									bind:value={newPassword}
								/>
								<button type="button" class="password-toggle" aria-label="Toggle password visibility">
									<span class="password-toggle-icon">👁</span>
								</button>
							</div>
						</div>

						<div class="form-group">
							<label for="confirmPassword" class="form-label">
								Confirm new password
							</label>
							<div class="password-input-wrapper">
								<input
									type="password"
									id="confirmPassword"
									class="form-input"
									bind:value={confirmPassword}
								/>
								<button type="button" class="password-toggle" aria-label="Toggle password visibility">
									<span class="password-toggle-icon">👁</span>
								</button>
							</div>
						</div>
					</div>

					<!-- Submit Button -->
					<div class="form-actions">
						<button type="submit" class="btn btn-orange" disabled={isSaving}>
							{isSaving ? 'Saving...' : 'Save changes'}
						</button>
					</div>
				</form>
			</div>
		</section>
	</div>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - Simpler Trading EXACT CSS
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER - WordPress: .dashboard__header
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		padding: 20px 30px;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.dashboard__header-left {
		display: flex;
		align-items: center;
	}

	.dashboard__page-title {
		color: #333;
		font-family: 'Open Sans Condensed', 'Open Sans', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
		line-height: 1.2;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT - WordPress: .dashboard__content
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		padding: 30px;
		background: #f4f4f4;
		min-height: calc(100vh - 80px);
	}

	.dashboard__content-main {
		width: 100%;
		max-width: 800px;
	}

	.dashboard__content-section {
		background: #fff;
		border-radius: 4px;
		padding: 30px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCOUNT DETAILS FORM - WordPress/WooCommerce Style
	   ═══════════════════════════════════════════════════════════════════════════ */

	.account-details-form {
		width: 100%;
	}

	.form-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 24px;
		font-family: 'Open Sans Condensed', sans-serif;
	}

	.form-section {
		margin-bottom: 32px;
	}

	.form-section-title {
		font-size: 14px;
		font-weight: 700;
		color: #0984ae;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		margin: 0 0 20px;
		padding-bottom: 10px;
		border-bottom: 2px solid #0984ae;
	}

	.form-row {
		display: flex;
		gap: 20px;
	}

	.form-row--two-columns {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 20px;
	}

	.form-group {
		margin-bottom: 20px;
	}

	.form-label {
		display: block;
		font-size: 14px;
		font-weight: 400;
		color: #333;
		margin-bottom: 8px;
	}

	.required {
		color: #e74c3c;
	}

	.form-input {
		width: 100%;
		padding: 12px 16px;
		font-size: 14px;
		color: #333;
		background: #fff;
		border: 1px solid #ddd;
		border-radius: 4px;
		transition: border-color 0.15s ease, box-shadow 0.15s ease;
	}

	.form-input:focus {
		outline: none;
		border-color: #0984ae;
		box-shadow: 0 0 0 3px rgba(9, 132, 174, 0.1);
	}

	.form-help {
		font-size: 13px;
		color: #666;
		margin: 8px 0 0;
	}

	.form-help em {
		font-style: italic;
	}

	/* Password Input with Toggle */
	.password-input-wrapper {
		position: relative;
	}

	.password-input-wrapper .form-input {
		padding-right: 48px;
	}

	.password-toggle {
		position: absolute;
		right: 12px;
		top: 50%;
		transform: translateY(-50%);
		background: none;
		border: none;
		padding: 4px;
		cursor: pointer;
		color: #666;
	}

	.password-toggle:hover {
		color: #333;
	}

	.password-toggle-icon {
		font-size: 16px;
	}

	/* Form Actions */
	.form-actions {
		margin-top: 24px;
	}

	.btn-orange {
		display: inline-block;
		padding: 12px 24px;
		font-size: 14px;
		font-weight: 700;
		color: #fff;
		background: #f99e31;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		transition: background-color 0.15s ease;
	}

	.btn-orange:hover:not(:disabled) {
		background: #dc7309;
	}

	.btn-orange:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* Messages */
	.message {
		padding: 12px 16px;
		border-radius: 4px;
		margin-bottom: 20px;
		font-size: 14px;
	}

	.message--success {
		background: #d4edda;
		color: #155724;
		border: 1px solid #c3e6cb;
	}

	.message--error {
		background: #f8d7da;
		color: #721c24;
		border: 1px solid #f5c6cb;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 768px) {
		.dashboard__header {
			padding: 16px 20px;
		}

		.dashboard__page-title {
			font-size: 28px;
		}

		.dashboard__content {
			padding: 20px;
		}

		.dashboard__content-section {
			padding: 20px;
		}

		.form-row--two-columns {
			grid-template-columns: 1fr;
		}
	}
</style>

<script lang="ts">
	/**
	 * Dashboard - Account Details Edit Page - Simpler Trading EXACT
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /dashboard/account/edit-account
	 * Shows personal details form and password change section.
	 *
	 * @version 1.0.0 (Simpler Trading Exact / December 2025)
	 */

	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { authStore, isAuthenticated, user } from '$lib/stores/auth';
	import { IconEye, IconEyeOff } from '$lib/icons';
	import Footer from '$lib/components/sections/Footer.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let firstName = $state('Zack');
	let lastName = $state('Stambowski');
	let displayName = $state('Zack Stambowski');
	let email = $state('welberribeirodrums@gmail.com');
	let currentPassword = $state('');
	let newPassword = $state('');
	let confirmPassword = $state('');
	let isSaving = $state(false);
	let message = $state<{ type: 'success' | 'error'; text: string } | null>(null);

	// Password visibility toggles
	let showCurrentPassword = $state(false);
	let showNewPassword = $state(false);
	let showConfirmPassword = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (browser && !$isAuthenticated && !$authStore.isInitializing) {
			goto('/login?redirect=/dashboard/account/edit-account', { replaceState: true });
		}
	});

	// Initialize form with user data
	$effect(() => {
		if ($user) {
			const nameParts = ($user.name || '').split(' ');
			firstName = nameParts[0] || 'Zack';
			lastName = nameParts.slice(1).join(' ') || 'Stambowski';
			displayName = $user.name || 'Zack Stambowski';
			email = $user.email || 'welberribeirodrums@gmail.com';
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

	function togglePasswordVisibility(field: 'current' | 'new' | 'confirm'): void {
		if (field === 'current') showCurrentPassword = !showCurrentPassword;
		if (field === 'new') showNewPassword = !showNewPassword;
		if (field === 'confirm') showConfirmPassword = !showConfirmPassword;
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>Account Details | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEADER
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="dashboard__header">
	<h1 class="dashboard__page-title">My Account</h1>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     MAIN CONTENT
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<div class="account-details-card">
			<h2 class="card-title">Account Details</h2>

			{#if message}
				<div class="message message--{message.type}">
					{message.text}
				</div>
			{/if}

			<form onsubmit={handleSubmit}>
				<!-- PERSONAL DETAILS Section -->
				<div class="form-section">
					<h3 class="form-section-title">PERSONAL DETAILS</h3>

					<div class="form-row-two">
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

				<!-- PASSWORD CHANGE Section -->
				<div class="form-section">
					<h3 class="form-section-title">PASSWORD CHANGE</h3>

					<div class="form-group">
						<label for="currentPassword" class="form-label">
							Current password (leave blank to leave unchanged)
						</label>
						<div class="password-wrapper">
							<input
								type={showCurrentPassword ? 'text' : 'password'}
								id="currentPassword"
								class="form-input"
								bind:value={currentPassword}
							/>
							<button
								type="button"
								class="password-toggle"
								onclick={() => togglePasswordVisibility('current')}
								aria-label="Toggle password visibility"
							>
								{#if showCurrentPassword}
									<IconEyeOff size={18} />
								{:else}
									<IconEye size={18} />
								{/if}
							</button>
						</div>
					</div>

					<div class="form-group">
						<label for="newPassword" class="form-label">
							New password (leave blank to leave unchanged)
						</label>
						<div class="password-wrapper">
							<input
								type={showNewPassword ? 'text' : 'password'}
								id="newPassword"
								class="form-input"
								bind:value={newPassword}
							/>
							<button
								type="button"
								class="password-toggle"
								onclick={() => togglePasswordVisibility('new')}
								aria-label="Toggle password visibility"
							>
								{#if showNewPassword}
									<IconEyeOff size={18} />
								{:else}
									<IconEye size={18} />
								{/if}
							</button>
						</div>
					</div>

					<div class="form-group">
						<label for="confirmPassword" class="form-label">
							Confirm new password
						</label>
						<div class="password-wrapper">
							<input
								type={showConfirmPassword ? 'text' : 'password'}
								id="confirmPassword"
								class="form-input"
								bind:value={confirmPassword}
							/>
							<button
								type="button"
								class="password-toggle"
								onclick={() => togglePasswordVisibility('confirm')}
								aria-label="Toggle password visibility"
							>
								{#if showConfirmPassword}
									<IconEyeOff size={18} />
								{:else}
									<IconEye size={18} />
								{/if}
							</button>
						</div>
					</div>
				</div>

				<!-- Save Button -->
				<div class="form-actions">
					<button type="submit" class="btn-save" disabled={isSaving}>
						{isSaving ? 'Saving...' : 'Save changes'}
					</button>
				</div>
			</form>
		</div>
	</div>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     FOOTER
     ═══════════════════════════════════════════════════════════════════════════ -->

<Footer />

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - Simpler Trading EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background: #fff;
		border-bottom: 1px solid #e9ebed;
		padding: 20px 30px;
	}

	.dashboard__page-title {
		color: #333;
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
		line-height: 1.2;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		padding: 30px;
		background: #fff;
		min-height: 500px;
	}

	.dashboard__content-main {
		max-width: 700px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCOUNT DETAILS CARD
	   ═══════════════════════════════════════════════════════════════════════════ */

	.account-details-card {
		background: #fff;
		border: 1px solid #e9ebed;
		border-radius: 4px;
		padding: 30px;
	}

	.card-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 24px;
		font-family: 'Open Sans Condensed', sans-serif;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FORM SECTIONS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.form-section {
		margin-bottom: 32px;
	}

	.form-section-title {
		font-size: 13px;
		font-weight: 700;
		color: #d4a017;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		margin: 0 0 20px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FORM FIELDS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.form-row-two {
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

	/* ═══════════════════════════════════════════════════════════════════════════
	   PASSWORD FIELD WITH TOGGLE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.password-wrapper {
		position: relative;
	}

	.password-wrapper .form-input {
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
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.password-toggle:hover {
		color: #333;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SAVE BUTTON
	   ═══════════════════════════════════════════════════════════════════════════ */

	.form-actions {
		margin-top: 24px;
	}

	.btn-save {
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

	.btn-save:hover:not(:disabled) {
		background: #dc7309;
	}

	.btn-save:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MESSAGES
	   ═══════════════════════════════════════════════════════════════════════════ */

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

		.account-details-card {
			padding: 20px;
		}

		.form-row-two {
			grid-template-columns: 1fr;
		}
	}
</style>

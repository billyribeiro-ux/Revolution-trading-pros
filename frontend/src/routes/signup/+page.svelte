<!--
	URL: /signup
-->

<script lang="ts">
	/**
	 * Signup Page - Svelte 5 Runes Implementation
	 * @version 2.0.0 - November 2025
	 */
	import { onMount } from 'svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { registerAndLogin } from '$lib/api/auth';
	import { goto } from '$app/navigation';

	// Svelte 5 state runes
	let name = $state('');
	let email = $state('');
	let password = $state('');
	let passwordConfirmation = $state('');
	let isLoading = $state(false);
	let errorMessage = $state('');
	let validationErrors = $state<Record<string, string[]>>({});

	// Redirect if already logged in
	// FIX-2026-04-26: comment-out, verify, delete in follow-up.
	// The old $effect below is the same legacy_pre_subscribe cascade pattern that was
	// fixed in admin/+layout.svelte — $effect re-fires on every reactive run that reads
	// $authStore, which can cause multiple redundant goto() calls during store hydration.
	// Replaced with onMount so the redirect fires exactly once, after the component mounts.
	// $effect(() => {
	// 	if ($authStore.user) {
	// 		goto('/account');
	// 	}
	// });
	onMount(() => {
		if ($authStore.user) goto('/account');
	});

	async function handleSignup(e: Event) {
		e.preventDefault();
		errorMessage = '';
		validationErrors = {};
		isLoading = true;

		// Client-side validation
		if (password !== passwordConfirmation) {
			errorMessage = 'Passwords do not match';
			isLoading = false;
			return;
		}

		if (password.length < 8) {
			errorMessage = 'Password must be at least 8 characters';
			isLoading = false;
			return;
		}

		try {
			await registerAndLogin({
				name,
				email,
				password,
				password_confirmation: passwordConfirmation
			});
			goto('/account');
		} catch (error: unknown) {
			const e = error as { message?: string; errors?: Record<string, string[]> };
			errorMessage = e.message || 'Registration failed. Please try again.';
			if (e.errors) {
				validationErrors = e.errors;
			}
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="signup-page">
	<div class="signup-shell">
		<!-- Card -->
		<div class="signup-card">
			<!-- Logo/Header -->
			<div class="signup-header">
				<h1 class="signup-title">Join Revolution Trading</h1>
				<p class="signup-subtitle">Start your trading journey today</p>
			</div>

			<!-- Error Message -->
			{#if errorMessage}
				<div class="error-panel">
					<p class="error-message">{errorMessage}</p>
					{#if Object.keys(validationErrors).length > 0}
						<ul class="error-list">
							{#each Object.entries(validationErrors) as [field, errors] (field)}
								{#each errors as error (error)}
									<li>{error}</li>
								{/each}
							{/each}
						</ul>
					{/if}
				</div>
			{/if}

			<!-- Signup Form -->
			<form onsubmit={handleSignup} class="signup-form">
				<!-- Name -->
				<div class="form-field">
					<label for="name" class="field-label"> Full Name </label>
					<input
						type="text"
						id="name"
						name="name"
						bind:value={name}
						required
						disabled={isLoading}
						class="text-input"
						placeholder="John Doe"
					/>
					{#if validationErrors.name}
						<p class="field-error">{validationErrors.name[0]}</p>
					{/if}
				</div>

				<!-- Email -->
				<div class="form-field">
					<label for="email" class="field-label"> Email Address </label>
					<input
						type="email"
						id="email"
						name="email"
						autocomplete="email"
						bind:value={email}
						required
						disabled={isLoading}
						class="text-input"
						placeholder="you@example.com"
					/>
					{#if validationErrors.email}
						<p class="field-error">{validationErrors.email[0]}</p>
					{/if}
				</div>

				<!-- Password -->
				<div class="form-field">
					<label for="password" class="field-label"> Password </label>
					<input
						type="password"
						id="password"
						name="password"
						autocomplete="new-password"
						bind:value={password}
						required
						minlength="8"
						disabled={isLoading}
						class="text-input"
						placeholder="••••••••"
					/>
					<p class="field-hint">Minimum 8 characters</p>
					{#if validationErrors.password}
						<p class="field-error">{validationErrors.password[0]}</p>
					{/if}
				</div>

				<!-- Confirm Password -->
				<div class="form-field">
					<label for="password_confirmation" class="field-label"> Confirm Password </label>
					<input
						type="password"
						id="password_confirmation"
						name="password_confirmation"
						autocomplete="new-password"
						bind:value={passwordConfirmation}
						required
						minlength="8"
						disabled={isLoading}
						class="text-input"
						placeholder="••••••••"
					/>
				</div>

				<!-- Terms Checkbox -->
				<div class="form-field">
					<label class="terms-row">
						<input
							type="checkbox"
							id="terms-agreement"
							name="terms-agreement"
							required
							class="terms-checkbox"
						/>
						<span class="terms-copy">
							I agree to the
							<a href="/terms" class="text-link">Terms of Service</a>
							and
							<a href="/privacy" class="text-link">Privacy Policy</a>
						</span>
					</label>
				</div>

				<!-- Submit Button -->
				<button type="submit" disabled={isLoading} class="submit-button">
					{#if isLoading}
						<span class="submit-loading">
							<svg
								class="submit-spinner"
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
							>
								<circle
									class="spinner-track"
									cx="12"
									cy="12"
									r="10"
									stroke="currentColor"
									stroke-width="4"
								></circle>
								<path
									class="spinner-head"
									fill="currentColor"
									d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
								></path>
							</svg>
							Creating account...
						</span>
					{:else}
						Create Account
					{/if}
				</button>
			</form>

			<!-- Divider -->
			<div class="divider">
				<span>Or</span>
			</div>

			<!-- Sign In Link -->
			<div class="signin-footer">
				<p>
					Already have an account?
					<a href="/login" class="text-link text-link--strong"> Sign in </a>
				</p>
			</div>
		</div>

		<!-- Benefits -->
		<div class="benefits-grid">
			<div class="benefit-item">
				<div class="benefit-value benefit-value--primary">10K+</div>
				<div class="benefit-label">Active Traders</div>
			</div>
			<div class="benefit-item">
				<div class="benefit-value benefit-value--success">78%</div>
				<div class="benefit-label">Win Rate</div>
			</div>
			<div class="benefit-item">
				<div class="benefit-value benefit-value--accent">$23M+</div>
				<div class="benefit-label">Total Profits</div>
			</div>
		</div>
	</div>
</div>

<style>
	.signup-page {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: calc(100vh - 120px);
		background: var(--rtp-bg, #05070d);
		padding: 2rem 1rem;
	}

	.signup-shell {
		width: 100%;
		max-width: 28rem;
	}

	.signup-card {
		border: 1px solid var(--rtp-border, rgba(255, 255, 255, 0.12));
		border-radius: 1rem;
		background: var(--rtp-surface, #101826);
		padding: 1.5rem;
		box-shadow: 0 20px 45px rgba(0, 0, 0, 0.28);
	}

	.signup-header {
		margin-bottom: 1.5rem;
		text-align: center;
	}

	.signup-title {
		margin: 0 0 0.5rem;
		color: var(--rtp-text, #f8fafc);
		font-family: var(--font-heading, inherit);
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 2rem;
	}

	.signup-subtitle,
	.field-hint,
	.terms-copy,
	.signin-footer,
	.benefit-label {
		color: var(--rtp-muted, #94a3b8);
	}

	.error-panel {
		margin-bottom: 1.5rem;
		border: 1px solid #fecaca;
		border-radius: 0.5rem;
		background: #fef2f2;
		padding: 1rem;
	}

	.error-message {
		margin: 0;
		color: #991b1b;
		font-size: 0.875rem;
	}

	.error-list {
		margin: 0.5rem 0 0;
		padding-left: 1.25rem;
		color: #b91c1c;
		font-size: 0.875rem;
	}

	.signup-form {
		display: grid;
		gap: 1.5rem;
	}

	.form-field {
		display: grid;
		gap: 0.5rem;
	}

	.field-label {
		color: var(--rtp-text, #f8fafc);
		font-size: 0.875rem;
		font-weight: 600;
	}

	.text-input {
		width: 100%;
		min-height: 44px;
		border: 1px solid var(--rtp-border, rgba(255, 255, 255, 0.12));
		border-radius: 0.5rem;
		background: rgba(255, 255, 255, 0.04);
		color: var(--rtp-text, #f8fafc);
		padding: 0.875rem 1rem;
		font: inherit;
		font-size: 1rem;
		transition:
			border-color 160ms ease,
			box-shadow 160ms ease,
			opacity 160ms ease;
	}

	.text-input:focus {
		border-color: var(--rtp-primary, #e6b800);
		box-shadow: 0 0 0 3px color-mix(in srgb, var(--rtp-primary, #e6b800) 26%, transparent);
		outline: none;
	}

	.text-input:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.field-hint {
		margin: 0;
		font-size: 0.75rem;
	}

	.field-error {
		margin: 0;
		color: #dc2626;
		font-size: 0.875rem;
	}

	.terms-row {
		display: flex;
		align-items: flex-start;
		margin: -0.5rem;
		border-radius: 0.5rem;
		padding: 0.5rem;
		cursor: pointer;
	}

	.terms-row:hover {
		background: color-mix(in srgb, var(--rtp-border, rgba(255, 255, 255, 0.12)) 18%, transparent);
	}

	.terms-checkbox {
		width: 1.25rem;
		min-width: 1.25rem;
		height: 1.25rem;
		margin-top: 0.125rem;
		accent-color: var(--rtp-primary, #e6b800);
	}

	.terms-copy {
		margin-left: 0.75rem;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.text-link {
		color: var(--rtp-primary, #e6b800);
		text-decoration: none;
	}

	.text-link:hover {
		text-decoration: underline;
	}

	.text-link--strong {
		font-weight: 600;
	}

	.submit-button {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		min-height: 48px;
		border: 0;
		border-radius: 0.5rem;
		background: linear-gradient(90deg, var(--rtp-primary, #e6b800), var(--rtp-blue, #2563eb));
		color: #ffffff;
		padding: 1rem;
		font-size: 1rem;
		font-weight: 700;
		transition:
			box-shadow 180ms ease,
			opacity 180ms ease,
			transform 180ms ease;
	}

	.submit-button:hover:not(:disabled) {
		box-shadow: 0 14px 28px rgba(37, 99, 235, 0.22);
		transform: scale(1.02);
	}

	.submit-button:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.submit-loading {
		display: inline-flex;
		align-items: center;
	}

	.submit-spinner {
		width: 1.25rem;
		height: 1.25rem;
		margin-right: 0.75rem;
		margin-left: -0.25rem;
		color: #ffffff;
		animation: spin 800ms linear infinite;
	}

	.spinner-track {
		opacity: 0.25;
	}

	.spinner-head {
		opacity: 0.75;
	}

	.divider {
		position: relative;
		display: flex;
		justify-content: center;
		margin: 1.5rem 0;
		color: var(--rtp-muted, #94a3b8);
		font-size: 0.875rem;
	}

	.divider::before {
		position: absolute;
		top: 50%;
		right: 0;
		left: 0;
		border-top: 1px solid var(--rtp-border, rgba(255, 255, 255, 0.12));
		content: '';
	}

	.divider span {
		position: relative;
		background: var(--rtp-surface, #101826);
		padding: 0 0.5rem;
	}

	.signin-footer {
		text-align: center;
	}

	.signin-footer p {
		margin: 0;
	}

	.benefits-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1rem;
		margin-top: 2rem;
		text-align: center;
	}

	.benefit-value {
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 2rem;
	}

	.benefit-value--primary {
		color: var(--rtp-primary, #e6b800);
	}

	.benefit-value--success {
		color: var(--rtp-emerald, #10b981);
	}

	.benefit-value--accent {
		color: var(--rtp-indigo, #6366f1);
	}

	.benefit-label {
		margin-top: 0.25rem;
		font-size: 0.75rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@media (min-width: 640px) {
		.signup-page {
			padding: 3rem 1.5rem;
		}

		.signup-card {
			padding: 2rem;
		}

		.signup-header {
			margin-bottom: 2rem;
		}

		.signup-title {
			font-size: 1.875rem;
			line-height: 2.25rem;
		}

		.benefits-grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}
</style>

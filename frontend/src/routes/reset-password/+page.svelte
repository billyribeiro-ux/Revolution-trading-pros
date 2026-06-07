<script lang="ts">
	/**
	 * Reset Password Page - Svelte 5 January 2026
	 * @version 2.0.0
	 */
	import { goto } from '$app/navigation';
	import { resetPassword } from '$lib/api/auth';
	import {
		IconLock,
		IconAlertCircle,
		IconCheck,
		IconShieldCheck,
		IconEye,
		IconEyeOff
	} from '$lib/icons';
	import { onDestroy, onMount } from 'svelte';
	import { browser } from '$app/environment';

	// Svelte 5 state runes
	let email = $state('');
	let password = $state('');
	let password_confirmation = $state('');
	let token = $state('');
	let showPassword = $state(false);
	let showConfirmPassword = $state(false);
	let errors = $state<Record<string, string[]>>({});
	let generalError = $state('');
	let successMessage = $state('');
	let isLoading = $state(false);
	let isVisible = $state(false);
	let redirectTimeout: ReturnType<typeof setTimeout> | null = null;

	// Svelte 5 effect for initialization
	onMount(() => {
		if (!browser) return;

		isVisible = true;
		// Get token and email from URL parameters
		const urlParams = new URLSearchParams(window.location.search);
		token = urlParams.get('token') || '';
		email = urlParams.get('email') || '';
	});

	onDestroy(() => {
		if (redirectTimeout) {
			clearTimeout(redirectTimeout);
		}
	});

	async function handleSubmit(e: Event) {
		e.preventDefault();
		errors = {};
		generalError = '';
		successMessage = '';
		isLoading = true;

		try {
			const message = await resetPassword({ token, email, password, password_confirmation });
			successMessage = message;

			// Redirect to login after 2 seconds
			if (redirectTimeout) {
				clearTimeout(redirectTimeout);
			}
			redirectTimeout = setTimeout(() => {
				void goto('/login');
			}, 2000);
		} catch (error) {
			if (error && typeof error === 'object' && 'errors' in error) {
				errors = (error as { errors: Record<string, string[]> }).errors;
			} else if (error instanceof Error) {
				generalError = error.message;
			} else {
				generalError = 'An unexpected error occurred. Please try again.';
			}
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="reset-password-page">
	<!-- Animated gradient background -->
	<div class="reset-password-background"></div>

	<!-- Floating orbs -->
	<div class="reset-password-orb reset-password-orb--primary"></div>
	<div class="reset-password-orb reset-password-orb--secondary"></div>

	<!-- Reset password card -->
	<div
		class={{
			'reset-password-card': true,
			'reset-password-card--visible': isVisible
		}}
	>
		<!-- Glow effect -->
		<div class="reset-password-glow"></div>

		<div class="reset-password-panel">
			<!-- Header -->
			<div class="reset-password-header">
				<div class="reset-password-icon-shell">
					<div class="reset-password-icon-inner">
						<IconShieldCheck size={40} class="reset-password-header-icon" />
					</div>
				</div>
				<h1 class="reset-password-title">Reset Password</h1>
				<p class="reset-password-subtitle">Enter your new password below</p>
			</div>

			<!-- Success message -->
			{#if successMessage}
				<div class="reset-alert reset-alert--success">
					<IconCheck size={20} class="reset-alert__icon reset-alert__icon--success" />
					<div class="reset-alert__content reset-alert__content--success">
						<p>{successMessage}</p>
						<p class="reset-alert__muted">Redirecting to login...</p>
					</div>
				</div>
			{/if}

			<!-- General error -->
			{#if generalError}
				<div class="reset-alert reset-alert--error">
					<IconAlertCircle size={20} class="reset-alert__icon reset-alert__icon--error" />
					<p class="reset-alert__content reset-alert__content--error">{generalError}</p>
				</div>
			{/if}

			<!-- Reset password form -->
			<form onsubmit={handleSubmit} class="reset-password-form">
				<!-- Email field (hidden, auto-filled from URL) -->
				<input type="hidden" name="email" id="email" bind:value={email} />
				<input type="hidden" name="token" id="token" bind:value={token} />

				<!-- Email display (read-only) -->
				<div class="reset-field">
					<div class="reset-field-label">Email Address</div>
					<div class="reset-readonly-field">
						{email}
					</div>
				</div>

				<!-- New password field -->
				<div class="reset-field">
					<label for="password" class="reset-field-label"> New Password </label>
					<div class="reset-input-wrap">
						<div class="reset-input-icon">
							<IconLock size={20} />
						</div>
						<input
							id="password"
							name="password"
							type={showPassword ? 'text' : 'password'}
							bind:value={password}
							required
							minlength="12"
							autocomplete="new-password"
							class={{
								'reset-input': true,
								'reset-input--error': Boolean(errors.password?.length)
							}}
							placeholder="••••••••"
						/>
						<button
							type="button"
							class="reset-password-toggle"
							onclick={() => (showPassword = !showPassword)}
							aria-label={showPassword ? 'Hide password' : 'Show password'}
							tabindex={-1}
						>
							{#if showPassword}
								<IconEyeOff size={20} />
							{:else}
								<IconEye size={20} />
							{/if}
						</button>
					</div>
					{#if errors.password}
						<p class="reset-field-message reset-field-message--error">{errors.password[0]}</p>
					{:else}
						<p class="reset-field-message reset-field-message--hint">
							Minimum 12 characters with uppercase, lowercase, and number
						</p>
					{/if}
				</div>

				<!-- Confirm password field -->
				<div class="reset-field">
					<label for="password_confirmation" class="reset-field-label">
						Confirm New Password
					</label>
					<div class="reset-input-wrap">
						<div class="reset-input-icon">
							<IconLock size={20} />
						</div>
						<input
							id="password_confirmation"
							name="password_confirmation"
							type={showConfirmPassword ? 'text' : 'password'}
							bind:value={password_confirmation}
							required
							autocomplete="new-password"
							class="reset-input"
							placeholder="••••••••"
						/>
						<button
							type="button"
							class="reset-password-toggle"
							onclick={() => (showConfirmPassword = !showConfirmPassword)}
							aria-label={showConfirmPassword ? 'Hide password' : 'Show password'}
							tabindex={-1}
						>
							{#if showConfirmPassword}
								<IconEyeOff size={20} />
							{:else}
								<IconEye size={20} />
							{/if}
						</button>
					</div>
				</div>

				<!-- Submit button -->
				<button type="submit" disabled={isLoading || !!successMessage} class="reset-submit">
					<span class="reset-submit__content">
						{#if isLoading}
							<div class="reset-submit__spinner"></div>
							<span>Resetting Password...</span>
						{:else if successMessage}
							<IconCheck size={20} />
							<span>Password Reset!</span>
						{:else}
							<IconShieldCheck size={20} />
							<span>Reset Password</span>
						{/if}
					</span>
					<div class="reset-submit__shine"></div>
				</button>
			</form>

			<!-- Back to login link -->
			<div class="reset-back">
				<a href="/login" class="reset-back__link">
					<svg
						aria-hidden="true"
						class="reset-back__icon"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M10 19l-7-7m0 0l7-7m-7 7h18"
						/>
					</svg>
					<span>Back to login</span>
				</a>
			</div>
		</div>
	</div>
</div>

<style>
	.reset-password-page {
		position: relative;
		display: flex;
		min-height: 100vh;
		align-items: center;
		justify-content: center;
		overflow: hidden;
		padding: 3rem 1rem;
		background: #020617;
	}

	.reset-password-background {
		position: absolute;
		inset: 0;
		background: linear-gradient(135deg, #020617 0%, rgba(59, 7, 100, 0.3) 50%, #020617 100%);
	}

	.reset-password-orb {
		position: absolute;
		width: 24rem;
		height: 24rem;
		border-radius: 999px;
		filter: blur(64px);
		pointer-events: none;
	}

	.reset-password-orb--primary {
		top: 5rem;
		right: 2.5rem;
		background: rgba(168, 85, 247, 0.1);
		animation: float 8s ease-in-out infinite;
	}

	.reset-password-orb--secondary {
		bottom: 5rem;
		left: 2.5rem;
		background: rgba(236, 72, 153, 0.1);
		animation: float-delayed 10s ease-in-out infinite;
	}

	.reset-password-card {
		position: relative;
		z-index: 10;
		width: min(100%, 28rem);
	}

	.reset-password-card--visible {
		animation: card-reveal 0.6s ease-out forwards;
		opacity: 0;
	}

	.reset-password-glow {
		position: absolute;
		inset: -0.25rem;
		border-radius: 1.5rem;
		background: linear-gradient(90deg, #a855f7, #ec4899, #f43f5e);
		opacity: 0.2;
		filter: blur(24px);
	}

	.reset-password-panel {
		position: relative;
		border: 1px solid rgba(168, 85, 247, 0.2);
		border-radius: 1.5rem;
		background: rgba(15, 23, 42, 0.9);
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.45);
		padding: 2.5rem;
		backdrop-filter: blur(24px);
	}

	.reset-password-header {
		margin-bottom: 2rem;
		text-align: center;
	}

	.reset-password-icon-shell {
		display: inline-flex;
		width: 5rem;
		height: 5rem;
		align-items: center;
		justify-content: center;
		margin-bottom: 1.5rem;
		border-radius: 1rem;
		background: linear-gradient(135deg, #a855f7, #ec4899);
		padding: 0.125rem;
	}

	.reset-password-icon-inner {
		display: flex;
		width: 100%;
		height: 100%;
		align-items: center;
		justify-content: center;
		border-radius: 1rem;
		background: #0f172a;
	}

	:global(.reset-password-header-icon) {
		color: #c084fc;
	}

	.reset-password-title {
		margin: 0 0 0.75rem;
		background: linear-gradient(90deg, #d8b4fe, #f9a8d4, #fda4af);
		background-clip: text;
		color: transparent;
		font-family: var(--font-display, inherit);
		font-size: 2.25rem;
		font-weight: 700;
		line-height: 2.5rem;
	}

	.reset-password-subtitle {
		margin: 0;
		color: #94a3b8;
	}

	.reset-alert {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
		border: 1px solid;
		border-radius: 0.75rem;
		padding: 1rem;
		animation: fade-in 0.3s ease-out;
	}

	.reset-alert--success {
		border-color: rgba(16, 185, 129, 0.3);
		background: rgba(16, 185, 129, 0.1);
	}

	.reset-alert--error {
		border-color: rgba(239, 68, 68, 0.3);
		background: rgba(239, 68, 68, 0.1);
	}

	:global(.reset-alert__icon) {
		flex-shrink: 0;
		margin-top: 0.125rem;
	}

	:global(.reset-alert__icon--success) {
		color: #34d399;
	}

	:global(.reset-alert__icon--error) {
		color: #f87171;
	}

	.reset-alert__content {
		margin: 0;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.reset-alert__content--success {
		color: #6ee7b7;
	}

	.reset-alert__content--error {
		color: #fca5a5;
	}

	.reset-alert__content p,
	.reset-alert__muted {
		margin: 0;
	}

	.reset-alert__muted {
		margin-top: 0.25rem;
		color: rgba(52, 211, 153, 0.7);
	}

	.reset-password-form {
		display: grid;
		gap: 1.25rem;
	}

	.reset-field-label {
		display: block;
		margin-bottom: 0.5rem;
		color: #cbd5e1;
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.25rem;
	}

	.reset-readonly-field,
	.reset-input {
		width: 100%;
		border: 1px solid;
		border-radius: 0.75rem;
		padding: 0.875rem 1rem;
	}

	.reset-readonly-field {
		border-color: rgba(51, 65, 85, 0.5);
		background: rgba(30, 41, 59, 0.3);
		color: #94a3b8;
	}

	.reset-input-wrap {
		position: relative;
	}

	.reset-input-icon {
		position: absolute;
		top: 50%;
		left: 1rem;
		color: #64748b;
		transform: translateY(-50%);
	}

	.reset-input {
		border-color: #334155;
		background: rgba(30, 41, 59, 0.5);
		color: #ffffff;
		padding-inline: 3rem;
		transition:
			border-color 0.3s ease,
			box-shadow 0.3s ease,
			background-color 0.3s ease;
	}

	.reset-input::placeholder {
		color: #64748b;
	}

	.reset-input:focus {
		border-color: #a855f7;
		box-shadow: 0 0 0 4px rgba(168, 85, 247, 0.1);
		outline: none;
	}

	.reset-input--error {
		border-color: #ef4444;
	}

	.reset-password-toggle {
		position: absolute;
		top: 50%;
		right: 1rem;
		color: #64748b;
		transform: translateY(-50%);
		transition: color 0.18s ease;
	}

	.reset-password-toggle:hover {
		color: #c084fc;
	}

	.reset-field-message {
		margin: 0.5rem 0 0;
	}

	.reset-field-message--error {
		color: #f87171;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.reset-field-message--hint {
		color: #64748b;
		font-size: 0.75rem;
		line-height: 1rem;
	}

	.reset-submit {
		position: relative;
		width: 100%;
		overflow: hidden;
		margin-top: 1.5rem;
		border: 0;
		border-radius: 0.75rem;
		background: linear-gradient(90deg, #a855f7, #ec4899, #f43f5e);
		color: #ffffff;
		padding: 1rem 1.5rem;
		font-family: var(--font-display, inherit);
		font-weight: 700;
		cursor: pointer;
		transition:
			box-shadow 0.3s ease,
			opacity 0.3s ease;
	}

	.reset-submit:hover:not(:disabled) {
		box-shadow: 0 10px 24px rgba(168, 85, 247, 0.5);
	}

	.reset-submit:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.reset-submit__content {
		position: relative;
		z-index: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
	}

	.reset-submit__spinner {
		width: 1.25rem;
		height: 1.25rem;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: #ffffff;
		border-radius: 999px;
		animation: spin 0.8s linear infinite;
	}

	.reset-submit__shine {
		position: absolute;
		inset: 0;
		background: rgba(255, 255, 255, 0.2);
		transform: scaleX(0);
		transform-origin: left;
		transition: transform 0.5s ease;
	}

	.reset-submit:hover:not(:disabled) .reset-submit__shine {
		transform: scaleX(1);
	}

	.reset-back {
		margin-top: 2rem;
		text-align: center;
	}

	.reset-back__link {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		color: #c084fc;
		font-weight: 600;
		text-decoration: none;
		transition: color 0.3s ease;
	}

	.reset-back__link:hover {
		color: #d8b4fe;
	}

	.reset-back__icon {
		width: 1rem;
		height: 1rem;
	}

	@keyframes float {
		0%,
		100% {
			transform: translateY(0) scale(1);
		}
		50% {
			transform: translateY(-20px) scale(1.05);
		}
	}

	@keyframes float-delayed {
		0%,
		100% {
			transform: translateY(0) scale(1);
		}
		50% {
			transform: translateY(30px) scale(0.95);
		}
	}

	@keyframes card-reveal {
		from {
			opacity: 0;
			transform: translateY(40px) scale(0.95);
		}
		to {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
	}

	@keyframes fade-in {
		from {
			opacity: 0;
			transform: translateY(-10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@media (max-width: 520px) {
		.reset-password-panel {
			padding: 2rem 1.25rem;
		}

		.reset-password-title {
			font-size: 1.875rem;
			line-height: 2.25rem;
		}
	}
</style>

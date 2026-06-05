<script lang="ts">
	/**
	 * Forgot Password Page - Svelte 5 January 2026
	 * @version 2.0.0
	 */
	import { onMount } from 'svelte';
	import { forgotPassword } from '$lib/api/auth';
	import { IconMail, IconAlertCircle, IconCheck, IconSend } from '$lib/icons';

	// Svelte 5 state runes
	let email = $state('');
	let errors = $state<Record<string, string[]>>({});
	let generalError = $state('');
	let successMessage = $state('');
	let isLoading = $state(false);
	let isVisible = $state(false);

	onMount(() => {
		isVisible = true;
	});

	async function handleSubmit(e: Event) {
		e.preventDefault();
		errors = {};
		generalError = '';
		successMessage = '';
		isLoading = true;

		try {
			const message = await forgotPassword({ email });
			successMessage = message;
			email = ''; // Clear email field after success
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

<div class="forgot-password-page">
	<!-- Animated gradient background -->
	<div class="page-background"></div>

	<!-- Floating orbs - hidden on small screens to save space -->
	<div class="ambient-orb ambient-orb--primary"></div>
	<div class="ambient-orb ambient-orb--secondary"></div>

	<!-- Forgot password card -->
	<div class={['forgot-password-card', isVisible && 'forgot-password-card--visible']}>
		<!-- Glow effect -->
		<div class="card-glow"></div>

		<div class="card-panel">
			<!-- Header -->
			<div class="card-header">
				<div class="mail-badge">
					<div class="mail-badge__inner">
						<span class="mail-icon mail-icon--mobile">
							<IconMail size={32} />
						</span>
						<span class="mail-icon mail-icon--desktop">
							<IconMail size={40} />
						</span>
					</div>
				</div>
				<h1 class="page-title">Forgot Password?</h1>
				<p class="page-copy">No worries, we'll send you reset instructions</p>
			</div>

			<!-- Success message -->
			{#if successMessage}
				<div class="status-message status-message--success">
					<span class="status-message__icon">
						<IconCheck size={20} />
					</span>
					<p class="status-message__copy">{successMessage}</p>
				</div>
			{/if}

			<!-- General error -->
			{#if generalError}
				<div class="status-message status-message--error">
					<span class="status-message__icon">
						<IconAlertCircle size={20} />
					</span>
					<p class="status-message__copy">{generalError}</p>
				</div>
			{/if}

			<!-- Forgot password form -->
			<form onsubmit={handleSubmit} class="reset-form">
				<!-- Email field -->
				<div class="field">
					<label for="email" class="field-label">Email Address</label>
					<div class="input-shell">
						<div class="input-icon">
							<IconMail size={20} />
						</div>
						<input
							id="email"
							name="email"
							autocomplete="email"
							type="email"
							bind:value={email}
							required
							class={['email-input', errors.email && 'email-input--invalid']}
							placeholder="you@example.com"
						/>
					</div>
					{#if errors.email}
						<p class="field-error">{errors.email[0]}</p>
					{/if}
				</div>

				<!-- Submit button -->
				<button type="submit" disabled={isLoading} class="submit-button">
					<span class="submit-button__content">
						{#if isLoading}
							<div class="submit-spinner"></div>
							<span>Sending...</span>
						{:else}
							<IconSend size={20} />
							<span>Send Reset Link</span>
						{/if}
					</span>
					<div class="submit-button__sheen"></div>
				</button>
			</form>

			<!-- Back to login link -->
			<div class="back-link-row">
				<a href="/login" class="back-link">
					<svg
						aria-hidden="true"
						class="back-link__icon"
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
	.forgot-password-page {
		position: relative;
		display: flex;
		min-height: 100vh;
		align-items: center;
		justify-content: center;
		overflow: hidden;
		padding: 2rem 1rem;
	}

	.page-background {
		position: absolute;
		inset: 0;
		background: linear-gradient(135deg, #020617 0%, rgba(69, 26, 3, 0.3) 50%, #020617 100%);
	}

	.ambient-orb {
		position: absolute;
		display: none;
		width: min(24rem, 42vw);
		aspect-ratio: 1;
		border-radius: 999px;
		filter: blur(64px);
	}

	.ambient-orb--primary {
		top: 5rem;
		left: 2.5rem;
		background: rgba(245, 158, 11, 0.1);
		animation: float 8s ease-in-out infinite;
	}

	.ambient-orb--secondary {
		right: 2.5rem;
		bottom: 5rem;
		background: rgba(234, 179, 8, 0.1);
		animation: float-delayed 10s ease-in-out infinite;
	}

	.forgot-password-card {
		position: relative;
		z-index: 1;
		width: 100%;
		max-width: 28rem;
		opacity: 0;
	}

	.forgot-password-card--visible {
		animation: card-reveal 0.6s ease-out forwards;
	}

	.card-glow {
		position: absolute;
		inset: -0.25rem;
		border-radius: 1.5rem;
		background: linear-gradient(90deg, #f59e0b 0%, #eab308 50%, #f97316 100%);
		filter: blur(24px);
		opacity: 0.2;
	}

	.card-panel {
		position: relative;
		padding: 1.5rem;
		border: 1px solid rgba(245, 158, 11, 0.2);
		border-radius: 1rem;
		background: rgba(15, 23, 42, 0.9);
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.55);
		backdrop-filter: blur(24px);
	}

	.card-header {
		margin-bottom: 1.5rem;
		text-align: center;
	}

	.mail-badge {
		display: inline-flex;
		width: 4rem;
		height: 4rem;
		align-items: center;
		justify-content: center;
		margin-bottom: 1rem;
		padding: 2px;
		border-radius: 0.75rem;
		background: linear-gradient(135deg, #f59e0b 0%, #f97316 100%);
	}

	.mail-badge__inner {
		display: flex;
		width: 100%;
		height: 100%;
		align-items: center;
		justify-content: center;
		border-radius: 0.75rem;
		background: #0f172a;
		color: #fbbf24;
	}

	.mail-icon {
		display: flex;
		color: #fbbf24;
	}

	.mail-icon--desktop {
		display: none;
	}

	.page-title {
		margin: 0 0 0.5rem;
		background: linear-gradient(90deg, #fcd34d 0%, #fde047 50%, #fdba74 100%);
		background-clip: text;
		color: transparent;
		font-family: var(--font-heading, inherit);
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 1.15;
	}

	.page-copy {
		margin: 0;
		color: #94a3b8;
	}

	.status-message {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
		padding: 1rem;
		border: 1px solid;
		border-radius: 0.75rem;
		animation: fade-in 0.3s ease-out;
	}

	.status-message--success {
		border-color: rgba(16, 185, 129, 0.3);
		background: rgba(16, 185, 129, 0.1);
		color: #6ee7b7;
	}

	.status-message--error {
		border-color: rgba(239, 68, 68, 0.3);
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
	}

	.status-message__icon {
		display: flex;
		flex: 0 0 auto;
		margin-top: 0.125rem;
	}

	.status-message__copy {
		margin: 0;
		color: currentColor;
		font-size: 0.875rem;
		line-height: 1.45;
	}

	.reset-form {
		display: grid;
		gap: 1.5rem;
	}

	.field-label {
		display: block;
		margin-bottom: 0.5rem;
		color: #cbd5e1;
		font-size: 0.875rem;
		font-weight: 600;
	}

	.input-shell {
		position: relative;
	}

	.input-icon {
		position: absolute;
		top: 50%;
		left: 1rem;
		display: flex;
		color: #64748b;
		transform: translateY(-50%);
		pointer-events: none;
	}

	.email-input {
		width: 100%;
		min-height: 44px;
		padding: 0.875rem 1rem 0.875rem 3rem;
		border: 1px solid #334155;
		border-radius: 0.75rem;
		background: rgba(30, 41, 59, 0.5);
		color: #fff;
		font-size: 1rem;
		transition:
			border-color 0.3s ease,
			box-shadow 0.3s ease,
			background-color 0.3s ease;
	}

	.email-input::placeholder {
		color: #64748b;
	}

	.email-input:focus {
		border-color: #f59e0b;
		outline: none;
		box-shadow: 0 0 0 4px rgba(251, 191, 36, 0.1);
	}

	.email-input--invalid {
		border-color: #ef4444;
	}

	.field-error {
		margin: 0.5rem 0 0;
		color: #f87171;
		font-size: 0.875rem;
		line-height: 1.4;
	}

	.submit-button {
		position: relative;
		width: 100%;
		overflow: hidden;
		padding: 1rem 1.5rem;
		border: 0;
		border-radius: 0.75rem;
		background: linear-gradient(90deg, #f59e0b 0%, #eab308 50%, #f97316 100%);
		color: #0f172a;
		cursor: pointer;
		font-family: var(--font-heading, inherit);
		font-weight: 700;
		transition:
			box-shadow 0.3s ease,
			opacity 0.3s ease;
	}

	.submit-button:hover:not(:disabled) {
		box-shadow: 0 10px 15px -3px rgba(245, 158, 11, 0.5);
	}

	.submit-button:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.submit-button__content {
		position: relative;
		z-index: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
	}

	.submit-button__sheen {
		position: absolute;
		inset: 0;
		background: rgba(255, 255, 255, 0.2);
		transform: scaleX(0);
		transform-origin: left;
		transition: transform 0.5s ease;
	}

	.submit-button:hover:not(:disabled) .submit-button__sheen {
		transform: scaleX(1);
	}

	.submit-spinner {
		width: 1.25rem;
		height: 1.25rem;
		border: 2px solid rgba(15, 23, 42, 0.3);
		border-top-color: #0f172a;
		border-radius: 999px;
		animation: spin 0.8s linear infinite;
	}

	.back-link-row {
		margin-top: 1.5rem;
		text-align: center;
	}

	.back-link {
		display: inline-flex;
		min-height: 44px;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding-inline: 1rem;
		color: #fbbf24;
		font-weight: 600;
		text-decoration: none;
		transition: color 0.3s ease;
	}

	.back-link:hover {
		color: #fcd34d;
	}

	.back-link__icon {
		width: 1rem;
		height: 1rem;
	}

	/* Floating animations */
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

	/* Card reveal animation */
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

	/* Fade in animation */
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

	@media (min-width: 640px) {
		.forgot-password-page {
			padding: 3rem 1.5rem;
		}

		.ambient-orb {
			display: block;
		}

		.card-glow,
		.card-panel {
			border-radius: 1.5rem;
		}

		.card-panel {
			padding: 2.5rem;
		}

		.card-header {
			margin-bottom: 2rem;
		}

		.mail-badge {
			width: 5rem;
			height: 5rem;
			margin-bottom: 1.5rem;
			border-radius: 1rem;
		}

		.mail-badge__inner {
			border-radius: 1rem;
		}

		.mail-icon--mobile {
			display: none;
		}

		.mail-icon--desktop {
			display: flex;
		}

		.page-title {
			margin-bottom: 0.75rem;
			font-size: 2.25rem;
		}

		.back-link-row {
			margin-top: 2rem;
		}
	}
</style>

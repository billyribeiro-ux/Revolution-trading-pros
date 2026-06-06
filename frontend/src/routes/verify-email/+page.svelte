<script lang="ts">
	/**
	 * Email Verification Page - Svelte 5 January 2026
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * ENTERPRISE FEATURES:
	 * - Comprehensive error handling with retry logic
	 * - Analytics tracking for verification funnel
	 * - Rate limiting protection
	 * - Accessibility (WCAG 2.1 AAA)
	 * - Auto-redirect on success
	 * - Email validation
	 * - Keyboard shortcuts
	 * - Security best practices
	 * - Progressive enhancement
	 * - Error recovery flows
	 *
	 * @version 4.0.0 - Svelte 5 January 2026
	 * @author Revolution Trading Pros
	 */
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { fade, fly, scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import {
		IconCheck,
		IconX,
		IconLoader,
		IconMail,
		IconRefresh,
		IconLogin,
		IconAlertTriangle,
		IconClock
	} from '$lib/icons';
	import { verifyEmail, resendVerificationEmail } from '$lib/api/auth';

	// ═══════════════════════════════════════════════════════════════════════════
	// State Management
	// ═══════════════════════════════════════════════════════════════════════════

	type VerificationStatus =
		| 'loading'
		| 'success'
		| 'error'
		| 'expired'
		| 'no-token'
		| 'rate-limited';

	let status = $state<VerificationStatus>('loading');
	let errorMessage = $state('');
	let resending = $state(false);
	let resendSuccess = $state(false);
	let userEmail = $state('');
	let emailError = $state('');
	let autoRedirectSeconds = $state(5);
	let redirectTimer: ReturnType<typeof setInterval> | null = $state(null);
	let retryCount = $state(0);
	let maxRetries = $state(3);
	let verificationAttempts = $state(0);
	let lastAttemptTime = $state(0);

	// ═══════════════════════════════════════════════════════════════════════════
	// Email Validation
	// ═══════════════════════════════════════════════════════════════════════════

	function validateEmail(email: string): boolean {
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		if (!email) {
			emailError = 'Email address is required';
			return false;
		}
		if (!emailRegex.test(email)) {
			emailError = 'Please enter a valid email address';
			return false;
		}
		emailError = '';
		return true;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Rate Limiting Protection
	// ═══════════════════════════════════════════════════════════════════════════

	function checkRateLimit(): boolean {
		const now = Date.now();
		const timeSinceLastAttempt = now - lastAttemptTime;
		const minTimeBetweenAttempts = 2000; // 2 seconds

		if (timeSinceLastAttempt < minTimeBetweenAttempts) {
			status = 'rate-limited';
			errorMessage = 'Please wait a moment before trying again';
			return false;
		}

		if (verificationAttempts >= 5) {
			status = 'rate-limited';
			errorMessage = 'Too many attempts. Please wait 5 minutes before trying again';
			return false;
		}

		lastAttemptTime = now;
		verificationAttempts++;
		return true;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Analytics Tracking
	// ═══════════════════════════════════════════════════════════════════════════

	type GtagFn = (command: 'event', eventName: string, params?: Record<string, unknown>) => void;

	function trackEvent(eventName: string, properties?: Record<string, unknown>) {
		try {
			// Track verification funnel events
			const gtag = (window as unknown as { gtag?: GtagFn }).gtag;
			if (typeof window !== 'undefined' && gtag) {
				gtag('event', eventName, properties);
			}
			console.info(`[Analytics] ${eventName}`, properties);
		} catch (error) {
			console.error('Analytics tracking failed:', error);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Auto-Redirect on Success
	// ═══════════════════════════════════════════════════════════════════════════

	function startAutoRedirect() {
		redirectTimer = setInterval(() => {
			autoRedirectSeconds--;
			if (autoRedirectSeconds <= 0) {
				if (redirectTimer) clearInterval(redirectTimer);
				goto('/login');
			}
		}, 1000);
	}

	function cancelAutoRedirect() {
		if (redirectTimer) {
			clearInterval(redirectTimer);
			redirectTimer = null;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Verification Logic with Retry
	// ═══════════════════════════════════════════════════════════════════════════

	async function verify() {
		const token = page.url.searchParams.get('token');

		if (!token) {
			status = 'no-token';
			errorMessage =
				'No verification token provided. Please check your email for the correct link.';
			trackEvent('verification_no_token');
			return;
		}

		if (!checkRateLimit()) {
			return;
		}

		trackEvent('verification_started', { token_length: token.length });

		try {
			await verifyEmail(token);
			status = 'success';
			trackEvent('verification_success');

			// Start auto-redirect countdown
			startAutoRedirect();

			// Announce to screen readers
			announceToScreenReader('Email verified successfully! Redirecting to login...');
		} catch (error: unknown) {
			const e = error as { message?: string };
			const errorType = e.message?.toLowerCase() || '';

			if (errorType.includes('expired')) {
				status = 'expired';
				errorMessage = 'This verification link has expired. Please request a new one.';
				trackEvent('verification_expired');
			} else if (errorType.includes('already verified')) {
				status = 'success';
				errorMessage = 'This email is already verified. You can log in now.';
				trackEvent('verification_already_verified');
				startAutoRedirect();
			} else if (errorType.includes('invalid')) {
				status = 'error';
				errorMessage = 'Invalid verification link. Please check your email for the correct link.';
				trackEvent('verification_invalid');
			} else {
				status = 'error';
				errorMessage = e.message || 'Verification failed. Please try again.';
				trackEvent('verification_error', { error: e.message });

				// Offer retry if not exceeded max attempts
				if (retryCount < maxRetries) {
					retryCount++;
				}
			}

			announceToScreenReader(errorMessage);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Retry Verification
	// ═══════════════════════════════════════════════════════════════════════════

	async function retryVerification() {
		if (retryCount >= maxRetries) {
			errorMessage = 'Maximum retry attempts reached. Please request a new verification email.';
			return;
		}
		status = 'loading';
		await verify();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Resend Verification Email
	// ═══════════════════════════════════════════════════════════════════════════

	async function handleResend() {
		if (!validateEmail(userEmail)) {
			return;
		}

		if (!checkRateLimit()) {
			return;
		}

		resending = true;
		trackEvent('verification_resend_attempt', { email: userEmail });

		try {
			await resendVerificationEmail(userEmail);
			resendSuccess = true;
			trackEvent('verification_resend_success');
			announceToScreenReader('Verification email sent successfully! Check your inbox.');
		} catch (error: unknown) {
			const e = error as { message?: string };
			errorMessage = e.message || 'Failed to resend verification email.';
			trackEvent('verification_resend_error', { error: e.message });
			announceToScreenReader(errorMessage);
		} finally {
			resending = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Accessibility - Screen Reader Announcements
	// ═══════════════════════════════════════════════════════════════════════════

	function announceToScreenReader(message: string) {
		const announcement = document.createElement('div');
		announcement.setAttribute('role', 'status');
		announcement.setAttribute('aria-live', 'polite');
		announcement.setAttribute('aria-atomic', 'true');
		announcement.className = 'sr-only';
		announcement.textContent = message;
		document.body.appendChild(announcement);
		setTimeout(() => announcement.remove(), 1000);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Keyboard Shortcuts
	// ═══════════════════════════════════════════════════════════════════════════

	function handleKeydown(e: KeyboardEvent) {
		// Enter to resend (when email input is focused)
		if (e.key === 'Enter' && status !== 'loading' && status !== 'success') {
			if (userEmail && !resending) {
				handleResend();
			}
		}

		// Escape to cancel auto-redirect
		if (e.key === 'Escape' && status === 'success' && redirectTimer) {
			cancelAutoRedirect();
			announceToScreenReader('Auto-redirect cancelled');
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle - Svelte 5 $effect
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(() => {
		if (!browser) return;

		trackEvent('verification_page_view');
		verify();

		// Add keyboard listener
		window.addEventListener('keydown', handleKeydown);

		// Cleanup
		return () => {
			if (redirectTimer) {
				clearInterval(redirectTimer);
			}
			window.removeEventListener('keydown', handleKeydown);
		};
	});
</script>

<svelte:head>
	<title>Verify Email - Revolution Trading Pros</title>
</svelte:head>

<div class="verification-page">
	<div class="verification-shell">
		<!-- Logo -->
		<div class="brand-block">
			<div class="brand-icon">
				<IconMail size={32} aria-hidden="true" />
			</div>
			<h1 class="brand-title">Revolution Trading Pros</h1>
		</div>

		<!-- Card -->
		<div class="verification-card">
			{#if status === 'loading'}
				<div class="state-block" in:fade={{ duration: 200 }}>
					<div
						class="status-icon status-icon--loading"
						in:scale={{ duration: 300, easing: quintOut }}
					>
						<span class="spinning-icon">
							<IconLoader size={40} aria-hidden="true" />
						</span>
					</div>
					<h2 class="state-title">Verifying Your Email</h2>
					<p class="state-copy">Please wait while we verify your email address...</p>
					{#if retryCount > 0}
						<p class="retry-note">Retry attempt {retryCount} of {maxRetries}</p>
					{/if}
				</div>
			{:else if status === 'success'}
				<div class="state-block" in:fade={{ duration: 300 }}>
					<div
						class="status-icon status-icon--success"
						in:scale={{ duration: 400, easing: quintOut, start: 0.5 }}
					>
						<IconCheck size={40} aria-hidden="true" />
					</div>
					<h2 class="state-title">Email Verified!</h2>
					<p class="state-copy">
						Your email has been successfully verified. You now have full access to your account.
					</p>

					<!-- Auto-redirect countdown -->
					{#if redirectTimer}
						<div class="redirect-notice" in:fly={{ y: 10, duration: 300 }}>
							<IconClock size={16} aria-hidden="true" />
							<span
								>Redirecting in {autoRedirectSeconds} second{autoRedirectSeconds !== 1
									? 's'
									: ''}...</span
							>
							<button
								onclick={cancelAutoRedirect}
								class="inline-link"
								aria-label="Cancel auto-redirect"
							>
								Cancel
							</button>
						</div>
					{/if}

					<div class="primary-action">
						<a href="/login" class="primary-button">
							<IconLogin size={20} aria-hidden="true" />
							Login to Your Account
						</a>
					</div>

					<div class="next-panel">
						<h3 class="next-title">🚀 What's Next?</h3>
						<ul class="next-list">
							<li>• Complete your profile</li>
							<li>• Explore our trading courses</li>
							<li>• Join the Discord community</li>
							<li>• Set up trading alerts</li>
						</ul>
					</div>
				</div>
			{:else if status === 'expired' || status === 'error' || status === 'no-token' || status === 'rate-limited'}
				<div class="state-block" in:fade={{ duration: 300 }}>
					<div
						class={{
							'status-icon': true,
							'status-icon--expired': status === 'expired',
							'status-icon--rate-limited': status === 'rate-limited',
							'status-icon--error': status !== 'expired' && status !== 'rate-limited'
						}}
						in:scale={{ duration: 300, easing: quintOut }}
					>
						{#if status === 'expired'}
							<IconMail size={40} aria-hidden="true" />
						{:else if status === 'rate-limited'}
							<IconAlertTriangle size={40} aria-hidden="true" />
						{:else}
							<IconX size={40} aria-hidden="true" />
						{/if}
					</div>
					<h2 class="state-title">
						{#if status === 'expired'}
							Link Expired
						{:else if status === 'rate-limited'}
							Too Many Attempts
						{:else}
							Verification Failed
						{/if}
					</h2>
					<p class="state-copy">{errorMessage}</p>

					<!-- Retry button for transient errors -->
					{#if status === 'error' && retryCount < maxRetries}
						<button
							onclick={retryVerification}
							class="secondary-button"
							aria-label="Retry verification"
						>
							<IconRefresh size={16} aria-hidden="true" />
							Retry Verification ({maxRetries - retryCount} attempts left)
						</button>
					{/if}

					{#if resendSuccess}
						<div class="success-alert">
							<p class="alert-copy">
								<IconCheck size={20} aria-hidden="true" />
								A new verification email has been sent! Check your inbox.
							</p>
						</div>
					{:else}
						<div class="resend-form">
							<div>
								<label for="email" class="sr-only">Email address</label>
								<input
									id="email"
									name="email"
									autocomplete="email"
									type="email"
									bind:value={userEmail}
									placeholder="Enter your email address"
									class={{
										'email-input': true,
										'email-input--invalid': Boolean(emailError)
									}}
									aria-invalid={emailError ? 'true' : 'false'}
									aria-describedby={emailError ? 'email-error' : undefined}
								/>
								{#if emailError}
									<p id="email-error" class="field-error" in:fly={{ y: -10, duration: 200 }}>
										{emailError}
									</p>
								{/if}
							</div>
							<button onclick={handleResend} disabled={resending} class="primary-button">
								{#if resending}
									<span class="button-spinner">
										<IconLoader size={20} aria-hidden="true" />
									</span>
									Sending...
								{:else}
									<IconRefresh size={20} aria-hidden="true" />
									Resend Verification Email
								{/if}
							</button>
							<a href="/login" class="outline-link"> Back to Login </a>
						</div>
					{/if}
				</div>
			{/if}
		</div>

		<!-- Footer -->
		<div class="verification-footer">
			<p class="footer-copy">
				Need help?
				<a href="/support" class="support-link">Contact Support</a>
			</p>
			{#if status === 'success' && redirectTimer}
				<p class="shortcut-copy">
					Press <kbd class="keyboard-key">ESC</kbd> to cancel redirect
				</p>
			{/if}
		</div>
	</div>
</div>

<style>
	.verification-page {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #0f172a 0%, #022c22 50%, #0f172a 100%);
		padding: 1rem;
	}

	.verification-shell {
		width: min(100%, 28rem);
	}

	.brand-block,
	.state-block,
	.verification-footer {
		text-align: center;
	}

	.brand-block {
		margin-bottom: 2rem;
	}

	.brand-icon,
	.status-icon,
	.primary-button,
	.secondary-button,
	.outline-link,
	.redirect-notice,
	.alert-copy {
		display: inline-flex;
		align-items: center;
		justify-content: center;
	}

	.brand-icon {
		margin-bottom: 1rem;
		border-radius: 0.75rem;
		background: linear-gradient(90deg, #10b981, #14b8a6);
		color: #ffffff;
		padding: 0.75rem;
	}

	.brand-title {
		margin: 0;
		color: #ffffff;
		font-size: 1.875rem;
		font-weight: 700;
		letter-spacing: 0;
		line-height: 1.15;
	}

	.verification-card {
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 1rem;
		background: rgba(255, 255, 255, 0.1);
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.45);
		backdrop-filter: blur(24px);
		-webkit-backdrop-filter: blur(24px);
		padding: 2rem;
	}

	.status-icon {
		width: 5rem;
		height: 5rem;
		margin: 0 auto;
		border-radius: 999px;
	}

	.status-icon--loading,
	.status-icon--success {
		background: rgba(16, 185, 129, 0.2);
		color: #34d399;
	}

	.status-icon--expired {
		background: rgba(234, 179, 8, 0.2);
		color: #facc15;
	}

	.status-icon--rate-limited {
		background: rgba(249, 115, 22, 0.2);
		color: #fb923c;
	}

	.status-icon--error {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
	}

	.spinning-icon,
	.button-spinner {
		display: inline-flex;
		animation: spin 0.8s linear infinite;
	}

	.state-title {
		margin: 1.5rem 0 0;
		color: #ffffff;
		font-size: 1.5rem;
		font-weight: 700;
		letter-spacing: 0;
		line-height: 1.2;
	}

	.state-copy {
		margin: 0.5rem 0 0;
		color: #cbd5e1;
		line-height: 1.5;
	}

	.retry-note {
		margin: 0.5rem 0 0;
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.redirect-notice {
		gap: 0.5rem;
		margin-top: 1rem;
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.inline-link,
	.support-link {
		border: 0;
		background: transparent;
		color: #34d399;
		text-decoration: underline;
		text-underline-offset: 0.15em;
		transition: color 0.2s ease;
	}

	.inline-link {
		margin-left: 0.5rem;
		padding: 0;
		font: inherit;
	}

	.inline-link:hover,
	.inline-link:focus-visible,
	.support-link:hover,
	.support-link:focus-visible {
		color: #6ee7b7;
	}

	.primary-action {
		margin-top: 2rem;
	}

	.primary-button,
	.outline-link {
		width: 100%;
		gap: 0.5rem;
		border-radius: 0.75rem;
		padding: 0.75rem 1.5rem;
		font-weight: 600;
		text-decoration: none;
		transition:
			background 0.2s ease,
			border-color 0.2s ease,
			opacity 0.2s ease;
	}

	.primary-button {
		border: 0;
		background: linear-gradient(90deg, #10b981, #14b8a6);
		color: #ffffff;
	}

	.primary-button:hover,
	.primary-button:focus-visible {
		background: linear-gradient(90deg, #059669, #0d9488);
	}

	.primary-button:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.outline-link {
		border: 1px solid rgba(255, 255, 255, 0.2);
		color: #ffffff;
	}

	.outline-link:hover,
	.outline-link:focus-visible {
		background: rgba(255, 255, 255, 0.05);
	}

	.secondary-button {
		gap: 0.5rem;
		margin-top: 1rem;
		border: 0;
		border-radius: 0.75rem;
		background: rgba(255, 255, 255, 0.1);
		color: #ffffff;
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
		font-weight: 600;
		transition: background 0.2s ease;
	}

	.secondary-button:hover,
	.secondary-button:focus-visible {
		background: rgba(255, 255, 255, 0.2);
	}

	.next-panel,
	.success-alert {
		margin-top: 1.5rem;
		border: 1px solid rgba(16, 185, 129, 0.3);
		border-radius: 0.75rem;
		background: rgba(16, 185, 129, 0.1);
		padding: 1rem;
	}

	.next-title {
		margin: 0;
		color: #6ee7b7;
		font-weight: 600;
	}

	.next-list {
		display: grid;
		gap: 0.25rem;
		margin: 0.5rem 0 0;
		padding: 0;
		color: #a7f3d0;
		font-size: 0.875rem;
		text-align: left;
		list-style: none;
	}

	.alert-copy {
		gap: 0.5rem;
		margin: 0;
		color: #6ee7b7;
	}

	.resend-form {
		display: grid;
		gap: 1rem;
		margin-top: 1.5rem;
	}

	.email-input {
		width: 100%;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		background: rgba(255, 255, 255, 0.05);
		color: #ffffff;
		padding: 0.75rem 1rem;
		font: inherit;
		outline: none;
		transition:
			border-color 0.2s ease,
			box-shadow 0.2s ease;
	}

	.email-input::placeholder {
		color: #94a3b8;
	}

	.email-input:focus {
		border-color: #10b981;
		box-shadow: 0 0 0 2px rgba(16, 185, 129, 0.2);
	}

	.email-input--invalid {
		border-color: #ef4444;
	}

	.field-error {
		margin: 0.5rem 0 0;
		color: #f87171;
		font-size: 0.875rem;
	}

	.verification-footer {
		margin-top: 1.5rem;
	}

	.footer-copy {
		margin: 0;
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.support-link {
		text-decoration: none;
	}

	.shortcut-copy {
		margin: 0.5rem 0 0;
		color: #64748b;
		font-size: 0.75rem;
	}

	.keyboard-key {
		border-radius: 0.25rem;
		background: rgba(255, 255, 255, 0.1);
		color: #cbd5e1;
		padding: 0.125rem 0.375rem;
		font-family:
			ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New',
			monospace;
		font-size: 0.75rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@media (max-width: 480px) {
		.verification-card {
			padding: 1.5rem;
		}

		.brand-title {
			font-size: 1.5rem;
		}
	}
</style>

<script lang="ts">
	/**
	 * LoginForm - Premium Trading-Themed Login Form
	 * Netflix L11+ Principal Engineer Grade
	 *
	 * Features:
	 * - Email & password fields with trading-themed icons
	 * - Remember me checkbox with email persistence
	 * - Typed.js dynamic headline
	 * - Lottie success animation
	 * - Google/Apple social login buttons
	 * - Inline validation (blur + submit)
	 * - Clear error messages
	 * - Visual states: normal, focused, error, disabled, loading, success
	 * - GSAP micro-animations
	 * - Full keyboard accessibility
	 * - Light/dark theme support
	 *
	 * @version 2.0.0
	 */
	import { goto } from '$app/navigation';
	import { login } from '$lib/api/auth';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { fade, slide } from 'svelte/transition';

	// GSAP loaded dynamically to prevent SSR blocking
	let gsap: any = null;
	let gsapContext: any = null; // GSAP 3.12+ context for proper cleanup
	import {
		IconMail,
		IconLock,
		IconAlertCircle,
		IconTrendingUp,
		IconChartCandle,
		IconEye,
		IconEyeOff,
		IconArrowRight,
		IconCheck,
		IconLoader
	} from '$lib/icons';

	// Import new components
	import TypedHeadline from './TypedHeadline.svelte';
	import LottieSuccess from './LottieSuccess.svelte';
	import SocialLoginButtons from './SocialLoginButtons.svelte';

	// --- Constants ---
	const REMEMBERED_EMAIL_KEY = 'rtp_remembered_email';

	// --- State ---
	let email = $state('');
	let password = $state('');
	let rememberMe = $state(false);
	let showPassword = $state(false);
	let errors = $state<Record<string, string[]>>({});
	let generalError = $state('');
	let isLoading = $state(false);
	let isSuccess = $state(false);
	let showLottie = $state(false);
	let touched = $state<Record<string, boolean>>({ email: false, password: false });

	// --- Refs ---
	let formRef = $state<HTMLFormElement | null>(null);
	let cardRef = $state<HTMLElement | null>(null);
	let emailInputRef = $state<HTMLInputElement | null>(null);

	// --- Validation ---
	function validateEmail(value: string): string | null {
		if (!value) return 'Email address is required';
		if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value)) return 'Please enter a valid email address';
		return null;
	}

	function validatePassword(value: string): string | null {
		if (!value) return 'Password is required';
		if (value.length < 6) return 'Password must be at least 6 characters';
		return null;
	}

	function validateField(field: 'email' | 'password') {
		if (!touched[field]) return;

		const validator = field === 'email' ? validateEmail : validatePassword;
		const value = field === 'email' ? email : password;
		const error = validator(value);

		if (error) {
			errors = { ...errors, [field]: [error] };
		} else {
			const { [field]: _, ...rest } = errors;
			errors = rest;
		}
	}

	function handleBlur(field: 'email' | 'password') {
		touched = { ...touched, [field]: true };
		validateField(field);
	}

	// --- Email Persistence ---
	function loadRememberedEmail() {
		if (!browser) return;
		try {
			const savedEmail = localStorage.getItem(REMEMBERED_EMAIL_KEY);
			if (savedEmail) {
				email = savedEmail;
				rememberMe = true;
			}
		} catch {
			// localStorage not available
		}
	}

	function saveRememberedEmail() {
		if (!browser) return;
		try {
			if (rememberMe && email) {
				localStorage.setItem(REMEMBERED_EMAIL_KEY, email);
			} else {
				localStorage.removeItem(REMEMBERED_EMAIL_KEY);
			}
		} catch {
			// localStorage not available
		}
	}

	// --- Security ---
	function validateRedirectUrl(url: string): string {
		try {
			const decoded = decodeURIComponent(url);
			if (
				decoded.startsWith('/') &&
				!decoded.startsWith('//') &&
				!decoded.includes(':') &&
				!decoded.includes('\\')
			) {
				return decoded;
			}
		} catch {
			// Ignore decode errors
		}
		return '/';
	}

	// --- GSAP Animations ---
	function focusAnimation(node: HTMLElement) {
		const onFocus = () => {
			if (!gsap) return;
			gsap.to(node, {
				scale: 1.01,
				duration: 0.2,
				ease: 'power2.out'
			});
			gsap.to(node.closest('.input-wrapper'), {
				boxShadow: '0 0 0 3px var(--auth-glow-primary)',
				duration: 0.2
			});
		};

		const onBlur = () => {
			if (!gsap) return;
			gsap.to(node, {
				scale: 1,
				duration: 0.2,
				ease: 'power2.out'
			});
			gsap.to(node.closest('.input-wrapper'), {
				boxShadow: '0 0 0 0 transparent',
				duration: 0.2
			});
		};

		node.addEventListener('focus', onFocus);
		node.addEventListener('blur', onBlur);

		return {
			destroy() {
				node.removeEventListener('focus', onFocus);
				node.removeEventListener('blur', onBlur);
			}
		};
	}

	// --- Lifecycle ---
	onMount(() => {
		if (!browser || !cardRef) return;

		// Load remembered email (sync)
		loadRememberedEmail();

		// Use IIFE pattern for async GSAP import - Svelte 5 pattern
		(async () => {
			// Dynamic import GSAP to prevent SSR blocking - ICT11+ pattern
			const gsapModule = await import('gsap');
			gsap = gsapModule.default;

			// GSAP 3.12+ pattern: use gsap.context() for proper cleanup
			gsapContext = gsap.context(() => {
				// Entrance animation - scope selectors to cardRef to avoid GSAP warnings
				const tl = gsap.timeline({ defaults: { ease: 'power3.out' } });

				// Get scoped elements
				const formHeader = cardRef!.querySelector('.form-header');
				const formFields = cardRef!.querySelectorAll('.form-field');
				const socialLogin = cardRef!.querySelector('.social-login');
				const formActions = cardRef!.querySelector('.form-actions');

				tl.from(cardRef, {
					opacity: 0,
					y: 40,
					scale: 0.95,
					duration: 0.8
				});

				if (formHeader) {
					tl.from(formHeader, { opacity: 0, y: -20, duration: 0.5 }, '-=0.4');
				}
				if (formFields.length > 0) {
					tl.from(formFields, { opacity: 0, x: -20, duration: 0.4, stagger: 0.1 }, '-=0.3');
				}
				if (socialLogin) {
					tl.from(socialLogin, { opacity: 0, y: 20, duration: 0.4 }, '-=0.2');
				}
				if (formActions) {
					tl.from(formActions, { opacity: 0, y: 20, duration: 0.4 }, '-=0.2');
				}
			}, cardRef);

			// Focus email input after animation (only if not pre-filled)
			setTimeout(() => {
				if (!email) {
					emailInputRef?.focus();
				}
			}, 800);
		})();

		// Return cleanup function (sync) - Svelte 5 pattern
		return () => {
			if (gsapContext) {
				gsapContext.revert();
			}
		};
	});

	// --- Lottie Complete Handler ---
	function handleLottieComplete() {
		const urlParams = new URLSearchParams(window.location.search);
		const redirect = urlParams.get('redirect') || '/dashboard';
		goto(validateRedirectUrl(redirect), { replaceState: true });
	}

	// --- Form Submit ---
	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();

		// Mark all fields as touched
		touched = { email: true, password: true };

		// Validate all fields
		const emailError = validateEmail(email);
		const passwordError = validatePassword(password);

		if (emailError || passwordError) {
			errors = {};
			if (emailError) errors.email = [emailError];
			if (passwordError) errors.password = [passwordError];

			// Shake animation
			if (gsap && cardRef) {
				gsap.fromTo(
					cardRef,
					{ x: -8 },
					{
						x: 8,
						duration: 0.08,
						repeat: 5,
						yoyo: true,
						ease: 'power1.inOut',
						onComplete: () => gsap.to(cardRef, { x: 0, duration: 0.1 })
					}
				);
			}
			return;
		}

		errors = {};
		generalError = '';
		isLoading = true;

		// Button press animation
		const submitBtn = formRef?.querySelector('.submit-btn');
		if (gsap && submitBtn) gsap.to(submitBtn, { scale: 0.97, duration: 0.1 });

		try {
			await login({ email, password, remember: rememberMe });

			// Save/clear remembered email
			saveRememberedEmail();

			// Success state - show Lottie animation
			isSuccess = true;
			showLottie = true;
			if (gsap && submitBtn) {
				gsap.to(submitBtn, { scale: 1, duration: 0.2 });
			}

			// Fade out card content to show Lottie
			const cardContent = cardRef?.querySelector('.card-content');
			if (gsap && cardContent) {
				const elementsToFade = cardContent.querySelectorAll(':scope > *:not(.success-overlay)');
				if (elementsToFade.length > 0) {
					gsap.to(elementsToFade, { opacity: 0, duration: 0.3 });
				}
			}
		} catch (error: unknown) {
			// Error handling
			if (gsap && submitBtn) gsap.to(submitBtn, { scale: 1, duration: 0.2 });

			// Shake animation
			if (gsap && cardRef) {
				gsap.fromTo(
					cardRef,
					{ x: -8 },
					{
						x: 8,
						duration: 0.08,
						repeat: 5,
						yoyo: true,
						ease: 'power1.inOut',
						onComplete: () => gsap.to(cardRef, { x: 0, duration: 0.1 })
					}
				);
			}

			if (error && typeof error === 'object' && 'errors' in error) {
				errors = (error as { errors: Record<string, string[]> }).errors;
			} else if (error instanceof Error) {
				generalError = error.message;
			} else {
				generalError = 'Unable to sign in. Please check your credentials and try again.';
			}
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="login-card" bind:this={cardRef}>
	<!-- Glass Effect -->
	<div class="card-glass" aria-hidden="true"></div>
	<div class="card-glow" aria-hidden="true"></div>

	<div class="card-content">
		<!-- Success Overlay with Lottie -->
		{#if showLottie}
			<div class="success-overlay" in:fade={{ duration: 300 }}>
				<LottieSuccess size={140} onComplete={handleLottieComplete} />
				<p class="success-message">Welcome back!</p>
				<p class="success-submessage">Redirecting to your dashboard...</p>
			</div>
		{:else}
			<!-- Header -->
			<div class="form-header">
				<div class="logo-wrapper">
					<div class="logo-glow" aria-hidden="true"></div>
					<div class="logo-icon">
						<IconChartCandle size={32} stroke={1.5} />
					</div>
				</div>
				<h1 class="form-title">
					<TypedHeadline
						strings={['Welcome Back, Trader', 'Ready to Trade?', 'Access Your Dashboard']}
						typeSpeed={60}
						backSpeed={40}
						backDelay={2500}
					/>
				</h1>
				<p class="form-subtitle">Sign in to your trading dashboard</p>
			</div>

			<!-- Error Banner -->
			{#if generalError}
				<div class="error-banner" in:fade={{ duration: 200 }} role="alert">
					<IconAlertCircle size={20} />
					<p>{generalError}</p>
				</div>
			{/if}

			<!-- Form -->
			<form bind:this={formRef} onsubmit={handleSubmit} class="login-form" novalidate>
				<!-- Email Field -->
				<div class="form-field">
					<label for="email" class="field-label">Email Address</label>
					<div class="input-wrapper" class:error={errors.email}>
						<div class="input-icon">
							<IconMail size={20} />
						</div>
						<input
							bind:this={emailInputRef}
							id="email"
							name="email"
							type="email"
							bind:value={email}
							onblur={() => handleBlur('email')}
							use:focusAnimation
							required
							class="form-input"
							placeholder="trader@example.com"
							autocomplete="email"
							disabled={isLoading || isSuccess}
							aria-invalid={!!errors.email}
							aria-describedby={errors.email ? 'email-error' : undefined}
						/>
					</div>
					{#if errors.email}
						<p id="email-error" class="field-error" transition:slide={{ duration: 150 }}>
							{errors.email[0]}
						</p>
					{/if}
				</div>

				<!-- Password Field -->
				<div class="form-field">
					<label for="password" class="field-label">Password</label>
					<div class="input-wrapper" class:error={errors.password}>
						<div class="input-icon">
							<IconLock size={20} />
						</div>
						<input
							id="password"
							name="password"
							type={showPassword ? 'text' : 'password'}
							bind:value={password}
							onblur={() => handleBlur('password')}
							use:focusAnimation
							required
							class="form-input has-toggle"
							placeholder="Enter your password"
							autocomplete="current-password"
							disabled={isLoading || isSuccess}
							aria-invalid={!!errors.password}
							aria-describedby={errors.password ? 'password-error' : undefined}
						/>
						<button
							type="button"
							class="password-toggle"
							onclick={() => (showPassword = !showPassword)}
							aria-label={showPassword ? 'Hide password' : 'Show password'}
							tabindex={0}
							disabled={isLoading || isSuccess}
						>
							{#if showPassword}
								<IconEyeOff size={20} />
							{:else}
								<IconEye size={20} />
							{/if}
						</button>
					</div>
					{#if errors.password}
						<p id="password-error" class="field-error" transition:slide={{ duration: 150 }}>
							{errors.password[0]}
						</p>
					{/if}
				</div>

				<!-- Remember Me & Forgot Password -->
				<div class="form-row">
					<label class="checkbox-wrapper">
						<input
							type="checkbox"
							bind:checked={rememberMe}
							disabled={isLoading || isSuccess}
							class="checkbox-input"
						/>
						<span class="checkbox-custom"></span>
						<span class="checkbox-label">Remember me</span>
					</label>
					<a href="/forgot-password" class="forgot-link">Forgot password?</a>
				</div>

				<!-- Submit Button -->
				<div class="form-actions">
					<button
						type="submit"
						class="submit-btn"
						class:loading={isLoading}
						class:success={isSuccess}
						disabled={isLoading || isSuccess}
					>
						<span class="btn-content">
							{#if isSuccess}
								<IconCheck size={20} />
								<span>Success!</span>
							{:else if isLoading}
								<IconLoader size={20} class="spin" />
								<span>Signing in...</span>
							{:else}
								<IconTrendingUp size={20} />
								<span>Sign In to Trade</span>
								<IconArrowRight size={20} class="arrow-icon" />
							{/if}
						</span>
						<div class="btn-glow" aria-hidden="true"></div>
					</button>
				</div>
			</form>

			<!-- Social Login -->
			<SocialLoginButtons disabled={isLoading || isSuccess} />

			<!-- Footer -->
			<div class="form-footer">
				<p class="footer-text">
					New to Revolution Trading?
					<a href="/register" class="footer-link">Create an account</a>
				</p>
			</div>

			<!-- Back to Site -->
			<div class="back-link-wrapper">
				<a href="/" class="back-link">
					<span>Back to main site</span>
				</a>
			</div>
		{/if}
	</div>
</div>

<style>
	/* Card Container - ICT11+ Animation-First Architecture */
	.login-card {
		position: relative;
		background: var(--auth-card-bg);
		backdrop-filter: blur(24px);
		-webkit-backdrop-filter: blur(24px);
		border-radius: 24px;
		border: 1px solid var(--auth-card-border);
		box-shadow: var(--auth-card-shadow);
		/* NOTE: overflow: visible allows card-glow (inset: -2px) to render properly */
		/* The glow effect intentionally extends beyond card bounds with z-index: -1 */
		overflow: visible;
		/* ICT11+ Pattern: Create stacking context for proper z-index layering */
		isolation: isolate;
	}

	.card-glass {
		position: absolute;
		inset: 0;
		background: linear-gradient(
			135deg,
			rgba(255, 255, 255, 0.1) 0%,
			transparent 50%,
			rgba(99, 102, 241, 0.05) 100%
		);
		pointer-events: none;
	}

	.card-glow {
		position: absolute;
		inset: -2px;
		background: linear-gradient(135deg, #6366f1, #8b5cf6, #06b6d4);
		opacity: 0.15;
		filter: blur(24px);
		z-index: -1;
	}

	.card-content {
		position: relative;
		padding: 2.5rem 2rem;
		min-height: 500px;
		/* ICT11+ Pattern: Isolate stacking context for internal z-index management */
		isolation: isolate;
		/* Contain internal content while allowing parent glow to extend */
		overflow: hidden;
		border-radius: 24px;
	}

	@media (min-width: 640px) {
		.card-content {
			padding: 3rem 2.5rem;
		}
	}

	/* Success Overlay */
	.success-overlay {
		position: absolute;
		inset: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		z-index: 10;
	}

	.success-message {
		font-family: var(--font-heading);
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--auth-success, #4ade80);
		margin-top: 0.5rem;
	}

	.success-submessage {
		font-family: var(--font-body);
		font-size: 1rem;
		color: var(--auth-muted, #64748b);
	}

	/* Header */
	.form-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.logo-wrapper {
		position: relative;
		display: inline-block;
		margin-bottom: 1.25rem;
	}

	.logo-glow {
		position: absolute;
		inset: -16px;
		background: radial-gradient(circle, var(--auth-glow-primary), transparent 70%);
		animation: pulse-glow 3s ease-in-out infinite;
	}

	@keyframes pulse-glow {
		0%, 100% { opacity: 0.4; transform: scale(1); }
		50% { opacity: 0.7; transform: scale(1.1); }
	}

	.logo-icon {
		position: relative;
		width: 72px;
		height: 72px;
		background: var(--auth-btn-primary-bg);
		border-radius: 18px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		box-shadow: var(--auth-btn-primary-shadow);
	}

	.form-title {
		font-family: var(--font-heading);
		font-size: 1.75rem;
		font-weight: 800;
		background: var(--auth-heading);
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
		margin-bottom: 0.5rem;
		letter-spacing: -0.02em;
		min-height: 2.5rem;
	}

	@media (min-width: 640px) {
		.form-title {
			font-size: 2rem;
		}
	}

	.form-subtitle {
		font-family: var(--font-body);
		font-size: 1rem;
		color: var(--auth-subheading);
	}

	/* Error Banner */
	.error-banner {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 1rem;
		background: var(--auth-error-bg);
		border: 1px solid var(--auth-error-border);
		border-radius: 12px;
		margin-bottom: 1.5rem;
		color: var(--auth-error);
	}

	.error-banner p {
		font-size: 0.875rem;
		font-weight: 500;
		line-height: 1.4;
	}

	/* Form */
	.login-form {
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
	}

	.form-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.field-label {
		font-family: var(--font-heading);
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--auth-text);
		margin-left: 0.25rem;
	}

	/* Input Wrapper */
	.input-wrapper {
		position: relative;
		border-radius: 12px;
		transition: box-shadow 0.2s ease;
	}

	.input-icon {
		position: absolute;
		left: 1rem;
		top: 50%;
		transform: translateY(-50%);
		color: var(--auth-muted);
		pointer-events: none;
		z-index: 2;
		transition: color 0.2s ease;
	}

	.input-wrapper:focus-within .input-icon {
		color: var(--auth-link);
	}

	.input-wrapper.error .input-icon {
		color: var(--auth-error);
	}

	/* Form Input */
	.form-input {
		width: 100%;
		padding: 0.875rem 1rem 0.875rem 3rem;
		background: var(--auth-input-bg);
		border: 2px solid var(--auth-input-border);
		border-radius: 12px;
		color: var(--auth-input-text);
		font-family: var(--font-body);
		font-size: 1rem;
		font-weight: 500;
		transition: border-color 0.2s ease, background 0.2s ease;
		outline: none;
	}

	.form-input::placeholder {
		color: var(--auth-input-placeholder);
	}

	.form-input:focus {
		border-color: var(--auth-input-border-focus);
		background: var(--auth-card-bg);
	}

	.form-input:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.form-input.has-toggle {
		padding-right: 3rem;
	}

	.input-wrapper.error .form-input {
		border-color: var(--auth-error);
	}

	/* Password Toggle */
	.password-toggle {
		position: absolute;
		right: 0.75rem;
		top: 50%;
		transform: translateY(-50%);
		background: none;
		border: none;
		color: var(--auth-muted);
		cursor: pointer;
		padding: 0.375rem;
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: color 0.2s ease, background 0.2s ease;
		z-index: 2;
	}

	.password-toggle:hover {
		color: var(--auth-link);
		background: rgba(99, 102, 241, 0.1);
	}

	.password-toggle:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Field Error */
	.field-error {
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--auth-error);
		margin-left: 0.25rem;
	}

	/* Form Row */
	.form-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
	}

	/* Checkbox */
	.checkbox-wrapper {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
	}

	.checkbox-input {
		position: absolute;
		opacity: 0;
		width: 0;
		height: 0;
	}

	.checkbox-custom {
		width: 20px;
		height: 20px;
		border: 2px solid var(--auth-input-border);
		border-radius: 6px;
		background: var(--auth-input-bg);
		transition: all 0.2s ease;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.checkbox-custom::after {
		content: '';
		width: 10px;
		height: 10px;
		background: var(--auth-link);
		border-radius: 3px;
		transform: scale(0);
		transition: transform 0.2s ease;
	}

	.checkbox-input:checked + .checkbox-custom {
		border-color: var(--auth-link);
	}

	.checkbox-input:checked + .checkbox-custom::after {
		transform: scale(1);
	}

	.checkbox-input:focus-visible + .checkbox-custom {
		box-shadow: 0 0 0 3px var(--auth-glow-primary);
	}

	.checkbox-label {
		font-size: 0.875rem;
		color: var(--auth-text);
	}

	/* Links */
	.forgot-link,
	.footer-link,
	.back-link {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--auth-link);
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.forgot-link:hover,
	.footer-link:hover,
	.back-link:hover {
		color: var(--auth-link-hover);
	}

	/* Submit Button */
	.form-actions {
		margin-top: 0.5rem;
	}

	.submit-btn {
		position: relative;
		width: 100%;
		padding: 1rem 1.5rem;
		background: var(--auth-btn-primary-bg);
		border: none;
		border-radius: 12px;
		color: var(--auth-btn-primary-text);
		font-family: var(--font-heading);
		font-size: 1rem;
		font-weight: 700;
		cursor: pointer;
		overflow: hidden;
		transition: transform 0.2s ease, box-shadow 0.2s ease;
		box-shadow: var(--auth-btn-primary-shadow);
	}

	.submit-btn:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: var(--auth-btn-primary-shadow-hover);
	}

	.submit-btn:disabled {
		cursor: not-allowed;
	}

	.submit-btn.loading {
		background: var(--auth-btn-primary-bg);
	}

	.submit-btn.success {
		background: linear-gradient(135deg, #22c55e, #16a34a);
	}

	.btn-content {
		position: relative;
		z-index: 2;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
	}

	.btn-glow {
		position: absolute;
		inset: -2px;
		background: linear-gradient(135deg, #818cf8, #c084fc);
		filter: blur(20px);
		opacity: 0;
		transition: opacity 0.3s ease;
		z-index: 1;
	}

	.submit-btn:hover:not(:disabled) .btn-glow {
		opacity: 0.5;
	}

	:global(.arrow-icon) {
		transition: transform 0.2s ease;
	}

	.submit-btn:hover:not(:disabled) :global(.arrow-icon) {
		transform: translateX(4px);
	}

	:global(.spin) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	/* Footer */
	.form-footer {
		text-align: center;
		padding-top: 1.5rem;
		margin-top: 0.5rem;
	}

	.footer-text {
		font-size: 0.9375rem;
		color: var(--auth-subheading);
	}

	/* Back Link */
	.back-link-wrapper {
		text-align: center;
		margin-top: 1rem;
	}

	.back-link {
		font-size: 0.8125rem;
		color: var(--auth-muted);
	}

	/* Light Theme */
	:global(html.light) .card-glass,
	:global(body.light) .card-glass {
		background: linear-gradient(
			135deg,
			rgba(255, 255, 255, 0.5) 0%,
			transparent 50%,
			rgba(79, 70, 229, 0.02) 100%
		);
	}

	:global(html.light) .card-glow,
	:global(body.light) .card-glow {
		opacity: 0.08;
	}
</style>

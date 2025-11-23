<script lang="ts">
	import { goto } from '$app/navigation';
	import { login } from '$lib/api/auth';
	import {
		IconLock,
		IconMail,
		IconAlertCircle,
		IconLogin,
		IconArrowRight
	} from '@tabler/icons-svelte';
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import gsap from 'gsap';

	let email = '';
	let password = '';
	let errors: Record<string, string[]> = {};
	let generalError = '';
	let isLoading = false;

	let cardRef: HTMLElement;
	let particlesRef: HTMLElement;
	let formRef: HTMLElement;

	// Track GSAP animations and event listeners for cleanup
	let mainTimeline: gsap.core.Timeline | null = null;
	let particleAnimations: gsap.core.Tween[] = [];
	let inputListeners: { element: Element; focusHandler: () => void; blurHandler: () => void }[] = [];

	/**
	 * Validates redirect URL to prevent XSS attacks
	 * Only allows relative paths starting with /
	 */
	function validateRedirectUrl(url: string): string {
		try {
			const decoded = decodeURIComponent(url);
			// Only allow relative paths starting with /
			// Reject absolute URLs, protocol handlers, and javascript: URIs
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

	onMount(() => {
		if (!browser) return;

		// GSAP Timeline for entrance animations
		mainTimeline = gsap.timeline({ defaults: { ease: 'power3.out' } });

		mainTimeline
			.from(cardRef, {
				opacity: 0,
				y: 60,
				scale: 0.9,
				duration: 1.2,
				ease: 'power4.out'
			})
			.from(
				'.login-header',
				{
					opacity: 0,
					y: -30,
					duration: 0.8
				},
				'-=0.6'
			)
			.from(
				'.form-field',
				{
					opacity: 0,
					x: -40,
					duration: 0.6,
					stagger: 0.15
				},
				'-=0.4'
			)
			.from(
				'.login-footer',
				{
					opacity: 0,
					y: 20,
					duration: 0.6
				},
				'-=0.3'
			);

		// Animated particles
		createFloatingParticles();

		// Hover effects on inputs with proper cleanup tracking
		const inputs = document.querySelectorAll('.enhanced-input');
		inputs.forEach((input) => {
			const focusHandler = () => {
				gsap.to(input, {
					scale: 1.02,
					boxShadow: '0 0 30px rgba(99, 102, 241, 0.3)',
					duration: 0.3,
					ease: 'power2.out'
				});
			};

			const blurHandler = () => {
				gsap.to(input, {
					scale: 1,
					boxShadow: '0 0 0 rgba(99, 102, 241, 0)',
					duration: 0.3,
					ease: 'power2.out'
				});
			};

			input.addEventListener('focus', focusHandler);
			input.addEventListener('blur', blurHandler);
			inputListeners.push({ element: input, focusHandler, blurHandler });
		});

		return cleanup;
	});

	/**
	 * Cleanup function to prevent memory leaks
	 * Removes all GSAP animations and event listeners
	 */
	function cleanup() {
		// Kill main timeline
		if (mainTimeline) {
			mainTimeline.kill();
			mainTimeline = null;
		}

		// Kill all particle animations
		particleAnimations.forEach((anim) => anim.kill());
		particleAnimations = [];

		// Clear particles from DOM
		if (particlesRef) {
			particlesRef.innerHTML = '';
		}

		// Remove event listeners
		inputListeners.forEach(({ element, focusHandler, blurHandler }) => {
			element.removeEventListener('focus', focusHandler);
			element.removeEventListener('blur', blurHandler);
		});
		inputListeners = [];

		// Kill all GSAP animations on this component
		gsap.killTweensOf([cardRef, formRef, '.login-header', '.form-field', '.login-footer']);
	}

	onDestroy(() => {
		if (browser) {
			cleanup();
		}
	});

	function createFloatingParticles() {
		if (!particlesRef) return;

		for (let i = 0; i < 30; i++) {
			const particle = document.createElement('div');
			particle.className = 'floating-particle';

			const size = Math.random() * 4 + 2;
			const startX = Math.random() * 100;
			const startY = Math.random() * 100;
			const duration = Math.random() * 20 + 15;
			const delay = Math.random() * 5;

			particle.style.width = `${size}px`;
			particle.style.height = `${size}px`;
			particle.style.left = `${startX}%`;
			particle.style.top = `${startY}%`;

			particlesRef.appendChild(particle);

			// Track animation for cleanup
			const anim = gsap.to(particle, {
				y: -100,
				x: `+=${Math.random() * 100 - 50}`,
				opacity: 0,
				duration: duration,
				delay: delay,
				repeat: -1,
				ease: 'none'
			});
			particleAnimations.push(anim);
		}
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();
		errors = {};
		generalError = '';
		isLoading = true;

		// Button loading animation
		const btn = (e.target as HTMLFormElement).querySelector('button[type="submit"]');
		if (btn) {
			gsap.to(btn, { scale: 0.95, duration: 0.2 });
		}

		try {
			await login({ email, password });

			// Success animation
			if (formRef) {
				gsap.to(formRef, {
					opacity: 0,
					scale: 0.9,
					duration: 0.5,
					onComplete: () => {
						// Check for redirect parameter in URL with XSS protection
						const urlParams = new URLSearchParams(window.location.search);
						const redirect = urlParams.get('redirect') || '/';
						goto(validateRedirectUrl(redirect));
					}
				});
			}
		} catch (error) {
			// Error shake animation
			if (formRef) {
				gsap.fromTo(
					formRef,
					{ x: -10 },
					{ x: 10, duration: 0.1, repeat: 5, yoyo: true, ease: 'power1.inOut' }
				);
			}

			if (error && typeof error === 'object' && 'errors' in error) {
				errors = (error as { errors: Record<string, string[]> }).errors;
			} else if (error instanceof Error) {
				generalError = error.message;
			} else {
				generalError = 'An unexpected error occurred. Please try again.';
			}
		} finally {
			isLoading = false;
			if (btn) {
				gsap.to(btn, { scale: 1, duration: 0.2 });
			}
		}
	}
</script>

<svelte:head>
	<title>Login | Revolution Trading Pros</title>
</svelte:head>

<div class="login-page">
	<!-- Animated gradient background -->
	<div class="gradient-bg"></div>

	<!-- Floating particles container -->
	<div bind:this={particlesRef} class="particles-container"></div>

	<!-- Grid overlay -->
	<div class="grid-overlay"></div>

	<!-- Spotlight effect that follows cursor -->
	<div class="spotlight"></div>

	<!-- Login card -->
	<div class="login-container">
		<div bind:this={cardRef} class="login-card">
			<!-- Glass morphism overlay -->
			<div class="glass-overlay"></div>

			<!-- Glow effect -->
			<div class="card-glow"></div>

			<!-- Content -->
			<div class="login-content" bind:this={formRef}>
				<!-- Header -->
				<div class="login-header">
					<div class="icon-wrapper">
						<div class="icon-glow"></div>
						<div class="icon-container">
							<div class="icon-inner">
								<IconLock size={48} class="icon-svg" />
							</div>
						</div>
					</div>
					<h1 class="login-title">Welcome Back</h1>
					<p class="login-subtitle">Sign in to access your trading dashboard</p>
				</div>

				<!-- General error -->
				{#if generalError}
					<div class="error-banner">
						<IconAlertCircle size={20} class="error-icon" />
						<p class="error-text">{generalError}</p>
					</div>
				{/if}

				<!-- Login form -->
				<form on:submit={handleSubmit} class="login-form">
					<!-- Email field -->
					<div class="form-field">
						<label for="email" class="field-label"> Email Address </label>
						<div class="input-wrapper">
							<div class="input-icon">
								<IconMail size={20} />
							</div>
							<input
								id="email"
								type="email"
								bind:value={email}
								required
								class="enhanced-input"
								class:error={errors.email}
								placeholder="trader@example.com"
								autocomplete="email"
							/>
							<div class="input-border"></div>
						</div>
						{#if errors.email}
							<p class="field-error">{errors.email[0]}</p>
						{/if}
					</div>

					<!-- Password field -->
					<div class="form-field">
						<label for="password" class="field-label"> Password </label>
						<div class="input-wrapper">
							<div class="input-icon">
								<IconLock size={20} />
							</div>
							<input
								id="password"
								type="password"
								bind:value={password}
								required
								class="enhanced-input"
								class:error={errors.password}
								placeholder="••••••••"
								autocomplete="current-password"
							/>
							<div class="input-border"></div>
						</div>
						{#if errors.password}
							<p class="field-error">{errors.password[0]}</p>
						{/if}
					</div>

					<!-- Forgot password link -->
					<div class="forgot-link-wrapper">
						<a href="/forgot-password" class="forgot-link"> Forgot your password? </a>
					</div>

					<!-- Submit button -->
					<button type="submit" disabled={isLoading} class="submit-btn">
						<span class="btn-content">
							{#if isLoading}
								<span class="spinner"></span>
								<span>Signing in...</span>
							{:else}
								<IconLogin size={20} />
								<span>Sign In</span>
								<IconArrowRight size={20} class="arrow-icon" />
							{/if}
						</span>
						<div class="btn-glow"></div>
					</button>
				</form>

				<!-- Register link -->
				<div class="login-footer">
					<p class="footer-text">
						Don't have an account?
						<a href="/register" class="footer-link"> Create one now </a>
					</p>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.login-page {
		position: relative;
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2rem 1rem;
		overflow: hidden;
	}

	/* Animated gradient background */
	.gradient-bg {
		position: absolute;
		inset: 0;
		background: linear-gradient(
			135deg,
			#0f172a 0%,
			#1e1b4b 25%,
			#312e81 50%,
			#1e1b4b 75%,
			#0f172a 100%
		);
		background-size: 400% 400%;
		animation: gradientShift 20s ease infinite;
	}

	@keyframes gradientShift {
		0%,
		100% {
			background-position: 0% 50%;
		}
		50% {
			background-position: 100% 50%;
		}
	}

	/* Particles container */
	.particles-container {
		position: absolute;
		inset: 0;
		pointer-events: none;
	}

	/* Grid overlay */
	.grid-overlay {
		position: absolute;
		inset: 0;
		background-image:
			linear-gradient(rgba(99, 102, 241, 0.03) 1px, transparent 1px),
			linear-gradient(90deg, rgba(99, 102, 241, 0.03) 1px, transparent 1px);
		background-size: 50px 50px;
		animation: gridMove 30s linear infinite;
	}

	@keyframes gridMove {
		0% {
			transform: translate(0, 0);
		}
		100% {
			transform: translate(50px, 50px);
		}
	}

	/* Spotlight effect */
	.spotlight {
		position: absolute;
		inset: 0;
		background: radial-gradient(
			circle 600px at var(--mouse-x, 50%) var(--mouse-y, 50%),
			rgba(99, 102, 241, 0.15),
			transparent 80%
		);
		pointer-events: none;
	}

	/* Login container */
	.login-container {
		position: relative;
		width: 100%;
		max-width: 480px;
		z-index: 10;
	}

	/* Login card */
	.login-card {
		position: relative;
		background: rgba(15, 23, 42, 0.6);
		backdrop-filter: blur(20px);
		border-radius: 32px;
		border: 1px solid rgba(99, 102, 241, 0.2);
		overflow: hidden;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
	}

	.glass-overlay {
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
		background: linear-gradient(135deg, #6366f1, #8b5cf6, #ec4899);
		opacity: 0.2;
		filter: blur(20px);
		z-index: -1;
	}

	/* Login content */
	.login-content {
		position: relative;
		padding: 3rem 2.5rem;
	}

	/* Header */
	.login-header {
		text-align: center;
		margin-bottom: 2.5rem;
	}

	.icon-wrapper {
		position: relative;
		display: inline-block;
		margin-bottom: 1.5rem;
	}

	.icon-glow {
		position: absolute;
		inset: -20px;
		background: radial-gradient(circle, rgba(99, 102, 241, 0.4), transparent 70%);
		animation: iconPulse 3s ease-in-out infinite;
	}

	@keyframes iconPulse {
		0%,
		100% {
			opacity: 0.4;
			transform: scale(1);
		}
		50% {
			opacity: 0.8;
			transform: scale(1.1);
		}
	}

	.icon-container {
		position: relative;
		width: 96px;
		height: 96px;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		border-radius: 24px;
		padding: 2px;
		transform-style: preserve-3d;
	}

	.icon-inner {
		width: 100%;
		height: 100%;
		background: linear-gradient(135deg, #1e1b4b, #0f172a);
		border-radius: 22px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	:global(.icon-svg) {
		color: #818cf8;
		filter: drop-shadow(0 0 20px rgba(99, 102, 241, 0.5));
	}

	.login-title {
		font-size: 3rem;
		font-weight: 800;
		background: linear-gradient(135deg, #e0e7ff, #c7d2fe, #a5b4fc);
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
		margin-bottom: 0.75rem;
		letter-spacing: -0.02em;
	}

	.login-subtitle {
		font-size: 1.125rem;
		color: #94a3b8;
		font-weight: 400;
	}

	/* Error banner */
	.error-banner {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 16px;
		margin-bottom: 1.5rem;
		animation: errorSlide 0.3s ease-out;
	}

	@keyframes errorSlide {
		from {
			opacity: 0;
			transform: translateY(-10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	:global(.error-icon) {
		color: #f87171;
		flex-shrink: 0;
	}

	.error-text {
		color: #fca5a5;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	/* Form */
	.login-form {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.form-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.field-label {
		font-size: 0.875rem;
		font-weight: 600;
		color: #e2e8f0;
		letter-spacing: 0.025em;
	}

	.input-wrapper {
		position: relative;
	}

	.input-icon {
		position: absolute;
		left: 1.25rem;
		top: 50%;
		transform: translateY(-50%);
		color: #64748b;
		pointer-events: none;
		z-index: 2;
		transition: color 0.3s ease;
	}

	.enhanced-input {
		width: 100%;
		padding: 1rem 1.25rem 1rem 3.5rem;
		background: rgba(15, 23, 42, 0.5);
		border: 2px solid rgba(100, 116, 139, 0.3);
		border-radius: 14px;
		color: #f1f5f9;
		font-size: 1rem;
		font-weight: 500;
		transition: all 0.3s ease;
		outline: none;
	}

	.enhanced-input::placeholder {
		color: #475569;
	}

	.enhanced-input:focus {
		background: rgba(15, 23, 42, 0.7);
		border-color: #6366f1;
	}

	.input-wrapper:focus-within .input-icon {
		color: #818cf8;
	}

	.enhanced-input.error {
		border-color: #ef4444;
		background: rgba(239, 68, 68, 0.05);
	}

	.input-border {
		position: absolute;
		inset: 0;
		border-radius: 14px;
		padding: 2px;
		background: linear-gradient(135deg, #6366f1, #8b5cf6, #ec4899);
		-webkit-mask:
			linear-gradient(#fff 0 0) content-box,
			linear-gradient(#fff 0 0);
		mask:
			linear-gradient(#fff 0 0) content-box,
			linear-gradient(#fff 0 0);
		-webkit-mask-composite: xor;
		mask-composite: exclude;
		opacity: 0;
		transition: opacity 0.3s ease;
		pointer-events: none;
	}

	.field-error {
		color: #f87171;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.forgot-link-wrapper {
		text-align: right;
		margin-top: -0.5rem;
	}

	.forgot-link {
		color: #818cf8;
		font-size: 0.875rem;
		font-weight: 600;
		text-decoration: none;
		transition: color 0.2s ease;
		position: relative;
	}

	.forgot-link:hover {
		color: #a5b4fc;
	}

	/* Submit button */
	.submit-btn {
		position: relative;
		width: 100%;
		padding: 1rem 2rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		border: none;
		border-radius: 14px;
		color: white;
		font-size: 1.125rem;
		font-weight: 700;
		cursor: pointer;
		overflow: hidden;
		margin-top: 1rem;
		transition: all 0.3s ease;
	}

	.submit-btn:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 20px 40px -12px rgba(99, 102, 241, 0.6);
	}

	.submit-btn:active:not(:disabled) {
		transform: translateY(0);
	}

	.submit-btn:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.btn-content {
		position: relative;
		z-index: 2;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
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
		opacity: 0.6;
	}

	:global(.arrow-icon) {
		transition: transform 0.3s ease;
	}

	.submit-btn:hover:not(:disabled) :global(.arrow-icon) {
		transform: translateX(4px);
	}

	/* Spinner */
	.spinner {
		width: 20px;
		height: 20px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Footer */
	.login-footer {
		text-align: center;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(100, 116, 139, 0.2);
	}

	.footer-text {
		color: #94a3b8;
		font-size: 0.9375rem;
	}

	.footer-link {
		color: #818cf8;
		font-weight: 600;
		text-decoration: none;
		transition: color 0.2s ease;
		position: relative;
	}

	.footer-link::after {
		content: '';
		position: absolute;
		bottom: -2px;
		left: 0;
		right: 0;
		height: 2px;
		background: linear-gradient(90deg, #6366f1, #8b5cf6);
		transform: scaleX(0);
		transition: transform 0.3s ease;
	}

	.footer-link:hover {
		color: #a5b4fc;
	}

	.footer-link:hover::after {
		transform: scaleX(1);
	}

	/* Responsive */
	@media (max-width: 640px) {
		.login-content {
			padding: 2rem 1.5rem;
		}

		.login-title {
			font-size: 2.25rem;
		}

		.login-subtitle {
			font-size: 1rem;
		}

		.icon-container {
			width: 80px;
			height: 80px;
		}
	}
</style>

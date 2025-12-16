<script lang="ts">
	import { goto } from '$app/navigation';
	import { register } from '$lib/api/auth';
	import {
		IconUser,
		IconMail,
		IconLock,
		IconAlertCircle,
		IconUserPlus,
		IconSparkles,
		IconArrowRight,
		IconCircleCheck,
		IconInbox,
		IconEye,
		IconEyeOff
	} from '$lib/icons';
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import SEOHead from '$lib/components/SEOHead.svelte';

	let name = '';
	let email = '';
	let password = '';
	let password_confirmation = '';
	let showPassword = false;
	let showConfirmPassword = false;
	let errors: Record<string, string[]> = {};
	let generalError = '';
	let isLoading = false;
	let registrationSuccess = false;
	let successMessage = '';

	let cardRef: HTMLElement;
	let particlesRef: HTMLElement;
	let formRef: HTMLElement;

	// Track GSAP animations for cleanup to prevent memory leaks
	let mainTimeline: any = null;
	let sparkleTimeline: any = null;
	let particleAnimations: any[] = [];
	let gsapLib: any = null;

	onMount(() => {
		if (!browser) return;

		// Dynamic GSAP import for SSR safety - use IIFE to handle async
		(async () => {
			const gsapModule = await import('gsap');
			gsapLib = gsapModule.gsap || gsapModule.default;

			// GSAP Timeline for entrance animations
			mainTimeline = gsapLib.timeline({ defaults: { ease: 'power3.out' } });

			if (cardRef) {
				mainTimeline
					.from(cardRef, {
						opacity: 0,
						y: 80,
						scale: 0.85,
						rotation: -5,
						duration: 1.4,
						ease: 'back.out(1.4)'
					})
					.from(
						'.register-header',
						{
							opacity: 0,
							y: -40,
							duration: 0.9
						},
						'-=0.8'
					)
					.from(
						'.form-field',
						{
							opacity: 0,
							x: -50,
							duration: 0.7,
							stagger: 0.12
						},
						'-=0.5'
					)
					.from(
						'.register-footer',
						{
							opacity: 0,
							scale: 0.9,
							duration: 0.6
						},
						'-=0.4'
					);
			}

			// Create floating emerald particles
			createEmeraldParticles();

			// Sparkle effect on icon - tracked for cleanup
			sparkleTimeline = gsapLib.timeline({ repeat: -1 });
			sparkleTimeline.to('.sparkle-icon', {
				rotation: 360,
				duration: 3,
				ease: 'none'
			});
		})();

		return cleanup;
	});

	/**
	 * Cleanup function to prevent memory leaks
	 */
	function cleanup() {
		// Kill main timeline
		if (mainTimeline) {
			mainTimeline.kill();
			mainTimeline = null;
		}

		// Kill sparkle timeline
		if (sparkleTimeline) {
			sparkleTimeline.kill();
			sparkleTimeline = null;
		}

		// Kill all particle animations
		particleAnimations.forEach((anim) => anim.kill());
		particleAnimations = [];

		// Clear particles from DOM
		if (particlesRef) {
			particlesRef.innerHTML = '';
		}

		// Kill all GSAP animations on this component
		if (gsapLib) {
			gsapLib.killTweensOf([
				cardRef,
				formRef,
				'.register-header',
				'.form-field',
				'.register-footer',
				'.sparkle-icon'
			]);
		}
	}

	onDestroy(() => {
		if (browser) {
			cleanup();
		}
	});

	function createEmeraldParticles() {
		if (!particlesRef) return;

		for (let i = 0; i < 40; i++) {
			const particle = document.createElement('div');
			particle.className = 'emerald-particle';

			const size = Math.random() * 6 + 3;
			const startX = Math.random() * 100;
			const startY = Math.random() * 100;
			const duration = Math.random() * 25 + 20;
			const delay = Math.random() * 8;

			particle.style.width = `${size}px`;
			particle.style.height = `${size}px`;
			particle.style.left = `${startX}%`;
			particle.style.top = `${startY}%`;

			particlesRef.appendChild(particle);

			// Track animation for cleanup
			if (gsapLib) {
				const anim = gsapLib.to(particle, {
					y: -150,
					x: `+=${Math.random() * 120 - 60}`,
					opacity: 0,
					rotation: 360,
					duration: duration,
					delay: delay,
					repeat: -1,
					ease: 'none'
				});
				particleAnimations.push(anim);
			}
		}
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();
		errors = {};
		generalError = '';
		isLoading = true;

		const btn = (e.target as HTMLFormElement).querySelector('button[type="submit"]');
		if (btn && gsapLib) {
			gsapLib.to(btn, { scale: 0.95, duration: 0.2 });
		}

		try {
			const message = await register({ name, email, password, password_confirmation });

			// Success confetti! (dynamic import)
			const confettiModule = await import('canvas-confetti');
			const confetti = confettiModule.default;
			confetti({
				particleCount: 150,
				spread: 100,
				origin: { y: 0.6 },
				colors: ['#10b981', '#14b8a6', '#06b6d4', '#22c55e']
			});

			// Show email verification message
			registrationSuccess = true;
			successMessage = message;

			// Animate form out and success message in
			if (formRef && gsapLib) {
				gsapLib.to(formRef, {
					opacity: 0,
					y: -20,
					duration: 0.5
				});
			}
		} catch (error) {
			// Error shake
			if (formRef && gsapLib) {
				gsapLib.fromTo(
					formRef,
					{ x: -12 },
					{ x: 12, duration: 0.08, repeat: 6, yoyo: true, ease: 'power1.inOut' }
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
			if (btn && gsapLib) {
				gsapLib.to(btn, { scale: 1, duration: 0.2 });
			}
		}
	}
</script>

<SEOHead
	title="Register - Join Revolution Trading Pros"
	description="Create your Revolution Trading Pros account. Get started with live trading rooms, professional alerts, and expert-led courses."
	canonical="/register"
	ogType="website"
	keywords={['register trading account', 'join revolution trading pros', 'create trading account']}
/>

<div class="register-page">
	<!-- Animated gradient background -->
	<div class="gradient-bg"></div>

	<!-- Radial glow effects -->
	<div class="radial-glow glow-1"></div>
	<div class="radial-glow glow-2"></div>
	<div class="radial-glow glow-3"></div>

	<!-- Floating particles container -->
	<div bind:this={particlesRef} class="particles-container"></div>

	<!-- Animated grid -->
	<div class="grid-pattern"></div>

	<!-- Register card -->
	<div class="register-container">
		<div bind:this={cardRef} class="register-card">
			<!-- Glass overlay -->
			<div class="glass-overlay"></div>

			<!-- Card glow -->
			<div class="card-glow"></div>

			<!-- Content -->
			<div
				class="register-content"
				bind:this={formRef}
				style:display={registrationSuccess ? 'none' : 'block'}
			>
				<!-- Header -->
				<div class="register-header">
					<div class="icon-wrapper">
						<div class="icon-pulse-bg"></div>
						<div class="icon-container">
							<div class="icon-inner">
								<IconSparkles size={52} class="sparkle-icon" />
							</div>
						</div>
					</div>
					<h1 class="register-title">Join Revolution</h1>
					<p class="register-subtitle">Start your journey to trading excellence</p>
				</div>

				<!-- General error -->
				{#if generalError}
					<div class="error-banner">
						<IconAlertCircle size={20} class="error-icon" />
						<p class="error-text">{generalError}</p>
					</div>
				{/if}

				<!-- Register form -->
				<form onsubmit={handleSubmit} class="register-form">
					<!-- Name field -->
					<div class="form-field">
						<label for="name" class="field-label">Full Name</label>
						<div class="input-wrapper">
							<div class="input-icon">
								<IconUser size={20} />
							</div>
							<input
								id="name"
								type="text"
								bind:value={name}
								required
								class="enhanced-input"
								class:error={errors.name}
								placeholder="John Doe"
								autocomplete="name"
							/>
							<div class="input-glow"></div>
						</div>
						{#if errors.name}
							<p class="field-error">{errors.name[0]}</p>
						{/if}
					</div>

					<!-- Email field -->
					<div class="form-field">
						<label for="email" class="field-label">Email Address</label>
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
							<div class="input-glow"></div>
						</div>
						{#if errors.email}
							<p class="field-error">{errors.email[0]}</p>
						{/if}
					</div>

					<!-- Password field -->
					<div class="form-field">
						<label for="password" class="field-label">Password</label>
						<div class="input-wrapper">
							<div class="input-icon">
								<IconLock size={20} />
							</div>
							<input
								id="password"
								type={showPassword ? 'text' : 'password'}
								bind:value={password}
								required
								class="enhanced-input has-toggle"
								class:error={errors.password}
								placeholder="••••••••"
								autocomplete="new-password"
							/>
							<button
								type="button"
								class="password-toggle"
								onclick={() => showPassword = !showPassword}
								aria-label={showPassword ? 'Hide password' : 'Show password'}
								tabindex={-1}
							>
								{#if showPassword}
									<IconEyeOff size={20} />
								{:else}
									<IconEye size={20} />
								{/if}
							</button>
							<div class="input-glow"></div>
						</div>
						{#if errors.password}
							<p class="field-error">{errors.password[0]}</p>
						{/if}
					</div>

					<!-- Confirm password field -->
					<div class="form-field">
						<label for="password_confirmation" class="field-label">Confirm Password</label>
						<div class="input-wrapper">
							<div class="input-icon">
								<IconLock size={20} />
							</div>
							<input
								id="password_confirmation"
								type={showConfirmPassword ? 'text' : 'password'}
								bind:value={password_confirmation}
								required
								class="enhanced-input has-toggle"
								placeholder="••••••••"
								autocomplete="new-password"
							/>
							<button
								type="button"
								class="password-toggle"
								onclick={() => showConfirmPassword = !showConfirmPassword}
								aria-label={showConfirmPassword ? 'Hide password' : 'Show password'}
								tabindex={-1}
							>
								{#if showConfirmPassword}
									<IconEyeOff size={20} />
								{:else}
									<IconEye size={20} />
								{/if}
							</button>
							<div class="input-glow"></div>
						</div>
					</div>

					<!-- Submit button -->
					<button type="submit" disabled={isLoading} class="submit-btn">
						<span class="btn-content">
							{#if isLoading}
								<span class="spinner"></span>
								<span>Creating Account...</span>
							{:else}
								<IconUserPlus size={22} />
								<span>Create Account</span>
								<IconArrowRight size={22} class="arrow-icon" />
							{/if}
						</span>
						<div class="btn-shine"></div>
						<div class="btn-glow"></div>
					</button>
				</form>

				<!-- Login link -->
				<div class="register-footer">
					<p class="footer-text">
						Already have an account?
						<a href="/login" class="footer-link">Sign in instead</a>
					</p>
				</div>
			</div>

			<!-- Success Message (Email Verification) -->
			{#if registrationSuccess}
				<div class="success-content">
					<div class="success-icon-wrapper">
						<IconCircleCheck size={80} class="success-check-icon" />
					</div>
					<h2 class="success-title">Check Your Email!</h2>
					<p class="success-message">{successMessage}</p>

					<div class="email-instructions">
						<div class="instruction-item">
							<IconInbox size={24} class="instruction-icon" />
							<p>We've sent a verification link to <strong>{email}</strong></p>
						</div>
						<div class="instruction-item">
							<IconMail size={24} class="instruction-icon" />
							<p>Click the link in the email to verify your account</p>
						</div>
						<div class="instruction-item">
							<IconAlertCircle size={24} class="instruction-icon" />
							<p>Don't forget to check your spam folder if you don't see it</p>
						</div>
					</div>

					<div class="success-actions">
						<a href="/login" class="success-btn-primary">
							Go to Login
							<IconArrowRight size={20} />
						</a>
					</div>

					<p class="success-footer">
						Didn't receive the email?
						<a href="/login" class="resend-link">Contact support</a>
					</p>
				</div>
			{/if}
		</div>
	</div>
</div>

<style>
	.register-page {
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
			#064e3b 0%,
			#0f766e 20%,
			#0d9488 40%,
			#0f766e 60%,
			#064e3b 80%,
			#022c22 100%
		);
		background-size: 400% 400%;
		animation: emeraldShift 25s ease infinite;
	}

	@keyframes emeraldShift {
		0%,
		100% {
			background-position: 0% 50%;
		}
		50% {
			background-position: 100% 50%;
		}
	}

	/* Radial glow effects */
	.radial-glow {
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.15;
	}

	.glow-1 {
		width: 500px;
		height: 500px;
		top: 10%;
		left: 5%;
		background: radial-gradient(circle, #10b981, transparent 70%);
		animation: float1 20s ease-in-out infinite;
	}

	.glow-2 {
		width: 400px;
		height: 400px;
		bottom: 15%;
		right: 10%;
		background: radial-gradient(circle, #14b8a6, transparent 70%);
		animation: float2 18s ease-in-out infinite;
	}

	.glow-3 {
		width: 450px;
		height: 450px;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		background: radial-gradient(circle, #06b6d4, transparent 70%);
		animation: float3 22s ease-in-out infinite;
	}

	@keyframes float1 {
		0%,
		100% {
			transform: translate(0, 0);
		}
		50% {
			transform: translate(50px, 30px);
		}
	}

	@keyframes float2 {
		0%,
		100% {
			transform: translate(0, 0);
		}
		50% {
			transform: translate(-40px, -50px);
		}
	}

	@keyframes float3 {
		0%,
		100% {
			transform: translate(-50%, -50%) scale(1);
		}
		50% {
			transform: translate(-50%, -50%) scale(1.2);
		}
	}

	/* Particles container */
	.particles-container {
		position: absolute;
		inset: 0;
		pointer-events: none;
	}

	:global(.emerald-particle) {
		position: absolute;
		background: radial-gradient(
			circle,
			rgba(16, 185, 129, 0.9),
			rgba(20, 184, 166, 0.5),
			transparent
		);
		border-radius: 50%;
		filter: blur(1px);
	}

	/* Grid pattern */
	.grid-pattern {
		position: absolute;
		inset: 0;
		background-image:
			linear-gradient(rgba(16, 185, 129, 0.05) 1px, transparent 1px),
			linear-gradient(90deg, rgba(16, 185, 129, 0.05) 1px, transparent 1px);
		background-size: 60px 60px;
		animation: gridSlide 40s linear infinite;
	}

	@keyframes gridSlide {
		0% {
			transform: translate(0, 0);
		}
		100% {
			transform: translate(60px, 60px);
		}
	}

	/* Register container */
	.register-container {
		position: relative;
		width: 100%;
		max-width: 520px;
		z-index: 10;
	}

	/* Register card */
	.register-card {
		position: relative;
		background: rgba(6, 78, 59, 0.3);
		backdrop-filter: blur(24px);
		border-radius: 36px;
		border: 2px solid rgba(16, 185, 129, 0.25);
		overflow: hidden;
		box-shadow: 0 30px 60px -15px rgba(0, 0, 0, 0.6);
	}

	.glass-overlay {
		position: absolute;
		inset: 0;
		background: linear-gradient(
			135deg,
			rgba(255, 255, 255, 0.12) 0%,
			transparent 50%,
			rgba(16, 185, 129, 0.08) 100%
		);
		pointer-events: none;
	}

	.card-glow {
		position: absolute;
		inset: -3px;
		background: linear-gradient(135deg, #10b981, #14b8a6, #22c55e);
		opacity: 0.25;
		filter: blur(25px);
		z-index: -1;
	}

	/* Register content */
	.register-content {
		position: relative;
		padding: 3rem 2.75rem;
	}

	/* Header */
	.register-header {
		text-align: center;
		margin-bottom: 2.25rem;
	}

	.icon-wrapper {
		position: relative;
		display: inline-block;
		margin-bottom: 1.5rem;
	}

	.icon-pulse-bg {
		position: absolute;
		inset: -25px;
		background: radial-gradient(circle, rgba(16, 185, 129, 0.5), transparent 65%);
		animation: iconPulseEffect 4s ease-in-out infinite;
	}

	@keyframes iconPulseEffect {
		0%,
		100% {
			opacity: 0.3;
			transform: scale(0.95);
		}
		50% {
			opacity: 0.9;
			transform: scale(1.15);
		}
	}

	.icon-container {
		position: relative;
		width: 100px;
		height: 100px;
		background: linear-gradient(135deg, #10b981, #14b8a6);
		border-radius: 26px;
		padding: 3px;
	}

	.icon-inner {
		width: 100%;
		height: 100%;
		background: linear-gradient(135deg, #064e3b, #022c22);
		border-radius: 23px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	:global(.sparkle-icon) {
		color: #6ee7b7;
		filter: drop-shadow(0 0 24px rgba(16, 185, 129, 0.6));
	}

	.register-title {
		font-size: 3.25rem;
		font-weight: 900;
		background: linear-gradient(135deg, #d1fae5, #a7f3d0, #6ee7b7);
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
		margin-bottom: 0.625rem;
		letter-spacing: -0.03em;
	}

	.register-subtitle {
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
		background: rgba(239, 68, 68, 0.12);
		border: 1.5px solid rgba(239, 68, 68, 0.35);
		border-radius: 18px;
		margin-bottom: 1.5rem;
		animation: errorAppear 0.35s ease-out;
	}

	@keyframes errorAppear {
		from {
			opacity: 0;
			transform: translateY(-12px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	:global(.error-icon) {
		color: #f87171;
		flex-shrink: 0;
		margin-top: 2px;
	}

	.error-text {
		color: #fca5a5;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	/* Form */
	.register-form {
		display: flex;
		flex-direction: column;
		gap: 1.375rem;
		margin-bottom: 1.75rem;
	}

	.form-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.field-label {
		font-size: 0.875rem;
		font-weight: 700;
		color: #d1fae5;
		letter-spacing: 0.03em;
		text-transform: uppercase;
		font-size: 0.75rem;
	}

	.input-wrapper {
		position: relative;
	}

	.input-icon {
		position: absolute;
		left: 1.25rem;
		top: 50%;
		transform: translateY(-50%);
		color: #6ee7b7;
		pointer-events: none;
		z-index: 2;
		transition: all 0.3s ease;
		opacity: 0.6;
	}

	.enhanced-input {
		width: 100%;
		padding: 1.125rem 1.25rem 1.125rem 3.75rem;
		background: rgba(6, 78, 59, 0.4);
		border: 2px solid rgba(16, 185, 129, 0.25);
		border-radius: 16px;
		color: #ecfdf5;
		font-size: 1rem;
		font-weight: 500;
		transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
		outline: none;
	}

	.enhanced-input.has-toggle {
		padding-right: 3.5rem;
	}

	/* Password visibility toggle */
	.password-toggle {
		position: absolute;
		right: 1rem;
		top: 50%;
		transform: translateY(-50%);
		background: none;
		border: none;
		color: #6ee7b7;
		opacity: 0.6;
		cursor: pointer;
		padding: 0.25rem;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 6px;
		transition: color 0.2s ease, background 0.2s ease, opacity 0.2s ease;
		z-index: 2;
	}

	.password-toggle:hover {
		opacity: 1;
		background: rgba(16, 185, 129, 0.15);
	}

	.password-toggle:focus {
		outline: none;
		opacity: 1;
	}

	.enhanced-input::placeholder {
		color: rgba(148, 163, 184, 0.5);
	}

	.enhanced-input:focus {
		background: rgba(6, 78, 59, 0.6);
		border-color: #10b981;
		box-shadow: 0 0 0 4px rgba(16, 185, 129, 0.1);
	}

	/* Note: Cannot use ~ selector here as .input-icon comes before .enhanced-input in DOM */
	/* Icon opacity is controlled via JavaScript or parent hover states */

	.enhanced-input:focus + .input-glow {
		opacity: 1;
	}

	.enhanced-input.error {
		border-color: #ef4444;
		background: rgba(239, 68, 68, 0.08);
	}

	.input-glow {
		position: absolute;
		inset: -2px;
		background: linear-gradient(135deg, #10b981, #14b8a6);
		border-radius: 16px;
		opacity: 0;
		filter: blur(8px);
		transition: opacity 0.4s ease;
		pointer-events: none;
		z-index: -1;
	}

	.field-error {
		color: #f87171;
		font-size: 0.875rem;
		font-weight: 600;
	}

	/* Submit button */
	.submit-btn {
		position: relative;
		width: 100%;
		padding: 1.125rem 2rem;
		background: linear-gradient(135deg, #10b981, #14b8a6);
		border: none;
		border-radius: 16px;
		color: white;
		font-size: 1.125rem;
		font-weight: 800;
		cursor: pointer;
		overflow: hidden;
		margin-top: 0.75rem;
		transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
		box-shadow: 0 10px 30px -10px rgba(16, 185, 129, 0.4);
	}

	.submit-btn:hover:not(:disabled) {
		transform: translateY(-3px) scale(1.01);
		box-shadow: 0 20px 50px -12px rgba(16, 185, 129, 0.7);
	}

	.submit-btn:active:not(:disabled) {
		transform: translateY(-1px) scale(0.99);
	}

	.submit-btn:disabled {
		opacity: 0.65;
		cursor: not-allowed;
	}

	.btn-content {
		position: relative;
		z-index: 3;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
	}

	.btn-shine {
		position: absolute;
		top: 0;
		left: -100%;
		width: 100%;
		height: 100%;
		background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.3), transparent);
		transition: left 0.5s ease;
		z-index: 2;
	}

	.submit-btn:hover:not(:disabled) .btn-shine {
		left: 100%;
	}

	.btn-glow {
		position: absolute;
		inset: -2px;
		background: linear-gradient(135deg, #6ee7b7, #5eead4);
		filter: blur(20px);
		opacity: 0;
		transition: opacity 0.4s ease;
		z-index: 1;
	}

	.submit-btn:hover:not(:disabled) .btn-glow {
		opacity: 0.7;
	}

	:global(.arrow-icon) {
		transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.submit-btn:hover:not(:disabled) :global(.arrow-icon) {
		transform: translateX(5px);
	}

	/* Spinner */
	.spinner {
		width: 22px;
		height: 22px;
		border: 3px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.7s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Footer */
	.register-footer {
		text-align: center;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(16, 185, 129, 0.2);
	}

	.footer-text {
		color: #94a3b8;
		font-size: 0.9375rem;
	}

	.footer-link {
		color: #6ee7b7;
		font-weight: 700;
		text-decoration: none;
		transition: color 0.2s ease;
		position: relative;
	}

	.footer-link::after {
		content: '';
		position: absolute;
		bottom: -3px;
		left: 0;
		right: 0;
		height: 2px;
		background: linear-gradient(90deg, #10b981, #14b8a6);
		transform: scaleX(0);
		transition: transform 0.3s ease;
	}

	.footer-link:hover {
		color: #a7f3d0;
	}

	.footer-link:hover::after {
		transform: scaleX(1);
	}

	/* Success content */
	.success-content {
		position: relative;
		padding: 3rem 2.75rem;
		text-align: center;
		animation: successFadeIn 0.6s ease-out;
	}

	@keyframes successFadeIn {
		from {
			opacity: 0;
			transform: translateY(20px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.success-icon-wrapper {
		margin-bottom: 2rem;
	}

	:global(.success-check-icon) {
		color: #34d399;
		filter: drop-shadow(0 0 30px rgba(52, 211, 153, 0.6));
		animation: iconBounce 0.6s cubic-bezier(0.68, -0.55, 0.265, 1.55);
	}

	@keyframes iconBounce {
		0% {
			transform: scale(0);
		}
		50% {
			transform: scale(1.1);
		}
		100% {
			transform: scale(1);
		}
	}

	.success-title {
		font-size: 2.5rem;
		font-weight: 900;
		background: linear-gradient(135deg, #d1fae5, #a7f3d0, #6ee7b7);
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
		margin-bottom: 1rem;
		letter-spacing: -0.02em;
	}

	.success-message {
		font-size: 1.125rem;
		color: #94a3b8;
		margin-bottom: 2.5rem;
		line-height: 1.6;
	}

	.email-instructions {
		background: rgba(16, 185, 129, 0.08);
		border: 1.5px solid rgba(16, 185, 129, 0.25);
		border-radius: 20px;
		padding: 2rem 1.5rem;
		margin-bottom: 2rem;
		text-align: left;
	}

	.instruction-item {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
		margin-bottom: 1.25rem;
	}

	.instruction-item:last-child {
		margin-bottom: 0;
	}

	:global(.instruction-icon) {
		color: #6ee7b7;
		flex-shrink: 0;
		margin-top: 2px;
	}

	.instruction-item p {
		color: #d1fae5;
		font-size: 0.9375rem;
		line-height: 1.6;
		margin: 0;
	}

	.instruction-item strong {
		color: #ecfdf5;
		font-weight: 700;
	}

	.success-actions {
		margin-bottom: 1.5rem;
	}

	.success-btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem 2.5rem;
		background: linear-gradient(135deg, #10b981, #14b8a6);
		border-radius: 14px;
		color: white;
		font-weight: 700;
		font-size: 1.0625rem;
		text-decoration: none;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		box-shadow: 0 10px 30px -10px rgba(16, 185, 129, 0.5);
	}

	.success-btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 15px 40px -10px rgba(16, 185, 129, 0.7);
	}

	.success-footer {
		color: #94a3b8;
		font-size: 0.875rem;
		margin-top: 1.5rem;
	}

	.resend-link {
		color: #6ee7b7;
		font-weight: 600;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.resend-link:hover {
		color: #a7f3d0;
	}

	/* Responsive */
	@media (max-width: 640px) {
		.register-content {
			padding: 2.25rem 1.75rem;
		}

		.register-title {
			font-size: 2.5rem;
		}

		.icon-container {
			width: 85px;
			height: 85px;
		}

		.success-content {
			padding: 2.25rem 1.75rem;
		}

		.success-title {
			font-size: 2rem;
		}

		.email-instructions {
			padding: 1.5rem 1.25rem;
		}
	}
</style>

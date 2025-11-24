<script lang="ts">
	import { goto } from '$app/navigation';
	import { login } from '$lib/api/auth'; // Ensure this path is correct in your project
	import {
		IconLock,
		IconMail,
		IconAlertCircle,
		IconLogin,
		IconArrowRight
	} from '@tabler/icons-svelte';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { fade, slide } from 'svelte/transition';
	import gsap from 'gsap';

	// --- State ---
	let email = '';
	let password = '';
	let errors: Record<string, string[]> = {};
	let generalError = '';
	let isLoading = false;

	// --- Refs ---
	let containerRef: HTMLElement; // Scope for GSAP Context
	let formRef: HTMLElement;

	// --- Helpers ---

	/**
	 * Security: Validates redirect URL to prevent Open Redirect / XSS attacks
	 */
	function validateRedirectUrl(url: string): string {
		try {
			const decoded = decodeURIComponent(url);
			// Only allow relative paths starting with /
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

	/**
	 * Svelte Action: Handles input focus/blur animations
	 * Replaces manual event listener tracking
	 */
	function focusAnimation(node: HTMLElement) {
		const onFocus = () => {
			gsap.to(node, {
				scale: 1.02,
				boxShadow: '0 0 30px rgba(99, 102, 241, 0.3)',
				duration: 0.3,
				ease: 'power2.out',
				borderColor: '#6366f1'
			});
		};

		const onBlur = () => {
			gsap.to(node, {
				scale: 1,
				boxShadow: '0 0 0 rgba(99, 102, 241, 0)',
				duration: 0.3,
				ease: 'power2.out',
				borderColor: 'rgba(100, 116, 139, 0.3)'
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
		if (!browser) return;

		// Use GSAP Context for automatic cleanup of all animations within containerRef
		let ctx = gsap.context(() => {
			// 1. Entrance Timeline
			const tl = gsap.timeline({ defaults: { ease: 'power3.out' } });

			tl.from('.login-card', {
				opacity: 0,
				y: 60,
				scale: 0.9,
				duration: 1.2,
				ease: 'power4.out'
			})
				.from('.login-header', { opacity: 0, y: -30, duration: 0.8 }, '-=0.6')
				.from('.form-field', { opacity: 0, x: -40, duration: 0.6, stagger: 0.15 }, '-=0.4')
				.from('.login-footer', { opacity: 0, y: 20, duration: 0.6 }, '-=0.3');

			// 2. Particle Animations
			// We target the particle elements rendered in the #each block
			gsap.utils.toArray('.floating-particle').forEach((particle: any) => {
				const duration = Math.random() * 20 + 15;
				const delay = Math.random() * 5;

				gsap.to(particle, {
					y: -100,
					x: `+=${Math.random() * 100 - 50}`,
					opacity: 0,
					duration: duration,
					delay: delay,
					repeat: -1,
					ease: 'none'
				});
			});
		}, containerRef);

		// Mouse spotlight effect
		const handleMouseMove = (e: MouseEvent) => {
			if (containerRef) {
				containerRef.style.setProperty('--mouse-x', `${e.clientX}px`);
				containerRef.style.setProperty('--mouse-y', `${e.clientY}px`);
			}
		};
		window.addEventListener('mousemove', handleMouseMove);

		// Cleanup function (Svelte automatically calls this on destroy)
		return () => {
			ctx.revert(); // Kills all timelines and tweens created in the context
			window.removeEventListener('mousemove', handleMouseMove);
		};
	});

	async function handleSubmit(e: Event) {
		e.preventDefault();
		errors = {};
		generalError = '';
		isLoading = true;

		const btn = (e.target as HTMLFormElement).querySelector('.submit-btn');
		if (btn) gsap.to(btn, { scale: 0.95, duration: 0.2 });

		try {
			await login({ email, password });

			// Success Exit Animation
			if (formRef) {
				gsap.to(formRef, {
					opacity: 0,
					scale: 0.95,
					duration: 0.4,
					onComplete: () => {
						const urlParams = new URLSearchParams(window.location.search);
						const redirect = urlParams.get('redirect') || '/';
						goto(validateRedirectUrl(redirect));
					}
				});
			}
		} catch (error: any) {
			// Error Shake
			if (formRef) {
				gsap.fromTo(
					formRef,
					{ x: -10 },
					{
						x: 10,
						duration: 0.1,
						repeat: 5,
						yoyo: true,
						ease: 'power1.inOut',
						onComplete: () => {
							gsap.to(formRef, { x: 0, duration: 0.1 });
						}
					}
				);
			}

			if (error?.errors) {
				errors = error.errors;
			} else if (error instanceof Error) {
				generalError = error.message;
			} else {
				generalError = 'An unexpected error occurred. Please try again.';
			}
		} finally {
			isLoading = false;
			if (btn) gsap.to(btn, { scale: 1, duration: 0.2 });
		}
	}
</script>

<svelte:head>
	<title>Login | Revolution Trading Pros</title>
</svelte:head>

<div class="login-page" bind:this={containerRef}>
	<div class="gradient-bg"></div>

	<div class="particles-container">
		{#each { length: 30 } as _, i}
			<div
				class="floating-particle"
				style="
                    width: {Math.random() * 4 + 2}px;
                    height: {Math.random() * 4 + 2}px;
                    left: {Math.random() * 100}%;
                    top: {Math.random() * 100}%;
                "
			></div>
		{/each}
	</div>

	<div class="grid-overlay"></div>

	<div class="spotlight"></div>

	<div class="login-container">
		<div class="login-card">
			<div class="glass-overlay"></div>
			<div class="card-glow"></div>

			<div class="login-content" bind:this={formRef}>
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

				{#if generalError}
					<div class="error-banner" in:fade={{ duration: 300 }}>
						<IconAlertCircle size={20} class="error-icon" />
						<p class="error-text">{generalError}</p>
					</div>
				{/if}

				<form on:submit={handleSubmit} class="login-form">
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
								use:focusAnimation
								required
								class="enhanced-input"
								class:error={errors.email}
								placeholder="trader@example.com"
								autocomplete="email"
							/>
						</div>
						{#if errors.email}
							<p class="field-error" transition:slide>{errors.email[0]}</p>
						{/if}
					</div>

					<div class="form-field">
						<label for="password" class="field-label">Password</label>
						<div class="input-wrapper">
							<div class="input-icon">
								<IconLock size={20} />
							</div>
							<input
								id="password"
								type="password"
								bind:value={password}
								use:focusAnimation
								required
								class="enhanced-input"
								class:error={errors.password}
								placeholder="••••••••"
								autocomplete="current-password"
							/>
						</div>
						{#if errors.password}
							<p class="field-error" transition:slide>{errors.password[0]}</p>
						{/if}
					</div>

					<div class="forgot-link-wrapper">
						<a href="/forgot-password" class="forgot-link">Forgot your password?</a>
					</div>

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

				<div class="login-footer">
					<p class="footer-text">
						Don't have an account?
						<a href="/register" class="footer-link">Create one now</a>
					</p>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	/* Core Structure */
	.login-page {
		position: relative;
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2rem 1rem;
		overflow: hidden;
		/* Spotlight variables */
		--mouse-x: 50%;
		--mouse-y: 50%;
	}

	/* Backgrounds */
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
		z-index: 0;
	}

	.particles-container {
		position: absolute;
		inset: 0;
		pointer-events: none;
		z-index: 1;
	}

	.floating-particle {
		position: absolute;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 50%;
		pointer-events: none;
	}

	.grid-overlay {
		position: absolute;
		inset: 0;
		background-image:
			linear-gradient(rgba(99, 102, 241, 0.03) 1px, transparent 1px),
			linear-gradient(90deg, rgba(99, 102, 241, 0.03) 1px, transparent 1px);
		background-size: 50px 50px;
		animation: gridMove 30s linear infinite;
		z-index: 2;
	}

	.spotlight {
		position: absolute;
		inset: 0;
		background: radial-gradient(
			circle 600px at var(--mouse-x),
			var(--mouse-y),
			rgba(99, 102, 241, 0.15),
			transparent 80%
		);
		pointer-events: none;
		z-index: 3;
	}

	/* Cards & Layout */
	.login-container {
		position: relative;
		width: 100%;
		max-width: 480px;
		z-index: 10;
	}

	.login-card {
		position: relative;
		background: rgba(15, 23, 42, 0.6);
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
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

	.login-content {
		position: relative;
		padding: 3rem 2.5rem;
	}

	/* Typography & Icons */
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

	.icon-container {
		position: relative;
		width: 96px;
		height: 96px;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		border-radius: 24px;
		padding: 2px;
		box-shadow: 0 10px 25px -5px rgba(99, 102, 241, 0.4);
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
		filter: drop-shadow(0 0 15px rgba(99, 102, 241, 0.5));
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

	/* Inputs */
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
		margin-left: 0.25rem;
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

	/* Focus state handled by GSAP action, but here is fallback/base */
	.enhanced-input:focus {
		background: rgba(15, 23, 42, 0.8);
	}

	.input-wrapper:focus-within .input-icon {
		color: #818cf8;
	}

	.enhanced-input.error {
		border-color: #ef4444;
		background: rgba(239, 68, 68, 0.05);
	}

	/* Buttons & Links */
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

	.submit-btn:disabled {
		opacity: 0.7;
		cursor: not-allowed;
		filter: grayscale(0.5);
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

	.btn-content {
		position: relative;
		z-index: 2;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
	}

	.forgot-link-wrapper {
		text-align: right;
		margin-top: -0.5rem;
	}

	.forgot-link,
	.footer-link {
		color: #818cf8;
		font-weight: 600;
		text-decoration: none;
		transition: color 0.2s ease;
		font-size: 0.875rem;
	}

	.forgot-link:hover,
	.footer-link:hover {
		color: #a5b4fc;
	}

	/* Error Banner */
	.error-banner {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 16px;
		margin-bottom: 1.5rem;
	}

	.field-error {
		color: #f87171;
		font-size: 0.875rem;
		font-weight: 500;
		margin-top: 0.25rem;
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

	/* Animations */
	@keyframes gradientShift {
		0%,
		100% {
			background-position: 0% 50%;
		}
		50% {
			background-position: 100% 50%;
		}
	}

	@keyframes gridMove {
		0% {
			transform: translate(0, 0);
		}
		100% {
			transform: translate(50px, 50px);
		}
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

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.spinner {
		width: 20px;
		height: 20px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@media (max-width: 640px) {
		.login-content {
			padding: 2rem 1.5rem;
		}
		.login-title {
			font-size: 2.25rem;
		}
		.icon-container {
			width: 80px;
			height: 80px;
		}
	}
</style>

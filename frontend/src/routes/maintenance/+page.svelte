<!--
	URL: /maintenance
	Purpose: Temporary maintenance page while upgrading infrastructure
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	// Animation states
	let mounted = $state(false);
	let particles: Array<{ x: number; y: number; size: number; speed: number; opacity: number }> =
		$state([]);

	// Generate subtle background particles (client-only)
	if (browser) {
		particles = Array.from({ length: 30 }, () => ({
			x: Math.random() * 100,
			y: Math.random() * 100,
			size: Math.random() * 2 + 1,
			speed: Math.random() * 0.5 + 0.2,
			opacity: Math.random() * 0.3 + 0.1
		}));
	}

	onMount(() => {
		mounted = true;
	});

	// Email capture state
	let email = $state('');
	let isSubmitting = $state(false);
	let isSubmitted = $state(false);
	let errorMessage = $state('');

	const handleNotifyMe = async () => {
		// Basic email validation
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		if (!email || !emailRegex.test(email)) {
			errorMessage = 'Please enter a valid email address';
			return;
		}

		isSubmitting = true;
		errorMessage = '';

		try {
			// Store in localStorage for now (replace with API call later)
			const subscribers = JSON.parse(localStorage.getItem('maintenance_subscribers') || '[]');
			subscribers.push({
				email: email,
				timestamp: new Date().toISOString()
			});
			localStorage.setItem('maintenance_subscribers', JSON.stringify(subscribers));

			// Simulate API delay
			await new Promise((resolve) => setTimeout(resolve, 800));

			isSubmitted = true;
			email = '';
		} catch (err) {
			errorMessage = 'Something went wrong. Please try again.';
		} finally {
			isSubmitting = false;
		}
	};
</script>

<div class="maintenance-container">
	<!-- Animated background particles -->
	<div class="particles-container">
		{#each particles as particle, i}
			<div
				class="particle"
				style="
					left: {particle.x}%;
					top: {particle.y}%;
					width: {particle.size}px;
					height: {particle.size}px;
					opacity: {particle.opacity};
					animation-delay: {i * 0.5}s;
					animation-duration: {20 / particle.speed}s;
				"
			></div>
		{/each}
	</div>

	<!-- Gradient orbs -->
	<div class="orb orb-1"></div>
	<div class="orb orb-2"></div>
	<div class="orb orb-3"></div>

	<!-- Main content -->
	<main class="content" class:mounted>
		<!-- Logo / Icon -->
		<div class="icon-wrapper">
			<svg
				class="maintenance-icon"
				viewBox="0 0 64 64"
				fill="none"
				xmlns="http://www.w3.org/2000/svg"
			>
				<circle cx="32" cy="32" r="28" stroke="currentColor" stroke-width="2" class="icon-circle" />
				<path
					d="M20 32L28 40L44 24"
					stroke="currentColor"
					stroke-width="3"
					stroke-linecap="round"
					stroke-linejoin="round"
					class="icon-check"
				/>
			</svg>
		</div>

		<!-- Headline -->
		<h1 class="headline">Improving Our Services</h1>

		<!-- Subheadline -->
		<p class="subheadline">
			We're upgrading our infrastructure to serve <strong>more students</strong> with better speed, reliability,
			and features.
		</p>

		<!-- Status indicators -->
		<div class="status-grid">
			<div class="status-item">
				<div class="status-dot upgrading"></div>
				<span>Server Scaling</span>
			</div>
			<div class="status-item">
				<div class="status-dot complete"></div>
				<span>Database Optimization</span>
			</div>
			<div class="status-item">
				<div class="status-dot upgrading"></div>
				<span>CDN Expansion</span>
			</div>
		</div>

		<!-- Progress bar -->
		<div class="progress-container">
			<div class="progress-label">
				<span>Upgrade Progress</span>
				<span class="progress-percent">75%</span>
			</div>
			<div class="progress-track">
				<div class="progress-fill"></div>
			</div>
		</div>

		<!-- Email Capture Form -->
		<div class="cta-section">
			{#if !isSubmitted}
				<p class="cta-text">Want to be notified when we're back?</p>
				<div class="email-form">
					<div class="input-wrapper">
						<svg class="input-icon" viewBox="0 0 20 20" fill="currentColor">
							<path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z" />
							<path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z" />
						</svg>
						<input
							type="email"
							placeholder="Enter your email"
							bind:value={email}
							onkeydown={(e) => e.key === 'Enter' && handleNotifyMe()}
							disabled={isSubmitting}
							class="email-input"
						/>
					</div>
					<button class="cta-button" onclick={handleNotifyMe} disabled={isSubmitting}>
						{#if isSubmitting}
							<span class="spinner"></span>
							Submitting...
						{:else}
							<svg class="cta-icon" viewBox="0 0 20 20" fill="currentColor">
								<path
									d="M10 2a6 6 0 00-6 6v3.586l-.707.707A1 1 0 004 14h12a1 1 0 00.707-1.707L16 11.586V8a6 6 0 00-6-6zM10 18a3 3 0 01-3-3h6a3 3 0 01-3 3z"
								/>
							</svg>
							Notify Me
						{/if}
					</button>
				</div>
				{#if errorMessage}
					<p class="error-message">{errorMessage}</p>
				{/if}
			{:else}
				<div class="success-message">
					<svg class="success-icon" viewBox="0 0 20 20" fill="currentColor">
						<path
							fill-rule="evenodd"
							d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
							clip-rule="evenodd"
						/>
					</svg>
					<p><strong>You're on the list!</strong></p>
					<p class="success-subtext">We'll notify you as soon as we're back online.</p>
				</div>
			{/if}
		</div>

		<!-- Estimated time -->
		<p class="eta">Expected completion: <span class="eta-highlight">Coming Soon</span></p>
	</main>
</div>

<style>
	.maintenance-container {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		position: relative;
		overflow: hidden;
		padding: 2rem;
	}

	/* Background particles */
	.particles-container {
		position: absolute;
		inset: 0;
		pointer-events: none;
		z-index: 0;
	}

	.particle {
		position: absolute;
		background: var(--rtp-primary);
		border-radius: 50%;
		animation: float linear infinite;
	}

	@keyframes float {
		0%,
		100% {
			transform: translateY(0) translateX(0);
			opacity: 0;
		}
		10% {
			opacity: var(--tw-opacity, 0.3);
		}
		90% {
			opacity: var(--tw-opacity, 0.3);
		}
		50% {
			transform: translateY(-100vh) translateX(20px);
		}
	}

	/* Gradient orbs */
	.orb {
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		pointer-events: none;
		z-index: 0;
	}

	.orb-1 {
		width: 400px;
		height: 400px;
		background: radial-gradient(circle, rgba(59, 130, 246, 0.15) 0%, transparent 70%);
		top: -10%;
		left: -10%;
		animation: orb-pulse 8s ease-in-out infinite;
	}

	.orb-2 {
		width: 300px;
		height: 300px;
		background: radial-gradient(circle, rgba(99, 102, 241, 0.12) 0%, transparent 70%);
		bottom: -5%;
		right: -5%;
		animation: orb-pulse 10s ease-in-out infinite reverse;
	}

	.orb-3 {
		width: 200px;
		height: 200px;
		background: radial-gradient(circle, rgba(16, 185, 129, 0.08) 0%, transparent 70%);
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		animation: orb-drift 12s ease-in-out infinite;
	}

	@keyframes orb-pulse {
		0%,
		100% {
			transform: scale(1);
			opacity: 0.6;
		}
		50% {
			transform: scale(1.2);
			opacity: 0.8;
		}
	}

	@keyframes orb-drift {
		0%,
		100% {
			transform: translate(-50%, -50%) translateX(0);
		}
		50% {
			transform: translate(-50%, -50%) translateX(50px);
		}
	}

	/* Main content */
	.content {
		position: relative;
		z-index: 1;
		max-width: 560px;
		text-align: center;
		opacity: 0;
		transform: translateY(30px);
		transition: all 0.8s var(--rtp-ease-out);
	}

	.content.mounted {
		opacity: 1;
		transform: translateY(0);
	}

	/* Icon */
	.icon-wrapper {
		margin-bottom: 2rem;
		display: flex;
		justify-content: center;
	}

	.maintenance-icon {
		width: 80px;
		height: 80px;
		color: var(--rtp-primary);
	}

	.icon-circle {
		stroke-dasharray: 176;
		stroke-dashoffset: 176;
		animation: draw-circle 1s ease-out forwards;
	}

	.icon-check {
		stroke-dasharray: 40;
		stroke-dashoffset: 40;
		animation: draw-check 0.6s ease-out 0.5s forwards;
	}

	@keyframes draw-circle {
		to {
			stroke-dashoffset: 0;
		}
	}

	@keyframes draw-check {
		to {
			stroke-dashoffset: 0;
		}
	}

	/* Typography */
	.headline {
		font-size: clamp(2rem, 5vw, 3rem);
		font-weight: 700;
		background: linear-gradient(135deg, var(--rtp-text) 0%, var(--rtp-text-soft) 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		margin-bottom: 1rem;
		letter-spacing: -0.02em;
	}

	.subheadline {
		font-size: 1.125rem;
		color: var(--rtp-text-muted);
		line-height: 1.7;
		margin-bottom: 2.5rem;
	}

	.subheadline strong {
		color: var(--rtp-text);
		font-weight: 600;
	}

	/* Status grid */
	.status-grid {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 1rem;
		margin-bottom: 2.5rem;
	}

	.status-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid var(--rtp-border);
		border-radius: 9999px;
		font-size: 0.875rem;
		color: var(--rtp-text-muted);
		transition: all 0.3s ease;
	}

	.status-item:hover {
		background: rgba(255, 255, 255, 0.06);
		border-color: var(--rtp-border-strong);
	}

	.status-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
	}

	.status-dot.upgrading {
		background: var(--rtp-amber);
		animation: pulse-amber 2s ease-in-out infinite;
	}

	.status-dot.complete {
		background: var(--rtp-emerald);
	}

	@keyframes pulse-amber {
		0%,
		100% {
			opacity: 1;
			transform: scale(1);
		}
		50% {
			opacity: 0.5;
			transform: scale(1.2);
		}
	}

	/* Progress bar */
	.progress-container {
		margin-bottom: 2.5rem;
	}

	.progress-label {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
		font-size: 0.875rem;
		color: var(--rtp-text-muted);
	}

	.progress-percent {
		color: var(--rtp-primary);
		font-weight: 600;
	}

	.progress-track {
		height: 8px;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 9999px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		width: 75%;
		background: linear-gradient(90deg, var(--rtp-primary) 0%, var(--rtp-indigo) 100%);
		border-radius: 9999px;
		animation: shimmer 2s ease-in-out infinite;
		background-size: 200% 100%;
	}

	@keyframes shimmer {
		0%,
		100% {
			background-position: 200% 0;
		}
		50% {
			background-position: -200% 0;
		}
	}

	/* CTA Section */
	.cta-section {
		margin-bottom: 1.5rem;
	}

	.cta-text {
		font-size: 0.875rem;
		color: var(--rtp-text-subtle);
		margin-bottom: 0.75rem;
	}

	.email-form {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		max-width: 360px;
		margin: 0 auto;
	}

	@media (min-width: 480px) {
		.email-form {
			flex-direction: row;
		}
	}

	.input-wrapper {
		position: relative;
		flex: 1;
	}

	.input-icon {
		position: absolute;
		left: 0.875rem;
		top: 50%;
		transform: translateY(-50%);
		width: 18px;
		height: 18px;
		color: var(--rtp-text-subtle);
		pointer-events: none;
	}

	.email-input {
		width: 100%;
		padding: 0.75rem 1rem 0.75rem 2.75rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid var(--rtp-border);
		border-radius: var(--rtp-radius-lg);
		color: var(--rtp-text);
		font-size: 0.9375rem;
		outline: none;
		transition: all 0.3s ease;
	}

	.email-input::placeholder {
		color: var(--rtp-text-subtle);
	}

	.email-input:focus {
		border-color: var(--rtp-primary);
		background: rgba(255, 255, 255, 0.08);
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.15);
	}

	.email-input:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.cta-button {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, var(--rtp-primary) 0%, var(--rtp-indigo) 100%);
		color: white;
		font-weight: 600;
		font-size: 0.9375rem;
		border: none;
		border-radius: var(--rtp-radius-lg);
		cursor: pointer;
		transition: all 0.3s ease;
		position: relative;
		overflow: hidden;
		white-space: nowrap;
	}

	.cta-button:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.cta-button:not(:disabled):hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 40px rgba(59, 130, 246, 0.3);
	}

	.cta-icon {
		width: 18px;
		height: 18px;
		opacity: 0.9;
		flex-shrink: 0;
	}

	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.error-message {
		margin-top: 0.5rem;
		font-size: 0.8125rem;
		color: var(--rtp-red-soft);
	}

	.success-message {
		padding: 1.5rem;
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.2);
		border-radius: var(--rtp-radius-lg);
	}

	.success-message p {
		margin: 0;
		color: var(--rtp-text);
	}

	.success-icon {
		width: 40px;
		height: 40px;
		color: var(--rtp-emerald);
		margin-bottom: 0.75rem;
	}

	.success-subtext {
		font-size: 0.875rem;
		color: var(--rtp-text-muted) !important;
		margin-top: 0.25rem !important;
	}

	/* ETA */
	.eta {
		font-size: 0.8125rem;
		color: var(--rtp-text-subtle);
	}

	.eta-highlight {
		color: var(--rtp-text-muted);
		font-weight: 500;
	}

	/* Responsive */
	@media (max-width: 640px) {
		.maintenance-container {
			padding: 1.5rem;
		}

		.status-grid {
			flex-direction: column;
			align-items: center;
		}

		.status-item {
			width: fit-content;
		}
	}
</style>

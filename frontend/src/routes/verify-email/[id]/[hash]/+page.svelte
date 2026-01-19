<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { IconCircleCheck, IconAlertCircle } from '$lib/icons';

	let verifying = $state(true);
	let success = $state(false);
	let error = $state('');
	let message = $state('');

	onMount(async () => {
		// ICT 11+ FIX: The hash IS the token - construct proper API call
		// Backend expects: GET /api/auth/verify-email?token=xxx
		const { hash } = page.params;
		const token = hash;

		try {
			// ICT 11+ CORB Fix: Use same-origin SvelteKit proxy endpoint
			const response = await fetch(
				`/api/auth/verify-email?token=${encodeURIComponent(token ?? '')}`,
				{
					method: 'GET',
					headers: {
						Accept: 'application/json'
					},
					credentials: 'include'
				}
			);

			const data = await response.json();

			if (response.ok) {
				success = true;
				message = data.message || 'Email verified successfully!';

				// Redirect to login after 3 seconds
				setTimeout(() => {
					goto('/login?verified=true');
				}, 3000);
			} else {
				error = data.message || 'Verification failed. The link may be invalid or expired.';
			}
		} catch (err: any) {
			error = 'Network error. Please check your connection and try again.';
		} finally {
			verifying = false;
		}
	});
</script>

<svelte:head>
	<title>Verify Email | Revolution Trading</title>
	<meta
		name="description"
		content="Verify your email address to complete your Revolution Trading registration."
	/>
	<meta property="og:title" content="Verify Email | Revolution Trading" />
	<meta
		property="og:description"
		content="Verify your email address to complete your Revolution Trading registration."
	/>
	<meta property="og:type" content="website" />
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="Verify Email | Revolution Trading" />
	<meta
		name="twitter:description"
		content="Verify your email address to complete your Revolution Trading registration."
	/>
</svelte:head>

<div class="verify-container">
	<div class="verify-card">
		<div class="verify-icon">
			{#if verifying}
				<div class="spinner"></div>
			{:else if success}
				<IconCircleCheck size={64} stroke={1.5} class="success-icon" />
			{:else}
				<IconAlertCircle size={64} stroke={1.5} class="error-icon" />
			{/if}
		</div>

		<div class="verify-content">
			{#if verifying}
				<h1>Verifying Your Email</h1>
				<p>Please wait while we verify your email address...</p>
			{:else if success}
				<h1>Email Verified!</h1>
				<p>{message}</p>
				<p class="redirect-message">Redirecting you to login...</p>
			{:else}
				<h1>Verification Failed</h1>
				<p class="error-message">{error}</p>
				<div class="action-buttons">
					<a href="/login" class="btn btn-primary">Go to Login</a>
					<a href="/register" class="btn btn-secondary">Register Again</a>
				</div>
			{/if}
		</div>
	</div>

	<div class="glow-orb-1"></div>
	<div class="glow-orb-2"></div>
</div>

<style>
	.verify-container {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		position: relative;
		overflow: hidden;
		background: linear-gradient(135deg, #0a0a0a 0%, #1a1a2e 100%);
	}

	.glow-orb-1,
	.glow-orb-2 {
		position: absolute;
		border-radius: 50%;
		filter: blur(100px);
		opacity: 0.15;
		pointer-events: none;
		z-index: 0;
	}

	.glow-orb-1 {
		width: 600px;
		height: 600px;
		background: radial-gradient(circle, #2e8eff 0%, transparent 70%);
		top: -200px;
		left: -200px;
		animation: float 20s ease-in-out infinite;
	}

	.glow-orb-2 {
		width: 500px;
		height: 500px;
		background: radial-gradient(circle, #34d399 0%, transparent 70%);
		bottom: -150px;
		right: -150px;
		animation: float 15s ease-in-out infinite reverse;
	}

	@keyframes float {
		0%,
		100% {
			transform: translate(0, 0) scale(1);
		}
		33% {
			transform: translate(30px, -30px) scale(1.1);
		}
		66% {
			transform: translate(-20px, 20px) scale(0.9);
		}
	}

	.verify-card {
		background: rgba(26, 26, 46, 0.8);
		backdrop-filter: blur(20px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 24px;
		padding: 4rem 3rem;
		max-width: 500px;
		width: 100%;
		text-align: center;
		position: relative;
		z-index: 1;
		box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
	}

	.verify-icon {
		margin-bottom: 2rem;
		display: flex;
		justify-content: center;
		align-items: center;
		height: 80px;
	}

	.spinner {
		width: 64px;
		height: 64px;
		border: 4px solid rgba(46, 142, 255, 0.2);
		border-top: 4px solid #2e8eff;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}

	:global(.success-icon) {
		color: #34d399;
		filter: drop-shadow(0 0 20px rgba(52, 211, 153, 0.5));
	}

	:global(.error-icon) {
		color: #ef4444;
		filter: drop-shadow(0 0 20px rgba(239, 68, 68, 0.5));
	}

	.verify-content h1 {
		font-size: 2rem;
		font-weight: 700;
		color: #ffffff;
		margin-bottom: 1rem;
		letter-spacing: -0.02em;
	}

	.verify-content p {
		font-size: 1.125rem;
		color: rgba(255, 255, 255, 0.7);
		line-height: 1.6;
		margin-bottom: 1rem;
	}

	.redirect-message {
		font-size: 1rem;
		color: rgba(52, 211, 153, 0.8);
		font-style: italic;
	}

	.error-message {
		color: rgba(239, 68, 68, 0.9);
	}

	.action-buttons {
		display: flex;
		gap: 1rem;
		margin-top: 2rem;
		flex-wrap: wrap;
		justify-content: center;
	}

	.btn {
		padding: 0.875rem 2rem;
		border-radius: 12px;
		font-weight: 600;
		font-size: 1rem;
		text-decoration: none;
		transition: all 0.3s ease;
		border: none;
		cursor: pointer;
		display: inline-block;
	}

	.btn-primary {
		background: linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%);
		color: #ffffff;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(46, 142, 255, 0.4);
	}

	.btn-secondary {
		background: rgba(255, 255, 255, 0.1);
		color: #ffffff;
		border: 1px solid rgba(255, 255, 255, 0.2);
	}

	.btn-secondary:hover {
		background: rgba(255, 255, 255, 0.15);
		transform: translateY(-2px);
	}

	@media (max-width: 768px) {
		.verify-card {
			padding: 3rem 2rem;
		}

		.verify-content h1 {
			font-size: 1.75rem;
		}

		.verify-content p {
			font-size: 1rem;
		}

		.glow-orb-1 {
			width: 400px;
			height: 400px;
		}

		.glow-orb-2 {
			width: 350px;
			height: 350px;
		}
	}

	@media (max-width: 640px) {
		.verify-container {
			padding: 1rem;
		}

		.verify-card {
			padding: 2rem 1.5rem;
		}

		.action-buttons {
			flex-direction: column;
		}

		.btn {
			width: 100%;
		}
	}
</style>

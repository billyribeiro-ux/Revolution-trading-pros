<script lang="ts">
	/**
	 * OAuth Callback Page - ICT Level 7 Principal Engineer Grade
	 *
	 * Handles OAuth callback from backend after successful authentication.
	 * Receives tokens from URL parameters and stores them securely.
	 *
	 * Security Features:
	 * - Tokens passed via URL are immediately processed and cleared from history
	 * - Access token stored in memory only (XSS-resistant)
	 * - Refresh token stored in httpOnly cookie via server endpoint
	 * - Redirects to dashboard after successful auth
	 *
	 * @version 1.0.0 - January 2026
	 */

	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { authStore } from '$lib/stores/auth.svelte';
	import { fade } from 'svelte/transition';

	let status = $state<'processing' | 'success' | 'error'>('processing');
	let errorMessage = $state('');
	let provider = $state('');

	$effect(() => {
		if (!browser) return;

		async function handleOAuthCallback() {
			try {
				// Get OAuth parameters from URL
				const params = page.url.searchParams;
				provider = params.get('provider') || 'oauth';
				const token = params.get('token');
				const refreshToken = params.get('refresh_token');
				const sessionId = params.get('session_id');
				const expiresIn = params.get('expires_in');
				const error = params.get('error');

				// Check for OAuth errors
				if (error) {
					status = 'error';
					errorMessage = decodeURIComponent(error);
					console.error('[OAuth Callback] Error from provider:', errorMessage);
					// Redirect to login after delay
					setTimeout(() => goto('/login?error=' + encodeURIComponent(errorMessage)), 2000);
					return;
				}

				// Validate required parameters
				if (!token || !sessionId) {
					status = 'error';
					errorMessage = 'Missing authentication parameters';
					console.error('[OAuth Callback] Missing required parameters');
					setTimeout(() => goto('/login?error=missing_params'), 2000);
					return;
				}

				// ICT 7 SECURITY: Clear tokens from URL history immediately
				// This prevents tokens from being stored in browser history
				window.history.replaceState({}, '', '/auth/callback');

				// Set session cookie via server endpoint
				try {
					const sessionResponse = await fetch('/api/auth/set-session', {
						method: 'POST',
						headers: {
							'Content-Type': 'application/json'
						},
						credentials: 'include',
						body: JSON.stringify({
							access_token: token,
							refresh_token: refreshToken,
							expires_in: expiresIn ? parseInt(expiresIn, 10) : 3600
						})
					});

					if (!sessionResponse.ok) {
						console.warn('[OAuth Callback] Failed to set session cookie');
					}
				} catch (cookieError) {
					console.warn('[OAuth Callback] Session cookie error:', cookieError);
					// Continue anyway - tokens are in memory
				}

				// Fetch user data
				const userResponse = await fetch('/api/auth/me', {
					headers: {
						Authorization: `Bearer ${token}`,
						Accept: 'application/json'
					},
					credentials: 'include'
				});

				if (!userResponse.ok) {
					throw new Error('Failed to fetch user data');
				}

				const userData = await userResponse.json();
				const user = userData.data || userData;

				// Store auth in store
				authStore.setAuth(
					user,
					token,
					sessionId,
					expiresIn ? parseInt(expiresIn, 10) : 3600,
					refreshToken
				);

				status = 'success';

				console.info('[OAuth Callback] Authentication successful:', {
					provider,
					userId: user.id,
					email: user.email
				});

				// Redirect to dashboard after brief success message
				setTimeout(() => {
					goto('/dashboard', { replaceState: true });
				}, 1000);
			} catch (error) {
				status = 'error';
				errorMessage = error instanceof Error ? error.message : 'Authentication failed';
				console.error('[OAuth Callback] Error:', error);

				// Redirect to login after delay
				setTimeout(() => goto('/login?error=callback_failed'), 2000);
			}
		}

		handleOAuthCallback();
	});

	// Format provider name for display
	function formatProvider(p: string): string {
		if (p === 'google') return 'Google';
		if (p === 'apple') return 'Apple';
		return p.charAt(0).toUpperCase() + p.slice(1);
	}
</script>

<svelte:head>
	<title>Authenticating... | Revolution Trading Pros</title>
</svelte:head>

<div class="callback-container">
	<div class="callback-card" transition:fade={{ duration: 300 }}>
		{#if status === 'processing'}
			<div class="status-icon processing">
				<div class="spinner"></div>
			</div>
			<h1>Signing in with {formatProvider(provider)}...</h1>
			<p class="status-text">Please wait while we complete your authentication.</p>
		{:else if status === 'success'}
			<div class="status-icon success">
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
					<path
						fill-rule="evenodd"
						d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zm13.36-1.814a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z"
						clip-rule="evenodd"
					/>
				</svg>
			</div>
			<h1>Welcome!</h1>
			<p class="status-text">Authentication successful. Redirecting to your dashboard...</p>
		{:else}
			<div class="status-icon error">
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
					<path
						fill-rule="evenodd"
						d="M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25zm-1.72 6.97a.75.75 0 10-1.06 1.06L10.94 12l-1.72 1.72a.75.75 0 101.06 1.06L12 13.06l1.72 1.72a.75.75 0 101.06-1.06L13.06 12l1.72-1.72a.75.75 0 10-1.06-1.06L12 10.94l-1.72-1.72z"
						clip-rule="evenodd"
					/>
				</svg>
			</div>
			<h1>Authentication Failed</h1>
			<p class="status-text error-text">{errorMessage || 'An error occurred during sign in.'}</p>
			<p class="redirect-text">Redirecting to login...</p>
		{/if}
	</div>
</div>

<style>
	.callback-container {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 1rem;
		background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
	}

	.callback-card {
		background: rgba(30, 41, 59, 0.8);
		backdrop-filter: blur(20px);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 1.5rem;
		padding: 3rem;
		text-align: center;
		max-width: 400px;
		width: 100%;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
	}

	.status-icon {
		width: 80px;
		height: 80px;
		margin: 0 auto 1.5rem;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 50%;
	}

	.status-icon svg {
		width: 48px;
		height: 48px;
	}

	.status-icon.processing {
		background: rgba(99, 102, 241, 0.1);
		border: 2px solid rgba(99, 102, 241, 0.3);
	}

	.status-icon.success {
		background: rgba(34, 197, 94, 0.1);
		border: 2px solid rgba(34, 197, 94, 0.3);
		color: #22c55e;
	}

	.status-icon.error {
		background: rgba(239, 68, 68, 0.1);
		border: 2px solid rgba(239, 68, 68, 0.3);
		color: #ef4444;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: #6366f1;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	h1 {
		font-family: var(--font-heading, 'Inter', sans-serif);
		font-size: 1.75rem;
		font-weight: 700;
		color: #e2e8f0;
		margin-bottom: 0.75rem;
	}

	.status-text {
		font-family: var(--font-body, 'Inter', sans-serif);
		font-size: 1rem;
		color: #94a3b8;
		margin-bottom: 0;
	}

	.error-text {
		color: #f87171;
	}

	.redirect-text {
		font-size: 0.875rem;
		color: #64748b;
		margin-top: 1rem;
	}

	/* Light theme */
	:global(html.light) .callback-container,
	:global(body.light) .callback-container {
		background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%);
	}

	:global(html.light) .callback-card,
	:global(body.light) .callback-card {
		background: rgba(255, 255, 255, 0.9);
		border-color: rgba(99, 102, 241, 0.1);
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.1);
	}

	:global(html.light) h1,
	:global(body.light) h1 {
		color: #1e293b;
	}

	:global(html.light) .status-text,
	:global(body.light) .status-text {
		color: #64748b;
	}
</style>

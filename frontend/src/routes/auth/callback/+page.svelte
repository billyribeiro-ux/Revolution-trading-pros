<script lang="ts">
	/**
	 * OAuth Callback Page
	 *
	 * FIX-C-1 (2026-04-29): the backend OAuth callbacks now set the access
	 * token, refresh token, and session ID as httpOnly cookies on the
	 * 302 redirect that lands the user here. This page no longer reads
	 * tokens from `?token=...&refresh_token=...&session_id=...` URL params,
	 * because those URLs leaked credentials into Cloudflare logs, browser
	 * history, and Referer headers.
	 *
	 * What we do here:
	 *   1. Read `?provider=...` (UX label) and `?error=...` (error path).
	 *   2. On success, call GET /api/auth/me — the cookies are already set,
	 *      so this just confirms the session and gets the user record.
	 *   3. Hand the user to the auth store and route to /dashboard.
	 *
	 * The previous /api/auth/set-session step is no longer needed — the
	 * cookies are already established by the backend's Set-Cookie headers.
	 */

	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { authStore } from '$lib/stores/auth.svelte';
	import { fade } from 'svelte/transition';

	let status = $state<'processing' | 'success' | 'error'>('processing');
	let errorMessage = $state('');
	let provider = $state('');

	onMount(async () => {
		try {
			const params = page.url.searchParams;
			provider = params.get('provider') || 'oauth';
			const error = params.get('error');

			if (error) {
				status = 'error';
				errorMessage = decodeURIComponent(error);
				console.error('[OAuth Callback] Error from provider:', errorMessage);
				setTimeout(() => goto('/login?error=' + encodeURIComponent(errorMessage)), 2000);
				return;
			}

			// Confirm session via cookie-authenticated /me. The httpOnly
			// cookies set on the redirect Response are sent automatically
			// because credentials:'include' + same-site.
			const userResponse = await fetch('/api/auth/me', {
				headers: { Accept: 'application/json' },
				credentials: 'include'
			});

			if (!userResponse.ok) {
				throw new Error(`Session not established (status ${userResponse.status})`);
			}

			const userData = await userResponse.json();
			const user = userData.data || userData;

			// Hand the user object to the in-memory auth store. The token
			// and session id remain in httpOnly cookies — the store does not
			// need them for UI rendering. Pass null/empty for the cookie-
			// only fields so the store does not try to mirror them.
			authStore.setAuth(user, '', '', 0, '');

			status = 'success';

			console.info('[OAuth Callback] Authentication successful:', {
				provider,
				userId: user.id
			});

			setTimeout(() => {
				goto('/dashboard', { replaceState: true });
			}, 1000);
		} catch (error) {
			status = 'error';
			errorMessage = error instanceof Error ? error.message : 'Authentication failed';
			console.error('[OAuth Callback] Error:', error);
			setTimeout(() => goto('/login?error=callback_failed'), 2000);
		}
	});

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
				<svg
					aria-hidden="true"
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 24 24"
					fill="currentColor"
				>
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
				<svg
					aria-hidden="true"
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 24 24"
					fill="currentColor"
				>
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

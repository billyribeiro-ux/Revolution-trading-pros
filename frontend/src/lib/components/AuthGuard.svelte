<!--
/**
 * AuthGuard Component - Google Enterprise Pattern
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Handles client-side authentication with proper history management.
 * Uses replaceState to prevent back-button issues.
 * 
 * Usage:
 * ```svelte
 * <AuthGuard redirectTo="/login" let:user>
 *   <Dashboard {user} />
 * </AuthGuard>
 * ```
 * 
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { browser } from '$app/environment';
	import { authStore, isAuthenticated, isInitializing, user } from '$lib/stores/auth';
	import { getUser } from '$lib/api/auth';
	import type { Snippet } from 'svelte';

	interface Props {
		/** Where to redirect if not authenticated */
		redirectTo?: string;
		/** Include current URL as redirect param */
		includeRedirect?: boolean;
		/** Loading component/content */
		loading?: Snippet;
		/** Main content - receives user prop */
		children: Snippet<[{ user: typeof $user }]>;
	}

	let {
		redirectTo = '/login',
		includeRedirect = true,
		loading,
		children
	}: Props = $props();

	let isChecking = $state(true);
	let authError = $state(false);

	// Handle auth check and redirect with proper history management
	async function checkAuth() {
		if (!browser) return;

		// Wait for auth store to finish initializing
		if ($isInitializing) {
			return;
		}

		const token = authStore.getToken();
		const sessionId = authStore.getSessionId();

		// No credentials at all - redirect immediately
		if (!token && !sessionId) {
			await redirectToLogin();
			return;
		}

		// Has session but no token - try to refresh
		if (!token && sessionId) {
			try {
				const refreshed = await authStore.refreshToken();
				if (!refreshed) {
					authStore.clearAuth();
					await redirectToLogin();
					return;
				}
			} catch (e) {
				authStore.clearAuth();
				await redirectToLogin();
				return;
			}
		}

		// Verify user data exists
		if (!$user) {
			try {
				await getUser();
			} catch (e) {
				authError = true;
				authStore.clearAuth();
				await redirectToLogin();
				return;
			}
		}

		isChecking = false;
	}

	async function redirectToLogin() {
		let url = redirectTo;
		
		if (includeRedirect) {
			const currentPath = $page.url.pathname + $page.url.search;
			url = `${redirectTo}?redirect=${encodeURIComponent(currentPath)}`;
		}

		// Use replaceState to prevent polluting browser history
		// This is the Google Enterprise way - back button goes to previous real page
		await goto(url, { replaceState: true });
	}

	// Re-check when auth state changes
	$effect(() => {
		if (browser && !$isInitializing) {
			checkAuth();
		}
	});

	onMount(() => {
		checkAuth();
	});
</script>

{#if isChecking || $isInitializing}
	{#if loading}
		{@render loading()}
	{:else}
		<!-- Default loading state -->
		<div class="auth-guard-loading">
			<div class="spinner"></div>
			<p>Verifying authentication...</p>
		</div>
	{/if}
{:else if $isAuthenticated && $user}
	{@render children({ user: $user })}
{/if}

<style>
	.auth-guard-loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 50vh;
		gap: 1rem;
		color: var(--color-rtp-text-secondary, #94a3b8);
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: #6366f1;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.auth-guard-loading p {
		font-size: 0.875rem;
	}
</style>

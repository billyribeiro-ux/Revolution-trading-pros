import { browser } from '$app/environment';
import { redirect } from '@sveltejs/kit';
import { authStore } from '$lib/stores/auth';
import { getUser } from '$lib/api/auth';
import type { Load } from '@sveltejs/kit';

/**
 * Protected checkout route loader - Google Enterprise Pattern
 * Uses SvelteKit's redirect() for proper history management
 */
export const load: Load = async ({ url }) => {
	// Server-side: Skip auth check, let client handle it
	if (!browser) {
		return { requiresAuth: true };
	}

	// Client-side: Check for token
	const token = authStore.getToken();
	const sessionId = authStore.getSessionId();

	// No token AND no session - definitely not logged in
	if (!token && !sessionId) {
		const returnUrl = encodeURIComponent(url.pathname + url.search);
		throw redirect(302, `/login?redirect=${returnUrl}`);
	}

	// Has session but no token - might need refresh, let client handle
	if (!token && sessionId) {
		return { requiresAuth: true, needsRefresh: true };
	}

	try {
		// Verify token is still valid by fetching user
		const user = await getUser();
		return { user, requiresAuth: true };
	} catch (error) {
		// Token invalid - let client-side handle the redirect
		return { requiresAuth: true, authError: true };
	}
};

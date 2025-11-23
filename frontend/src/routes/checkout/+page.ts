import { browser } from '$app/environment';
import { goto } from '$app/navigation';
import { authStore } from '$lib/stores/auth';
import { getUser } from '$lib/api/auth';
import type { LoadEvent } from '@sveltejs/kit';

type PageLoad = (event: LoadEvent) => Promise<Record<string, any>> | Record<string, any>;

/**
 * Protected checkout route loader
 * Redirects to login if user is not authenticated
 */
export const load: PageLoad = async ({ url }) => {
	if (!browser) {
		return {};
	}

	const token = authStore.getToken();

	if (!token) {
		// No token - redirect to login with return URL
		const returnUrl = encodeURIComponent(url.pathname + url.search);
		goto(`/login?redirect=${returnUrl}`);
		return {};
	}

	try {
		// Verify token is still valid by fetching user
		const user = await getUser();
		return { user };
	} catch (error) {
		// Token invalid - clear auth and redirect to login
		authStore.clearAuth();
		const returnUrl = encodeURIComponent(url.pathname + url.search);
		goto(`/login?redirect=${returnUrl}`);
		return {};
	}
};

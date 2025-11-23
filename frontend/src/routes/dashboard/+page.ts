import { browser } from '$app/environment';
import { goto } from '$app/navigation';
import { authStore } from '$lib/stores/auth';
import { getUser } from '$lib/api/auth';
import type { LoadEvent } from '@sveltejs/kit';

type PageLoad = (event: LoadEvent) => Promise<Record<string, any>> | Record<string, any>;

/**
 * Protected route loader
 * Ensures user is authenticated before accessing dashboard
 */
export const load: PageLoad = async () => {
	if (!browser) {
		return {};
	}

	const token = authStore.getToken();

	if (!token) {
		// No token - redirect to login
		goto('/login');
		return {};
	}

	try {
		// Verify token is still valid by fetching user
		const user = await getUser();
		return { user };
	} catch (error) {
		// Token invalid - clear auth and redirect to login
		authStore.clearAuth();
		goto('/login');
		return {};
	}
};

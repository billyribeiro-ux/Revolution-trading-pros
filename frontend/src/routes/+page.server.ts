import { API_BASE_URL, API_ENDPOINTS } from '$lib/api/config';
import type { Post } from '$lib/types/post';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	try {
		// Create abort controller with 3 second timeout to prevent hanging
		const controller = new AbortController();
		const timeoutId = setTimeout(() => controller.abort(), 3000);

		const response = await fetch(`${API_BASE_URL}${API_ENDPOINTS.posts.list}?per_page=6`, {
			signal: controller.signal
		});

		clearTimeout(timeoutId);

		if (!response.ok) {
			console.error('Failed to fetch posts:', response.status, response.statusText);
			return { posts: [] };
		}

		const data = await response.json();
		return { posts: data.data || [] };
	} catch (error) {
		// Silently handle errors - don't block page load
		if (error instanceof Error && error.name === 'AbortError') {
			console.warn('Posts fetch timed out - API may be unavailable');
		} else {
			console.error('Error loading posts:', error);
		}
		return { posts: [] };
	}
};

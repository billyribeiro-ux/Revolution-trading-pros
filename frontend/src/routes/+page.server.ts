import { API_BASE_URL, API_ENDPOINTS } from '$lib/api/config';
import type { PageServerLoad } from './$types';

/**
 * ICT11+ Performance: Simple server load without streaming
 * Streaming was causing hydration issues with __sveltekit variable
 */
export const load: PageServerLoad = async ({ fetch }) => {
	// Fetch posts with timeout - don't block too long
	const posts = await fetchPosts(fetch);
	
	return {
		posts
	};
};

async function fetchPosts(fetch: typeof globalThis.fetch) {
	try {
		const controller = new AbortController();
		const timeoutId = setTimeout(() => controller.abort(), 2000);
		
		const response = await fetch(
			`${API_BASE_URL}${API_ENDPOINTS.posts.list}?per_page=6`,
			{ signal: controller.signal }
		);
		
		clearTimeout(timeoutId);

		if (!response.ok) {
			return [];
		}

		const data = await response.json();
		return data.data || [];
	} catch {
		return [];
	}
}

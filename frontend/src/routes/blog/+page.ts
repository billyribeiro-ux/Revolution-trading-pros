import type { Load } from '@sveltejs/kit';
import { apiFetch, API_ENDPOINTS } from '$lib/api/config';
import type { PaginatedPosts } from '$lib/types/post';
import { logger } from '$lib/utils/logger';

export const prerender = false; // Disable prerendering - dynamic content
// SSR enabled for SEO - blog content should be server-rendered

export const load: Load = async ({ fetch: svelteKitFetch }) => {
	try {
		// Pass SvelteKit's fetch to apiFetch for proper SSR support
		const posts = await apiFetch<PaginatedPosts>(API_ENDPOINTS.posts.list, {
			fetch: svelteKitFetch
		});

		return {
			posts: posts.data,
			pagination: {
				currentPage: posts.current_page,
				lastPage: posts.last_page,
				total: posts.total
			}
		};
	} catch (error) {
		logger.error('Failed to load posts:', error);
		return {
			posts: [],
			pagination: {
				currentPage: 1,
				lastPage: 1,
				total: 0
			},
			error: error instanceof Error ? error.message : 'Failed to fetch posts'
		};
	}
};

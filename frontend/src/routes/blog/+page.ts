import type { PageLoad } from './$types';
import { apiFetch, API_ENDPOINTS } from '$lib/api/config';
import type { PaginatedPosts } from '$lib/types/post';

export const prerender = false; // Disable prerendering for this page
export const ssr = false; // Client-side rendering only

export const load: PageLoad = async ({ fetch: svelteKitFetch }) => {
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
		console.error('Failed to load posts:', error);
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

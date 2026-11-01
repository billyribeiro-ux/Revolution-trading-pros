import type { PageServerLoad } from './$types';
import { apiFetch, API_ENDPOINTS } from '#lib/api/config.js';
import type { PaginatedPosts } from '#lib/types/post.js';

export const prerender = false;

export const load: PageServerLoad = async ({ fetch: svelteKitFetch, url }) => {
	const pageParam = Number(url.searchParams.get('page') ?? '1');
	const pageNum = Number.isFinite(pageParam) && pageParam > 1 ? Math.floor(pageParam) : 1;

	try {
		const endpoint =
			pageNum > 1 ? `${API_ENDPOINTS.posts.list}?page=${pageNum}` : API_ENDPOINTS.posts.list;
		const posts = await apiFetch<PaginatedPosts>(endpoint, {
			fetch: svelteKitFetch
		});

		return {
			posts: posts.data,
			pagination: {
				currentPage: posts.current_page || pageNum,
				lastPage: posts.last_page || 1,
				total: posts.total || posts.data.length
			}
		};
	} catch {
		return {
			posts: [],
			pagination: {
				currentPage: pageNum,
				lastPage: 1,
				total: 0
			}
		};
	}
};


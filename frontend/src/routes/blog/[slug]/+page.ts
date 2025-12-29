import type { PageLoad } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';
import { apiFetch, API_ENDPOINTS } from '$lib/api/config';
import type { Post } from '$lib/types/post';

export const prerender = false; // Disable prerendering for this page
export const ssr = false; // Client-side rendering only

export const load: PageLoad = async ({ params }) => {
	try {
		const post = await apiFetch<Post>(API_ENDPOINTS.posts.single(params.slug));

		return {
			post
		};
	} catch (err) {
		console.error('Failed to load post:', err);
		throw error(404, {
			message: 'Blog post not found'
		});
	}
};

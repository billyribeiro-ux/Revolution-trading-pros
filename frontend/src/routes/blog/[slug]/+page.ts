import type { Load } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';
import { apiFetch, API_ENDPOINTS } from '$lib/api/config';
import type { Post } from '$lib/types/post';

export const prerender = false; // Disable prerendering - dynamic content
// SSR enabled for SEO - blog content should be server-rendered

export const load: Load = async ({ params }) => {
	try {
		const post = await apiFetch<Post>(API_ENDPOINTS.posts.single(params.slug ?? ''));

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

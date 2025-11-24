import { API_BASE_URL, API_ENDPOINTS } from '$lib/api/config';
import type { Post } from '$lib/types/post';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	try {
		const response = await fetch(`${API_BASE_URL}${API_ENDPOINTS.posts.list}?per_page=6`);

		if (!response.ok) {
			console.error('Failed to fetch posts:', response.status, response.statusText);
			return { posts: [] };
		}

		const data = await response.json();
		return { posts: data.data || [] };
	} catch (error) {
		console.error('Error loading posts:', error);
		return { posts: [] };
	}
};

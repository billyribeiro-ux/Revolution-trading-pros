import { API_BASE_URL, API_ENDPOINTS } from '$lib/api/config';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	try {
		const response = await Promise.race([
			fetch(`${API_BASE_URL}${API_ENDPOINTS.posts.list}?per_page=6`),
			new Promise<Response>((_, reject) => 
				setTimeout(() => reject(new Error('timeout')), 2000)
			)
		]);

		if (!response.ok) {
			return { posts: [] };
		}

		const data = await response.json();
		return { posts: data.data || [] };
	} catch {
		return { posts: [] };
	}
};

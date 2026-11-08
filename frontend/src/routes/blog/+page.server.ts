import type { PageServerLoad } from './$types';
import { error, isHttpError } from '@sveltejs/kit';
import { readPublicPosts } from '#lib/server/public-posts.js';
export const prerender = false;
export const load: PageServerLoad = async ({ url, setHeaders }) => {
	const raw = url.searchParams.get('page') ?? '1';
	if (!/^[1-9]\d{0,6}$/.test(raw)) error(400, 'Choose a positive blog page');
	const pageNum = Number(raw);
	try {
		const posts = await readPublicPosts(pageNum);
		if (pageNum > Math.max(1, posts.last_page)) error(404, 'Blog page not found');
		return {
			posts: posts.data,
			postsUnavailable: false,
			pagination: {
				currentPage: posts.current_page,
				lastPage: Math.max(1, posts.last_page),
				total: posts.total
			}
		};
	} catch (cause) {
		if (isHttpError(cause) && cause.status === 404) throw cause;
		setHeaders({ 'Cache-Control': 'no-store', 'X-Robots-Tag': 'noindex, nofollow' });
		return {
			posts: [],
			postsUnavailable: true,
			pagination: { currentPage: pageNum, lastPage: 1, total: 0 }
		};
	}
};

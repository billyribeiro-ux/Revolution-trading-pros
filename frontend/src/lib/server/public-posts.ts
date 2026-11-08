import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import { error } from '@sveltejs/kit';
import type { Post, PaginatedPosts } from '#lib/types/post.js';

function isPost(value: unknown): value is Post {
	if (!value || typeof value !== 'object') return false;
	const p = value as Record<string, unknown>;
	return (
		typeof p.id === 'number' &&
		typeof p.title === 'string' &&
		typeof p.slug === 'string' &&
		(p.excerpt === null || typeof p.excerpt === 'string') &&
		(p.content_blocks === null || Array.isArray(p.content_blocks)) &&
		(p.featured_image === null || typeof p.featured_image === 'string') &&
		typeof p.published_at === 'string'
	);
}
async function read(path: string): Promise<unknown> {
	let response: Response;
	try {
		response = await fetch(
			`${API_BASE_URL || BACKEND_URL || 'http://localhost:8080'}/api/posts${path}`,
			{
				headers: { Accept: 'application/json' },
				credentials: 'omit',
				redirect: 'error',
				cache: 'no-store',
				signal: AbortSignal.timeout(10000)
			}
		);
	} catch {
		error(503, 'Published posts are temporarily unavailable');
	}
	if (response.status === 404 && path.startsWith('/')) error(404, 'Blog post not found');
	if (!response.ok) error(503, 'Published posts are temporarily unavailable');
	try {
		return await response.json();
	} catch {
		error(503, 'Published posts response is invalid');
	}
}
export async function readPublicPost(slug: string): Promise<Post> {
	if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(slug)) error(404, 'Blog post not found');
	const post = await read(`/${slug}`);
	if (!isPost(post) || post.slug !== slug) error(503, 'Published post response is invalid');
	return post;
}
export async function readPublicPosts(page: number): Promise<PaginatedPosts> {
	const value = await read(`?page=${page}`);
	if (!value || typeof value !== 'object') error(503, 'Published posts response is invalid');
	const row = value as Record<string, unknown>;
	if (
		!Array.isArray(row.data) ||
		!row.data.every(isPost) ||
		!Number.isSafeInteger(row.current_page) ||
		row.current_page !== page ||
		typeof row.last_page !== 'number' ||
		!Number.isSafeInteger(row.last_page) ||
		row.last_page < 0 ||
		typeof row.total !== 'number' ||
		!Number.isSafeInteger(row.total) ||
		row.total < 0 ||
		typeof row.per_page !== 'number' ||
		!Number.isSafeInteger(row.per_page) ||
		row.per_page < 1
	)
		error(503, 'Published posts response is invalid');
	return {
		data: row.data,
		current_page: page,
		last_page: row.last_page,
		total: row.total,
		per_page: row.per_page,
		links: { first: null, last: null, prev: null, next: null }
	};
}

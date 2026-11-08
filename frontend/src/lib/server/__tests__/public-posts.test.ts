// @vitest-environment node
import { afterEach, describe, expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env/private', () => ({ API_BASE_URL: 'http://backend.test', BACKEND_URL: '' }));
import { readPublicPost, readPublicPosts } from '../public-posts.js';
afterEach(() => vi.unstubAllGlobals());
const post = {
	id: 1,
	title: 'Test publication',
	slug: 'test-publication',
	excerpt: null,
	content_blocks: [],
	featured_image: null,
	published_at: '2026-01-01T00:00:00Z'
};
describe('public post publication reads', () => {
	it('uses the runtime backend without visitor credentials', async () => {
		const fetch = vi.fn().mockResolvedValue(Response.json(post));
		vi.stubGlobal('fetch', fetch);
		expect(await readPublicPost(post.slug)).toEqual(post);
		expect(fetch).toHaveBeenCalledWith(
			'http://backend.test/api/posts/test-publication',
			expect.objectContaining({
				credentials: 'omit',
				redirect: 'error',
				signal: expect.any(AbortSignal)
			})
		);
		expect(fetch.mock.calls[0][1].headers).toEqual({ Accept: 'application/json' });
	});
	it('preserves a genuinely empty catalog', async () => {
		vi.stubGlobal(
			'fetch',
			vi
				.fn()
				.mockResolvedValue(
					Response.json({ data: [], current_page: 1, last_page: 0, per_page: 20, total: 0 })
				)
		);
		expect((await readPublicPosts(1)).data).toEqual([]);
	});
	it.each([500, 503, 404])(
		'rejects catalog failure %s instead of returning an empty publication',
		async (status) => {
			vi.stubGlobal('fetch', vi.fn().mockResolvedValue(new Response('', { status })));
			await expect(readPublicPosts(1)).rejects.toMatchObject({ status: 503 });
		}
	);
	it('preserves a missing article as 404', async () => {
		vi.stubGlobal('fetch', vi.fn().mockResolvedValue(new Response('', { status: 404 })));
		await expect(readPublicPost('missing')).rejects.toMatchObject({ status: 404 });
	});
	it('rejects a mismatched article response', async () => {
		vi.stubGlobal('fetch', vi.fn().mockResolvedValue(Response.json(post)));
		await expect(readPublicPost('another-article')).rejects.toMatchObject({ status: 503 });
	});
	it('rejects malformed pagination', async () => {
		vi.stubGlobal(
			'fetch',
			vi
				.fn()
				.mockResolvedValue(
					Response.json({ data: [post], current_page: 1, last_page: 'two', per_page: 20, total: 1 })
				)
		);
		await expect(readPublicPosts(1)).rejects.toMatchObject({ status: 503 });
	});
	it('rejects network failures', async () => {
		vi.stubGlobal('fetch', vi.fn().mockRejectedValue(new TypeError('offline')));
		await expect(readPublicPosts(1)).rejects.toMatchObject({ status: 503 });
	});
});

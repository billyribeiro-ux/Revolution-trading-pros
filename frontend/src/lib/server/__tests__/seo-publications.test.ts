// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env/private', () => ({ API_BASE_URL: 'http://backend.test', BACKEND_URL: '' }));
const auth = vi.hoisted(() => ({ admin: vi.fn(() => ({ token: 'fixture-token' })) }));
vi.mock('../auth.js', () => ({ requireAdmin: auth.admin }));
import { proxyPublicationAdmin, sourceSitemap, readPublication } from '../seo-publications.js';
function event(method: string, rest: string, body?: string) {
	const url = new URL('http://frontend.test/api/admin/seo/' + rest);
	return {
		url,
		params: { rest },
		request: new Request(url, { method, ...(body ? { body } : {}) })
	} as unknown as Parameters<typeof proxyPublicationAdmin>[0];
}
afterEach(() => {
	vi.unstubAllGlobals();
	vi.clearAllMocks();
});
it('forwards saved version intent once to the actual backend and preserves conflicts', async () => {
	const upstream = vi
		.fn<typeof fetch>()
		.mockResolvedValue(Response.json({ error: 'Reload saved state' }, { status: 409 }));
	vi.stubGlobal('fetch', upstream);
	const response = await proxyPublicationAdmin(
		event('PUT', 'sitemaps/news', '{"expected_version":4,"config":{}}'),
		'seo'
	);
	expect(response.status).toBe(409);
	expect(await response.json()).toEqual({ error: 'Reload saved state' });
	expect(upstream).toHaveBeenCalledExactlyOnceWith(
		'http://backend.test/api/admin/seo/sitemaps/news',
		expect.objectContaining({
			method: 'PUT',
			body: '{"expected_version":4,"config":{}}',
			headers: expect.objectContaining({ Authorization: 'Bearer fixture-token' }),
			redirect: 'error'
		})
	);
	expect(response.headers.get('Cache-Control')).toContain('no-store');
});
it('whitelists exact publication methods, prevents oversized writes and routes source saves through existing video CRUD without replay', async () => {
	const upstream = vi.fn<typeof fetch>().mockResolvedValue(Response.json({ success: true }));
	vi.stubGlobal('fetch', upstream);
	for (const [method, rest] of [
		['GET', '../videos'],
		['GET', 'sitemaps/news/generate'],
		['POST', 'video-publications/2'],
		['DELETE', 'video-sources/2']
	] as const)
		expect((await proxyPublicationAdmin(event(method, rest), 'seo')).status).toBe(404);
	expect(
		(await proxyPublicationAdmin(event('POST', 'video-publications', 'x'.repeat(100001)), 'seo'))
			.status
	).toBe(413);
	expect(upstream).not.toHaveBeenCalled();
	await proxyPublicationAdmin(event('POST', 'video-sources', '{}'), 'seo');
	expect(upstream.mock.calls[0]?.[0]).toBe('http://backend.test/api/admin/videos');
	upstream.mockClear().mockRejectedValue(new Error('Controlled lost response'));
	expect((await proxyPublicationAdmin(event('PUT', 'video-sources/2', '{}'), 'seo')).status).toBe(
		503
	);
	expect(upstream).toHaveBeenCalledOnce();
});
it('fetches public facts without credentials and reports unavailable or malformed sources as failures instead of empty successful XML', async () => {
	const upstream = vi
		.fn<typeof fetch>()
		.mockResolvedValue(Response.json({ error: 'unavailable' }, { status: 503 }));
	vi.stubGlobal('fetch', upstream);
	await expect(readPublication('sitemaps/news')).rejects.toMatchObject({ status: 503 });
	expect(upstream.mock.calls[0]?.[1]).toMatchObject({ credentials: 'omit', redirect: 'error' });
	upstream.mockResolvedValue(Response.json({ kind: 'news', entries: [] }));
	await expect(
		sourceSitemap(new URL('http://frontend.test/news-sitemap.xml'), 'news')
	).rejects.toMatchObject({ status: 503 });
	upstream.mockClear();
	await expect(
		sourceSitemap(new URL('http://frontend.test/news-sitemap.xml?page=-1'), 'news')
	).rejects.toMatchObject({ status: 400 });
	expect(upstream).not.toHaveBeenCalled();
});

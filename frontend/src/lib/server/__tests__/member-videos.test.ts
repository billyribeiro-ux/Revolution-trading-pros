// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env/private', () => ({ API_BASE_URL: 'http://backend.test', BACKEND_URL: '' }));
import { GET } from '../../../routes/api/videos/+server.js';
import { memberVideoHeaders, MEMBER_VIDEO_CACHE, proxyMemberVideo } from '../member-videos.js';
function event(token: string | null = 'fixture-private-token') {
	const url = new URL(
		'http://frontend.test/api/videos?room_id=42&video_date=2026-11-05&per_page=9'
	);
	return {
		url,
		request: new Request(url),
		locals: { accessToken: token },
		cookies: { get: () => undefined }
	} as unknown as Parameters<typeof GET>[0];
}
afterEach(() => {
	vi.unstubAllGlobals();
	vi.restoreAllMocks();
});
it('forwards only the authenticated bearer and preserves actual legacy pagination with private cache semantics', async () => {
	const upstream = vi.fn<typeof fetch>().mockResolvedValue(
		Response.json({
			success: true,
			data: [{ id: 17, title: 'Actual member video' }],
			meta: { current_page: 2, per_page: 9, total: 10, last_page: 2 }
		})
	);
	vi.stubGlobal('fetch', upstream);
	const response = await GET(event());
	expect(response.status).toBe(200);
	expect(await response.json()).toEqual({
		success: true,
		data: {
			videos: [{ id: 17, title: 'Actual member video' }],
			pagination: { page: 2, limit: 9, total: 10, total_pages: 2 }
		}
	});
	expect(upstream).toHaveBeenCalledExactlyOnceWith(
		'http://backend.test/api/videos?per_page=9&room_id=42&video_date=2026-11-05',
		expect.objectContaining({
			headers: expect.any(Headers),
			credentials: 'omit',
			redirect: 'error'
		})
	);
	expect(new Headers(upstream.mock.calls[0]?.[1]?.headers).get('Authorization')).toBe(
		'Bearer fixture-private-token'
	);
	expect(response.headers.get('Cache-Control')).toBe('private, no-store');
	expect(response.headers.get('Vary')).toBe('Authorization, Cookie');
});
it('never replaces an unavailable or inconsistent catalog with a successful zero count', async () => {
	vi.spyOn(console, 'error').mockImplementation(() => undefined);
	const upstream = vi.fn<typeof fetch>().mockResolvedValue(
		Response.json({
			success: true,
			data: [],
			meta: { current_page: 1, per_page: 9, total: 42, last_page: 5 }
		})
	);
	vi.stubGlobal('fetch', upstream);
	const response = await GET(event(null));
	expect(response.status).toBe(503);
	const value = await response.json();
	expect(value.success).toBe(false);
	expect(value.data).toBeUndefined();
	expect(upstream).toHaveBeenCalledOnce();
});
it('member loaders require an existing server token and expose private cache headers rather than a token in page data', () => {
	expect(() => memberVideoHeaders({} as App.Locals)).toThrow();
	const headers = memberVideoHeaders({ accessToken: 'fixture' } as App.Locals);
	expect(headers.get('Authorization')).toBe('Bearer fixture');
	expect(MEMBER_VIDEO_CACHE['Cache-Control']).toBe('private, no-store');
	expect(MEMBER_VIDEO_CACHE.Vary).toContain('Cookie');
});

it('preserves direct visibility failures and successful member envelopes privately without replaying a tracking write', async () => {
	const upstream = vi
		.fn<typeof fetch>()
		.mockResolvedValue(Response.json({ error: 'Video not found' }, { status: 404 }));
	vi.stubGlobal('fetch', upstream);
	const source = { ...event(), params: { rest: '42/related' } };
	const response = await proxyMemberVideo(source);
	expect(response.status).toBe(404);
	expect(await response.json()).toEqual({ error: 'Video not found' });
	expect(response.headers.get('Vary')).toContain('Authorization');
	expect(response.headers.get('Cache-Control')).toContain('private');
	expect(upstream.mock.calls[0]?.[1]).toMatchObject({
		credentials: 'omit',
		headers: expect.objectContaining({ Authorization: 'Bearer fixture-private-token' })
	});
	const tracking = {
		...source,
		params: { rest: '42/track' },
		request: new Request(source.url, { method: 'POST', body: '{"event_type":"play"}' })
	};
	upstream.mockClear().mockRejectedValue(new Error('Controlled unavailable service'));
	expect((await proxyMemberVideo(tracking)).status).toBe(503);
	expect(upstream).toHaveBeenCalledOnce();
});

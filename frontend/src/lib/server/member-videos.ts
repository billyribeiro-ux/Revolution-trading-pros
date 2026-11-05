import { error } from '@sveltejs/kit';
/** Only server loaders/proxies forward the existing authenticated access token. */
export function memberVideoHeaders(locals: App.Locals): Headers {
	if (!locals.accessToken) error(401, 'Authentication required for member videos');
	return new Headers({ Authorization: `Bearer ${locals.accessToken}`, Accept: 'application/json' });
}
export const MEMBER_VIDEO_CACHE = {
	'Cache-Control': 'private, no-store',
	Vary: 'Authorization, Cookie, Accept-Encoding'
};

/** Detail/related/history/tracking retain their backend envelopes and private cache boundary. */
export async function proxyMemberVideo(
	event: import('@sveltejs/kit').RequestEvent
): Promise<Response> {
	const { API_BASE_URL, BACKEND_URL } = await import('$app/env/private');
	const path = event.params.rest ?? '',
		method = event.request.method;
	const readable = /^(?:[a-zA-Z0-9_-]{1,200}|[1-9]\d{0,18}\/related)$/.test(path),
		tracking = /^[1-9]\d{0,18}\/track$/.test(path);
	const headers = new Headers(MEMBER_VIDEO_CACHE);
	if (!((method === 'GET' && readable) || (method === 'POST' && tracking)))
		return Response.json({ error: 'Video endpoint not found' }, { status: 404, headers });
	const token =
		event.locals.accessToken ||
		event.cookies.get('rtp_access_token') ||
		event.request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	let body: string | undefined;
	if (method === 'POST') {
		body = await event.request.text();
		if (new TextEncoder().encode(body).byteLength > 4096)
			return Response.json({ error: 'Video event exceeds4KB' }, { status: 413, headers });
	}
	try {
		const response = await fetch(
			`${API_BASE_URL || BACKEND_URL || 'http://localhost:8080'}/api/videos/${path}${event.url.search}`,
			{
				method,
				headers: {
					Accept: 'application/json',
					'Content-Type': 'application/json',
					...(token ? { Authorization: `Bearer ${token}` } : {})
				},
				body,
				credentials: 'omit',
				redirect: 'error',
				signal: AbortSignal.timeout(20000)
			}
		);
		headers.set('Content-Type', response.headers.get('Content-Type') ?? 'application/json');
		return new Response(response.body, { status: response.status, headers });
	} catch {
		return Response.json({ error: 'Video service is unavailable' }, { status: 503, headers });
	}
}

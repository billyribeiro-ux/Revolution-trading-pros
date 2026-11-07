import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import type { RequestHandler } from './$types';

export const prerender = false;

export const GET: RequestHandler = async (event) => {
	const { token } = event.params;
	if (!/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(token)) {
		return new Response(null, { status: 404 });
	}
	const width = event.url.searchParams.get('width');
	if (width !== null && (!/^\d{1,4}$/.test(width) || Number(width) < 1 || Number(width) > 4096)) {
		return new Response(null, { status: 400 });
	}
	const query = width === null ? '' : `?width=${width}`;
	try {
		const upstream = await event.fetch(
			`${API_BASE_URL || BACKEND_URL || 'http://localhost:8080'}/api/media/image/${token}${query}`,
			{
				headers: { Accept: event.request.headers.get('accept') ?? '' },
				redirect: 'manual',
				signal: AbortSignal.timeout(5000)
			}
		);
		if (upstream.status === 302) {
			const location = upstream.headers.get('location');
			if (!location) return new Response(null, { status: 502 });
			return new Response(null, {
				status: 302,
				headers: {
					Location: location,
					'Cache-Control': upstream.headers.get('cache-control') ?? 'public, max-age=30',
					Vary: 'Accept',
					'X-Content-Type-Options': 'nosniff'
				}
			});
		}
		return new Response(null, {
			status: upstream.status === 404 ? 404 : 503,
			headers: { 'Cache-Control': 'no-store' }
		});
	} catch {
		return new Response(null, {
			status: 503,
			headers: { 'Cache-Control': 'no-store' }
		});
	}
};

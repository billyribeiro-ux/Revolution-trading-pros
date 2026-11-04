import { API_BASE_URL, BACKEND_URL } from '$app/env/private';

type TrackingEvent = {
	fetch: typeof globalThis.fetch;
	request: Request;
	params: { kind?: string; token?: string };
};

/** Forward a capability request once, without credentials or following its redirect. */
export async function campaignTracking({
	fetch,
	request,
	params
}: TrackingEvent): Promise<Response> {
	const headers = {
		'cache-control': 'private, no-store',
		'referrer-policy': 'no-referrer',
		'x-robots-tag': 'noindex, nofollow',
		'x-content-type-options': 'nosniff'
	};
	const fail = (status: number) => new Response(null, { status, headers });
	if (
		(params.kind !== 'open' && params.kind !== 'click') ||
		!params.token ||
		!/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(params.token)
	)
		return fail(404);
	let upstream: Response;
	try {
		upstream = await fetch(
			`${API_BASE_URL || BACKEND_URL || 'http://localhost:8080'}/api/newsletter/tracking/${params.kind}/${params.token}`,
			{
				method: request.method === 'HEAD' ? 'HEAD' : 'GET',
				credentials: 'omit',
				redirect: 'manual',
				signal: AbortSignal.timeout(15000)
			}
		);
	} catch {
		return fail(503);
	}
	if (params.kind === 'click' && upstream.status === 302) {
		await upstream.body?.cancel();
		const location = upstream.headers.get('location');
		try {
			const target = new URL(location ?? '');
			if (!['http:', 'https:'].includes(target.protocol) || target.username || target.password)
				return fail(502);
			return new Response(null, { status: 302, headers: { ...headers, location: target.href } });
		} catch {
			return fail(502);
		}
	}
	if (
		params.kind === 'open' &&
		upstream.status === 200 &&
		upstream.headers.get('content-type') === 'image/gif'
	)
		return new Response(request.method === 'HEAD' ? null : upstream.body, {
			status: 200,
			headers: { ...headers, 'content-type': 'image/gif' }
		});
	await upstream.body?.cancel();
	return fail(upstream.status >= 400 && upstream.status <= 599 ? upstream.status : 502);
}

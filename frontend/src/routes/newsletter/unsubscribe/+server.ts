import { json, type RequestHandler } from '@sveltejs/kit';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';

// RFC 8058 clients use the same capability URL as the browser callback.
export const POST: RequestHandler = async ({ request, url, fetch }) => {
	const headers = { 'cache-control': 'no-store', 'referrer-policy': 'no-referrer' };
	const token = url.searchParams.get('token');
	if (!token || token.length > 2048)
		return json({ error: 'Invalid unsubscribe link' }, { status: 400, headers });
	if (!request.headers.get('content-type')?.startsWith('application/x-www-form-urlencoded'))
		return json({ error: 'Expected one-click form' }, { status: 415, headers });
	// Do not buffer an unbounded provider body.
	const reader = request.body?.getReader();
	let length = 0;
	let body = '';
	const decoder = new TextDecoder();
	if (reader) {
		try {
			while (true) {
				const chunk = await reader.read();
				if (chunk.done) break;
				length += chunk.value.byteLength;
				if (length > 1024) {
					await reader.cancel();
					return json({ error: 'One-click form too large' }, { status: 413, headers });
				}
				body += decoder.decode(chunk.value, { stream: true });
			}
			body += decoder.decode();
		} finally {
			reader.releaseLock();
		}
	}
	const form = new URLSearchParams(body);
	if (form.get('List-Unsubscribe') !== 'One-Click')
		return json({ error: 'Invalid one-click form' }, { status: 400, headers });
	try {
		const response = await fetch(
			`${API_BASE_URL || BACKEND_URL || 'http://localhost:8080'}/api/newsletter/unsubscribe?${new URLSearchParams({ token })}`,
			{
				method: 'POST',
				credentials: 'omit',
				redirect: 'error',
				signal: AbortSignal.timeout(15000),
				headers: {
					'content-type': 'application/x-www-form-urlencoded',
					accept: 'application/json'
				},
				body: 'List-Unsubscribe=One-Click'
			}
		);
		const result: unknown = await response.json().catch(() => null);
		if (
			response.ok &&
			(!result ||
				typeof result !== 'object' ||
				!('status' in result) ||
				result.status !== 'unsubscribed')
		)
			return json({ error: 'Unsubscribe outcome not confirmed' }, { status: 502, headers });
		return json(result ?? { error: 'Unsubscribe outcome not confirmed' }, {
			status: response.status,
			headers
		});
	} catch {
		return json(
			{ error: 'Unsubscribe outcome could not be verified; retry this link' },
			{ status: 503, headers }
		);
	}
};

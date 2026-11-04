import { error, type RequestEvent } from '@sveltejs/kit';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';

/** Complete the existing token callback without exposing the token in page data. */
export async function newsletterCallback(event: RequestEvent, action: 'confirm' | 'unsubscribe') {
	event.setHeaders({ 'cache-control': 'no-store', 'referrer-policy': 'no-referrer' });
	const token = event.url.searchParams.get('token');
	if (!token || token.length > 2048) error(400, 'This newsletter link is missing or invalid.');
	const backend = API_BASE_URL || BACKEND_URL || 'http://localhost:8080';
	let response: Response;
	try {
		response = await event.fetch(
			`${backend}/api/newsletter/${action}?${new URLSearchParams({ token })}`,
			{
				credentials: 'omit',
				redirect: 'error',
				signal: AbortSignal.timeout(15000),
				headers: { accept: 'application/json' }
			}
		);
	} catch {
		error(503, 'We could not verify your subscription change. Please try the link again.');
	}
	const body: unknown = await response.json().catch(() => null);
	if (!response.ok) {
		const message =
			body && typeof body === 'object' && 'error' in body && typeof body.error === 'string'
				? body.error
				: 'We could not verify your subscription change.';
		error(response.status >= 400 && response.status <= 599 ? response.status : 502, message);
	}
	const expected = action === 'confirm' ? 'confirmed' : 'unsubscribed';
	if (!body || typeof body !== 'object' || !('status' in body) || body.status !== expected)
		error(502, 'The subscription service did not confirm the requested change.');
	return {
		status: expected,
		message:
			'message' in body && typeof body.message === 'string'
				? body.message
				: action === 'confirm'
					? 'Your newsletter subscription is confirmed.'
					: 'Your newsletter subscription has been cancelled.'
	};
}

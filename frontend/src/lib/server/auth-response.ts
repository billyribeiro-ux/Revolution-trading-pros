import type { RequestEvent } from '@sveltejs/kit';
import { version } from '$app/env';

/** Only the identity service's validated response may populate trusted locals. */
export function authenticatedUser(input: unknown): NonNullable<App.Locals['user']> {
	if (!input || typeof input !== 'object') throw new Error('Invalid identity response');
	const envelope = input as Record<string, unknown>;
	const payload = envelope.data ?? input;
	if (!payload || typeof payload !== 'object') throw new Error('Invalid identity response');
	const user = payload as Record<string, unknown>;
	if (
		typeof user.id !== 'number' ||
		!Number.isSafeInteger(user.id) ||
		user.id <= 0 ||
		typeof user.email !== 'string' ||
		!user.email ||
		typeof user.role !== 'string' ||
		!user.role
	)
		throw new Error('Invalid identity response');
	return {
		id: user.id,
		email: user.email,
		name: typeof user.name === 'string' ? user.name : undefined,
		role: user.role,
		is_superadmin: user.is_superadmin === true,
		is_developer: user.is_developer === true
	};
}

/** Preserve secure production cookies while allowing explicit loopback HTTP development. */
export function secureAuthCookie(input: string | URL): boolean {
	const url = typeof input === 'string' ? new URL(input) : input;
	const loopback = ['localhost', '127.0.0.1', '[::1]'].includes(url.hostname);
	return url.protocol === 'https:' || !loopback;
}
export function privateAuthHeaders(refreshedCookie?: string): Headers {
	const headers = new Headers({
		'Cache-Control': 'no-store',
		'X-Robots-Tag': 'noindex, nofollow',
		'X-Content-Type-Options': 'nosniff'
	});
	if (refreshedCookie) headers.append('Set-Cookie', refreshedCookie);
	return headers;
}
/** Kit3 redirects thrown from handle bypass setHeaders. Preserve its data/action
 * redirect protocol explicitly so auth denials keep their private headers. */
export function privateAuthRedirect(
	event: Pick<RequestEvent, 'request' | 'isDataRequest'>,
	location: string,
	refreshedCookie?: string
): Response {
	const headers = privateAuthHeaders(refreshedCookie);
	const actionJson =
		event.request.method === 'POST' &&
		(event.request.headers.get('x-sveltekit-action') === 'true' ||
			event.request.headers.get('accept')?.includes('application/json'));
	if (event.isDataRequest || actionJson) {
		if (version) headers.set('x-sveltekit-version', version);
		return Response.json({ type: 'redirect', status: 303, location }, { headers });
	}
	headers.set('Location', location);
	return new Response(null, { status: 303, headers });
}

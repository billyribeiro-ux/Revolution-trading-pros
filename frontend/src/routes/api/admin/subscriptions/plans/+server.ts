/**
 * Admin Subscription Plans API Proxy (root)
 * FIX-2026-04-26 (P0-4): The Subscription Plans admin page calls
 * `GET /api/admin/subscriptions/plans?per_page=100` and (eventually)
 * `POST /api/admin/subscriptions/plans`. Backend exists at
 * `subscriptions_admin.rs::plans_router()` mounted at
 * `/admin/subscriptions/plans`. The proxy was missing entirely.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

function authHeaderFrom({
	cookies,
	request
}: {
	cookies: { get: (name: string) => string | undefined };
	request: Request;
}): string | null {
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	return token ? `Bearer ${token}` : null;
}

async function relayResponse(response: Response): Promise<Response> {
	const text = await response.text();
	if (!text) {
		return json({ error: 'Empty response from server' }, { status: response.status || 500 });
	}
	try {
		const data = JSON.parse(text);
		return json(data, { status: response.status });
	} catch {
		return json({ error: 'Invalid response from server' }, { status: 500 });
	}
}

export const GET: RequestHandler = async ({ url, cookies, request }) => {
	const auth = authHeaderFrom({ cookies, request });
	if (!auth) return json({ error: 'Missing or invalid authorization header' }, { status: 401 });
	try {
		const qs = url.search ? url.search : '';
		const response = await fetch(`${API_URL}/api/admin/subscriptions/plans${qs}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: auth
			}
		});
		return relayResponse(response);
	} catch (err) {
		console.error('[API Proxy] List subscription plans error:', err);
		return json({ error: 'Failed to load subscription plans' }, { status: 500 });
	}
};

export const POST: RequestHandler = async ({ cookies, request }) => {
	const auth = authHeaderFrom({ cookies, request });
	if (!auth) return json({ error: 'Missing or invalid authorization header' }, { status: 401 });
	try {
		const body = await request.json();
		const response = await fetch(`${API_URL}/api/admin/subscriptions/plans`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: auth
			},
			body: JSON.stringify(body)
		});
		return relayResponse(response);
	} catch (err) {
		console.error('[API Proxy] Create subscription plan error:', err);
		return json({ error: 'Failed to create subscription plan' }, { status: 500 });
	}
};

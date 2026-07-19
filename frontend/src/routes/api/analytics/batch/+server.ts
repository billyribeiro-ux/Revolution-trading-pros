/**
 * Same-origin proxy for analytics batch events
 * ICT 11+ CORB Fix: Routes client-side analytics batch calls through SvelteKit server
 */
import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// FIX-2026-07-19: env.VITE_API_URL → canonical pattern
// const PROD_API_ROOT = 'http://localhost:8080';
// const API_ROOT = env.VITE_API_URL || env.BACKEND_URL || PROD_API_ROOT;
const API_ROOT = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export const POST: RequestHandler = async ({ request, fetch, cookies }) => {
	try {
		const body = await request.json();
		const token = cookies.get('rtp_access_token');

		const headers: Record<string, string> = {
			Accept: 'application/json',
			'Content-Type': 'application/json'
		};
		if (token) headers.Authorization = `Bearer ${token}`;

		const response = await fetch(`${API_ROOT}/api/analytics/batch`, {
			method: 'POST',
			headers,
			body: JSON.stringify(body)
		});

		if (!response.ok) {
			return json({ success: false, processed: 0 }, { status: response.status });
		}

		const data = await response.json();
		return json(data, { status: 200 });
	} catch (error) {
		console.error('[Analytics Batch Proxy] Error:', error);
		return json({ success: false, processed: 0 }, { status: 200 });
	}
};

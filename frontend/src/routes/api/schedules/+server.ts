/**
 * Same-origin proxy for trading room schedules
 * ICT 11+ CORB Fix: Routes client-side schedule fetches through SvelteKit server
 */
import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const PROD_API_ROOT = 'https://revolution-trading-pros-api.fly.dev';
const API_ROOT = env.VITE_API_URL || env.BACKEND_URL || PROD_API_ROOT;

export const GET: RequestHandler = async ({ url, fetch, cookies }) => {
	try {
		// Forward all query params
		const targetUrl = new URL(`${API_ROOT}/api/schedules`);
		url.searchParams.forEach((value, key) => {
			targetUrl.searchParams.set(key, value);
		});

		const token = cookies.get('rtp_access_token');
		const headers: Record<string, string> = {
			Accept: 'application/json'
		};
		if (token) headers.Authorization = `Bearer ${token}`;

		const response = await fetch(targetUrl.toString(), { headers });
		const data = await response.json();

		return json(data, { status: response.status });
	} catch (error) {
		console.error('[Schedules Proxy] Error:', error);
		return json({ events: [], error: 'Failed to fetch schedules' }, { status: 200 });
	}
};

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// FIX-2026-04-26: env.VITE_API_URL → canonical pattern
// const PROD_API_ROOT = 'https://revolution-trading-pros-api.fly.dev';
// const API_ROOT = env.VITE_API_URL || env.BACKEND_URL || PROD_API_ROOT;
const API_ROOT =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';
const API_URL = `${API_ROOT}/api`;

export const GET: RequestHandler = async ({ url, fetch, cookies }) => {
	try {
		const anonymousId = url.searchParams.get('anonymous_id');
		const targetUrl = new URL(`${API_URL}/experiments/config`);
		if (anonymousId) targetUrl.searchParams.set('anonymous_id', anonymousId);

		const token = cookies.get('rtp_access_token');
		const headers: Record<string, string> = {
			Accept: 'application/json'
		};
		if (token) headers.Authorization = `Bearer ${token}`;

		const response = await fetch(targetUrl.toString(), { headers });
		const text = await response.text();

		try {
			const data = JSON.parse(text);
			return json(data, { status: response.status });
		} catch {
			return json({}, { status: 200 });
		}
	} catch {
		return json({}, { status: 200 });
	}
};

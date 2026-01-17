import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const PROD_API_ROOT = 'https://revolution-trading-pros-api.fly.dev';
const API_ROOT = env.VITE_API_URL || env.BACKEND_URL || PROD_API_ROOT;
const API_URL = `${API_ROOT}/api`;

export const POST: RequestHandler = async ({ request, cookies, fetch }) => {
	try {
		const rawBody = await request.text();

		if (!rawBody) {
			return json({ ok: true }, { status: 204 });
		}

		// Validate payload is JSON (performance metrics are expected to be JSON)
		try {
			JSON.parse(rawBody);
		} catch {
			return json({ ok: true }, { status: 204 });
		}

		const token = cookies.get('rtp_access_token');
		const headers: Record<string, string> = {
			Accept: 'application/json',
			'Content-Type': 'application/json'
		};
		if (token) headers.Authorization = `Bearer ${token}`;

		await fetch(`${API_URL}/analytics/performance`, {
			method: 'POST',
			headers,
			body: rawBody
		});

		return json({ ok: true }, { status: 204 });
	} catch {
		return json({ ok: true }, { status: 204 });
	}
};

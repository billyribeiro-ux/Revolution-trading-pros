/**
 * Public posts endpoint — takes precedence over `api/[...path]` proxy.
 * GET never returns 5xx: homepage client fallback uses this URL; we degrade to
 * an empty list when the upstream API errors so DevTools stays clean.
 */
import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const PROD_API_ROOT = 'https://revolution-trading-pros-api.fly.dev';
const API_ROOT = env.VITE_API_URL || env.BACKEND_URL || env.API_BASE_URL || PROD_API_ROOT;

export const GET: RequestHandler = async ({ url, fetch }) => {
	const target = new URL(`${API_ROOT.replace(/\/$/, '')}/api/posts`);
	url.searchParams.forEach((value, key) => {
		target.searchParams.set(key, value);
	});

	const controller = new AbortController();
	const timeoutId = setTimeout(() => controller.abort(), 10_000);

	try {
		const response = await fetch(target.toString(), {
			method: 'GET',
			headers: { Accept: 'application/json' },
			signal: controller.signal
		});

		if (!response.ok) {
			return json({ data: [] }, { status: 200 });
		}

		const data = (await response.json()) as { data?: unknown };
		const list = Array.isArray(data?.data) ? data.data : [];
		return json({ data: list }, { status: 200 });
	} catch {
		return json({ data: [] }, { status: 200 });
	} finally {
		clearTimeout(timeoutId);
	}
};

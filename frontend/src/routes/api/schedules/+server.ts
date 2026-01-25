/**
 * ═══════════════════════════════════════════════════════════════════════════
 * Trading Room Schedules Proxy - Apple ICT 7 Principal Engineer Grade
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Pattern: Graceful degradation with timeout + retry
 * Returns empty schedule on failure to prevent UI breakage.
 * Single retry for transient failures.
 */
import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const PROD_API_ROOT = 'https://revolution-trading-pros-api.fly.dev';
const API_ROOT = env.VITE_API_URL || env.BACKEND_URL || PROD_API_ROOT;
const TIMEOUT_MS = 10000; // 10 second timeout
const CACHE_CONTROL = 'public, max-age=60'; // Cache for 1 minute

function generateRequestId(): string {
	return `rtp-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 9)}`;
}

function sleep(ms: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

export const GET: RequestHandler = async ({ url, fetch, cookies }) => {
	const requestId = generateRequestId();
	const targetUrl = new URL(`${API_ROOT}/api/schedules`);
	url.searchParams.forEach((value, key) => {
		targetUrl.searchParams.set(key, value);
	});

	const token = cookies.get('rtp_access_token');
	const headers: Record<string, string> = {
		Accept: 'application/json',
		'X-Request-ID': requestId
	};
	if (token) headers.Authorization = `Bearer ${token}`;

	// ICT 7: Single retry with timeout protection
	for (let attempt = 0; attempt < 2; attempt++) {
		if (attempt > 0) await sleep(1000);

		try {
			const controller = new AbortController();
			const timeoutId = setTimeout(() => controller.abort(), TIMEOUT_MS);

			const response = await fetch(targetUrl.toString(), {
				headers,
				signal: controller.signal
			});

			clearTimeout(timeoutId);

			if (response.ok) {
				const data = await response.json();
				return json(data, {
					status: 200,
					headers: {
						'X-Request-ID': requestId,
						'Cache-Control': CACHE_CONTROL
					}
				});
			}
		} catch {
			// Retry on timeout/network error
		}
	}

	// ICT 7: Graceful degradation - return empty schedule
	return json(
		{ events: [], requestId, degraded: true },
		{
			status: 200,
			headers: {
				'X-Request-ID': requestId,
				'Cache-Control': 'no-cache'
			}
		}
	);
};

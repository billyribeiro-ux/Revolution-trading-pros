/**
 * ═══════════════════════════════════════════════════════════════════════════
 * Analytics Event Tracking Proxy - Apple ICT 7 Principal Engineer Grade
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Pattern: Silent failure with timeout protection
 * Analytics errors must NEVER affect user experience.
 * Returns success even on backend failure - data loss acceptable.
 */
import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const PROD_API_ROOT = 'https://revolution-trading-pros-api.fly.dev';
const API_ROOT = env.VITE_API_URL || env.BACKEND_URL || PROD_API_ROOT;
const TIMEOUT_MS = 5000; // 5 second timeout for analytics

function generateRequestId(): string {
	return `rtp-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 9)}`;
}

export const POST: RequestHandler = async ({ request, fetch, cookies }) => {
	const requestId = generateRequestId();

	try {
		const body = await request.json();
		const token = cookies.get('rtp_access_token');

		const headers: Record<string, string> = {
			Accept: 'application/json',
			'Content-Type': 'application/json',
			'X-Request-ID': requestId
		};
		if (token) headers.Authorization = `Bearer ${token}`;

		// ICT 7: Timeout protection
		const controller = new AbortController();
		const timeoutId = setTimeout(() => controller.abort(), TIMEOUT_MS);

		const response = await fetch(`${API_ROOT}/api/analytics/track`, {
			method: 'POST',
			headers,
			body: JSON.stringify(body),
			signal: controller.signal
		});

		clearTimeout(timeoutId);

		if (!response.ok) {
			// Silent failure - return success anyway
			return json({ success: true, requestId }, { status: 200 });
		}

		const data = await response.json();
		return json({ ...data, requestId }, { status: 200 });
	} catch (_error) {
		// ICT 7: Silent failure - analytics errors never affect UX
		return json({ success: true, requestId }, { status: 200 });
	}
};

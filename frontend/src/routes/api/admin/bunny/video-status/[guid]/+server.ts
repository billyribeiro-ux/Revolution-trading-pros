/**
 * Bunny Video Status API - Proxy to backend
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/admin/bunny/video-status/[guid] - Check video processing status
 *
 * Proxies to backend at /api/admin/bunny/video-status/:guid
 *
 * @version 1.1.0 - 2026-04-26 audit fix
 *
 * FIX-2026-04-26-audit (P1-1): Stop swallowing every backend error as
 * `{ status: 'processing' }`. The poller had no way to ever stop, so videos
 * appeared to hang forever on auth failures, deletions, or backend outages.
 * Now we propagate `success: false` with the upstream status so the client
 * can stop polling.
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdmin } from '$lib/server/auth';

const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

// GET - Check video processing status
export const GET: RequestHandler = async (event) => {
	const { params } = event;
	// FIX-2026-04-26-audit (P1-2): defense-in-depth admin gate.
	const { token } = requireAdmin(event);
	const guid = (params as { guid: string }).guid;

	if (!guid) {
		error(400, 'Video GUID is required');
	}

	try {
		const headers: Record<string, string> = {
			'Content-Type': 'application/json',
			Accept: 'application/json',
			Authorization: `Bearer ${token}`
		};

		const response = await fetch(`${BACKEND_URL}/api/admin/bunny/video-status/${guid}`, {
			headers
		});

		const text = await response.text();
		let payload: unknown = null;
		if (text) {
			try {
				payload = JSON.parse(text);
			} catch {
				payload = { error: text };
			}
		}

		if (!response.ok) {
			console.error(`[Bunny video-status] Backend error ${response.status}:`, text);
			return json(
				{
					success: false,
					error:
						(payload as { error?: string })?.error || `Backend returned ${response.status}`,
					status_code: response.status
				},
				{ status: response.status }
			);
		}

		return json(payload ?? { success: false, error: 'Empty response from backend' });
	} catch (err) {
		console.error('[Bunny video-status] Fetch failed:', err);
		return json(
			{
				success: false,
				error: err instanceof Error ? err.message : 'Failed to reach backend'
			},
			{ status: 502 }
		);
	}
};

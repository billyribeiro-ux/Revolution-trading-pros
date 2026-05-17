/**
 * Admin Indicators API Proxy — /api/admin/indicators
 * ══════════════════════════════════════════════════════════════════════
 *
 * Forwards GET (list) and POST (create) to the Rust backend.
 * Also handles sub-paths such as /api/admin/indicators/{id}/files,
 * /api/admin/indicators/{id}/videos, and /api/admin/indicators/{id}/documentation
 * via the sibling [id]/+server.ts proxy.
 *
 * Built 2026-04-26-audit (P0-8): this proxy was entirely absent; every
 * adminFetch('/api/admin/indicators...') call was returning a SvelteKit 404.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdmin } from '$lib/server/auth';

const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

// ── GET — list indicators ────────────────────────────────────────────────────
export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { url } = event;

	try {
		const queryString = url.search || '';
		const response = await fetch(`${BACKEND_URL}/api/admin/indicators${queryString}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
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
			console.error(`[Indicators proxy] GET list error ${response.status}:`, text);
			return json(
				{
					success: false,
					error: (payload as { error?: string })?.error || `Backend returned ${response.status}`
				},
				{ status: response.status }
			);
		}

		return json(payload ?? { success: true, data: [] });
	} catch (err) {
		console.error('[Indicators proxy] GET list fetch failed:', err);
		return json(
			{
				success: false,
				error: err instanceof Error ? err.message : 'Failed to reach backend'
			},
			{ status: 502 }
		);
	}
};

// ── POST — create indicator ──────────────────────────────────────────────────
export const POST: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { request } = event;

	try {
		const contentType = request.headers.get('content-type') || 'application/json';
		const body = await request.text();

		const response = await fetch(`${BACKEND_URL}/api/admin/indicators`, {
			method: 'POST',
			headers: {
				'Content-Type': contentType,
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			},
			body
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
			console.error(`[Indicators proxy] POST create error ${response.status}:`, text);
			return json(
				{
					success: false,
					error: (payload as { error?: string })?.error || `Backend returned ${response.status}`
				},
				{ status: response.status }
			);
		}

		return json(payload ?? { success: true });
	} catch (err) {
		console.error('[Indicators proxy] POST create fetch failed:', err);
		return json(
			{
				success: false,
				error: err instanceof Error ? err.message : 'Failed to reach backend'
			},
			{ status: 502 }
		);
	}
};

/**
 * Media AI Alt-Text Proxy
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * POST /api/admin/media/ai/alt-text/[id]
 *   - Forwards to backend `/api/admin/media/ai/alt-text/{id}` with Bearer auth.
 *
 * FIX-2026-04-26-audit (P1-7): see sibling analyze proxy.
 */

import { error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdmin } from '$lib/server/auth';

const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export const POST: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const id = event.params.id;
	if (!id) error(400, 'Missing media id');

	try {
		const upstream = await fetch(`${BACKEND_URL}/api/admin/media/ai/alt-text/${id}`, {
			method: 'POST',
			headers: {
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});
		const text = await upstream.text();
		const respHeaders: Record<string, string> = {};
		const ct = upstream.headers.get('content-type');
		if (ct) respHeaders['Content-Type'] = ct;
		return new Response(text, { status: upstream.status, headers: respHeaders });
	} catch (err) {
		console.error('[media/ai/alt-text proxy] forward failed:', err);
		error(502, err instanceof Error ? err.message : 'Failed to reach backend');
	}
};

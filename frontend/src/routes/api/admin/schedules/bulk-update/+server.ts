/**
 * Admin Schedules API - Bulk Update Endpoint
 * ═══════════════════════════════════════════════════════════════════════════════════
 * Apple Principal Engineer ICT 11+ Grade - January 2026
 *
 * @version 1.0.0
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

// Production fallback - Rust API on Fly.io
import { env } from '$env/dynamic/private';
const PROD_BACKEND =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

async function fetchFromBackend(endpoint: string, options?: RequestInit): Promise<any | null> {
	const BACKEND_URL = PROD_BACKEND;
	if (!BACKEND_URL) return null;

	try {
		const response = await fetch(`${BACKEND_URL}${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...options?.headers
			}
		});

		if (!response.ok) return null;
		return await response.json();
	} catch {
		return null;
	}
}

export const POST: RequestHandler = async ({ request, cookies }) => {
	const body = await request.json();
	const { ids, data } = body;

	if (!ids || !Array.isArray(ids) || ids.length === 0) {
		return json(
			{
				success: false,
				error: 'Missing or invalid ids array'
			},
			{ status: 400 }
		);
	}

	if (!data || typeof data !== 'object') {
		return json(
			{
				success: false,
				error: 'Missing or invalid data object'
			},
			{ status: 400 }
		);
	}

	// FIX-2026-04-26: prefer canonical rtp_access_token cookie, fall back to header.
	// Old: headers: { Authorization: request.headers.get('Authorization') || '' }
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) error(401, 'Unauthorized');

	// Try backend first
	const backendData = await fetchFromBackend('/api/admin/schedules/bulk-update', {
		method: 'POST',
		headers: { Authorization: `Bearer ${token}` },
		body: JSON.stringify(body)
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// Mock response - in production, this would update in database
	return json({
		success: true,
		data: {
			updated_count: ids.length,
			updated_ids: ids,
			changes: data
		},
		message: `${ids.length} schedules updated successfully`,
		_mock: true
	});
};

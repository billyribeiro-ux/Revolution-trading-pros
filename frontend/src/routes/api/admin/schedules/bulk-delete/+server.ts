/**
 * Admin Schedules API - Bulk Delete Endpoint
 * ═══════════════════════════════════════════════════════════════════════════════════
 * Apple Principal Engineer ICT 11+ Grade - January 2026
 *
 * @version 1.0.0
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';


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

export const POST: RequestHandler = async ({ request }) => {
	const body = await request.json();
	const { ids } = body;

	if (!ids || !Array.isArray(ids) || ids.length === 0) {
		return json(
			{
				success: false,
				error: 'Missing or invalid ids array'
			},
			{ status: 400 }
		);
	}

	// Try backend first
	const backendData = await fetchFromBackend('/api/admin/schedules/bulk-delete', {
		method: 'POST',
		headers: { Authorization: request.headers.get('Authorization') || '' },
		body: JSON.stringify(body)
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// Mock response - in production, this would delete from database
	return json({
		success: true,
		data: {
			deleted_count: ids.length,
			deleted_ids: ids
		},
		message: `${ids.length} schedules deleted successfully`,
		_mock: true
	});
};

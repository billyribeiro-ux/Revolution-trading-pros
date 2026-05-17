/**
 * Trading Room Videos API Endpoint
 *
 * Handles video management for trading rooms.
 * Proxies to the Rust backend; on backend failure returns a 502 (the
 * local mock fallback was removed in FIX-2026-04-26-audit P1-11 because
 * admins were seeing phantom data with no failure indication).
 *
 * @version 1.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

// Production fallback - Rust API on Fly.io
import { env } from '$env/dynamic/private';
const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';


// Try to fetch from backend
async function fetchFromBackend(endpoint: string, options?: RequestInit): Promise<any | null> {
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
	} catch (_error) {
		return null;
	}
}

// GET - List videos
export const GET: RequestHandler = async ({ url, request, cookies }) => {
	// NOTE (audit 2026-05-16): the per-filter consts (trading_room_id,
	// trader_id, published_only, page, per_page) were vestigial from the
	// local mock path removed in FIX-2026-04-26-audit P1-11. The real
	// filters/pagination are forwarded intact to the backend below via
	// `url.searchParams.toString()`, so parsing them here was dead code.

	// FIX-2026-04-26: prefer canonical rtp_access_token cookie, fall back to header.
	// Old: headers: { Authorization: request.headers.get('Authorization') || '' }
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) error(401, 'Unauthorized');

	// Try backend first
	const backendData = await fetchFromBackend(
		`/api/admin/trading-rooms/videos?${url.searchParams.toString()}`,
		{
			headers: { Authorization: `Bearer ${token}` }
		}
	);

	if (backendData?.success) {
		return json(backendData);
	}

	// FIX-2026-04-26-audit (P1-11): surface backend failure instead of returning phantom
	// mock videos. Admins were seeing fake data with no indication of a real failure.
	// TODO(2026-04-26-audit): gate mock on env.ENABLE_MOCK_DATA if needed for local dev.
	console.error('[Trading-rooms videos proxy] Backend unavailable or non-success:', backendData);
	return json(
		{
			success: false,
			error: 'Unable to load videos from backend. Please check the API connection.',
			_mock: false
		},
		{ status: 502 }
	);
};

// POST - Create video
export const POST: RequestHandler = async ({ request, cookies }) => {
	const body = await request.json();

	// FIX-2026-04-26: prefer canonical rtp_access_token cookie, fall back to header.
	// Old: headers: { Authorization: request.headers.get('Authorization') || '' }
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) error(401, 'Unauthorized');

	// Try backend first
	const backendData = await fetchFromBackend('/api/admin/trading-rooms/videos', {
		method: 'POST',
		headers: { Authorization: `Bearer ${token}` },
		body: JSON.stringify(body)
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// FIX-2026-04-26-audit (P1-11): surface POST failure — a silent mock-create would
	// persist nothing to the DB while appearing to succeed.
	console.error('[Trading-rooms videos proxy POST] Backend unavailable or non-success:', backendData);
	return json(
		{
			success: false,
			error: 'Unable to create video — backend is unavailable. Please try again.'
		},
		{ status: 502 }
	);
};

/**
 * Traders API Endpoint
 *
 * Handles trader management for trading rooms.
 * Proxies to the Rust backend; on backend failure returns a 502 (the
 * local mock fallback was removed in FIX-2026-04-26-audit P1-11).
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

// GET - List traders
export const GET: RequestHandler = async ({ url, request, cookies }) => {
	// NOTE (audit 2026-05-16): `active_only` is forwarded intact to the
	// backend via `url.searchParams.toString()` below; parsing it into a
	// local const here was vestigial from the removed mock path.

	// FIX-2026-04-26: prefer canonical rtp_access_token cookie, fall back to header.
	// Old: headers: { Authorization: request.headers.get('Authorization') || '' }
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) error(401, 'Unauthorized');

	// Try backend first
	const backendData = await fetchFromBackend(
		`/api/admin/trading-rooms/traders?${url.searchParams.toString()}`,
		{
			headers: { Authorization: `Bearer ${token}` }
		}
	);

	if (backendData?.success) {
		return json(backendData);
	}

	// FIX-2026-04-26-audit (P1-11): surface backend failure instead of returning phantom
	// mock traders (Mike/Sarah/James/Emily). Admins were seeing fake dropdowns on any
	// backend hiccup with no indication that real data was unavailable.
	// TODO(2026-04-26-audit): gate mock on env.ENABLE_MOCK_DATA if needed for local dev.
	console.error('[Trading-rooms traders proxy] Backend unavailable or non-success:', backendData);
	return json(
		{
			success: false,
			error: 'Unable to load traders from backend. Please check the API connection.',
			_mock: false
		},
		{ status: 502 }
	);
};

// POST - Create trader
export const POST: RequestHandler = async ({ request, cookies }) => {
	const body = await request.json();

	// FIX-2026-04-26: prefer canonical rtp_access_token cookie, fall back to header.
	// Old: headers: { Authorization: request.headers.get('Authorization') || '' }
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) error(401, 'Unauthorized');

	// Try backend first
	const backendData = await fetchFromBackend('/api/admin/trading-rooms/traders', {
		method: 'POST',
		headers: { Authorization: `Bearer ${token}` },
		body: JSON.stringify(body)
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// FIX-2026-04-26-audit (P1-11): surface backend failure on POST — a silent mock-create
	// would persist nothing to the DB while appearing to succeed.
	console.error('[Trading-rooms traders proxy POST] Backend unavailable or non-success:', backendData);
	return json(
		{
			success: false,
			error: 'Unable to create trader — backend is unavailable. Please try again.'
		},
		{ status: 502 }
	);
};

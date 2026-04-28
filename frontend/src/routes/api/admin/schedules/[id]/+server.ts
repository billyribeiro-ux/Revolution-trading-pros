/**
 * Admin Schedules API - Individual Schedule Endpoint
 * ═══════════════════════════════════════════════════════════════════════════════════
 *
 * Handles individual schedule operations: GET, PUT, DELETE.
 *
 * FIX-2026-04-26 (P0-1): Removed mock fallback that fabricated `success: true`
 * responses for missing/failed schedules. Now forwards backend status verbatim.
 *
 * @version 2.0.0
 */

import { json, error as kitError } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

// ═══════════════════════════════════════════════════════════════════════════
// BACKEND HELPER
// ═══════════════════════════════════════════════════════════════════════════

interface BackendResult {
	data: unknown;
	status: number;
	reachable: boolean;
}

async function callBackend(endpoint: string, options?: RequestInit): Promise<BackendResult> {
	if (!BACKEND_URL) {
		return { data: null, status: 0, reachable: false };
	}

	try {
		const response = await fetch(`${BACKEND_URL}${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...options?.headers
			}
		});

		let parsed: unknown = null;
		const text = await response.text();
		if (text) {
			try {
				parsed = JSON.parse(text);
			} catch {
				parsed = { message: text };
			}
		}
		return { data: parsed, status: response.status, reachable: true };
	} catch (err) {
		console.warn(`Schedule backend unreachable for ${endpoint}:`, err);
		return { data: null, status: 0, reachable: false };
	}
}

function extractErrorMessage(data: unknown, fallback: string): string {
	if (data && typeof data === 'object') {
		const obj = data as { message?: unknown; error?: unknown };
		if (typeof obj.message === 'string') return obj.message;
		if (typeof obj.error === 'string') return obj.error;
	}
	return fallback;
}

// ═══════════════════════════════════════════════════════════════════════════
// GET - Get single schedule by ID
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, request, cookies }) => {
	const id = parseInt(params.id || '0');

	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) kitError(401, 'Unauthorized');

	const result = await callBackend(`/api/admin/schedules/${id}`, {
		headers: { Authorization: `Bearer ${token}` }
	});

	if (result.reachable && result.status >= 200 && result.status < 300) {
		return json(result.data);
	}

	if (result.reachable) {
		kitError(result.status, extractErrorMessage(result.data, 'Schedule not found'));
	}

	return json(
		{ success: false, error: 'Schedules backend is not reachable.', _degraded: true },
		{ status: 503 }
	);
};

// ═══════════════════════════════════════════════════════════════════════════
// PUT - Update schedule
// ═══════════════════════════════════════════════════════════════════════════

export const PUT: RequestHandler = async ({ params, request, cookies }) => {
	const id = parseInt(params.id || '0');
	const body = await request.json();

	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) kitError(401, 'Unauthorized');

	const result = await callBackend(`/api/admin/schedules/${id}`, {
		method: 'PUT',
		headers: { Authorization: `Bearer ${token}` },
		body: JSON.stringify(body)
	});

	if (result.reachable && result.status >= 200 && result.status < 300) {
		return json(result.data);
	}

	if (result.reachable) {
		kitError(result.status, extractErrorMessage(result.data, 'Failed to update schedule'));
	}

	return json(
		{
			success: false,
			error: 'Schedules backend is not reachable. Schedule was NOT updated.',
			_degraded: true
		},
		{ status: 503 }
	);
};

// ═══════════════════════════════════════════════════════════════════════════
// DELETE - Delete schedule
// ═══════════════════════════════════════════════════════════════════════════

export const DELETE: RequestHandler = async ({ params, request, cookies }) => {
	const id = parseInt(params.id || '0');

	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) kitError(401, 'Unauthorized');

	const result = await callBackend(`/api/admin/schedules/${id}`, {
		method: 'DELETE',
		headers: { Authorization: `Bearer ${token}` }
	});

	if (result.reachable && result.status >= 200 && result.status < 300) {
		return json(result.data);
	}

	if (result.reachable) {
		kitError(result.status, extractErrorMessage(result.data, 'Failed to delete schedule'));
	}

	return json(
		{
			success: false,
			error: 'Schedules backend is not reachable. Schedule was NOT deleted.',
			_degraded: true
		},
		{ status: 503 }
	);
};

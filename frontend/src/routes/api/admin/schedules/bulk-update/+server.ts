/**
 * Admin Schedules API - Bulk Update Endpoint
 * ═══════════════════════════════════════════════════════════════════════════════════
 *
 * FIX-2026-04-26 (P0-2): Removed mock fallback that fabricated `updated_count: ids.length`
 * regardless of what the backend actually did. Now forwards upstream counts verbatim.
 *
 * @version 2.0.0
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { requireSuperadmin } from '$lib/server/auth';

import { env } from '$env/dynamic/private';
const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

interface BackendResult {
	data: unknown;
	status: number;
	reachable: boolean;
}

async function callBackend(endpoint: string, options?: RequestInit): Promise<BackendResult> {
	if (!BACKEND_URL) return { data: null, status: 0, reachable: false };

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
		console.warn(`Schedule bulk-update backend unreachable:`, err);
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

export const POST: RequestHandler = async (event) => {
	const { token } = requireSuperadmin(event);
	const body = await event.request.json();
	const { ids, data } = body;

	if (!ids || !Array.isArray(ids) || ids.length === 0) {
		return json(
			{ success: false, error: 'Missing or invalid ids array' },
			{ status: 400 }
		);
	}

	if (!data || typeof data !== 'object') {
		return json(
			{ success: false, error: 'Missing or invalid data object' },
			{ status: 400 }
		);
	}

	const result = await callBackend('/api/admin/schedules/bulk-update', {
		method: 'POST',
		headers: { Authorization: `Bearer ${token}` },
		body: JSON.stringify(body)
	});

	// FIX-2026-04-26 (P0-2): Forward upstream {updated_count, updated_ids, failed_ids}
	// without fabricating counts in the proxy.
	if (result.reachable && result.status >= 200 && result.status < 300) {
		return json(result.data);
	}

	if (result.reachable) {
		error(result.status, extractErrorMessage(result.data, 'Failed to update schedules'));
	}

	return json(
		{
			success: false,
			error: 'Schedules backend is not reachable. No schedules were updated.',
			_degraded: true
		},
		{ status: 503 }
	);
};

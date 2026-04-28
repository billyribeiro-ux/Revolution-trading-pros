/**
 * Admin Schedules API Endpoint
 * ═══════════════════════════════════════════════════════════════════════════════════
 *
 * Handles trading room schedule management with full CRUD operations.
 *
 * FIX-2026-04-26 (P0-1): The previous implementation tried the backend, then on any
 * non-2xx response (including 4xx validation, 5xx server errors) silently fell back
 * to a process-scoped in-memory mock array and returned `success: true`. This lied
 * to clients about persistence (mock writes are lost on every dyno cycle and not
 * shared between proxy files). The new contract:
 *
 *   - Always proxy to the Rust backend.
 *   - Forward upstream status + body verbatim on errors so the frontend can render
 *     a real toast/alert instead of seeing fake success.
 *   - Only fall through to a stub when the backend is unreachable (network error or
 *     no `BACKEND_URL` configured) — and even then, return 503 so the caller knows
 *     the data was NOT persisted.
 *
 * @version 2.0.0
 */

import { json, error as kitError } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

// ═══════════════════════════════════════════════════════════════════════════
// TYPE DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════════

interface ScheduleEvent {
	id: number;
	room_id: string;
	title: string;
	description: string | null;
	trader_name: string | null;
	day_of_week: number;
	start_time: string;
	end_time: string;
	timezone: string;
	is_active: boolean;
	room_type: 'live' | 'recorded' | 'hybrid';
	recurrence: 'weekly' | 'biweekly' | 'monthly' | null;
	exceptions: ScheduleException[];
	created_at: string;
	updated_at: string;
}

interface ScheduleException {
	id: number;
	schedule_id: number;
	date: string;
	type: string;
	reason: string | null;
}

// TODO(2026-04-26-audit): Remove the read-only seed list below once the Rust
// backend ships GET /api/admin/schedules. Today the backend route is missing,
// so we keep this as a *read-only* seed for the room selector to render. We do
// NOT mutate this list anymore — every write is propagated to the backend or
// returns a real error.
const SEED_SCHEDULES: ReadonlyArray<ScheduleEvent> = [];

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
// GET - List schedules (with optional room_id filter)
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ url, request, cookies }) => {
	const roomId = url.searchParams.get('room_id');
	const activeOnly = url.searchParams.get('active_only') === 'true';
	const dayOfWeek = url.searchParams.get('day_of_week');

	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) kitError(401, 'Unauthorized');

	const result = await callBackend(`/api/admin/schedules?${url.searchParams.toString()}`, {
		headers: { Authorization: `Bearer ${token}` }
	});

	if (result.reachable && result.status >= 200 && result.status < 300) {
		return json(result.data);
	}

	// FIX-2026-04-26 (P0-1): if the backend is reachable but returned an error,
	// surface the upstream status instead of swallowing into a fake-success mock.
	if (result.reachable) {
		kitError(result.status, extractErrorMessage(result.data, 'Failed to load schedules'));
	}

	// Backend unreachable (no network / no env). Render seed list read-only so the
	// admin UI doesn't blow up, but flag clearly that this is degraded mode.
	let filtered = [...SEED_SCHEDULES];
	if (roomId) filtered = filtered.filter((s) => s.room_id === roomId);
	if (activeOnly) filtered = filtered.filter((s) => s.is_active);
	if (dayOfWeek !== null && dayOfWeek !== '') {
		filtered = filtered.filter((s) => s.day_of_week === parseInt(dayOfWeek));
	}
	filtered.sort((a, b) =>
		a.day_of_week !== b.day_of_week
			? a.day_of_week - b.day_of_week
			: a.start_time.localeCompare(b.start_time)
	);

	return json(
		{
			success: false,
			data: filtered,
			meta: { total: filtered.length, room_id: roomId },
			_degraded: true,
			error: 'Schedules backend is not reachable. Showing read-only seed data.'
		},
		{ status: 503 }
	);
};

// ═══════════════════════════════════════════════════════════════════════════
// POST - Create schedule
// ═══════════════════════════════════════════════════════════════════════════

export const POST: RequestHandler = async ({ request, cookies }) => {
	const body = await request.json();

	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) kitError(401, 'Unauthorized');

	if (
		!body.room_id ||
		!body.title ||
		body.day_of_week === undefined ||
		body.day_of_week === null ||
		!body.start_time ||
		!body.end_time
	) {
		return json(
			{
				success: false,
				error: 'Missing required fields: room_id, title, day_of_week, start_time, end_time'
			},
			{ status: 400 }
		);
	}

	const result = await callBackend('/api/admin/schedules', {
		method: 'POST',
		headers: { Authorization: `Bearer ${token}` },
		body: JSON.stringify(body)
	});

	if (result.reachable && result.status >= 200 && result.status < 300) {
		return json(result.data);
	}

	if (result.reachable) {
		// FIX-2026-04-26 (P0-1): never lie about persistence. Forward the real error.
		kitError(result.status, extractErrorMessage(result.data, 'Failed to create schedule'));
	}

	// Backend unreachable — refuse the write rather than fabricate success.
	return json(
		{
			success: false,
			error: 'Schedules backend is not reachable. Schedule was NOT created.',
			_degraded: true
		},
		{ status: 503 }
	);
};

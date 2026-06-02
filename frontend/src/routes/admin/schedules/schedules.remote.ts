/**
 * Admin Schedules — Remote Functions (query + commands)
 * ─────────────────────────────────────────────────────────────────────────────
 * Replaces the page's raw `fetch('/api/admin/schedules…')` calls with typed
 * remote functions. Consumed **imperatively** (the page keeps `loadSchedules()`
 * and re-loads after each mutation), matching the existing architecture — so the
 * commands deliberately do NOT add a server-side `getSchedules(roomId).refresh()`
 * single-flight: with no reactive `.current` reader it would be dead code (see
 * the RF-2 dead-refresh cleanup). The page's explicit reload is the complete and
 * correct refresh path here.
 *
 * `postSchedule` backs both create and duplicate (a duplicate is a create with a
 * tweaked payload); `putSchedule` backs both edit and the active-toggle (a toggle
 * is a PUT with just `{ is_active }`). Both surface the backend's error/message
 * so the page's existing error display is preserved.
 *
 * Auth: `getRequestEvent().fetch` forwards the request cookies to the admin
 * schedules proxies (`requireAdmin`). The page's old client-side `401 →
 * /login` redirect is dropped: `admin/+layout.server.ts` + `hooks.server.ts`
 * already enforce admin auth server-side (the layout's own comment notes the
 * client redirect was bypassable and was superseded by the server guard).
 */
import * as v from 'valibot';
import { error } from '@sveltejs/kit';
import { command, getRequestEvent, query } from '$app/server';
import type { ScheduleEvent } from './_components/types';

const RoomIdSchema = v.pipe(v.string(), v.nonEmpty());
const IdSchema = v.pipe(v.number(), v.integer(), v.minValue(1));
// Form/duplicate payloads are validated by the backend; accept any object here.
const PayloadSchema = v.record(v.string(), v.unknown());

/** Surface the backend's `{ error | message }` on a failed mutation. */
async function failFrom(res: Response, fallback: string): Promise<never> {
	let message = fallback;
	try {
		const body = await res.json();
		message = body?.error || body?.message || fallback;
	} catch {
		/* non-JSON body — keep fallback */
	}
	error(res.status, message);
}

export const getSchedules = query(RoomIdSchema, async (roomId): Promise<ScheduleEvent[]> => {
	const { fetch } = getRequestEvent();
	const res = await fetch(`/api/admin/schedules?room_id=${encodeURIComponent(roomId)}`);
	if (!res.ok) error(res.status, 'Failed to load schedules');
	const body = await res.json();
	const rows = body?.data ?? body?.schedules ?? [];
	return Array.isArray(rows) ? (rows as ScheduleEvent[]) : [];
});

/** Create (or duplicate) a schedule. */
export const postSchedule = command(PayloadSchema, async (payload) => {
	const { fetch } = getRequestEvent();
	const res = await fetch('/api/admin/schedules', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(payload)
	});
	if (!res.ok) await failFrom(res, 'Failed to create schedule');
});

/** Update a schedule (full edit, or a partial like `{ is_active }` for toggles). */
export const putSchedule = command(
	v.object({ id: IdSchema, data: PayloadSchema }),
	async ({ id, data }) => {
		const { fetch } = getRequestEvent();
		const res = await fetch(`/api/admin/schedules/${id}`, {
			method: 'PUT',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(data)
		});
		if (!res.ok) await failFrom(res, 'Failed to update schedule');
	}
);

export const deleteScheduleById = command(IdSchema, async (id) => {
	const { fetch } = getRequestEvent();
	const res = await fetch(`/api/admin/schedules/${id}`, { method: 'DELETE' });
	if (!res.ok) error(res.status, 'Failed to delete schedule');
});

export const bulkDeleteSchedules = command(v.array(IdSchema), async (ids) => {
	const { fetch } = getRequestEvent();
	const res = await fetch('/api/admin/schedules/bulk-delete', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ ids })
	});
	if (!res.ok) error(res.status, 'Failed to delete schedules');
});

export const bulkUpdateSchedules = command(
	v.object({ ids: v.array(IdSchema), data: PayloadSchema }),
	async ({ ids, data }) => {
		const { fetch } = getRequestEvent();
		const res = await fetch('/api/admin/schedules/bulk-update', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ ids, data })
		});
		if (!res.ok) error(res.status, 'Failed to update schedules');
	}
);

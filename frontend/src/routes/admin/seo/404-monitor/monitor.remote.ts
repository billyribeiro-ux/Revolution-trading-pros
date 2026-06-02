/**
 * 404 Monitor — Remote Functions (queries + commands w/ single-flight)
 * ─────────────────────────────────────────────────────────────────────────────
 * Replaces the page's raw `fetch` + manual `loadLogs()/loadStats()` re-fetch
 * dance with typed remote functions:
 *   - `getLogs(sort)` / `getStats()` — reactive queries the page reads via
 *     `.current` (changing the sort re-runs `getLogs`).
 *   - `ignoreLog` / `bulkDeleteLogs` — commands that, after the mutation, perform
 *     a JUNE-2026 server-driven single-flight refresh: they call
 *     `getLogs(sort).refresh()` + `getStats().refresh()` so the updated list and
 *     stats ride back on the SAME response — no second round-trip, no manual
 *     re-fetch. `sort` is threaded through the command so the refresh targets the
 *     exact `getLogs(sort)` instance the client is rendering.
 *
 * Auth: `getRequestEvent().fetch('/api/seo/404-logs…')` forwards the request
 * cookies to the existing SEO proxies — the same path the old client `fetch`
 * took.
 */
import * as v from 'valibot';
import { error } from '@sveltejs/kit';
import { command, getRequestEvent, query } from '$app/server';
import type { Seo404Log, Seo404Stats } from './monitor.types';

const SortSchema = v.optional(v.picklist(['hits', 'latest', 'oldest']), 'hits');
const IdSchema = v.pipe(v.number(), v.integer(), v.minValue(1));

export const getLogs = query(SortSchema, async (sort): Promise<Seo404Log[]> => {
	const { fetch } = getRequestEvent();
	const res = await fetch(`/api/seo/404-logs?sort=${sort}`);
	if (!res.ok) return [];
	const body = await res.json();
	return Array.isArray(body?.data) ? (body.data as Seo404Log[]) : [];
});

export const getStats = query(async (): Promise<Seo404Stats | null> => {
	const { fetch } = getRequestEvent();
	const res = await fetch('/api/seo/404-logs/stats');
	if (!res.ok) return null;
	return (await res.json()) as Seo404Stats;
});

/** Ignore one log, then single-flight refresh the list + stats. */
export const ignoreLog = command(
	v.object({ id: IdSchema, sort: SortSchema }),
	async ({ id, sort }) => {
		const { fetch } = getRequestEvent();
		const res = await fetch(`/api/seo/404-logs/${id}/ignore`, { method: 'POST' });
		if (!res.ok) error(res.status, 'Failed to ignore log');

		await getLogs(sort).refresh();
		await getStats().refresh();
	}
);

/** Bulk-delete logs, then single-flight refresh the list + stats. */
export const bulkDeleteLogs = command(
	v.object({ ids: v.array(IdSchema), sort: SortSchema }),
	async ({ ids, sort }) => {
		const { fetch } = getRequestEvent();
		const res = await fetch('/api/seo/404-logs/bulk-delete', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ ids })
		});
		if (!res.ok) error(res.status, 'Failed to delete logs');

		await getLogs(sort).refresh();
		await getStats().refresh();
	}
);

/**
 * Redirect Manager — Remote Functions (queries + commands w/ single-flight)
 * ─────────────────────────────────────────────────────────────────────────────
 * `getRedirects()` / `getRedirectStats()` are no-arg reactive queries the page
 * reads via `.current`. `removeRedirect` / `toggleRedirectActive` /
 * `bulkRemoveRedirects` mutate, then single-flight refresh both queries on the
 * server so the table and stat cards update on the same response (the queries
 * take no argument, so the refresh targets the single client instance directly).
 *
 * Auth: `getRequestEvent().fetch('/api/seo/redirects…')` forwards the request
 * cookies to the existing SEO proxies.
 */
import * as v from 'valibot';
import { error } from '@sveltejs/kit';
import { command, getRequestEvent, query } from '$app/server';
import type { Redirect, RedirectStats } from './redirects.types';

const IdSchema = v.pipe(v.number(), v.integer(), v.minValue(1));

export const getRedirects = query(async (): Promise<Redirect[]> => {
	const { fetch } = getRequestEvent();
	const res = await fetch('/api/seo/redirects');
	if (!res.ok) return [];
	const body = await res.json();
	return Array.isArray(body?.data) ? (body.data as Redirect[]) : [];
});

export const getRedirectStats = query(async (): Promise<RedirectStats | null> => {
	const { fetch } = getRequestEvent();
	const res = await fetch('/api/seo/redirects/stats');
	if (!res.ok) return null;
	return (await res.json()) as RedirectStats;
});

async function refreshAll() {
	await getRedirects().refresh();
	await getRedirectStats().refresh();
}

/** Delete one redirect, then single-flight refresh the list + stats. */
export const removeRedirect = command(IdSchema, async (id) => {
	const { fetch } = getRequestEvent();
	const res = await fetch(`/api/seo/redirects/${id}`, { method: 'DELETE' });
	if (!res.ok) error(res.status, 'Failed to delete redirect');
	await refreshAll();
});

/** Toggle a redirect's active state, then single-flight refresh the list + stats. */
export const toggleRedirectActive = command(IdSchema, async (id) => {
	const { fetch } = getRequestEvent();
	const res = await fetch(`/api/seo/redirects/${id}/toggle`, { method: 'POST' });
	if (!res.ok) error(res.status, 'Failed to toggle redirect');
	await refreshAll();
});

/** Bulk-delete redirects, then single-flight refresh the list + stats. */
export const bulkRemoveRedirects = command(v.array(IdSchema), async (ids) => {
	const { fetch } = getRequestEvent();
	const res = await fetch('/api/seo/redirects/bulk-delete', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ ids })
	});
	if (!res.ok) error(res.status, 'Failed to delete redirects');
	await refreshAll();
});

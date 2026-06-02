/**
 * Keyword Tracking — Remote Functions (queries + command w/ single-flight)
 * ─────────────────────────────────────────────────────────────────────────────
 * `getKeywords()` / `getKeywordStats()` are no-arg reactive queries the page
 * reads via `.current`. `removeKeyword` deletes one, then single-flight refreshes
 * both queries on the server so the table and stat cards update on the same
 * response.
 *
 * Auth: `getRequestEvent().fetch('/api/seo/keywords…')` forwards the request
 * cookies to the existing SEO proxies.
 */
import * as v from 'valibot';
import { error } from '@sveltejs/kit';
import { command, getRequestEvent, query } from '$app/server';
import type { Keyword, KeywordStats } from './keywords.types';

const IdSchema = v.pipe(v.number(), v.integer(), v.minValue(1));

export const getKeywords = query(async (): Promise<Keyword[]> => {
	const { fetch } = getRequestEvent();
	const res = await fetch('/api/seo/keywords');
	if (!res.ok) return [];
	const body = await res.json();
	return Array.isArray(body?.data) ? (body.data as Keyword[]) : [];
});

export const getKeywordStats = query(async (): Promise<KeywordStats | null> => {
	const { fetch } = getRequestEvent();
	const res = await fetch('/api/seo/keywords/stats');
	if (!res.ok) return null;
	return (await res.json()) as KeywordStats;
});

/** Delete one keyword, then single-flight refresh the list + stats. */
export const removeKeyword = command(IdSchema, async (id) => {
	const { fetch } = getRequestEvent();
	const res = await fetch(`/api/seo/keywords/${id}`, { method: 'DELETE' });
	if (!res.ok) error(res.status, 'Failed to delete keyword');

	await getKeywords().refresh();
	await getKeywordStats().refresh();
});

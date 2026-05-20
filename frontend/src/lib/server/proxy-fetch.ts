/**
 * Shared helper for SvelteKit `+server.ts` proxy `fetchFromBackend` calls.
 *
 * Background — R18-A / R19-A:
 *   13+ proxy files declare a near-identical async helper that:
 *     - reads BACKEND_URL from $env/dynamic/private (with the CLAUDE.md
 *       hard-rule fallback chain `API_BASE_URL || BACKEND_URL || localhost`)
 *     - fetches with JSON headers
 *     - returns `null` on non-2xx or network failure
 *     - logs a tagged info/error line at each transition
 *
 *   Each copy was typed `Promise<any | null>` — 6 of those have been
 *   converted to `Promise<unknown>` in R18-A; R19-A converts the next 7
 *   and routes them through this shared helper instead of copy-pasting
 *   the body again.
 *
 * CLAUDE.md hard rule (project-level):
 *   Every proxy reads `env.API_BASE_URL || env.BACKEND_URL ||
 *   'http://localhost:8080'`. This module is the single point of truth
 *   for that chain across every proxy that adopts it.
 *
 * Usage:
 *   import { fetchBackend, hasSuccess } from '$lib/server/proxy-fetch';
 *
 *   const backendData = await fetchBackend(
 *     `/api/room-content/rooms/${slug}/alerts`,
 *     { headers },
 *     '[Alerts API]'
 *   );
 *   if (hasSuccess(backendData) && backendData.success) { … }
 */

import { env } from '$env/dynamic/private';

// CLAUDE.md hard rule — API_BASE_URL primary, BACKEND_URL fallback,
// localhost last (dev-only). Do NOT collapse to BACKEND_URL alone.
const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

/**
 * Fetch from the backend and return the parsed JSON body on 2xx, or
 * `null` on any non-2xx / network failure. Returns `unknown` rather than
 * `any` — callers MUST narrow before reading fields. See {@link hasSuccess},
 * {@link hasData}, {@link isObject} below for the narrowing helpers.
 *
 * @param endpoint - path-only URL, joined to API_URL (e.g. `/api/foo`)
 * @param options  - standard `fetch` `RequestInit`. Caller controls method,
 *                   body, headers. JSON `Content-Type` and `Accept` are
 *                   injected unless the caller already set them.
 * @param logTag   - bracketed tag prefix for console.info/error lines
 *                   (e.g. `[Alerts API]`). Pass an empty string to suppress.
 */
export async function fetchBackend(
	endpoint: string,
	options: RequestInit = {},
	logTag: string = '[Backend]'
): Promise<unknown> {
	const url = `${API_URL}${endpoint}`;
	try {
		if (logTag) console.info(`${logTag} Fetching: ${url}`);
		const response = await fetch(url, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...options.headers
			}
		});

		if (!response.ok) {
			if (logTag) {
				console.error(
					`${logTag} Backend error: ${response.status} ${response.statusText}`
				);
			}
			return null;
		}

		return await response.json();
	} catch (err) {
		if (logTag) console.error(`${logTag} Backend fetch failed:`, err);
		return null;
	}
}

/**
 * Type-guard: is `value` a non-null object (vs. primitive / null / array)?
 *
 * Note: arrays are objects too — this returns `true` for arrays. Use
 * `Array.isArray(value)` when array-specific narrowing matters.
 */
export function isObject(value: unknown): value is Record<string, unknown> {
	return typeof value === 'object' && value !== null;
}

/**
 * Type-guard: does `value` have a `success` field? Replaces the unsound
 * `backendData?.success` pattern (which threw if `backendData` was a
 * non-null primitive).
 */
export function hasSuccess(value: unknown): value is { success: unknown } {
	return isObject(value) && 'success' in value;
}

/**
 * Type-guard: does `value` have a `data` field? Replaces the unsound
 * `backendData?.data` pattern.
 */
export function hasData(value: unknown): value is { data: unknown } {
	return isObject(value) && 'data' in value;
}

/**
 * Read the `meta.total` field from an envelope shape if present, else
 * return the fallback. Encapsulates the `backendData.meta?.total ||
 * fallback` pattern used in ~5 proxies.
 */
export function extractMetaTotal(value: unknown, fallback: number): number {
	if (!isObject(value) || !('meta' in value)) return fallback;
	const meta = value.meta;
	if (!isObject(meta) || !('total' in meta)) return fallback;
	const total = meta.total;
	return typeof total === 'number' ? total : fallback;
}

/**
 * Read the `data` field from an envelope shape if present, else return
 * the value itself. This preserves the legacy `backendData.data ||
 * backendData` short-circuit BUT fixes the bug where `data: null`
 * accidentally fell through to the envelope (Latent Bug §3 in R18-A).
 *
 * If `data` is present (even if `null`), it is returned — that's the
 * backend telling us "no data here". If `data` is absent, the raw value
 * is returned for endpoints that don't wrap.
 */
export function extractBackendData(value: unknown): unknown {
	if (hasData(value)) return value.data;
	return value;
}

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
 *
 * Why `interface XBody + as XBody`, not zod/valibot (R22-A decision):
 *   Proxy layer is hot-path SSR — adding a runtime validator (zod ~12 KB,
 *   valibot ~4 KB modular) lifts every cold-start render and every
 *   `+server.ts` import. Backend already validates with serde; the proxy's
 *   only job is forward-and-narrow. `isObject` + structural cast catches
 *   the "client posted `null`" 500-NPE class (R18-A §2) at zero runtime cost.
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
 * Status-preserving variant of {@link fetchBackend}.
 *
 * Background — R21-A:
 *   6 admin proxies (`admin/email/{settings,templates}`, `admin/posts`,
 *   `admin/forms`, `admin/trading-rooms`, `admin/categories`) each declared
 *   a near-identical helper that returned `{ data, status }` so the proxy
 *   could forward the upstream HTTP status (and the upstream error
 *   payload's `.message` / `.error`) to the client via SvelteKit's
 *   `error(status, msg)`. `fetchBackend` collapses every non-2xx to `null`,
 *   losing both status and body — useless for admin handlers that need to
 *   surface a backend 400/422 with the validation message.
 *
 *   This helper preserves both. On network failure / unreachable backend
 *   it returns `{ data: null, status: 500 }` (same fallback the 6 local
 *   copies used).
 *
 * Body parsing strategy (R21-A robustness fix):
 *   The 6 local copies split into two sub-patterns:
 *     - `email/settings`, `forms` — `text()` then `JSON.parse(text)` with
 *       a try/catch and a `{ error: text }` fallback for non-JSON bodies.
 *     - `email/templates`, `posts`, `trading-rooms`, `categories` — bare
 *       `response.json()` which THROWS on empty 204 / non-JSON 5xx and
 *       triggers the outer catch → `{ data: null, status: 500 }`. That
 *       silently mints a 500 on what was actually a 204/422-with-text, so
 *       the proxy returns the WRONG status to the client.
 *
 *   Consolidating to the resilient `text() → try-JSON.parse` shape is a
 *   strict improvement: every former call site now gets the correct
 *   upstream status even on empty/non-JSON bodies. Latent Bug §1 fixed
 *   inline.
 *
 * CLAUDE.md hard rule: same URL-fallback chain as {@link fetchBackend} —
 * single source of truth in this module.
 *
 * @returns `{ data, status }` where `data` is the parsed JSON body, or
 *   `{ error: rawText }` for non-JSON bodies, or `null` for empty bodies /
 *   network failures. `status` is the upstream HTTP status, or `500` on
 *   network failure.
 */
export async function fetchBackendWithStatus(
	endpoint: string,
	options: RequestInit = {},
	logTag: string = '[Backend]'
): Promise<{ data: unknown; status: number }> {
	const url = `${API_URL}${endpoint}`;
	try {
		const response = await fetch(url, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...options.headers
			}
		});

		// Tolerate empty 204 / non-JSON 5xx bodies. The 6 R21-A migration
		// candidates split on this — the resilient pattern is now canonical.
		const text = await response.text();
		let data: unknown = null;
		if (text) {
			try {
				data = JSON.parse(text);
			} catch {
				data = { error: text };
			}
		}
		return { data, status: response.status };
	} catch (err) {
		if (logTag) console.error(`${logTag} Backend fetch failed for ${endpoint}:`, err);
		return { data: null, status: 500 };
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

/**
 * Extract a human-readable error message from a backend error response
 * body, falling back to a caller-supplied default. Replaces the 6+
 * copy-pasted blocks that read `.message` or `.error` off `unknown`
 * with manual `typeof === 'object'` narrowing.
 *
 * Lookup order:
 *   1. `value.message` if it's a non-empty string
 *   2. `value.error`   if it's a non-empty string
 *   3. `fallback`
 *
 * Used in the `error(status, msg)` branches of status-preserving proxies
 * so the backend's validation message reaches the client.
 */
export function extractBackendErrorMessage(
	value: unknown,
	fallback: string
): string {
	if (!isObject(value)) return fallback;
	const message = value.message;
	if (typeof message === 'string' && message.length > 0) return message;
	const errorField = value.error;
	if (typeof errorField === 'string' && errorField.length > 0) return errorField;
	return fallback;
}

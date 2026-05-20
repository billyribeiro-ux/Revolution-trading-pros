/**
 * proxy-fetch — Unit Tests (R24-D)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * The proxy-fetch module is the shared SvelteKit `+server.ts` helper used by
 * 24+ proxy files (audit grep on 2026-05-20). It is the single source of
 * truth for the CLAUDE.md hard rule:
 *     `env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080'`
 *
 * Why these tests matter:
 *   - Type-guards (hasSuccess / hasData / isObject) replace unsound
 *     `?.success` patterns that threw on non-null primitives — must
 *     return `false` (not throw) for `0`, `""`, arrays, `null`.
 *   - extractMetaTotal / extractBackendData / extractBackendErrorMessage
 *     encapsulate copy-pasted shapes from 6+ proxies. Fixing them once
 *     must keep every former call site behaviorally identical.
 *   - fetchBackend collapses non-2xx → null (callers depend on this).
 *   - fetchBackendWithStatus preserves status + tolerates 204/non-JSON
 *     bodies (Latent Bug §1 in R21-A — regression here would silently
 *     mint 500s on legitimate 204/422-with-text responses).
 *
 * Mocking note: `$env/dynamic/private` is a SvelteKit virtual module.
 * We mock it to keep API_URL deterministic across tests.
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

vi.mock('$env/dynamic/private', () => ({
	env: {
		API_BASE_URL: 'https://test-api.example.com',
		BACKEND_URL: undefined
	}
}));

import {
	fetchBackend,
	fetchBackendWithStatus,
	isObject,
	hasSuccess,
	hasData,
	extractMetaTotal,
	extractBackendData,
	extractBackendErrorMessage
} from '../proxy-fetch';

// ═══════════════════════════════════════════════════════════════════════════
// Type-guard tests
// ═══════════════════════════════════════════════════════════════════════════

describe('isObject', () => {
	it('returns true for a plain object', () => {
		expect(isObject({})).toBe(true);
		expect(isObject({ foo: 'bar' })).toBe(true);
	});

	it('returns true for arrays (documented contract)', () => {
		// Documented in proxy-fetch.ts: arrays are objects too. Callers
		// who need array-specific narrowing must use Array.isArray.
		expect(isObject([])).toBe(true);
		expect(isObject([1, 2, 3])).toBe(true);
	});

	it('returns false for null', () => {
		expect(isObject(null)).toBe(false);
	});

	it('returns false for primitives', () => {
		expect(isObject(undefined)).toBe(false);
		expect(isObject(0)).toBe(false);
		expect(isObject('')).toBe(false);
		expect(isObject('foo')).toBe(false);
		expect(isObject(true)).toBe(false);
		expect(isObject(false)).toBe(false);
	});
});

describe('hasSuccess', () => {
	it('returns true when value is an object with a `success` field', () => {
		expect(hasSuccess({ success: true })).toBe(true);
		expect(hasSuccess({ success: false })).toBe(true);
		// Field presence — not truthiness — is what we check.
		expect(hasSuccess({ success: null })).toBe(true);
		expect(hasSuccess({ success: 0 })).toBe(true);
	});

	it('returns false when value is missing `success` field', () => {
		expect(hasSuccess({})).toBe(false);
		expect(hasSuccess({ data: 'x' })).toBe(false);
	});

	it('returns false (does NOT throw) for non-objects', () => {
		// Replaces unsound `value?.success` which threw on non-null primitives.
		expect(hasSuccess(null)).toBe(false);
		expect(hasSuccess(undefined)).toBe(false);
		expect(hasSuccess(42)).toBe(false);
		expect(hasSuccess('success')).toBe(false);
	});
});

describe('hasData', () => {
	it('returns true when value is an object with a `data` field', () => {
		expect(hasData({ data: [] })).toBe(true);
		expect(hasData({ data: null })).toBe(true);
		expect(hasData({ data: { id: 1 } })).toBe(true);
	});

	it('returns false when value is missing `data` field', () => {
		expect(hasData({})).toBe(false);
		expect(hasData({ success: true })).toBe(false);
	});

	it('returns false for non-objects', () => {
		expect(hasData(null)).toBe(false);
		expect(hasData(undefined)).toBe(false);
		expect(hasData('data')).toBe(false);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// Envelope extraction tests
// ═══════════════════════════════════════════════════════════════════════════

describe('extractMetaTotal', () => {
	it('returns meta.total when present and numeric', () => {
		expect(extractMetaTotal({ meta: { total: 42 } }, 0)).toBe(42);
		expect(extractMetaTotal({ meta: { total: 0 } }, 100)).toBe(0);
	});

	it('returns fallback when value is not an object', () => {
		expect(extractMetaTotal(null, 7)).toBe(7);
		expect(extractMetaTotal(undefined, 7)).toBe(7);
		expect(extractMetaTotal('not an object', 7)).toBe(7);
	});

	it('returns fallback when meta is missing or wrong shape', () => {
		expect(extractMetaTotal({}, 7)).toBe(7);
		expect(extractMetaTotal({ meta: null }, 7)).toBe(7);
		expect(extractMetaTotal({ meta: 'not an object' }, 7)).toBe(7);
	});

	it('returns fallback when total is missing or non-numeric', () => {
		expect(extractMetaTotal({ meta: {} }, 7)).toBe(7);
		expect(extractMetaTotal({ meta: { total: '42' } }, 7)).toBe(7);
		expect(extractMetaTotal({ meta: { total: null } }, 7)).toBe(7);
	});
});

describe('extractBackendData', () => {
	it('returns value.data when present', () => {
		expect(extractBackendData({ data: [1, 2, 3] })).toEqual([1, 2, 3]);
	});

	it('returns value.data even when null (R18-A Latent Bug §3 fix)', () => {
		// The legacy `backendData.data || backendData` short-circuit had a bug:
		// `data: null` accidentally fell through to the envelope. The fix:
		// presence beats truthiness — null `data` is the backend telling us
		// "no data here" and we MUST surface that.
		expect(extractBackendData({ data: null, meta: { total: 0 } })).toBeNull();
	});

	it('returns the raw value when `data` is absent', () => {
		// For endpoints that don't wrap their response in `{ data: ... }`.
		expect(extractBackendData([1, 2, 3])).toEqual([1, 2, 3]);
		expect(extractBackendData({ id: 1, name: 'foo' })).toEqual({ id: 1, name: 'foo' });
	});

	it('returns primitives unchanged', () => {
		expect(extractBackendData(null)).toBeNull();
		expect(extractBackendData('raw string')).toBe('raw string');
		expect(extractBackendData(42)).toBe(42);
	});
});

describe('extractBackendErrorMessage', () => {
	it('returns value.message when present and non-empty', () => {
		expect(extractBackendErrorMessage({ message: 'Validation failed' }, 'fallback')).toBe(
			'Validation failed'
		);
	});

	it('falls back to value.error when message is missing', () => {
		expect(extractBackendErrorMessage({ error: 'Bad request' }, 'fallback')).toBe('Bad request');
	});

	it('prefers message over error when both are present', () => {
		expect(
			extractBackendErrorMessage({ message: 'primary', error: 'secondary' }, 'fallback')
		).toBe('primary');
	});

	it('returns fallback when value is not an object', () => {
		expect(extractBackendErrorMessage(null, 'fallback')).toBe('fallback');
		expect(extractBackendErrorMessage('raw', 'fallback')).toBe('fallback');
	});

	it('returns fallback for empty strings (treated as "no message")', () => {
		// Empty string is not a useful error — fall through.
		expect(extractBackendErrorMessage({ message: '' }, 'fallback')).toBe('fallback');
		expect(extractBackendErrorMessage({ message: '', error: '' }, 'fallback')).toBe('fallback');
	});

	it('returns fallback for non-string message/error fields', () => {
		expect(extractBackendErrorMessage({ message: 42 }, 'fallback')).toBe('fallback');
		expect(extractBackendErrorMessage({ message: null, error: { nested: 'x' } }, 'fallback')).toBe(
			'fallback'
		);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// fetchBackend tests
// ═══════════════════════════════════════════════════════════════════════════

describe('fetchBackend', () => {
	let originalFetch: typeof globalThis.fetch;
	let consoleInfoSpy: ReturnType<typeof vi.spyOn>;
	let consoleErrorSpy: ReturnType<typeof vi.spyOn>;

	beforeEach(() => {
		originalFetch = globalThis.fetch;
		consoleInfoSpy = vi.spyOn(console, 'info').mockImplementation(() => {});
		consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
	});

	afterEach(() => {
		globalThis.fetch = originalFetch;
		consoleInfoSpy.mockRestore();
		consoleErrorSpy.mockRestore();
	});

	it('returns parsed JSON body on 2xx', async () => {
		const body = { success: true, data: [{ id: 1 }] };
		globalThis.fetch = vi.fn().mockResolvedValueOnce({
			ok: true,
			status: 200,
			statusText: 'OK',
			json: () => Promise.resolve(body)
		});

		const result = await fetchBackend('/api/test');
		expect(result).toEqual(body);
	});

	it('joins endpoint to API_URL from mocked env', async () => {
		globalThis.fetch = vi.fn().mockResolvedValueOnce({
			ok: true,
			status: 200,
			json: () => Promise.resolve({})
		});

		await fetchBackend('/api/items?limit=10');
		const [url] = (globalThis.fetch as ReturnType<typeof vi.fn>).mock.calls[0];
		expect(url).toBe('https://test-api.example.com/api/items?limit=10');
	});

	it('injects JSON Content-Type / Accept headers (caller may override)', async () => {
		globalThis.fetch = vi.fn().mockResolvedValueOnce({
			ok: true,
			status: 200,
			json: () => Promise.resolve({})
		});

		await fetchBackend('/api/test', { headers: { Authorization: 'Bearer x' } });
		const [, options] = (globalThis.fetch as ReturnType<typeof vi.fn>).mock.calls[0];
		expect(options.headers['Content-Type']).toBe('application/json');
		expect(options.headers['Accept']).toBe('application/json');
		expect(options.headers['Authorization']).toBe('Bearer x');
	});

	it('returns null on non-2xx response (NEGATIVE)', async () => {
		globalThis.fetch = vi.fn().mockResolvedValueOnce({
			ok: false,
			status: 404,
			statusText: 'Not Found',
			json: () => Promise.resolve({ message: 'nope' })
		});

		const result = await fetchBackend('/api/missing');
		expect(result).toBeNull();
		expect(consoleErrorSpy).toHaveBeenCalled();
	});

	it('returns null when fetch throws (NEGATIVE — network failure)', async () => {
		globalThis.fetch = vi.fn().mockRejectedValueOnce(new TypeError('Network down'));

		const result = await fetchBackend('/api/test');
		expect(result).toBeNull();
		expect(consoleErrorSpy).toHaveBeenCalled();
	});

	it('suppresses logs when logTag is empty', async () => {
		globalThis.fetch = vi.fn().mockResolvedValueOnce({
			ok: false,
			status: 500,
			statusText: 'Internal',
			json: () => Promise.resolve({})
		});

		await fetchBackend('/api/test', {}, '');
		expect(consoleInfoSpy).not.toHaveBeenCalled();
		expect(consoleErrorSpy).not.toHaveBeenCalled();
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// fetchBackendWithStatus tests
// ═══════════════════════════════════════════════════════════════════════════

describe('fetchBackendWithStatus', () => {
	let originalFetch: typeof globalThis.fetch;
	let consoleErrorSpy: ReturnType<typeof vi.spyOn>;

	beforeEach(() => {
		originalFetch = globalThis.fetch;
		consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
	});

	afterEach(() => {
		globalThis.fetch = originalFetch;
		consoleErrorSpy.mockRestore();
	});

	it('returns parsed JSON body + 200 status on success', async () => {
		const body = { data: { id: 1 } };
		globalThis.fetch = vi.fn().mockResolvedValueOnce({
			ok: true,
			status: 200,
			text: () => Promise.resolve(JSON.stringify(body))
		});

		const result = await fetchBackendWithStatus('/api/items/1');
		expect(result).toEqual({ data: body, status: 200 });
	});

	it('preserves upstream non-2xx status (admin proxy contract)', async () => {
		// Admin proxies need the backend's 422 + validation message
		// surfaced via SvelteKit's error(status, msg).
		const body = { message: 'Validation failed' };
		globalThis.fetch = vi.fn().mockResolvedValueOnce({
			ok: false,
			status: 422,
			text: () => Promise.resolve(JSON.stringify(body))
		});

		const result = await fetchBackendWithStatus('/api/admin/posts');
		expect(result).toEqual({ data: body, status: 422 });
	});

	it('returns { data: null, status: <upstream> } for empty 204 body (R21-A Latent Bug §1 fix)', async () => {
		// The "bare response.json()" pattern threw on empty 204 and triggered
		// the outer catch → wrongly returned 500. The resilient text() pattern
		// returns the correct 204 + null data.
		globalThis.fetch = vi.fn().mockResolvedValueOnce({
			ok: true,
			status: 204,
			text: () => Promise.resolve('')
		});

		const result = await fetchBackendWithStatus('/api/admin/items/1', { method: 'DELETE' });
		expect(result).toEqual({ data: null, status: 204 });
	});

	it('wraps non-JSON bodies in { error: rawText } (R21-A fallback)', async () => {
		globalThis.fetch = vi.fn().mockResolvedValueOnce({
			ok: false,
			status: 502,
			text: () => Promise.resolve('Bad Gateway')
		});

		const result = await fetchBackendWithStatus('/api/items');
		expect(result).toEqual({ data: { error: 'Bad Gateway' }, status: 502 });
	});

	it('returns { data: null, status: 500 } on network failure (NEGATIVE)', async () => {
		globalThis.fetch = vi.fn().mockRejectedValueOnce(new TypeError('Connection reset'));

		const result = await fetchBackendWithStatus('/api/admin/posts');
		expect(result).toEqual({ data: null, status: 500 });
		expect(consoleErrorSpy).toHaveBeenCalled();
	});
});

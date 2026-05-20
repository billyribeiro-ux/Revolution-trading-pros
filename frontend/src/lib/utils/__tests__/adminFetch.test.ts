/**
 * adminFetch utility — Unit Tests (R28-D)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * `src/lib/utils/adminFetch.ts` is the singular authenticated `fetch` wrapper
 * for the /admin surface. Every admin page hits it. The contracts pinned
 * here are the boring ones nobody documents — but that break loudly:
 *
 *   1. **URL normalization** — three rules:
 *        - Absolute `http(s)://...`  → passed through unchanged.
 *        - Already `/api/...`         → passed through unchanged.
 *        - Anything else              → prefixed with `/api`.
 *      Pre-this contract, code in 73 admin callsites passed e.g.
 *      `/admin/coupons` and got `/api/admin/coupons` (CORB-safe). Breaking
 *      this would 404 every admin call in production but pass dev (where
 *      a Vite proxy normalises silently).
 *
 *   2. **Authorization: Bearer header injected when token present.** The
 *      token comes from the auth store; absence means anonymous request
 *      (no header — NOT `Bearer null`).
 *
 *   3. **FormData bodies must NOT get `Content-Type: application/json`.**
 *      Setting it strips the auto-generated `multipart/form-data;
 *      boundary=...` and the backend rejects the upload. This is a
 *      sneaky one — every file-upload page depends on the absence.
 *
 *   4. **401 → goto(/login) unless `skipAuthRedirect`.** The admin auth
 *      check uses `skipAuthRedirect: true` to avoid an infinite loop
 *      (the auth probe IS the 401 source).
 *
 *   5. **Error JSON body is surfaced via `AdminApiError.message`** — not
 *      swallowed into `"HTTP 422"`. Validation messages are read by
 *      admin forms to drive inline field errors.
 *
 *   6. **`AdminApiError.isAuthError` / `isValidationError` / `isServerError`**
 *      are getters that bucket the HTTP status. Every admin error UI
 *      branches on these — change them at your peril.
 *
 *   7. **`adminApi.get/post/put/patch/delete` wrappers** call adminFetch
 *      with the right method + body serialization. POST with FormData
 *      passes the FormData through (no JSON.stringify on a Blob).
 */

import { describe, expect, it, vi, beforeEach, afterEach } from 'vitest';

// `$app/environment` — browser=true gates the 401 redirect path.
vi.mock('$app/environment', () => ({
	browser: true,
	dev: true,
	building: false,
	version: 'test'
}));

// `$app/paths` — `base` must be a string for the goto() URL.
vi.mock('$app/paths', () => ({
	base: ''
}));

// `$app/navigation` — capture goto calls so we can assert redirects.
// vi.mock is hoisted; use vi.hoisted to share state with the factory.
const mocks = vi.hoisted(() => ({
	gotoMock: (...args: unknown[]) => {
		mocks.gotoCalls.push(args);
	},
	gotoCalls: [] as unknown[][],
	authToken: null as string | null,
	logoutMock: () => Promise.resolve(undefined),
	refreshMock: () => Promise.resolve(false)
}));

vi.mock('$app/navigation', () => ({
	goto: mocks.gotoMock
}));

// `$lib/stores/auth.svelte` — return a controllable token. Tests flip
// it through `setAuthToken(...)` below.
vi.mock('$lib/stores/auth.svelte', () => ({
	getAuthToken: () => mocks.authToken,
	authStore: {
		logout: mocks.logoutMock,
		refreshToken: mocks.refreshMock
	}
}));

import { adminFetch, adminApi, AdminApiError } from '../adminFetch';

function setAuthToken(t: string | null) {
	mocks.authToken = t;
}

function getGotoCalls(): unknown[][] {
	return mocks.gotoCalls;
}

function clearGotoCalls(): void {
	mocks.gotoCalls.length = 0;
}

// ═══════════════════════════════════════════════════════════════════════════
// Helper: build a fake `fetch` response so we don't hit the network
// ═══════════════════════════════════════════════════════════════════════════

interface FakeResponseOptions {
	status?: number;
	body?: unknown;
	headers?: Record<string, string>;
	isJson?: boolean;
}

function fakeResponse(opts: FakeResponseOptions = {}): Response {
	const { status = 200, body = {}, headers = {}, isJson = true } = opts;
	const headerMap = new Map<string, string>(
		Object.entries({
			'content-type': isJson ? 'application/json' : 'text/plain',
			...headers
		})
	);
	return {
		ok: status >= 200 && status < 300,
		status,
		headers: {
			get: (name: string) => headerMap.get(name.toLowerCase()) ?? null
		} as Headers,
		json: () => Promise.resolve(body),
		text: () => Promise.resolve(typeof body === 'string' ? body : JSON.stringify(body))
	} as unknown as Response;
}

// ═══════════════════════════════════════════════════════════════════════════
// URL normalization
// ═══════════════════════════════════════════════════════════════════════════

describe('adminFetch — URL normalization', () => {
	beforeEach(() => {
		setAuthToken('test-token-1');
		clearGotoCalls();
	});

	afterEach(() => {
		vi.restoreAllMocks();
	});

	it('happy path: relative path "/admin/posts" is prefixed with /api', async () => {
		const fetchSpy = vi.spyOn(globalThis, 'fetch').mockResolvedValue(fakeResponse());
		await adminFetch('/admin/posts');
		expect(fetchSpy).toHaveBeenCalled();
		expect(fetchSpy.mock.calls[0]?.[0]).toBe('/api/admin/posts');
	});

	it('CONTRACT: paths already starting with /api/ are passed through unchanged', async () => {
		const fetchSpy = vi.spyOn(globalThis, 'fetch').mockResolvedValue(fakeResponse());
		await adminFetch('/api/admin/coupons');
		expect(fetchSpy.mock.calls[0]?.[0]).toBe('/api/admin/coupons');
	});

	it('CONTRACT: absolute http(s) URLs are passed through unchanged', async () => {
		const fetchSpy = vi.spyOn(globalThis, 'fetch').mockResolvedValue(fakeResponse());
		await adminFetch('https://external.example.com/data');
		expect(fetchSpy.mock.calls[0]?.[0]).toBe('https://external.example.com/data');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// Auth headers
// ═══════════════════════════════════════════════════════════════════════════

describe('adminFetch — auth header', () => {
	beforeEach(() => {
		clearGotoCalls();
	});

	afterEach(() => {
		vi.restoreAllMocks();
	});

	it('happy path: includes Bearer token when present', async () => {
		setAuthToken('jwt-abc');
		const fetchSpy = vi.spyOn(globalThis, 'fetch').mockResolvedValue(fakeResponse());
		await adminFetch('/admin/me');
		const init = fetchSpy.mock.calls[0]?.[1] as RequestInit;
		const headers = init.headers as Record<string, string>;
		expect(headers['Authorization']).toBe('Bearer jwt-abc');
	});

	it('NEGATIVE: no token → NO Authorization header (not "Bearer null")', async () => {
		setAuthToken(null);
		const fetchSpy = vi.spyOn(globalThis, 'fetch').mockResolvedValue(fakeResponse());
		await adminFetch('/admin/public');
		const init = fetchSpy.mock.calls[0]?.[1] as RequestInit;
		const headers = init.headers as Record<string, string>;
		expect(headers['Authorization']).toBeUndefined();
	});

	it('CONTRACT: Content-Type is application/json for non-FormData bodies', async () => {
		setAuthToken('t');
		const fetchSpy = vi.spyOn(globalThis, 'fetch').mockResolvedValue(fakeResponse());
		await adminFetch('/admin/x', { method: 'POST', body: '{"a":1}' });
		const init = fetchSpy.mock.calls[0]?.[1] as RequestInit;
		const headers = init.headers as Record<string, string>;
		expect(headers['Content-Type']).toBe('application/json');
	});

	it('CONTRACT: FormData body does NOT get Content-Type (browser fills boundary)', async () => {
		setAuthToken('t');
		const fetchSpy = vi.spyOn(globalThis, 'fetch').mockResolvedValue(fakeResponse());
		const form = new FormData();
		form.append('file', new Blob(['x']), 'a.txt');
		await adminFetch('/admin/upload', { method: 'POST', body: form });
		const init = fetchSpy.mock.calls[0]?.[1] as RequestInit;
		const headers = init.headers as Record<string, string>;
		expect(headers['Content-Type']).toBeUndefined();
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// 401 handling
// ═══════════════════════════════════════════════════════════════════════════

describe('adminFetch — 401 handling', () => {
	beforeEach(() => {
		setAuthToken('t');
		clearGotoCalls();
	});

	afterEach(() => {
		vi.restoreAllMocks();
	});

	it('happy path: 401 triggers goto(/login?redirect=...) and throws AdminApiError(401)', async () => {
		vi.spyOn(globalThis, 'fetch').mockResolvedValue(
			fakeResponse({ status: 401, body: { message: 'no-auth' } })
		);

		await expect(adminFetch('/admin/me')).rejects.toBeInstanceOf(AdminApiError);
		const calls = getGotoCalls();
		expect(calls.length).toBeGreaterThanOrEqual(1);
		const calledUrl = calls[0]?.[0] as string;
		expect(calledUrl).toContain('/login?redirect=');
	});

	it('NEGATIVE: skipAuthRedirect=true skips goto but still propagates the API error', async () => {
		vi.spyOn(globalThis, 'fetch').mockResolvedValue(
			fakeResponse({ status: 401, body: { message: 'no-auth' } })
		);

		// Without redirect, the JSON-body branch throws an AdminApiError with
		// the 401 status and the server's message.
		await expect(adminFetch('/admin/me', { skipAuthRedirect: true })).rejects.toMatchObject({
			status: 401,
			message: 'no-auth'
		});
		expect(getGotoCalls()).toHaveLength(0);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// Error surfacing
// ═══════════════════════════════════════════════════════════════════════════

describe('adminFetch — error surfacing', () => {
	beforeEach(() => {
		setAuthToken('t');
		clearGotoCalls();
	});

	afterEach(() => {
		vi.restoreAllMocks();
	});

	it('happy path: 422 with validation errors surfaces via AdminApiError.errors', async () => {
		vi.spyOn(globalThis, 'fetch').mockResolvedValue(
			fakeResponse({
				status: 422,
				body: {
					message: 'Validation failed',
					errors: { email: ['Email is required'] }
				}
			})
		);

		try {
			await adminFetch('/admin/posts', { method: 'POST', body: '{}' });
			expect.unreachable('expected throw');
		} catch (err) {
			expect(err).toBeInstanceOf(AdminApiError);
			const apiErr = err as AdminApiError;
			expect(apiErr.status).toBe(422);
			expect(apiErr.message).toBe('Validation failed');
			expect(apiErr.errors).toEqual({ email: ['Email is required'] });
			expect(apiErr.isValidationError).toBe(true);
			expect(apiErr.isAuthError).toBe(false);
			expect(apiErr.isServerError).toBe(false);
		}
	});

	it('NEGATIVE: 500 → AdminApiError.isServerError=true', async () => {
		vi.spyOn(globalThis, 'fetch').mockResolvedValue(
			fakeResponse({ status: 500, body: { message: 'boom' } })
		);

		try {
			await adminFetch('/admin/x');
			expect.unreachable('expected throw');
		} catch (err) {
			const apiErr = err as AdminApiError;
			expect(apiErr.status).toBe(500);
			expect(apiErr.isServerError).toBe(true);
			expect(apiErr.isAuthError).toBe(false);
		}
	});

	it('NEGATIVE: 403 → AdminApiError.isAuthError=true (bundles 401 and 403)', async () => {
		vi.spyOn(globalThis, 'fetch').mockResolvedValue(
			fakeResponse({ status: 403, body: { message: 'forbidden' } })
		);

		try {
			await adminFetch('/admin/secret', { skipAuthRedirect: true });
			expect.unreachable('expected throw');
		} catch (err) {
			const apiErr = err as AdminApiError;
			expect(apiErr.status).toBe(403);
			expect(apiErr.isAuthError).toBe(true);
		}
	});

	it('NEGATIVE: fetch throw (network error) wraps in AdminApiError(status=0)', async () => {
		vi.spyOn(globalThis, 'fetch').mockRejectedValue(new Error('NetworkError'));
		try {
			await adminFetch('/admin/x');
			expect.unreachable('expected throw');
		} catch (err) {
			expect(err).toBeInstanceOf(AdminApiError);
			const apiErr = err as AdminApiError;
			expect(apiErr.status).toBe(0);
			expect(apiErr.message).toBe('NetworkError');
		}
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// rawResponse
// ═══════════════════════════════════════════════════════════════════════════

describe('adminFetch — rawResponse', () => {
	beforeEach(() => {
		setAuthToken('t');
	});

	afterEach(() => {
		vi.restoreAllMocks();
	});

	it('CONTRACT: rawResponse=true returns the Response object unparsed (for blob downloads)', async () => {
		const resp = fakeResponse({ status: 200, body: 'binary-bytes', isJson: false });
		vi.spyOn(globalThis, 'fetch').mockResolvedValue(resp);

		const result = await adminFetch('/admin/export.csv', { rawResponse: true });
		expect(result).toBe(resp);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// adminApi convenience wrappers
// ═══════════════════════════════════════════════════════════════════════════

describe('adminApi.* wrappers', () => {
	beforeEach(() => {
		setAuthToken('t');
		clearGotoCalls();
	});

	afterEach(() => {
		vi.restoreAllMocks();
	});

	it('adminApi.get sends GET', async () => {
		const fetchSpy = vi.spyOn(globalThis, 'fetch').mockResolvedValue(fakeResponse());
		await adminApi.get('/admin/things');
		expect((fetchSpy.mock.calls[0]?.[1] as RequestInit).method).toBe('GET');
	});

	it('adminApi.post serializes JSON body and sends POST', async () => {
		const fetchSpy = vi.spyOn(globalThis, 'fetch').mockResolvedValue(fakeResponse());
		await adminApi.post('/admin/things', { name: 'X' });
		const init = fetchSpy.mock.calls[0]?.[1] as RequestInit;
		expect(init.method).toBe('POST');
		expect(init.body).toBe('{"name":"X"}');
	});

	it('CONTRACT: adminApi.post with FormData passes the FormData through (no JSON.stringify)', async () => {
		const fetchSpy = vi.spyOn(globalThis, 'fetch').mockResolvedValue(fakeResponse());
		const form = new FormData();
		form.append('a', 'b');
		await adminApi.post('/admin/upload', form);
		const init = fetchSpy.mock.calls[0]?.[1] as RequestInit;
		expect(init.body).toBe(form); // same reference, not a string
	});

	it('adminApi.put / patch / delete map to the right method', async () => {
		const fetchSpy = vi.spyOn(globalThis, 'fetch').mockResolvedValue(fakeResponse());

		await adminApi.put('/admin/x', { v: 1 });
		expect((fetchSpy.mock.calls[0]?.[1] as RequestInit).method).toBe('PUT');

		await adminApi.patch('/admin/x', { v: 2 });
		expect((fetchSpy.mock.calls[1]?.[1] as RequestInit).method).toBe('PATCH');

		await adminApi.delete('/admin/x');
		expect((fetchSpy.mock.calls[2]?.[1] as RequestInit).method).toBe('DELETE');
	});
});

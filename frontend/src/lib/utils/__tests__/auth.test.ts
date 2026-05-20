/**
 * auth utilities — Unit Tests (R26-D)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * `src/lib/utils/auth.ts` exposes two coarse-grained helpers used at every
 * admin route guard and "show admin chip" UI decision point:
 *
 *   - checkAdminStatus(): GET /api/auth/me, true if the current user has
 *     admin or super_admin role. Tolerates TWO response shapes:
 *       Format A: { is_admin, role }
 *       Format B: { user: { is_admin, role } }
 *     because the backend historically returned both.
 *
 *   - checkAuthStatus(): GET /api/auth/me, true if status is 2xx.
 *
 * Critical contracts pinned here:
 *
 *   1. `credentials: 'include'` — without this the cookie isn't sent and
 *      every admin check returns false silently. This is THE bug pattern
 *      that locked Billy out of /admin during the dev-server refactor.
 *   2. Network failure (fetch throws) returns false, NOT undefined; the
 *      route guard branches on a boolean.
 *   3. The whitelist is exactly `['admin', 'super_admin']` — adding a
 *      role like 'editor' to the whitelist should require an explicit
 *      test diff, not a one-line slip.
 *   4. Format A precedence: if `is_admin: true` at top level, return true
 *      without descending into `user`.
 *   5. Format B: if the top level says non-admin but `user.is_admin: true`,
 *      we still return true (this was the back-compat reason for the
 *      two formats).
 */

import { describe, expect, it, beforeEach, afterEach, vi } from 'vitest';
import { checkAdminStatus, checkAuthStatus } from '../auth';

// Helper to build a mock Response object
function jsonResponse(body: unknown, status = 200): Response {
	return {
		ok: status >= 200 && status < 300,
		status,
		json: async () => body,
		// Required by the Response interface; not used here.
		headers: new Headers(),
		statusText: 'OK',
		text: async () => JSON.stringify(body),
		url: '',
		redirected: false,
		type: 'basic' as ResponseType,
		clone() {
			return jsonResponse(body, status);
		},
		body: null,
		bodyUsed: false,
		arrayBuffer: async () => new ArrayBuffer(0),
		blob: async () => new Blob([]),
		formData: async () => new FormData(),
		bytes: async () => new Uint8Array(0)
	} as unknown as Response;
}

// ═══════════════════════════════════════════════════════════════════════════
// checkAdminStatus — Format A (top-level fields)
// ═══════════════════════════════════════════════════════════════════════════

describe('checkAdminStatus — Format A', () => {
	let fetchSpy: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		fetchSpy = vi.fn();
		vi.stubGlobal('fetch', fetchSpy);
	});

	afterEach(() => {
		vi.unstubAllGlobals();
	});

	it('returns true when top-level is_admin === true', async () => {
		fetchSpy.mockResolvedValueOnce(jsonResponse({ is_admin: true }));
		expect(await checkAdminStatus()).toBe(true);
	});

	it('returns true when top-level role === "admin"', async () => {
		fetchSpy.mockResolvedValueOnce(jsonResponse({ role: 'admin' }));
		expect(await checkAdminStatus()).toBe(true);
	});

	it('returns true when top-level role === "super_admin"', async () => {
		fetchSpy.mockResolvedValueOnce(jsonResponse({ role: 'super_admin' }));
		expect(await checkAdminStatus()).toBe(true);
	});

	it('returns false for a non-privileged role like "user"', async () => {
		fetchSpy.mockResolvedValueOnce(jsonResponse({ role: 'user' }));
		expect(await checkAdminStatus()).toBe(false);
	});

	it('returns false when role is "editor" (regression guard for whitelist)', async () => {
		// CONTRACT: the admin whitelist is exactly ['admin', 'super_admin'].
		// Adding 'editor' to it later should require an explicit test diff.
		fetchSpy.mockResolvedValueOnce(jsonResponse({ role: 'editor' }));
		expect(await checkAdminStatus()).toBe(false);
	});

	it('returns false when is_admin === false explicitly', async () => {
		fetchSpy.mockResolvedValueOnce(jsonResponse({ is_admin: false }));
		expect(await checkAdminStatus()).toBe(false);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// checkAdminStatus — Format B (nested under .user)
// ═══════════════════════════════════════════════════════════════════════════

describe('checkAdminStatus — Format B', () => {
	let fetchSpy: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		fetchSpy = vi.fn();
		vi.stubGlobal('fetch', fetchSpy);
	});

	afterEach(() => {
		vi.unstubAllGlobals();
	});

	it('returns true when user.is_admin === true', async () => {
		fetchSpy.mockResolvedValueOnce(jsonResponse({ user: { is_admin: true } }));
		expect(await checkAdminStatus()).toBe(true);
	});

	it('returns true when user.role === "admin"', async () => {
		fetchSpy.mockResolvedValueOnce(jsonResponse({ user: { role: 'admin' } }));
		expect(await checkAdminStatus()).toBe(true);
	});

	it('returns true when user.role === "super_admin"', async () => {
		fetchSpy.mockResolvedValueOnce(jsonResponse({ user: { role: 'super_admin' } }));
		expect(await checkAdminStatus()).toBe(true);
	});

	it('returns false when user is missing entirely', async () => {
		fetchSpy.mockResolvedValueOnce(jsonResponse({}));
		expect(await checkAdminStatus()).toBe(false);
	});

	it('returns false when user.role is a non-admin role', async () => {
		fetchSpy.mockResolvedValueOnce(jsonResponse({ user: { role: 'subscriber' } }));
		expect(await checkAdminStatus()).toBe(false);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// checkAdminStatus — credentials + failure modes
// ═══════════════════════════════════════════════════════════════════════════

describe('checkAdminStatus — fetch contract', () => {
	let fetchSpy: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		fetchSpy = vi.fn();
		vi.stubGlobal('fetch', fetchSpy);
	});

	afterEach(() => {
		vi.unstubAllGlobals();
	});

	it('calls /api/auth/me with credentials: "include" (cookie-bearing)', async () => {
		// CONTRACT pin: without `credentials: 'include'` the session cookie
		// is dropped and the admin check silently returns false. This is
		// the lockout-bug shape that must not regress.
		fetchSpy.mockResolvedValueOnce(jsonResponse({ is_admin: true }));
		await checkAdminStatus();
		expect(fetchSpy).toHaveBeenCalledWith('/api/auth/me', { credentials: 'include' });
	});

	it('returns false when the response is non-ok (e.g. 401)', async () => {
		fetchSpy.mockResolvedValueOnce(jsonResponse({}, 401));
		expect(await checkAdminStatus()).toBe(false);
	});

	it('returns false when fetch rejects (network failure)', async () => {
		fetchSpy.mockRejectedValueOnce(new Error('network down'));
		// CONTRACT: a thrown fetch must NOT propagate; route guards branch
		// on a boolean.
		expect(await checkAdminStatus()).toBe(false);
	});

	it('returns false when fetch resolves with non-JSON body (json() throws)', async () => {
		const badJsonResponse = {
			ok: true,
			status: 200,
			json: async () => {
				throw new SyntaxError('Unexpected token');
			}
		};
		fetchSpy.mockResolvedValueOnce(badJsonResponse as unknown as Response);
		expect(await checkAdminStatus()).toBe(false);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// checkAuthStatus
// ═══════════════════════════════════════════════════════════════════════════

describe('checkAuthStatus', () => {
	let fetchSpy: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		fetchSpy = vi.fn();
		vi.stubGlobal('fetch', fetchSpy);
	});

	afterEach(() => {
		vi.unstubAllGlobals();
	});

	it('returns true when the response is ok (regardless of body)', async () => {
		fetchSpy.mockResolvedValueOnce(jsonResponse({}, 200));
		expect(await checkAuthStatus()).toBe(true);
	});

	it('returns false when the response is non-ok (401/403/etc.)', async () => {
		fetchSpy.mockResolvedValueOnce(jsonResponse({}, 401));
		expect(await checkAuthStatus()).toBe(false);
	});

	it('returns false when fetch throws', async () => {
		fetchSpy.mockRejectedValueOnce(new Error('offline'));
		expect(await checkAuthStatus()).toBe(false);
	});

	it('calls /api/auth/me with credentials: "include"', async () => {
		fetchSpy.mockResolvedValueOnce(jsonResponse({}, 200));
		await checkAuthStatus();
		expect(fetchSpy).toHaveBeenCalledWith('/api/auth/me', { credentials: 'include' });
	});
});

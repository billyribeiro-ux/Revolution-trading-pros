/**
 * Server-side auth helpers for `+server.ts` proxies.
 *
 * PRINCIPAL-2026-04-26 (audit 09-system §P0-2 / §P2-1):
 * The system cluster had two divergent auth-extraction patterns across proxies
 * (cookie-only in `connections/*`, cookie+header fallback in `users/*`),
 * a `connections/status/+server.ts` with NO auth at all, and zero `super-admin`
 * gate on role/credential mutating endpoints. This module unifies the pattern.
 *
 * Usage:
 *   import { requireAdminToken, requireSuperadmin } from '$lib/server/auth';
 *
 *   export const GET: RequestHandler = async (event) => {
 *     const token = requireAdminToken(event);
 *     // ... read-only admin work
 *   };
 *
 *   export const PUT: RequestHandler = async (event) => {
 *     const { token, user } = requireSuperadmin(event);
 *     // ... role/credential/destructive work
 *   };
 *
 * Both helpers throw a SvelteKit `error()` (HttpError) on failure so the proxy
 * short-circuits with the correct status — no extra plumbing required.
 */

import { error, type RequestEvent } from '@sveltejs/kit';

/**
 * Names that count as super-admin in `event.locals.user.role`.
 * Mirrors the list in `$lib/config/roles.ts:isSuperadmin` so server and client
 * agree.
 */
const SUPERADMIN_ROLE_VALUES = new Set(['super-admin', 'super_admin', 'superadmin']);

/**
 * Names that count as admin (or higher) in `event.locals.user.role`.
 */
const ADMIN_ROLE_VALUES = new Set([
	'admin',
	'administrator',
	'super-admin',
	'super_admin',
	'superadmin'
]);

/**
 * Canonical server-side role predicates. These are the single source of
 * truth for "is this role privileged?" on the server — consumed by the
 * proxy guards below AND by `routes/admin/+layout.server.ts` so the page
 * gate and the API gate can never drift apart.
 */
export function isAdminRole(role: string | null | undefined): boolean {
	return ADMIN_ROLE_VALUES.has((role ?? '').toLowerCase());
}

export function isSuperadminRole(role: string | null | undefined): boolean {
	return SUPERADMIN_ROLE_VALUES.has((role ?? '').toLowerCase());
}

/**
 * Read the canonical `rtp_access_token` cookie, falling back to the
 * `Authorization: Bearer` header for server-to-server callers. Throws
 * `error(401)` if neither is present.
 *
 * Pattern matches `users/+server.ts` (cookie + header fallback), which is the
 * more permissive of the two divergent patterns the audit found. Read-only
 * admin GETs should use this; mutations should use {@link requireSuperadmin}
 * (or {@link requireAdmin}) instead.
 */
export function requireAdminToken(event: RequestEvent): string {
	const cookieToken = event.cookies.get('rtp_access_token');
	const headerToken = event.request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) error(401, 'Unauthorized');
	return token;
}

/**
 * Like {@link requireAdminToken} but ALSO requires that the authenticated user
 * (as populated by `hooks.server.ts:authHandler`) holds an admin or
 * super-admin role. Returns both the bearer token and the user object so the
 * caller can forward the token and reason about the user without re-parsing.
 *
 * Throws `error(403)` when the cookie/header is present but the user is not
 * an admin. Throws `error(401)` when no token is supplied at all.
 *
 * Use this on read-only admin GETs that should never be reachable by
 * non-admin sessions (e.g. `connections/status`, `site-health`).
 */
export function requireAdmin(event: RequestEvent): {
	token: string;
	user: NonNullable<App.Locals['user']>;
} {
	const token = requireAdminToken(event);
	const user = event.locals.user;
	if (!user) error(403, 'Forbidden');
	if (!isAdminRole(user.role)) error(403, 'Forbidden');
	return { token, user };
}

/**
 * Like {@link requireAdmin} but requires the strictly-higher `super-admin`
 * role. Use on every endpoint that mutates roles, passwords, OAuth
 * credentials, or other privilege-bearing fields.
 *
 * Throws `error(403)` when the user is authenticated but only an admin (not
 * super-admin). Throws `error(401)` when no token is supplied.
 *
 * @see audit `docs/audits/admin-2026-04-26/09-system.md` §P0-2
 */
export function requireSuperadmin(event: RequestEvent): {
	token: string;
	user: NonNullable<App.Locals['user']>;
} {
	const token = requireAdminToken(event);
	const user = event.locals.user;
	if (!user) error(403, 'Forbidden');
	if (!isSuperadminRole(user.role)) error(403, 'Forbidden');
	return { token, user };
}

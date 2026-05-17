/**
 * Session cookie teardown — logout only.
 *
 * P1-6 (audit FULL_REPO_AUDIT_2026-05-17 §P1-6): the former `POST`
 * handler wrote an attacker-suppliable `access_token` from the request
 * body straight into the `rtp_access_token` httpOnly cookie with NO
 * authentication, origin, or token validation — a session-fixation /
 * login-CSRF primitive. It had zero live callers: the OAuth callback
 * delivers tokens via backend `Set-Cookie`, and `lib/api/auth.ts` only
 * ever calls this endpoint with `method: 'DELETE'`. An endpoint that
 * mints a session from an unauthenticated body must not exist, so the
 * `POST` is deleted outright.
 *
 * `DELETE` is the only legitimate purpose — clearing the server-set
 * httpOnly auth cookies on logout. It now also clears `rtp_session_id`
 * (set by the OAuth callback, CHANGELOG C-1) so logout fully tears the
 * session down rather than leaving a dangling session identifier.
 */

import { json, type RequestEvent } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

/** Every cookie the auth / OAuth flow sets (see CHANGELOG C-1). */
const AUTH_COOKIES = ['rtp_access_token', 'rtp_refresh_token', 'rtp_session_id'] as const;

export const DELETE: RequestHandler = async ({ cookies }: RequestEvent) => {
	for (const name of AUTH_COOKIES) {
		// Path must match how the cookie was set ('/') for the browser
		// to actually drop it.
		cookies.delete(name, { path: '/' });
	}
	return json({ success: true });
};

import { redirect } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

/**
 * Logout Page Server Load
 * ICT 11 Protocol: Secure session termination with complete cleanup
 *
 * Security measures:
 * - Revokes session on backend via API call
 * - Clears all auth-related cookies
 * - Redirects to home with confirmation
 */

// Disable prerendering for this dynamic route
export const prerender = false;

export const load = async ({ locals, cookies, fetch }: RequestEvent) => {
	// Get the session for verification
	const session = await locals.auth();

	// Call logout API to revoke session on backend
	if (session) {
		try {
			await fetch('/api/logout', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				credentials: 'include'
			});
		} catch (err) {
			// ICT 11 Protocol: Log but don't block logout on API failure
			console.error('[Logout] API error:', err);
		}
	}

	// FIX-2026-04-26: cookie names corrected. The previous 4 names
	// (auth_token, session_id, refresh_token, access_token) were NEVER set by
	// the login proxy — only `rtp_access_token` and `rtp_refresh_token` are.
	// Result: the dashboard sidebar's "Log out" link routed here, "deleted"
	// 4 cookies that didn't exist, and the actual session survived. Real
	// security-grade silent bug.
	// Old code (kept for one revision per FIX-2026-04-26 marker — delete in follow-up):
	// cookies.delete('auth_token', cookieOptions);
	// cookies.delete('session_id', cookieOptions);
	// cookies.delete('refresh_token', cookieOptions);
	// cookies.delete('access_token', cookieOptions);
	const cookieOptions = { path: '/', secure: true, httpOnly: true };
	cookies.delete('rtp_access_token', cookieOptions);
	cookies.delete('rtp_refresh_token', cookieOptions);

	// Redirect to home page with logged out message
	redirect(303, '/?message=logged_out');
};

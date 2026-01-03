import { redirect } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

export const load = async ({ locals, cookies }: RequestEvent) => {
	// Get the session
	const session = await locals.auth();

	// Clear the session
	if (session) {
		// Call the logout API endpoint
		try {
			await fetch('/api/logout', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				credentials: 'include'
			});
		} catch (err) {
			console.error('Logout API error:', err);
		}
	}

	// Clear all auth-related cookies
	cookies.delete('auth_token', { path: '/' });
	cookies.delete('session_id', { path: '/' });
	cookies.delete('refresh_token', { path: '/' });

	// Redirect to home page with logged out message
	throw redirect(303, '/?message=logged_out');
};

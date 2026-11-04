import type { RequestHandler } from './$types';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';

const API_ROOT = API_BASE_URL || BACKEND_URL || 'http://localhost:8080';

/** Clear local cookies on every path; report server revocation truthfully. */
export const POST: RequestHandler = async ({ request, cookies, fetch }) => {
	const token =
		cookies.get('rtp_access_token') ||
		request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const refreshToken = cookies.get('rtp_refresh_token');
	const sessionId = request.headers.get('X-Session-ID');
	try {
		if (!token) {
			return Response.json({
				success: true,
				server_revoked: false,
				message: 'Local session cleared'
			});
		}
		const response = await fetch(`${API_ROOT}/api/auth/logout`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			},
			body: JSON.stringify({ session_id: sessionId, refresh_token: refreshToken }),
			signal: AbortSignal.timeout(5000)
		});
		if (!response.ok) {
			return Response.json(
				{
					success: false,
					server_revoked: false,
					message: 'Local session cleared; server revocation failed'
				},
				{ status: response.status }
			);
		}
		return Response.json({
			success: true,
			server_revoked: true,
			message: 'Logged out successfully'
		});
	} catch {
		return Response.json(
			{
				success: false,
				server_revoked: false,
				message: 'Local session cleared; session service unavailable'
			},
			{ status: 503 }
		);
	} finally {
		cookies.delete('rtp_access_token', { path: '/' });
		cookies.delete('rtp_refresh_token', { path: '/' });
	}
};

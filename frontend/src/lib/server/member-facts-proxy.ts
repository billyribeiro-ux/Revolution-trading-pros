import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import type { RequestEvent } from '@sveltejs/kit';
import { requireAdmin } from './auth.js';
/** One network read. Failures never become successful zero-valued member facts. */
export async function proxyMemberFacts(event: RequestEvent, path: string): Promise<Response> {
	const { token } = requireAdmin(event);
	const base = API_BASE_URL || BACKEND_URL || 'http://localhost:8080';
	const headers = new Headers({
		'Cache-Control': 'private, no-store',
		'X-Content-Type-Options': 'nosniff'
	});
	try {
		const response = await fetch(`${base}/api/admin/members/${path}${event.url.search}`, {
			method: 'GET',
			headers: { Authorization: `Bearer ${token}`, Accept: 'application/json' },
			credentials: 'omit',
			redirect: 'error',
			signal: AbortSignal.timeout(15_000)
		});
		headers.set('Content-Type', response.headers.get('content-type') || 'application/json');
		return new Response(response.body, { status: response.status, headers });
	} catch {
		return Response.json(
			{ message: 'Member facts service is unavailable. Retry to fetch current records.' },
			{ status: 503, headers }
		);
	}
}

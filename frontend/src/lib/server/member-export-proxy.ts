import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import type { RequestEvent } from '@sveltejs/kit';
import { requireAdmin } from './auth.js';

/** One authenticated request, preserving the real binary/error and receipt headers. */
export async function proxyMemberExport(event: RequestEvent, advanced: boolean): Promise<Response> {
	const { token } = requireAdmin(event);
	const base = API_BASE_URL || BACKEND_URL || 'http://localhost:8080';
	try {
		const upstream = await fetch(
			`${base}/api/admin/${advanced ? 'member-management' : 'members'}/export${event.url.search}`,
			{
				method: 'GET',
				headers: { Authorization: `Bearer ${token}` },
				credentials: 'omit',
				redirect: 'error',
				signal: AbortSignal.timeout(60_000)
			}
		);
		const headers = new Headers({
			'Cache-Control': 'private, no-store',
			'X-Content-Type-Options': 'nosniff'
		});
		for (const name of [
			'content-type',
			'content-disposition',
			'x-export-row-count',
			'x-export-sha256'
		]) {
			const value = upstream.headers.get(name);
			if (value) headers.set(name, value);
		}
		return new Response(upstream.body, { status: upstream.status, headers });
	} catch {
		return Response.json(
			{ message: 'Member export service is unavailable; no download was created' },
			{ status: 503, headers: { 'Cache-Control': 'private, no-store' } }
		);
	}
}

import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import type { RequestEvent } from '@sveltejs/kit';
import { memberIdentity, type MemberRouteFamily } from '../api/member-profile.js';
import { requireAdmin } from './auth.js';

/** One binary read; preserves the backend's actual member/file receipt and errors. */
export async function proxyMemberOrderExport(event: RequestEvent, family: MemberRouteFamily) {
	const { token } = requireAdmin(event);
	const headers = new Headers({
		'Cache-Control': 'private, no-store',
		Vary: 'Authorization, Cookie',
		'X-Content-Type-Options': 'nosniff'
	});
	let id: string;
	try {
		id = memberIdentity(event.params.id);
	} catch {
		return Response.json(
			{ message: 'Provide a positive member identifier' },
			{ status: 400, headers }
		);
	}
	try {
		const base = API_BASE_URL || BACKEND_URL || 'http://localhost:8080';
		const upstream = await fetch(
			`${base}/api/admin/${family}/${id}/orders/export${event.url.search}`,
			{
				method: 'GET',
				headers: { Authorization: `Bearer ${token}` },
				credentials: 'omit',
				redirect: 'error',
				cache: 'no-store',
				signal: AbortSignal.any([event.request.signal, AbortSignal.timeout(60_000)])
			}
		);
		for (const name of [
			'content-type',
			'content-disposition',
			'x-export-member-id',
			'x-export-row-count',
			'x-export-sha256',
			'x-export-scope'
		]) {
			const value = upstream.headers.get(name);
			if (value) headers.set(name, value);
		}
		return new Response(upstream.body, { status: upstream.status, headers });
	} catch {
		return Response.json(
			{ message: 'Member order export is unavailable; no file was created' },
			{ status: 503, headers }
		);
	}
}

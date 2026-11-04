import type { RequestEvent } from '@sveltejs/kit';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import { requireAdmin, requireSuperadmin } from './auth.js';

/** Bound configuration writes and preserve actual statuses, including DELETE bodies. */
export async function proxyMemberConfig(
	event: RequestEvent,
	resource: 'segments' | 'tags' | 'filters',
	selected?: string
): Promise<Response> {
	const rest = selected ?? event.params.id ?? '';
	const headers = new Headers({
		'Cache-Control': 'private, no-store',
		'X-Content-Type-Options': 'nosniff'
	});
	const fail = (status: number, error: string) => Response.json({ error }, { status, headers });
	const numeric = /^[1-9]\d{0,18}$/.test(rest);
	const action = resource === 'tags' && ['assign', 'unassign', 'bulk-assign'].includes(rest);
	const segmentRead = resource === 'segments' && /^[1-9]\d{0,18}\/(?:analytics|export)$/.test(rest);
	if (rest && !numeric && !action && !segmentRead)
		return fail(404, 'Member configuration endpoint not found');
	const { token } =
		event.request.method === 'DELETE' && numeric ? requireSuperadmin(event) : requireAdmin(event);
	let body: BodyInit | undefined;
	if (!['GET', 'HEAD'].includes(event.request.method)) {
		const reader = event.request.body?.getReader();
		if (reader) {
			const chunks: Uint8Array[] = [];
			let length = 0;
			try {
				while (true) {
					const part = await reader.read();
					if (part.done) break;
					length += part.value.byteLength;
					if (length > 262_144) {
						await reader.cancel();
						return fail(413, 'Member configuration request exceeds256KiB');
					}
					chunks.push(part.value);
				}
				const bytes = new Uint8Array(length);
				let offset = 0;
				for (const part of chunks) {
					bytes.set(part, offset);
					offset += part.byteLength;
				}
				body = bytes;
			} catch {
				return fail(400, 'Unable to read member configuration request');
			} finally {
				reader.releaseLock();
			}
		}
	}
	try {
		const upstream = await fetch(
			`${API_BASE_URL || BACKEND_URL || 'http://localhost:8080'}/api/admin/members/${resource}${rest ? `/${rest}` : ''}${event.url.search}`,
			{
				method: event.request.method,
				headers: {
					Authorization: `Bearer ${token}`,
					'Content-Type': 'application/json',
					Accept: 'application/json'
				},
				body,
				credentials: 'omit',
				redirect: 'error',
				signal: AbortSignal.timeout(20000)
			}
		);
		for (const name of ['Content-Type', 'Retry-After', 'Content-Disposition']) {
			const value = upstream.headers.get(name);
			if (value) headers.set(name, value);
		}
		return new Response(upstream.body, { status: upstream.status, headers });
	} catch {
		return fail(503, 'Member configuration service is unavailable');
	}
}

import type { RequestHandler } from '@sveltejs/kit';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import { requireAdmin } from '#lib/server/auth.js';

const backend = API_BASE_URL || BACKEND_URL || 'http://localhost:8080';
const proxy: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const segments = (event.params.path || '').split('/').filter(Boolean);
	if (segments.some((s) => !/^[a-zA-Z0-9-]+$/.test(s)))
		return Response.json({ message: 'Invalid subscriber path' }, { status: 400 });
	const path = segments.length ? `/${segments.join('/')}` : '';
	const method = event.request.method;
	const headers: Record<string, string> = {
		Authorization: `Bearer ${token}`,
		Accept: event.request.headers.get('accept') || 'application/json'
	};
	const contentType = event.request.headers.get('content-type');
	if (contentType) headers['Content-Type'] = contentType;
	let body: ArrayBuffer | undefined;

	if (!['GET', 'HEAD'].includes(method)) {
		const reader = event.request.body?.getReader();
		if (reader) {
			const chunks: Uint8Array[] = [];
			let total = 0;
			try {
				while (true) {
					const chunk = await reader.read();
					if (chunk.done) break;
					total += chunk.value.byteLength;
					if (total > 65_536) {
						await reader.cancel();
						return Response.json(
							{ message: 'Subscriber request exceeds 64 KiB' },
							{ status: 413, headers: { 'Cache-Control': 'no-store' } }
						);
					}
					chunks.push(chunk.value);
				}
				const bytes = new Uint8Array(total);
				let offset = 0;
				for (const chunk of chunks) {
					bytes.set(chunk, offset);
					offset += chunk.byteLength;
				}
				body = bytes.buffer;
			} catch {
				return Response.json(
					{ message: 'Unable to read subscriber request' },
					{ status: 400, headers: { 'Cache-Control': 'no-store' } }
				);
			} finally {
				reader.releaseLock();
			}
		}
	}

	try {
		const response = await fetch(
			`${backend}/api/admin/email/subscribers${path}${event.url.search}`,
			{ method, headers, body, redirect: 'error', signal: AbortSignal.timeout(35_000) }
		);
		const forwarded = new Headers({
			'Cache-Control': 'no-store',
			'X-Content-Type-Options': 'nosniff'
		});
		for (const key of ['Content-Type', 'Content-Disposition', 'Retry-After']) {
			const value = response.headers.get(key);
			if (value) forwarded.set(key, value);
		}
		return new Response([204, 205, 304].includes(response.status) ? null : response.body, {
			status: response.status,
			headers: forwarded
		});
	} catch {
		return Response.json(
			{
				message: 'Subscriber service unavailable; if a save was submitted, refresh before retrying'
			},
			{ status: 502, headers: { 'Cache-Control': 'no-store' } }
		);
	}
};
export const GET = proxy;
export const POST = proxy;
export const PUT = proxy;
export const DELETE = proxy;

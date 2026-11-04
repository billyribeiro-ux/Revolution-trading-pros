import type { RequestEvent } from '@sveltejs/kit';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import { requireAdmin, requireSuperadmin } from './auth.js';
const backend = API_BASE_URL || BACKEND_URL || 'http://localhost:8080';

export async function proxyConnections(
	event: RequestEvent,
	operation?: 'connect' | 'test' | 'disconnect' | 'detail'
): Promise<Response> {
	const { token } = event.request.method === 'GET' ? requireAdmin(event) : requireSuperadmin(event);
	const headers = new Headers({ 'Cache-Control': 'no-store', 'X-Content-Type-Options': 'nosniff' });
	const failure = (status: number, message: string) =>
		Response.json({ success: false, error: message }, { status, headers });
	let suffix = '';
	if (operation) {
		const key = event.params.key;
		if (!key || !/^[a-z][a-z0-9_]{0,99}$/.test(key)) return failure(400, 'Invalid service key');
		suffix = `/${key}${operation === 'detail' ? '' : `/${operation}`}`;
	}
	let body: ArrayBuffer | undefined;
	if (!['GET', 'HEAD'].includes(event.request.method)) {
		const reader = event.request.body?.getReader();
		if (reader) {
			let total = 0;
			const chunks: Uint8Array[] = [];
			try {
				while (true) {
					const chunk = await reader.read();
					if (chunk.done) break;
					total += chunk.value.byteLength;
					if (total > 65_536) {
						await reader.cancel();
						return failure(413, 'Connection request exceeds 64 KiB');
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
				return failure(400, 'Unable to read connection request');
			} finally {
				reader.releaseLock();
			}
		}
	}
	try {
		const response = await fetch(`${backend}/api/admin/connections${suffix}${event.url.search}`, {
			method: event.request.method,
			headers: {
				Authorization: `Bearer ${token}`,
				Accept: 'application/json',
				...(body ? { 'Content-Type': 'application/json' } : {})
			},
			body,
			redirect: 'error',
			signal: AbortSignal.timeout(25_000)
		});
		for (const name of ['Content-Type', 'Retry-After']) {
			const value = response.headers.get(name);
			if (value) headers.set(name, value);
		}
		return new Response([204, 205, 304].includes(response.status) ? null : response.body, {
			status: response.status,
			headers
		});
	} catch {
		return failure(502, 'Connection service unavailable; refresh before retrying a submitted save');
	}
}

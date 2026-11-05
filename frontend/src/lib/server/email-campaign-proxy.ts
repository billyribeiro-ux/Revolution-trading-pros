import type { RequestEvent } from '@sveltejs/kit';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import { requireAdmin } from './auth.js';

/** Preserve the campaign contract, query and failure status without retrying writes. */
export async function proxyEmailCampaigns(event: RequestEvent): Promise<Response> {
	const rest = event.params.rest ?? '';
	const { token } = requireAdmin(event);
	const headers = new Headers({
		'Cache-Control': 'private, no-store',
		'X-Content-Type-Options': 'nosniff'
	});
	const fail = (status: number, error: string) => Response.json({ error }, { status, headers });
	const method = event.request.method;
	const record = /^[1-9]\d{0,18}$/.test(rest);
	const read =
		rest === 'stats' || /^[1-9]\d{0,18}\/(?:preview|analytics|report|stats|recipients)$/.test(rest);
	const action = /^[1-9]\d{0,18}\/(?:duplicate|test|send|schedule|cancel|pause|resume)$/.test(rest);
	if (
		!(rest === ''
			? ['GET', 'POST'].includes(method)
			: record
				? ['GET', 'PUT', 'DELETE'].includes(method)
				: read
					? method === 'GET'
					: action && method === 'POST')
	)
		return fail(404, 'Campaign endpoint not found');
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
					if (length > 9_000_000) {
						await reader.cancel();
						return fail(413, 'Email campaign request exceeds9MB');
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
				return fail(400, 'Unable to read email campaign request');
			} finally {
				reader.releaseLock();
			}
		}
	}
	try {
		const upstream = await fetch(
			`${API_BASE_URL || BACKEND_URL || 'http://localhost:8080'}/api/admin/email/campaigns${rest ? `/${rest}` : ''}${event.url.search}`,
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
		for (const name of ['Content-Type', 'Retry-After']) {
			const value = upstream.headers.get(name);
			if (value) headers.set(name, value);
		}
		return new Response(upstream.body, { status: upstream.status, headers });
	} catch {
		return fail(503, 'Email campaign service is unavailable');
	}
}

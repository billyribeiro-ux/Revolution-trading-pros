import type { RequestEvent } from '@sveltejs/kit';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import { requireAdmin } from './auth.js';
import { proxySmtp } from './smtp-proxy.js';

/** Preserve the template contract, query and failure status without retrying writes. */
export async function proxyEmailTemplates(event: RequestEvent): Promise<Response> {
	const rest = event.params.rest ?? '';
	if (rest === 'settings' || rest === 'settings/test')
		return proxySmtp(event, rest.endsWith('/test'));
	const { token } = requireAdmin(event);
	const headers = new Headers({
		'Cache-Control': 'private, no-store',
		'X-Content-Type-Options': 'nosniff'
	});
	const fail = (status: number, error: string) => Response.json({ error }, { status, headers });
	if (rest && !/^[1-9]\d{0,18}(?:\/(?:preview|test|duplicate))?$/.test(rest))
		return fail(404, 'Template endpoint not found');
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
						return fail(413, 'Email template request exceeds9MB');
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
				return fail(400, 'Unable to read email template request');
			} finally {
				reader.releaseLock();
			}
		}
	}
	try {
		const upstream = await fetch(
			`${API_BASE_URL || BACKEND_URL || 'http://localhost:8080'}/api/admin/email/templates${rest ? `/${rest}` : ''}${event.url.search}`,
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
		return fail(503, 'Email template service is unavailable');
	}
}

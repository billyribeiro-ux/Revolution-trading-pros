import type { RequestEvent } from '@sveltejs/kit';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import { requireAdmin, requireSuperadmin } from './auth.js';

const backend = API_BASE_URL || BACKEND_URL || 'http://localhost:8080';
export async function proxySmtp(event: RequestEvent, test = false): Promise<Response> {
	const { token } = event.request.method === 'GET' ? requireAdmin(event) : requireSuperadmin(event);
	const headers = new Headers({ 'Cache-Control': 'no-store', 'X-Content-Type-Options': 'nosniff' });
	const failure = (status: number, message: string) =>
		Response.json({ success: false, error: message }, { status, headers });
	let body: string | undefined;
	if (event.request.method !== 'GET') {
		const reader = event.request.body?.getReader();
		if (!reader) return failure(400, 'An SMTP settings body is required');
		const chunks: Uint8Array[] = [];
		let length = 0;
		try {
			while (true) {
				const part = await reader.read();
				if (part.done) break;
				length += part.value.byteLength;
				if (length > 16_384) {
					await reader.cancel();
					return failure(413, 'SMTP settings exceed 16 KiB');
				}
				chunks.push(part.value);
			}
			const bytes = new Uint8Array(length);
			let offset = 0;
			for (const part of chunks) {
				bytes.set(part, offset);
				offset += part.byteLength;
			}
			const value: unknown = JSON.parse(new TextDecoder('utf-8', { fatal: true }).decode(bytes));
			if (typeof value !== 'object' || value === null || Array.isArray(value))
				return failure(400, 'Invalid SMTP settings');
			const fields = { ...value } as Record<string, unknown>;
			if (['', '••••••••', '********'].includes(String(fields.password))) delete fields.password;
			body = JSON.stringify(fields);
		} catch {
			return failure(400, 'Invalid SMTP settings JSON');
		} finally {
			reader.releaseLock();
		}
	}
	try {
		const response = await fetch(`${backend}/api/admin/email/settings${test ? '/test' : ''}`, {
			method: event.request.method,
			headers: {
				Authorization: `Bearer ${token}`,
				Accept: 'application/json',
				'Content-Type': 'application/json'
			},
			body,
			redirect: 'error',
			signal: AbortSignal.timeout(20_000)
		});
		const retryAfter = response.headers.get('Retry-After');
		if (retryAfter) headers.set('Retry-After', retryAfter);
		let value: unknown;
		try {
			value = await response.json();
		} catch {
			return failure(
				response.ok ? 502 : response.status,
				response.status >= 400 && response.status < 500
					? 'Invalid SMTP settings request'
					: 'Invalid SMTP settings service response'
			);
		}
		if (typeof value !== 'object' || value === null || Array.isArray(value))
			return failure(502, 'Invalid SMTP settings response');
		const safe = { ...value } as Record<string, unknown>;
		delete safe.password;
		return Response.json(safe, { status: response.status, headers });
	} catch {
		return failure(
			502,
			'SMTP settings service unavailable; reload before retrying a submitted save'
		);
	}
}

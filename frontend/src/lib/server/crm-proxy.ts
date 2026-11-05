import type { RequestEvent } from '@sveltejs/kit';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import { crmAccessFailure, crmPrivateHeaders } from './crm-context.js';
import { crmRequestContext, requireCrmOperation } from './crm-auth.js';

/** One exact-policy CRM request, no transport retries and no response-envelope rewriting. */
export async function proxyCrmCanonical(event: RequestEvent): Promise<Response> {
	const headers = crmPrivateHeaders();
	try {
		const context = crmRequestContext(event);
		if (event.url.pathname === '/api/admin/crm/permissions') {
			if (!['GET', 'HEAD'].includes(event.request.method) || event.url.search)
				return Response.json(
					{ error: 'CRM capabilities endpoint not found' },
					{ status: 404, headers }
				);
			const current = await context.current();
			return event.request.method === 'HEAD'
				? new Response(null, { headers })
				: Response.json(current, { headers });
		}
		const { token } = await requireCrmOperation(event);
		const method = event.request.method;
		let body: Uint8Array<ArrayBuffer> | undefined;
		if (!['GET', 'HEAD'].includes(method) && event.request.body) {
			const reader = event.request.body.getReader(),
				parts: Uint8Array[] = [];
			let size = 0;
			try {
				while (true) {
					const part = await reader.read();
					if (part.done) break;
					size += part.value.byteLength;
					if (size > 2 * 1024 * 1024) {
						await reader.cancel();
						return Response.json({ error: 'CRM request exceeds2MiB' }, { status: 413, headers });
					}
					parts.push(part.value);
				}
				body = new Uint8Array(size);
				let at = 0;
				for (const part of parts) {
					body.set(part, at);
					at += part.byteLength;
				}
			} catch {
				return Response.json({ error: 'Unable to read CRM request' }, { status: 400, headers });
			} finally {
				reader.releaseLock();
			}
		}
		const outbound = new Headers({
			Authorization: `Bearer ${token}`,
			Accept: event.request.headers.get('Accept') ?? 'application/json'
		});
		const contentType = event.request.headers.get('Content-Type');
		if (contentType) outbound.set('Content-Type', contentType);
		const base = API_BASE_URL || BACKEND_URL || 'http://localhost:8080';
		const response = await fetch(`${base}${event.url.pathname}${event.url.search}`, {
			method,
			headers: outbound,
			body,
			credentials: 'omit',
			redirect: 'error',
			cache: 'no-store',
			signal: AbortSignal.timeout(30000)
		});
		for (const name of ['Content-Type', 'Content-Disposition', 'Retry-After']) {
			const value = response.headers.get(name);
			if (value) headers.set(name, value);
		}
		return new Response(method === 'HEAD' ? null : response.body, {
			status: response.status,
			headers
		});
	} catch (error) {
		return crmAccessFailure(error);
	}
}

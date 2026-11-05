import type { RequestEvent } from '@sveltejs/kit';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import { requireCrmOperation } from './crm-auth.js';
import { crmAccessFailure, crmPrivateHeaders } from './crm-context.js';

/** Preserve the canonical DTO and provider receipts, including non-2xx outcomes. */
export async function proxyCrmContacts(event: RequestEvent): Promise<Response> {
	let token: string;
	try {
		({ token } = await requireCrmOperation(event));
	} catch (error) {
		return crmAccessFailure(error);
	}
	const rest = event.params.rest ?? '',
		method = event.request.method;
	const headers = crmPrivateHeaders();
	const fail = (status: number, error: string) => Response.json({ error }, { status, headers });
	const id = '[1-9]\\d{0,18}';
	const nativeTemplates = rest === 'native-templates';
	const base = new RegExp(`^${id}$`).test(rest),
		panel = new RegExp(`^${id}/(?:timeline|emails|notes|tags|lists)$`).test(rest),
		action = new RegExp(`^${id}/(?:recalculate-score|send-email)$`).test(rest),
		membership = new RegExp(`^${id}/(?:tags|lists)/${id}$`).test(rest);
	if (
		!(nativeTemplates
			? ['GET', 'HEAD'].includes(method)
			: rest === ''
				? ['GET', 'HEAD', 'POST'].includes(method)
				: base
					? ['GET', 'HEAD', 'PUT', 'DELETE'].includes(method)
					: panel
						? ['GET', 'HEAD'].includes(method) ||
							(method === 'POST' && /\/(?:notes|tags|lists)$/.test(rest))
						: action
							? method === 'POST'
							: membership && method === 'DELETE')
	)
		return fail(404, 'Contact endpoint not found');
	let body: Uint8Array<ArrayBuffer> | undefined;
	if (['POST', 'PUT'].includes(method)) {
		if (!(event.request.headers.get('Content-Type') ?? '').startsWith('application/json'))
			return fail(415, 'Provide a JSON contact request');
		const reader = event.request.body?.getReader(),
			chunks: Uint8Array[] = [];
		let size = 0;
		if (reader)
			try {
				while (true) {
					const item = await reader.read();
					if (item.done) break;
					size += item.value.byteLength;
					if (size > 2 * 1024 * 1024) {
						await reader.cancel();
						return fail(413, 'Contact request exceeds 2 MiB');
					}
					chunks.push(item.value);
				}
				body = new Uint8Array(size);
				let offset = 0;
				for (const item of chunks) {
					body.set(item, offset);
					offset += item.byteLength;
				}
			} catch {
				return fail(400, 'Unable to read contact request');
			} finally {
				reader.releaseLock();
			}
	}
	try {
		const response = await fetch(
			`${API_BASE_URL || BACKEND_URL || 'http://localhost:8080'}/api/admin/crm/contacts${rest ? `/${rest}` : ''}${event.url.search}`,
			{
				method,
				headers: {
					Authorization: `Bearer ${token}`,
					Accept: 'application/json',
					'Content-Type': 'application/json'
				},
				body,
				credentials: 'omit',
				redirect: 'error',
				signal: AbortSignal.timeout(30000)
			}
		);
		headers.set('Content-Type', response.headers.get('Content-Type') ?? 'application/json');
		const retry = response.headers.get('Retry-After');
		if (retry) headers.set('Retry-After', retry);
		return new Response(response.body, { status: response.status, headers });
	} catch {
		return fail(503, 'Contact service unavailable. Refresh saved state before retrying a write.');
	}
}

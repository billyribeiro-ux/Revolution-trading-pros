import type { RequestEvent } from '@sveltejs/kit';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import { requireAdmin } from './auth.js';
import { requireCrmOperation } from './crm-auth.js';
import { crmAccessFailure, crmPrivateHeaders } from './crm-context.js';

/** Sequence authoring and delivery actions never retry an unconfirmed write. */
export async function proxySequences(event: RequestEvent): Promise<Response> {
	const nativePrefix = '/api/admin/email/sequences';
	const canonicalPrefix = '/api/admin/crm/sequences';
	const actual = event.url.pathname;
	let token: string;
	if (actual === canonicalPrefix || actual.startsWith(canonicalPrefix + '/')) {
		try {
			({ token } = await requireCrmOperation(event));
		} catch (error) {
			return crmAccessFailure(error);
		}
	} else if (actual === nativePrefix || actual.startsWith(nativePrefix + '/'))
		({ token } = requireAdmin(event));
	else
		return Response.json(
			{ error: 'Sequence endpoint not found' },
			{ status: 404, headers: crmPrivateHeaders() }
		);
	const rest = event.params.rest ?? '';
	const method = event.request.method;
	const headers = crmPrivateHeaders();
	const fail = (status: number, error: string) => Response.json({ error }, { status, headers });
	const id = '[1-9]\\d{0,18}';
	const uuid = '[0-9a-fA-F]{8}(?:-[0-9a-fA-F]{4}){3}-[0-9a-fA-F]{12}';
	const record = new RegExp(`^${id}$`).test(rest);
	const emails = new RegExp(`^${id}/emails$`).test(rest);
	const mail = new RegExp(`^${id}/emails/${uuid}$`).test(rest);
	const read = new RegExp(`^${id}/(?:stats|subscribers|tests|triggers)$`).test(rest);
	const action = new RegExp(
		`^${id}/(?:duplicate|activate|pause|subscribe|unsubscribe|test|emails/reorder)$`
	).test(rest);
	if (
		!(rest === 'events'
			? method === 'POST'
			: rest === ''
				? ['GET', 'HEAD', 'POST'].includes(method)
				: record
					? ['GET', 'HEAD', 'PUT', 'DELETE'].includes(method)
					: emails
						? ['GET', 'HEAD', 'POST'].includes(method)
						: mail
							? ['PUT', 'DELETE'].includes(method)
							: read
								? ['GET', 'HEAD'].includes(method)
								: action && method === 'POST')
	)
		return fail(404, 'Sequence endpoint not found');
	let body: Uint8Array<ArrayBuffer> | undefined;
	if (!['GET', 'HEAD'].includes(method)) {
		const reader = event.request.body?.getReader();
		if (reader) {
			const chunks: Uint8Array[] = [];
			let size = 0;
			try {
				while (true) {
					const part = await reader.read();
					if (part.done) break;
					size += part.value.byteLength;
					if (size > 9_000_000) {
						await reader.cancel();
						return fail(413, 'Sequence request exceeds9MB');
					}
					chunks.push(part.value);
				}
				body = new Uint8Array(size);
				let offset = 0;
				for (const chunk of chunks) {
					body.set(chunk, offset);
					offset += chunk.byteLength;
				}
			} catch {
				return fail(400, 'Unable to read sequence request');
			} finally {
				reader.releaseLock();
			}
		}
	}
	try {
		const response = await fetch(
			`${API_BASE_URL || BACKEND_URL || 'http://localhost:8080'}/api/admin/crm/sequences${rest ? `/${rest}` : ''}${event.url.search}`,
			{
				method,
				headers: {
					Authorization: `Bearer ${token}`,
					'Content-Type': 'application/json',
					Accept: 'application/json'
				},
				body,
				credentials: 'omit',
				redirect: 'error',
				signal: AbortSignal.timeout(rest.endsWith('/test') ? 45000 : 20000)
			}
		);
		for (const name of ['Content-Type', 'Retry-After']) {
			const value = response.headers.get(name);
			if (value) headers.set(name, value);
		}
		return new Response(response.body, { status: response.status, headers });
	} catch {
		return fail(
			503,
			'Sequence service is unavailable; read its saved state before retrying an action'
		);
	}
}

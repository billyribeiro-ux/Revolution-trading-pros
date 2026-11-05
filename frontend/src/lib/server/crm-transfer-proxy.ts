import type { RequestEvent } from '@sveltejs/kit';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import { requireCrmOperation } from './crm-auth.js';
import { crmAccessFailure, crmPrivateHeaders } from './crm-context.js';

/** Forward one bounded transfer request, preserving multipart boundaries and downloads. */
export async function proxyCrmTransfers(
	event: RequestEvent,
	direction: 'imports' | 'exports'
): Promise<Response> {
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
	const uuid = '[0-9a-fA-F]{8}(?:-[0-9a-fA-F]{4}){3}-[0-9a-fA-F]{12}';
	const record = new RegExp(`^${uuid}$`).test(rest);
	const read = new RegExp(`^${uuid}/${direction === 'imports' ? 'preview' : 'download'}$`).test(
		rest
	);
	const action = direction === 'imports' && new RegExp(`^${uuid}/(?:start|cancel)$`).test(rest);
	const fields =
		direction === 'exports' &&
		/^fields\/(?:contacts|tags|lists|sequences|campaigns|automations|templates)$/.test(rest);
	const options = direction === 'imports' && rest === 'options';
	if (
		!(rest === ''
			? ['GET', 'HEAD', 'POST'].includes(method)
			: record || read || fields || options
				? ['GET', 'HEAD'].includes(method)
				: action && method === 'POST')
	)
		return fail(404, 'Transfer endpoint not found');
	const contentType = event.request.headers.get('Content-Type') ?? 'application/json';
	if (
		method === 'POST' &&
		!contentType.startsWith('application/json') &&
		!(direction === 'imports' && rest === '' && contentType.startsWith('multipart/form-data;'))
	)
		return fail(415, 'Provide JSON or a multipart contact upload');
	let body: Uint8Array<ArrayBuffer> | undefined;
	if (method === 'POST' && event.request.body) {
		const reader = event.request.body.getReader(),
			chunks: Uint8Array[] = [];
		let size = 0;
		try {
			while (true) {
				const result = await reader.read();
				if (result.done) break;
				size += result.value.byteLength;
				if (size > 2 * 1024 * 1024 + 65536) {
					await reader.cancel();
					return fail(413, 'Transfer upload exceeds2MiB');
				}
				chunks.push(result.value);
			}
			body = new Uint8Array(size);
			let offset = 0;
			for (const chunk of chunks) {
				body.set(chunk, offset);
				offset += chunk.byteLength;
			}
		} catch {
			return fail(400, 'Unable to read transfer request');
		} finally {
			reader.releaseLock();
		}
	}
	try {
		const upstream = await fetch(
			`${API_BASE_URL || BACKEND_URL || 'http://localhost:8080'}/api/admin/crm/${direction}${rest ? `/${rest}` : ''}${event.url.search}`,
			{
				method,
				headers: {
					Authorization: `Bearer ${token}`,
					'Content-Type': contentType,
					Accept: event.request.headers.get('Accept') ?? 'application/json'
				},
				body,
				redirect: 'error',
				credentials: 'omit',
				signal: AbortSignal.timeout(30000)
			}
		);
		for (const name of ['Content-Type', 'Content-Disposition', 'Retry-After']) {
			const value = upstream.headers.get(name);
			if (value) headers.set(name, value);
		}
		return new Response(upstream.body, { status: upstream.status, headers });
	} catch {
		return fail(
			503,
			'Transfer service is unavailable. Refresh saved job status before retrying a write.'
		);
	}
}

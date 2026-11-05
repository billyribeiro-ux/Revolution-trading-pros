/** A single authenticated upstream attempt for factual member reads and note writes. */
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import type { RequestEvent } from '@sveltejs/kit';
import { requireAdmin } from './auth.js';
import { parseMemberNoteCreated, parseMemberNoteRemoved } from '../api/member-profile.js';

type Family = 'members' | 'member-management';
export type MemberHistoryPath =
	| 'memberships'
	| 'orders'
	| 'notes'
	| 'activity'
	| 'emails'
	| 'timeline';
const base = API_BASE_URL || BACKEND_URL || 'http://localhost:8080';
const responseLimit = 8 * 1024 * 1024;
const noteLimit = 41 * 1024;
const privateHeaders = {
	'Cache-Control': 'private, no-store',
	Vary: 'Authorization, Cookie',
	'X-Content-Type-Options': 'nosniff'
};
function failure(status: number, message: string, mutation: boolean): Response {
	return Response.json(
		{
			message,
			...(mutation ? { outcome: 'unconfirmed', reconcile_history_before_retry: true } : {})
		},
		{ status, headers: privateHeaders }
	);
}
function identity(value: string | undefined): string {
	if (!value || !/^[1-9]\d{0,18}$/.test(value) || BigInt(value) > 9223372036854775807n)
		throw new Error('Invalid member or note identifier');
	return value;
}
async function boundedText(
	body: ReadableStream<Uint8Array> | null,
	limit: number
): Promise<string> {
	if (!body) throw new Error('The service returned an empty body');
	const reader = body.getReader(),
		chunks: Uint8Array[] = [];
	let length = 0;
	try {
		for (;;) {
			const { done, value } = await reader.read();
			if (done) break;
			length += value.byteLength;
			if (length > limit) {
				await reader.cancel();
				throw new Error('The response exceeds the supported size');
			}
			chunks.push(value);
		}
	} finally {
		reader.releaseLock();
	}
	const bytes = new Uint8Array(length);
	let offset = 0;
	for (const chunk of chunks) {
		bytes.set(chunk, offset);
		offset += chunk.byteLength;
	}
	return new TextDecoder('utf-8', { fatal: true }).decode(bytes);
}
function historyQuery(event: RequestEvent, history: MemberHistoryPath | undefined): string {
	const query = event.url.searchParams;
	for (const name of query.keys()) {
		if (
			!history ||
			!['page', 'per_page', ...(history === 'activity' ? ['action'] : [])].includes(name) ||
			query.getAll(name).length !== 1
		)
			throw new Error('Unsupported or duplicate member-history query');
	}
	for (const name of ['page', 'per_page']) {
		const value = query.get(name);
		if (value === null) continue;
		if (!/^[1-9]\d*$/.test(value) || !Number.isSafeInteger(Number(value)))
			throw new Error('Invalid member-history pagination');
		if (name === 'per_page' && Number(value) > 100)
			throw new Error('Member-history page size must be between 1 and 100');
	}
	const action = query.get('action');
	if (
		action !== null &&
		(!action.trim() || Array.from(action).length > 100 || /\p{Cc}/u.test(action))
	)
		throw new Error('Invalid recorded activity action');
	return query.size ? `?${query.toString()}` : '';
}
async function noteBody(event: RequestEvent): Promise<string> {
	if (
		event.request.headers.get('Content-Type')?.split(';', 1)[0].trim().toLowerCase() !==
		'application/json'
	)
		throw new Error('A note requires an application/json body');
	const raw = await boundedText(event.request.body, noteLimit),
		value: unknown = JSON.parse(raw);
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Provide a note content object');
	const record = value as Record<string, unknown>;
	if (
		Object.keys(record).length !== 1 ||
		typeof record.content !== 'string' ||
		!record.content.trim() ||
		Array.from(record.content).length > 10000 ||
		new TextEncoder().encode(record.content).byteLength > 40000 ||
		/[^\P{Cc}\n\r\t]/u.test(record.content)
	)
		throw new Error('Provide a nonempty note within 10000 characters, without control characters');
	return JSON.stringify({ content: record.content });
}

export async function proxyMemberProfile(
	event: RequestEvent,
	family: Family,
	history?: MemberHistoryPath,
	noteId?: string,
	fetcher: typeof fetch = fetch
): Promise<Response> {
	const { token, user } = requireAdmin(event),
		method = event.request.method,
		mutation = method !== 'GET';
	if (
		method !== 'GET' &&
		!(method === 'POST' && history === 'notes' && !noteId) &&
		!(method === 'DELETE' && history === 'notes' && noteId)
	)
		return failure(405, 'Unsupported member-profile method', false);
	let target: string, body: string | undefined;
	try {
		target = `${base}/api/admin/${family}/${identity(event.params.id)}`;
		if (history) target += `/${history}`;
		if (noteId) target += `/${identity(noteId)}`;
		if (mutation && event.url.search) throw new Error('Note writes do not accept a query string');
		if (!mutation) target += historyQuery(event, history);
		if (method === 'POST') body = await noteBody(event);
	} catch (error) {
		return failure(400, error instanceof Error ? error.message : 'Invalid member request', false);
	}
	try {
		const upstream = await fetcher(target, {
			method,
			headers: {
				Accept: 'application/json',
				Authorization: `Bearer ${token}`,
				...(body === undefined ? {} : { 'Content-Type': 'application/json' })
			},
			body,
			credentials: 'omit',
			redirect: 'error',
			cache: 'no-store',
			signal: AbortSignal.any([event.request.signal, AbortSignal.timeout(15_000)])
		});
		const raw = await boundedText(upstream.body, responseLimit);
		let payload: unknown;
		try {
			payload = JSON.parse(raw);
		} catch {
			if (!upstream.ok)
				return failure(
					upstream.status,
					'The member service rejected the request',
					mutation && upstream.status >= 500
				);
			return failure(502, 'The member service returned an unreadable acknowledgment', mutation);
		}
		if (payload === null || typeof payload !== 'object' || Array.isArray(payload))
			return failure(502, 'The member service returned an invalid acknowledgment', mutation);
		if (mutation && upstream.status >= 500) {
			return Response.json(
				{ ...payload, outcome: 'unconfirmed', reconcile_history_before_retry: true },
				{ status: upstream.status, headers: privateHeaders }
			);
		}
		if (upstream.ok && mutation) {
			try {
				if (method === 'POST' && body) {
					const submitted = JSON.parse(body) as { content: string };
					const saved = parseMemberNoteCreated(
						payload,
						identity(event.params.id),
						submitted.content
					);
					if (saved.created_by !== String(user.id))
						throw new Error('The recorded note actor differs');
				} else if (method === 'DELETE' && noteId) {
					parseMemberNoteRemoved(payload, identity(event.params.id), identity(noteId));
				}
			} catch {
				return failure(
					502,
					'The note outcome is unconfirmed. Refresh its history before retrying.',
					true
				);
			}
		}
		return Response.json(payload, { status: upstream.status, headers: privateHeaders });
	} catch {
		return failure(
			503,
			mutation
				? 'The note outcome is unconfirmed. Refresh its history before retrying.'
				: 'The member service is unavailable. Retry to load current records.',
			mutation
		);
	}
}

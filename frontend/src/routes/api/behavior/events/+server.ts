import type { RequestHandler } from './$types';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';

const API_ROOT = API_BASE_URL || BACKEND_URL || 'http://localhost:8080';
const MAX_BYTES = 64 * 1024;

/** A receipt is returned only after the Rust API commits the batch. */
export const POST: RequestHandler = async ({ request, fetch, url }) => {
	const origin = request.headers.get('origin');
	if (origin && origin !== url.origin) {
		return Response.json({ error: 'Origin not allowed' }, { status: 403 });
	}
	if (!request.body) return Response.json({ error: 'Empty batch' }, { status: 400 });
	const reader = request.body.getReader();
	const chunks: Uint8Array[] = [];
	let size = 0;
	try {
		while (true) {
			const { done, value } = await reader.read();
			if (done) break;
			size += value.byteLength;
			if (size > MAX_BYTES) {
				await reader.cancel();
				return Response.json({ error: 'Batch too large' }, { status: 413 });
			}
			chunks.push(value);
		}
	} finally {
		reader.releaseLock();
	}
	const body = new Uint8Array(size);
	let offset = 0;
	for (const chunk of chunks) {
		body.set(chunk, offset);
		offset += chunk.byteLength;
	}
	try {
		const upstream = await fetch(`${API_ROOT}/api/behavior/events`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json', Accept: 'application/json' },
			body,
			signal: AbortSignal.timeout(15_000)
		});
		return new Response(await upstream.text(), {
			status: upstream.status,
			headers: { 'Content-Type': 'application/json', 'Cache-Control': 'no-store' }
		});
	} catch {
		return Response.json(
			{ success: false, error: 'Behavior storage unavailable' },
			{ status: 503 }
		);
	}
};

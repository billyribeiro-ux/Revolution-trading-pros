import type { RequestHandler } from '@sveltejs/kit';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';

const API_ROOT = API_BASE_URL || BACKEND_URL || 'http://localhost:8080';
const MAX_BYTES = 16_384;
const headers = { 'Cache-Control': 'no-store' };

/** Await the backend's durable receipt; never acknowledge detached work. */
export const performanceProxy: RequestHandler = async ({ request, fetch }) => {
	const reader = request.body?.getReader();
	if (!reader)
		return Response.json(
			{ success: false, message: 'A metric body is required' },
			{ status: 400, headers }
		);
	let total = 0;
	const chunks: Uint8Array[] = [];
	try {
		while (true) {
			const part = await reader.read();
			if (part.done) break;
			total += part.value.byteLength;
			if (total > MAX_BYTES) {
				await reader.cancel();
				return Response.json(
					{ success: false, message: 'Metric body exceeds 16 KiB' },
					{ status: 413, headers }
				);
			}
			chunks.push(part.value);
		}
		const bytes = new Uint8Array(total);
		let offset = 0;
		for (const chunk of chunks) {
			bytes.set(chunk, offset);
			offset += chunk.byteLength;
		}
		const forwarded = new Headers({
			'Content-Type': 'application/json',
			Accept: 'application/json'
		});
		for (const name of ['referer', 'user-agent']) {
			const value = request.headers.get(name);
			if (value) forwarded.set(name, value);
		}
		const response = await fetch(`${API_ROOT}/api/analytics/web-vitals`, {
			method: 'POST',
			headers: forwarded,
			body: bytes.buffer,
			signal: AbortSignal.timeout(10_000)
		});
		return new Response(response.body, {
			status: response.status,
			headers: {
				...headers,
				'Content-Type': response.headers.get('content-type') || 'application/json'
			}
		});
	} catch {
		return Response.json(
			{ success: false, message: 'Performance storage is unavailable' },
			{ status: 502, headers }
		);
	} finally {
		reader.releaseLock();
	}
};

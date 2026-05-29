/**
 * SSE (Server-Sent Events) API Proxy
 */

import { env } from '$env/dynamic/private';
import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// FIX-2026-04-26: was empty-stream stub; now real SSE proxy to backend
const API_URL = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

// FIX-2026-04-26: old empty-stream stub commented out below
// export const GET: RequestHandler = async () => {
// 	// Return an empty event stream that closes immediately
// 	// This prevents 404 errors when SSE is not implemented on backend
// 	const headers = new Headers({
// 		'Content-Type': 'text/event-stream',
// 		'Cache-Control': 'no-cache',
// 		Connection: 'keep-alive'
// 	});
//
// 	// Create a readable stream that sends a heartbeat and closes
// 	const stream = new ReadableStream({
// 		start(controller) {
// 			// Send initial comment to indicate connection
// 			controller.enqueue(
// 				new TextEncoder().encode(': SSE endpoint placeholder - backend SSE not configured\n\n')
// 			);
// 			// Close the stream immediately - SSE not implemented
// 			controller.close();
// 		}
// 	});
//
// 	return new Response(stream, { headers });
// };

export const GET: RequestHandler = async ({ cookies, fetch, request }) => {
	const token = cookies.get('rtp_access_token');
	const headers: HeadersInit = { Accept: 'text/event-stream' };
	if (token) headers.Authorization = `Bearer ${token}`;

	const upstream = await fetch(`${API_URL}/api/realtime/events`, {
		headers,
		signal: request.signal
	});
	if (!upstream.ok || !upstream.body) {
		error(upstream.status || 502, 'SSE upstream unavailable');
	}
	return new Response(upstream.body, {
		status: 200,
		headers: {
			'Content-Type': 'text/event-stream',
			'Cache-Control': 'no-cache, no-transform',
			'X-Accel-Buffering': 'no'
		}
	});
};

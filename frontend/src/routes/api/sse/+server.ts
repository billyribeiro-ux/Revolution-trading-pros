/**
 * SSE (Server-Sent Events) API Proxy
 * ICT 7 FIX: Graceful fallback when SSE is not available on backend
 * Returns empty event stream to prevent 404 errors
 */

import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async () => {
	// Return an empty event stream that closes immediately
	// This prevents 404 errors when SSE is not implemented on backend
	const headers = new Headers({
		'Content-Type': 'text/event-stream',
		'Cache-Control': 'no-cache',
		'Connection': 'keep-alive'
	});

	// Create a readable stream that sends a heartbeat and closes
	const stream = new ReadableStream({
		start(controller) {
			// Send initial comment to indicate connection
			controller.enqueue(new TextEncoder().encode(': SSE endpoint placeholder - backend SSE not configured\n\n'));
			// Close the stream immediately - SSE not implemented
			controller.close();
		}
	});

	return new Response(stream, { headers });
};

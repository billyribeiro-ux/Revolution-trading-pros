import type { RequestHandler } from './$types';
import { proxyConnections } from '#lib/server/connection-proxy.js';
import { connectionStatusView } from '#lib/server/connection-status.js';

export const GET: RequestHandler = async (event) => {
	// Retrieve every canonical row, then apply compatibility catalog filters once.
	const url = new URL(event.url);
	url.searchParams.delete('category');
	url.searchParams.delete('status');
	const response = await proxyConnections({ ...event, url });
	if (!response.ok) return response;
	const headers = new Headers(response.headers);
	try {
		return Response.json(connectionStatusView(await response.json(), event.url.searchParams), {
			headers
		});
	} catch {
		return Response.json(
			{ success: false, error: 'Invalid connection status response' },
			{ status: 502, headers }
		);
	}
};

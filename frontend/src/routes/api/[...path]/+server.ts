/**
 * Generic Catch-All API Proxy
 * ICT 11+ CORB Fix: Routes all /api/* requests through SvelteKit server to prevent CORB
 * 
 * This handles any API endpoint that doesn't have a dedicated proxy route.
 * All requests are forwarded to the backend API with proper auth headers.
 */
import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const PROD_API_ROOT = 'https://revolution-trading-pros-api.fly.dev';
const API_ROOT = env.VITE_API_URL || env.BACKEND_URL || PROD_API_ROOT;

async function proxyRequest(
	request: Request,
	path: string,
	url: URL,
	cookies: any,
	fetchFn: typeof fetch
) {
	try {
		// Build target URL
		const targetUrl = new URL(`${API_ROOT}/api/${path}`);
		
		// Forward query params
		url.searchParams.forEach((value, key) => {
			targetUrl.searchParams.set(key, value);
		});

		// Build headers
		const token = cookies.get('rtp_access_token');
		const headers: Record<string, string> = {
			Accept: 'application/json'
		};
		
		// Forward content-type for POST/PUT/PATCH
		const contentType = request.headers.get('content-type');
		if (contentType) {
			headers['Content-Type'] = contentType;
		}
		
		if (token) {
			headers.Authorization = `Bearer ${token}`;
		}

		// Get body for non-GET requests
		let body: string | null = null;
		if (request.method !== 'GET' && request.method !== 'HEAD') {
			try {
				body = await request.text();
			} catch {
				// No body
			}
		}

		// Forward request to backend
		const response = await fetchFn(targetUrl.toString(), {
			method: request.method,
			headers,
			body
		});

		// Try to parse as JSON, fallback to text
		const responseText = await response.text();
		try {
			const data = JSON.parse(responseText);
			return json(data, { 
				status: response.status,
				headers: {
					'Cache-Control': response.headers.get('Cache-Control') || 'no-cache'
				}
			});
		} catch {
			// Return as-is if not JSON
			return new Response(responseText, {
				status: response.status,
				headers: {
					'Content-Type': response.headers.get('Content-Type') || 'text/plain'
				}
			});
		}
	} catch (error) {
		console.error('[API Proxy] Error:', error);
		return json({ error: 'Proxy request failed' }, { status: 500 });
	}
}

export const GET: RequestHandler = async ({ params, url, cookies, fetch, request }) => {
	const path = params.path || '';
	return proxyRequest(request, path, url, cookies, fetch);
};

export const POST: RequestHandler = async ({ params, url, cookies, fetch, request }) => {
	const path = params.path || '';
	return proxyRequest(request, path, url, cookies, fetch);
};

export const PUT: RequestHandler = async ({ params, url, cookies, fetch, request }) => {
	const path = params.path || '';
	return proxyRequest(request, path, url, cookies, fetch);
};

export const PATCH: RequestHandler = async ({ params, url, cookies, fetch, request }) => {
	const path = params.path || '';
	return proxyRequest(request, path, url, cookies, fetch);
};

export const DELETE: RequestHandler = async ({ params, url, cookies, fetch, request }) => {
	const path = params.path || '';
	return proxyRequest(request, path, url, cookies, fetch);
};

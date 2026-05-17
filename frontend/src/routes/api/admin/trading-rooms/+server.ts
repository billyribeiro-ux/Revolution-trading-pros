/**
 * Trading Rooms API Endpoint
 * ICT 7 Principal Engineer Grade
 *
 * Proxies to backend for trading room management.
 *
 * @version 2.0.0 - January 2026
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
import { requireAdmin } from '$lib/server/auth';
const PROD_BACKEND =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

async function fetchFromBackend(
	endpoint: string,
	options?: RequestInit
): Promise<{ data: unknown; status: number }> {
	const backendUrl = PROD_BACKEND;

	try {
		const response = await fetch(`${backendUrl}/api${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...(options?.headers || {})
			}
		});

		const data = await response.json();
		return { data, status: response.status };
	} catch (err) {
		console.error(`Backend error for ${endpoint}:`, err);
		return { data: null, status: 500 };
	}
}

// GET - List trading rooms
export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { url } = event;
	const authHeader = `Bearer ${token}`;
	const queryParams = url.searchParams.toString();
	const endpoint = `/trading-rooms${queryParams ? `?${queryParams}` : ''}`;

	const { data, status } = await fetchFromBackend(endpoint, {
		headers: { Authorization: authHeader }
	});

	if (status >= 400 || !data) {
		return json({
			success: false,
			data: [],
			error: 'Failed to fetch trading rooms from backend'
		});
	}

	return json(data);
};

// POST - Create trading room
export const POST: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { request } = event;
	const authHeader = `Bearer ${token}`;

	try {
		const body = await request.json();

		const { data, status } = await fetchFromBackend('/admin/trading-rooms', {
			method: 'POST',
			headers: { Authorization: authHeader },
			body: JSON.stringify(body)
		});

		if (status >= 400) {
			error(status, 'Failed to create trading room');
		}

		return json(data);
	} catch (err) {
		console.error('POST /api/admin/trading-rooms error:', err);
		error(400, 'Invalid request body');
	}
};

/**
 * Email Templates API Endpoint
 * ICT 7 Principal Engineer Grade
 *
 * Proxies to backend for email template CRUD operations.
 *
 * @version 2.0.0 - January 2026
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';

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

export const GET: RequestHandler = async ({ request }) => {
	const authHeader = request.headers.get('Authorization') || '';

	const { data, status } = await fetchFromBackend('/admin/email/templates', {
		headers: { Authorization: authHeader }
	});

	if (status >= 400 || !data) {
		return json({
			success: false,
			data: [],
			total: 0,
			error: 'Failed to fetch email templates'
		});
	}

	return json(data);
};

export const POST: RequestHandler = async ({ request }) => {
	const authHeader = request.headers.get('Authorization') || '';

	try {
		const body = await request.json();

		if (!body.name || !body.subject) {
			return json({ success: false, error: 'Name and subject are required' }, { status: 400 });
		}

		const { data, status } = await fetchFromBackend('/admin/email/templates', {
			method: 'POST',
			headers: { Authorization: authHeader },
			body: JSON.stringify(body)
		});

		if (status >= 400) {
			error(status, 'Failed to create template');
		}

		return json(data);
	} catch (err) {
		console.error('POST /api/admin/email/templates error:', err);
		error(400, 'Invalid request body');
	}
};

/**
 * Customer Billing Portal Proxy
 *
 * POST /api/payments/portal → backend creates a Stripe Customer Portal session
 * and returns { url }. Frontend redirects the user to that URL.
 *
 * Auth: reads rtp_access_token cookie (standard user session, not admin).
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

const API_URL = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export const POST: RequestHandler = async ({ cookies, request }) => {
	const token = cookies.get('rtp_access_token');
	if (!token) error(401, 'Unauthorized');

	let body: unknown;
	try {
		body = await request.json();
	} catch {
		body = {};
	}

	const upstream = await fetch(`${API_URL}/api/payments/portal`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Accept: 'application/json',
			Authorization: `Bearer ${token}`
		},
		body: JSON.stringify(body)
	}).catch((err) => {
		console.error('[portal proxy] backend unreachable:', err);
		return null;
	});

	if (!upstream) {
		return json({ error: 'Payment service unreachable' }, { status: 502 });
	}

	const text = await upstream.text();
	let data: unknown;
	try {
		data = JSON.parse(text);
	} catch {
		return json({ error: 'Invalid response from payment service' }, { status: 502 });
	}

	return json(data, { status: upstream.status });
};

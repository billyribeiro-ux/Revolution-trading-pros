/**
 * Email Templates API Endpoint
 * ICT 7 Principal Engineer Grade
 *
 * Proxies to backend for email template CRUD operations.
 *
 * R21-A: migrated to shared `fetchBackendWithStatus` helper. Body narrowing
 * added to POST handler (`isObject` guard on `request.json()`) — was: implicit
 * `any` followed by unchecked `.name` / `.subject` reads. Latent NPE on
 * `null` / primitive bodies now returns 400 instead of 500.
 *
 * @version 3.0.0 — 2026-05-20
 */

import { json, error, isHttpError } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { requireAdmin } from '$lib/server/auth';
import { fetchBackendWithStatus, isObject } from '$lib/server/proxy-fetch';

export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const authHeader = `Bearer ${token}`;

	const { data, status } = await fetchBackendWithStatus(
		'/api/admin/email/templates',
		{ headers: { Authorization: authHeader } },
		'[Email Templates API]'
	);

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

export const POST: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { request } = event;
	const authHeader = `Bearer ${token}`;

	try {
		const body: unknown = await request.json();

		if (!isObject(body) || typeof body.name !== 'string' || typeof body.subject !== 'string') {
			return json({ success: false, error: 'Name and subject are required' }, { status: 400 });
		}

		const { data, status } = await fetchBackendWithStatus(
			'/api/admin/email/templates',
			{
				method: 'POST',
				headers: { Authorization: authHeader },
				body: JSON.stringify(body)
			},
			'[Email Templates API]'
		);

		if (status >= 400) {
			error(status, 'Failed to create template');
		}

		return json(data);
	} catch (err) {
		if (isHttpError(err)) throw err;
		console.error('POST /api/admin/email/templates error:', err);
		error(400, 'Invalid request body');
	}
};

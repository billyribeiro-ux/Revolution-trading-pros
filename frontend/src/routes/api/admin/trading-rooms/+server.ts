/**
 * Trading Rooms API Endpoint
 * ICT 7 Principal Engineer Grade
 *
 * Proxies to backend for trading room management.
 *
 * R21-A: migrated to shared `fetchBackendWithStatus` helper. POST body
 * narrowed with `isObject`. Backend-error-message extraction consolidated
 * (was: generic "Failed to create trading room" regardless of backend
 * validation message — now forwards backend's `.message` / `.error`).
 *
 * @version 2.1.0 — 2026-05-20
 */

import { json, error, isHttpError } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { requireAdmin } from '$lib/server/auth';
import {
	fetchBackendWithStatus,
	isObject,
	extractBackendErrorMessage
} from '$lib/server/proxy-fetch';

// GET - List trading rooms
export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { url } = event;
	const authHeader = `Bearer ${token}`;
	const queryParams = url.searchParams.toString();
	const endpoint = `/api/trading-rooms${queryParams ? `?${queryParams}` : ''}`;

	const { data, status } = await fetchBackendWithStatus(
		endpoint,
		{ headers: { Authorization: authHeader } },
		'[Trading Rooms API]'
	);

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
		const body: unknown = await request.json();

		if (!isObject(body)) {
			error(400, 'Invalid request body');
		}

		const { data, status } = await fetchBackendWithStatus(
			'/api/admin/trading-rooms',
			{
				method: 'POST',
				headers: { Authorization: authHeader },
				body: JSON.stringify(body)
			},
			'[Trading Rooms API]'
		);

		if (status >= 400) {
			error(status, extractBackendErrorMessage(data, 'Failed to create trading room'));
		}

		return json(data);
	} catch (err) {
		if (isHttpError(err)) throw err;
		console.error('POST /api/admin/trading-rooms error:', err);
		error(400, 'Invalid request body');
	}
};

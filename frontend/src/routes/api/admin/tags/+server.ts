/**
 * Tags API Endpoint
 * ICT 7 Principal Engineer Grade
 *
 * Proxies to backend for blog post tags.
 *
 * @version 2.0.0 - January 2026
 */

import { json, error, isHttpError } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
import { requireAdmin } from '$lib/server/auth';
const API_URL = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);

	try {
		const response = await fetch(`${API_URL}/api/admin/tags`, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});

		if (response.ok) {
			const data = await response.json();
			return json({ success: true, data: data.data || data });
		}

		console.warn(`Backend tags returned ${response.status}`);
	} catch (err) {
		console.warn('Backend tags not available:', err);
	}

	return json({
		success: false,
		data: [],
		error: 'Failed to fetch tags from backend'
	});
};

// FIX-2026-04-26 (cross-cutting audit §I.3 method-coverage gap): admin/blog/{create,edit}
// pages POST new tags via api.post('/api/admin/tags', { name }). Without this handler,
// the SK router returns 405 because the bare-path proxy only declared GET.
export const POST: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { request } = event;

	try {
		const body = await request.json();

		const upstream = await fetch(`${API_URL}/api/admin/tags`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			},
			body: JSON.stringify(body)
		});

		const text = await upstream.text();
		let data: unknown = null;
		if (text) {
			try {
				data = JSON.parse(text);
			} catch {
				data = { error: text };
			}
		}

		if (!upstream.ok) {
			const message =
				(data && typeof data === 'object' && ('message' in data || 'error' in data)
					? (data as { message?: string; error?: string }).message ||
						(data as { error?: string }).error
					: undefined) || 'Failed to create tag';
			error(upstream.status, message);
		}

		return json(data ?? { success: true });
	} catch (err) {
		if (isHttpError(err)) throw err;
		console.error('POST /api/admin/tags error:', err);
		error(400, 'Invalid request body');
	}
};

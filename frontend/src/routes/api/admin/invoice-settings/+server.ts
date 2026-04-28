/**
 * Admin Invoice Settings API Proxy
 *
 * FIX-2026-04-26 (audit 02 §P1-5): The Invoice Settings page targets
 * `/api/admin/invoice-settings` for load/save/preview/reset and
 * `/api/admin/invoice-settings/logo` for upload/remove. The audit confirmed
 * NO backend route exists yet (`grep -nE "invoice.settings|invoice_settings"`
 * against `api/src/routes/*.rs` returned nothing).
 *
 * Per task instructions we build the proxy now (so the page no longer hangs
 * on a SvelteKit 405). Until the backend ships, every call will round-trip
 * to the backend, get a 404, and surface as a structured error to the page —
 * never as a fake success.
 *
 * Backend work tracked in `02-members-subscriptions-DEFERRED.md` §D2.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdminToken } from '$lib/server/auth';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

async function relay(response: Response): Promise<Response> {
	const text = await response.text();
	if (!text) {
		return json(
			{ error: 'Empty response from backend' },
			{ status: response.status >= 400 ? response.status : 502 }
		);
	}
	try {
		const data = JSON.parse(text);
		return json(data, { status: response.status });
	} catch {
		return json({ error: 'Invalid JSON from backend' }, { status: 502 });
	}
}

export const GET: RequestHandler = async (event) => {
	const token = requireAdminToken(event);
	try {
		const response = await fetch(`${API_URL}/api/admin/invoice-settings`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] invoice-settings GET error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

export const PUT: RequestHandler = async (event) => {
	const token = requireAdminToken(event);
	try {
		const body = await event.request.json();
		const response = await fetch(`${API_URL}/api/admin/invoice-settings`, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			},
			body: JSON.stringify(body)
		});
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] invoice-settings PUT error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

export const POST: RequestHandler = async (event) => {
	// POST is used by `loadPreview` / `downloadPreview` / `resetToDefaults` —
	// we forward verbatim and let the (eventual) backend disambiguate via
	// query string or sub-path on its end.
	const token = requireAdminToken(event);
	try {
		const body = await event.request.json();
		const qs = event.url.search ?? '';
		const response = await fetch(`${API_URL}/api/admin/invoice-settings${qs}`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			},
			body: JSON.stringify(body)
		});
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] invoice-settings POST error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

/** Public popup lookup proxy. Preserve backend errors so outages are observable. */

import type { RequestEvent } from '@sveltejs/kit';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';

const API_URL = API_BASE_URL || BACKEND_URL || 'http://127.0.0.1:8080';

export const GET = async ({ url, fetch }: RequestEvent) => {
	try {
		const response = await fetch(`${API_URL}/api/popups/active${url.search}`, {
			headers: { Accept: 'application/json' }
		});
		const data = await response.json();
		return Response.json(data, { status: response.status });
	} catch {
		return Response.json({ error: 'Popup service unavailable' }, { status: 503 });
	}
};

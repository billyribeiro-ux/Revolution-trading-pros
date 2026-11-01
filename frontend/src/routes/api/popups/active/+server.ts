/**
 * Active Popups API Proxy
 *
 * R22-A: Deleted the `return json({ popups: [] })` mock-immediate path.
 *   This proxy used to never call the backend at all — it just returned an
 *   empty popup list to suppress 404 console noise. That hid the fact that
 *   the backend endpoint is unimplemented from anyone running the app:
 *   product wouldn't know popups don't work, ops wouldn't see the gap in
 *   metrics, and a future "real" backend implementation would have no
 *   adoption signal. Now: 501 Not Implemented so the missing endpoint is
 *   visible. The caller (`src/lib/api/popups.ts`) already handles non-2xx
 *   silently (`if (!response.ok) return;`) so end users see no behaviour
 *   change — only the truth about the missing endpoint changes.
 *
 * TODO(R22-A): wire to a real backend endpoint when popups are implemented.
 */

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
		return Response.json({ popups: [] }, { status: 200 });
	}
};

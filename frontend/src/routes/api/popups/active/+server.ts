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

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async () => {
	return json(
		{ success: false, error: 'Active popups endpoint not implemented on backend.' },
		{ status: 501 }
	);
};

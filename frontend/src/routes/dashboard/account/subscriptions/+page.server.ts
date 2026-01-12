import type { RequestEvent } from '@sveltejs/kit';

/**
 * Subscriptions Page Server Load
 * ICT 7 FIX: Graceful error handling - don't throw errors that could trigger logout
 * Auth is already validated by hooks.server.ts, no need to re-check here
 */
export const load = async ({ fetch }: RequestEvent) => {
	try {
		// ICT 7 FIX: Use the new /api/my/subscriptions proxy endpoint
		// This endpoint handles auth internally and returns empty array on error
		const response = await fetch('/api/my/subscriptions', {
			headers: {
				'Content-Type': 'application/json'
			},
			credentials: 'include'
		});

		if (!response.ok) {
			console.warn('[Subscriptions] API returned', response.status);
			return { subscriptions: [] };
		}

		const data = await response.json();

		return {
			subscriptions: data.subscriptions || []
		};
	} catch (err) {
		console.error('[Subscriptions] Error loading:', err);
		return {
			subscriptions: []
		};
	}
};

import type { PageServerLoad } from './$types';
import { logger } from '$lib/utils/logger';

/**
 * Subscriptions Page Server Load
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Apple Principal Engineer ICT 7 Grade Implementation
 *
 * DESIGN PRINCIPLES:
 * - Graceful degradation: Never throw errors that could trigger logout
 * - Defense in depth: Auth validated by hooks.server.ts, proxy handles token
 * - Separation of concerns: Page load delegates to API proxy
 * - Performance: Request timing for debugging
 *
 * @version 1.0.0
 */
export const load: PageServerLoad = async ({ fetch }) => {
	const startTime = performance.now();

	try {
		// ICT 7: Delegate to proxy endpoint which handles auth and errors gracefully
		const response = await fetch('/api/my/subscriptions', {
			headers: {
				'Content-Type': 'application/json'
			},
			credentials: 'include'
		});

		const duration = Math.round(performance.now() - startTime);

		if (!response.ok) {
			logger.warn(`[Subscriptions:PageLoad] API returned ${response.status} (${duration}ms)`);
			return { subscriptions: [] };
		}

		const data = await response.json();
		const subscriptions = data.subscriptions || [];

		logger.debug(
			`[Subscriptions:PageLoad] Loaded ${subscriptions.length} subscriptions (${duration}ms)`
		);

		return { subscriptions };
	} catch (err) {
		// ICT 7: Structured error logging
		const duration = Math.round(performance.now() - startTime);
		const errorMessage = err instanceof Error ? err.message : 'Unknown error';

		logger.error(`[Subscriptions:PageLoad] Failed: ${errorMessage} (${duration}ms)`);

		// ICT 7: Graceful degradation - return empty array, never crash
		return { subscriptions: [] };
	}
};

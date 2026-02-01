/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Dev Routes Layout Server - Production Guard
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Prevents access to dev routes in production
 * @version 1.0.0 - January 2026
 * @standards Apple Principal Engineer ICT Level 7+
 *
 * This layout guard ensures all routes under (dev)/ return 404 in production.
 * Only accessible when NODE_ENV === 'development'.
 */

import { error } from '@sveltejs/kit';
import { dev } from '$app/environment';

export const load = async () => {
	// Block access in production - return 404 as if route doesn't exist
	if (!dev) {
		error(404, 'Not found');
	}

	return {
		isDev: true,
		timestamp: Date.now()
	};
};

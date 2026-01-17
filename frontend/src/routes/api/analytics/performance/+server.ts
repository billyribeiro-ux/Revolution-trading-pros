import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

/**
 * Performance Analytics Proxy - Apple ICT 7 Principal Engineer
 * ══════════════════════════════════════════════════════════════════════════════
 * 
 * CRITICAL: This endpoint MUST always return 200/204 to prevent console spam.
 * Performance metrics are fire-and-forget - we don't care if they fail.
 * 
 * Design principle: Analytics should NEVER break the user experience.
 * ══════════════════════════════════════════════════════════════════════════════
 */

export const POST: RequestHandler = async () => {
	// Always return success immediately - analytics is fire-and-forget
	// Backend will receive metrics when it's ready, but we never block or error
	return json({ ok: true }, { status: 204 });
};

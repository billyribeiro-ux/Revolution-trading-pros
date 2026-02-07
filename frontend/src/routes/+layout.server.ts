/**
 * Root Layout Server Load Function
 *
 * Provides server-side data to the layout, including:
 * - Initial consent state from cookie
 * - User session data (if authenticated)
 *
 * @see https://kit.svelte.dev/docs/load
 * @module routes/+layout.server
 * @version 1.1.0
 */

import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals }) => {
	// Return consent data for hydration on client
	return {
		// Initial consent state from cookie (read in hooks.server.ts)
		initialConsent: (locals as { consent?: unknown }).consent,
		hasConsentInteraction: (locals as { hasConsentInteraction?: boolean }).hasConsentInteraction
	};
};

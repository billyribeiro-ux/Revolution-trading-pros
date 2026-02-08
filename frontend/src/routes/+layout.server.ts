/**
 * Root Layout Server Load Function
 *
 * Provides server-side data to the layout, including:
 * - Initial consent state from cookie
 * - User session data (if authenticated)
 * - SEO defaults and route context for the unified SEO layer
 *
 * @see https://kit.svelte.dev/docs/load
 * @module routes/+layout.server
 * @version 2.0.0
 */

import type { LayoutServerLoad } from './$types';
import { seoDefaults } from '$lib/seo/defaults';
import { createSEOContext } from '$lib/seo/resolve';
import type { Environment } from '$lib/seo/types';

function detectEnv(): Environment {
	const mode = import.meta.env.MODE;
	if (mode === 'production') return 'production';
	if (mode === 'staging') return 'staging';
	return 'development';
}

export const load: LayoutServerLoad = async ({ locals, url }) => {
	const env = detectEnv();

	const seoContext = createSEOContext({
		pathname: url.pathname,
		env,
		isPrivate: seoDefaults.privatePathPrefixes.some((p) => url.pathname.startsWith(p)),
		isSearchPage: seoDefaults.searchPathPrefixes.some((p) => url.pathname.startsWith(p)),
		isErrorPage: false
	});

	return {
		// Initial consent state from cookie (read in hooks.server.ts)
		initialConsent: (locals as { consent?: unknown }).consent,
		hasConsentInteraction: (locals as { hasConsentInteraction?: boolean }).hasConsentInteraction,
		// SEO layer data
		seoContext,
		seoDefaults
	};
};

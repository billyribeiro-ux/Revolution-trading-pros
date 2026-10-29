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
import { building } from '$app/env';
import { seoDefaults } from '#lib/seo/defaults.js';
import { createSEOContext } from '#lib/seo/resolve.js';
import type { Environment } from '#lib/seo/types.js';

function detectEnv(): Environment {
	const mode = import.meta.env.MODE;
	if (mode === 'production') return 'production';
	if (mode === 'staging') return 'staging';
	return 'development';
}

/**
 * Kit throws on `url.search` for prerendered routes (build and dev alike);
 * those routes can't carry a query, so treat that one error as "no search".
 */
function readSearch(url: URL): string {
	if (building) return '';
	try {
		return url.search;
	} catch (err) {
		if (String(err).includes('url_search_unavailable_prerender')) return '';
		throw err;
	}
}

export const load: LayoutServerLoad = async ({ locals, url }) => {
	const env = detectEnv();
	const search = readSearch(url);
	const fullPath = url.pathname + search;

	const seoContext = createSEOContext({
		pathname: url.pathname,
		env,
		isPrivate: seoDefaults.privatePathPrefixes.some((p) => url.pathname.startsWith(p)),
		isSearchPage:
			seoDefaults.searchPathPrefixes.some((p) => fullPath.startsWith(p)) ||
			new URLSearchParams(search).has('q') ||
			new URLSearchParams(search).has('search'),
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

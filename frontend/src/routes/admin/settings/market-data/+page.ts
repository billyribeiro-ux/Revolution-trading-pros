/**
 * Admin Market Data Configuration - Page Metadata
 *
 * Disable SSR since this page uses browser-only APIs
 * for testing provider connections and managing settings.
 */

export const prerender = false;
export const ssr = false;

export function load() {
	return {
		title: 'Market Data Configuration'
	};
}

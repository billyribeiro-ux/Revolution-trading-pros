/**
 * Account Page - Server Load with SEO (Private/NoIndex)
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Example: Private page that should never be indexed.
 * The SEO layer automatically detects /account as a private path prefix,
 * but we also explicitly set robots for defense-in-depth.
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 * @since February 2026
 */

import type { SEOInput } from '$lib/seo/types';
import type { ServerLoad } from '@sveltejs/kit';

export const load: ServerLoad = async () => {
	const seo: SEOInput = {
		title: 'My Account',
		description: 'Manage your Revolution Trading Pros account settings and subscriptions.',
		robots: {
			index: false,
			follow: false
		}
	};

	return {
		seo
	};
};

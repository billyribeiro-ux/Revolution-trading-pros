/**
 * Cookie Policy Page Server Configuration
 *
 * Disable prerendering since this page uses browser-only APIs
 * for scanning cookies and displaying consent information.
 */

import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildWebPage } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = false;
export const ssr = false;

export const load = () => ({
	seo: {
		title: 'Cookie Policy',
		description:
			'How Revolution Trading Pros uses cookies and similar technologies, what data is collected, and how to manage your consent.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Cookie Policy', url: `${SITE}/cookie-policy` }
			]),
			buildWebPage({
				url: `${SITE}/cookie-policy`,
				name: 'Cookie Policy',
				description:
					'Cookie usage policy for Revolution Trading Pros — what cookies we use, why, and how to manage them.',
				dateModified: '2026-05-25'
			})
		]
	} satisfies SEOInput
});

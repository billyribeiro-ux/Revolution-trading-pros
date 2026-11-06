import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildWebPage } from '#lib/seo/schemas.js';

import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Terms of Service',
		description:
			'The Terms of Service are awaiting publication. Contact Revolution Trading Pros support with questions or browse our other policies.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Terms of Service', url: `${SITE}/terms` }
			]),
			buildWebPage({
				url: `${SITE}/terms`,
				name: 'Terms of Service',
				description: 'Terms of service for Revolution Trading Pros.',
				dateModified: '2026-05-25'
			})
		]
	} satisfies SEOInput
});

import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildWebPage } from '#lib/seo/schemas.js';

import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Privacy Policy',
		description:
			'The Privacy Policy is awaiting publication. Contact Revolution Trading Pros support with privacy questions or browse our other policies.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Privacy Policy', url: `${SITE}/privacy` }
			]),
			buildWebPage({
				url: `${SITE}/privacy`,
				name: 'Privacy Policy',
				description: 'Privacy policy for Revolution Trading Pros.',
				dateModified: '2026-05-25'
			})
		]
	} satisfies SEOInput
});

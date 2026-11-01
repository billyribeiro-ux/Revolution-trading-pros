import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildWebPage } from '#lib/seo/schemas.js';

import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Careers — Build the Future of Trading Education',
		description:
			"Join the Revolution Trading Pros team. We're building the best institutional-grade trading education platform in the world — come help us do it.",
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Careers', url: `${SITE}/careers` }
			]),
			buildWebPage({
				url: `${SITE}/careers`,
				name: 'Careers at Revolution Trading Pros',
				description:
					'Explore open roles in trading education, engineering, community growth, and content production at Revolution Trading Pros.',
				dateModified: '2026-10-28'
			})
		]
	} satisfies SEOInput
});

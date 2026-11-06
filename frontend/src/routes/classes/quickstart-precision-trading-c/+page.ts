import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildCourse } from '#lib/seo/schemas.js';

import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Quickstart To Precision Trading',
		description:
			'Master precision entry and exit timing, chart setups, and institutional execution techniques in the Quickstart To Precision Trading class.',
		og: {
			type: 'article'
		},
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Classes', url: `${SITE}/classes` },
				{
					name: 'Quickstart To Precision Trading',
					url: `${SITE}/classes/quickstart-precision-trading-c`
				}
			]),
			buildCourse({
				url: `${SITE}/classes/quickstart-precision-trading-c`,
				name: 'Quickstart To Precision Trading',
				description:
					'Master precision entry and exit timing, chart setups, and institutional execution techniques.',
				provider: { name: 'Revolution Trading Pros', url: SITE },
				hasCourseInstance: {
					courseMode: 'Online'
				}
			})
		]
	} satisfies SEOInput
});

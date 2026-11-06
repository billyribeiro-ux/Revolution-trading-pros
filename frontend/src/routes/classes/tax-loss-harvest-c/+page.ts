import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildCourse } from '#lib/seo/schemas.js';

import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Tax Loss Harvest Strategy Class',
		description:
			'Learn institutional tax-loss harvesting strategies for active traders and investors to optimize portfolio tax efficiency at year-end.',
		og: {
			type: 'article'
		},
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Classes', url: `${SITE}/classes` },
				{ name: 'Tax Loss Harvest', url: `${SITE}/classes/tax-loss-harvest-c` }
			]),
			buildCourse({
				url: `${SITE}/classes/tax-loss-harvest-c`,
				name: 'Tax Loss Harvest Strategy Class',
				description:
					'Institutional tax-loss harvesting strategies for active traders and options investors.',
				provider: { name: 'Revolution Trading Pros', url: SITE },
				hasCourseInstance: {
					courseMode: 'Online'
				}
			})
		]
	} satisfies SEOInput
});

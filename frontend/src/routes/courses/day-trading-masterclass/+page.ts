import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildCourse } from '#lib/seo/schemas.js';

import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Day Trading Masterclass',
		description:
			'Master day trading with institutional-grade strategies, real-time execution tactics, and professional risk management.',
		og: { type: 'website' },
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Courses', url: `${SITE}/courses` },
				{ name: 'Day Trading Masterclass', url: `${SITE}/courses/day-trading-masterclass` }
			]),
			buildCourse({
				url: `${SITE}/courses/day-trading-masterclass`,
				name: 'Day Trading Masterclass',
				description:
					'Master day trading with institutional-grade strategies, real-time execution tactics, and professional risk management.',
				provider: { name: 'Revolution Trading Pros', url: SITE },
				educationalLevel: 'Advanced',
				offers: { price: 497, priceCurrency: 'USD', availability: 'InStock' },
				hasCourseInstance: { courseMode: 'Online' }
			})
		]
	} satisfies SEOInput
});

import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildCourse } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Swing Trading Pro',
		description:
			'Master swing trading with professional strategies to capture multi-day moves and optimal risk-reward setups.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Courses', url: `${SITE}/courses` },
				{ name: 'Swing Trading Pro', url: `${SITE}/courses/swing-trading-pro` }
			]),
			buildCourse({
				url: `${SITE}/courses/swing-trading-pro`,
				name: 'Swing Trading Pro',
				description:
					'Master swing trading with professional strategies to capture multi-day moves and optimal risk-reward setups.',
				provider: { name: 'Revolution Trading Pros', url: SITE },
				educationalLevel: 'Beginner',
				offers: { price: 397, priceCurrency: 'USD', availability: 'InStock' },
				hasCourseInstance: { courseMode: 'Online' }
			})
		]
	} satisfies SEOInput
});

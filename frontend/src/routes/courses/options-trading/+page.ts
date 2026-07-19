import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildCourse } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Options Trading Fundamentals',
		description:
			'Master options trading with comprehensive training on strategies, Greeks, volatility analysis, and institutional-grade risk management.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Courses', url: `${SITE}/courses` },
				{ name: 'Options Trading', url: `${SITE}/courses/options-trading` }
			]),
			buildCourse({
				url: `${SITE}/courses/options-trading`,
				name: 'Options Trading Fundamentals',
				description:
					'Master options trading with comprehensive training on strategies, Greeks, volatility analysis, and institutional-grade risk management.',
				provider: { name: 'Revolution Trading Pros', url: SITE },
				educationalLevel: 'Intermediate',
				offers: { price: 597, priceCurrency: 'USD', availability: 'InStock' },
				hasCourseInstance: { courseMode: 'Online' }
			})
		]
	} satisfies SEOInput
});

import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildCourse } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Risk Management Mastery',
		description:
			'Learn professional risk management techniques used by institutional traders to preserve capital and maximize long-term profitability.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Courses', url: `${SITE}/courses` },
				{ name: 'Risk Management Mastery', url: `${SITE}/courses/risk-management` }
			]),
			buildCourse({
				url: `${SITE}/courses/risk-management`,
				name: 'Risk Management Mastery',
				description:
					'Learn professional risk management techniques used by institutional traders to preserve capital and maximize long-term profitability.',
				provider: { name: 'Revolution Trading Pros', url: SITE },
				educationalLevel: 'Beginner',
				offers: { price: 297, priceCurrency: 'USD', availability: 'InStock' },
				hasCourseInstance: { courseMode: 'Online' }
			})
		]
	} satisfies SEOInput
});

import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildCourse, buildItemList } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

const courseSummary = [
	{
		slug: 'day-trading-masterclass',
		name: 'Day Trading Masterclass',
		description:
			'Decode institutional order flow, Level 2 data, and volume analysis to execute sniper-like entries.',
		level: 'Advanced',
		price: 497
	},
	{
		slug: 'swing-trading-pro',
		name: 'Swing Trading Pro',
		description: 'Capture major market moves. Identify macro trends and execute low-stress setups.',
		level: 'Beginner',
		price: 397
	},
	{
		slug: 'options-trading',
		name: 'Options Tactics',
		description:
			'Defined-risk spreads that profit even if the market goes nowhere. Handle leverage safely.',
		level: 'Intermediate',
		price: 597
	},
	{
		slug: 'risk-management',
		name: 'Risk Protocol',
		description:
			'The mathematical framework used by proprietary desks to ensure you never blow up.',
		level: 'Beginner',
		price: 297
	}
] as const;

export const load = () => ({
	seo: {
		title: 'Trading Courses & Mentorship',
		description:
			'Institutional-grade trading education. Learn to read order flow, manage risk, and execute with precision. Join the top 1% of disciplined traders.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Courses', url: `${SITE}/courses` }
			]),
			buildItemList(
				'Trading Courses',
				courseSummary.map((c) => ({
					name: c.name,
					url: `${SITE}/courses/${c.slug}`,
					description: c.description
				}))
			),
			...courseSummary.map((c) =>
				buildCourse({
					url: `${SITE}/courses/${c.slug}`,
					name: c.name,
					description: c.description,
					provider: { name: 'Revolution Trading Pros', url: SITE },
					educationalLevel: c.level as 'Beginner' | 'Intermediate' | 'Advanced',
					offers: { price: c.price, priceCurrency: 'USD', availability: 'InStock' },
					hasCourseInstance: { courseMode: 'Online' }
				})
			)
		]
	} satisfies SEOInput
});

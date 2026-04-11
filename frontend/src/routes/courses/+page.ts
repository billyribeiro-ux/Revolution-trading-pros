import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';
import { breadcrumbSchema } from '$lib/seo/jsonld';

const SITE_URL = 'https://revolution-trading-pros.pages.dev';

// Static catalog of courses offered (keep in sync with /courses/* subpages).
// Used both for the visible listing and the ItemList structured data.
const courses = [
	{
		slug: 'day-trading-masterclass',
		name: 'Day Trading Masterclass',
		description:
			'Order flow, opening range breakouts, and risk-first execution for active day traders.'
	},
	{
		slug: 'swing-trading-pro',
		name: 'Swing Trading Pro',
		description:
			'Multi-day positioning, sector rotation, and weekend prep routines for busy professionals.'
	},
	{
		slug: 'options-trading',
		name: 'Options Trading',
		description:
			'0DTE strategies, volatility positioning, and defined-risk spreads for income and growth.'
	},
	{
		slug: 'risk-management',
		name: 'Risk Management Fundamentals',
		description:
			'Position sizing, drawdown control, and the psychology of consistent profitability.'
	}
];

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Trading Courses & Mentorship | Revolution Trading Pros',
		description:
			'Institutional-grade trading education. Learn to read order flow, manage risk, and execute with precision. Join the top 1% of disciplined traders.',
		canonical: `${SITE_URL}/courses`,
		og: { type: 'website' },
		jsonld: [
			{
				'@context': 'https://schema.org',
				'@type': 'ItemList',
				'@id': `${SITE_URL}/courses#itemlist`,
				name: 'Revolution Trading Pros — Course Catalog',
				itemListOrder: 'https://schema.org/ItemListUnordered',
				numberOfItems: courses.length,
				itemListElement: courses.map((c, i) => ({
					'@type': 'ListItem',
					position: i + 1,
					item: {
						'@type': 'Course',
						name: c.name,
						description: c.description,
						url: `${SITE_URL}/courses/${c.slug}`,
						provider: {
							'@type': 'Organization',
							'@id': `${SITE_URL}/#organization`,
							name: 'Revolution Trading Pros',
							sameAs: SITE_URL
						}
					}
				}))
			},
			breadcrumbSchema(
				[
					{ name: 'Home', url: SITE_URL },
					{ name: 'Courses', url: `${SITE_URL}/courses` }
				],
				`${SITE_URL}/courses#breadcrumb`
			)
		]
	};
	return { seo, courses };
};

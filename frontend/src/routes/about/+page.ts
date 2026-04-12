import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';
import { breadcrumbSchema, organizationSchema } from '$lib/seo/jsonld';

const SITE_URL = 'https://revolution-trading-pros.pages.dev';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'About Revolution Trading Pros | Supportive Live Trading Community',
		description:
			'Join a professional trading floor that genuinely cares. Real trades, real-time voice guidance, and institutional data without the hype. Established 2018.',
		canonical: `${SITE_URL}/about`,
		og: { type: 'website' },
		jsonld: [
			{
				'@context': 'https://schema.org',
				'@type': 'AboutPage',
				'@id': `${SITE_URL}/about#aboutpage`,
				url: `${SITE_URL}/about`,
				name: 'About Revolution Trading Pros',
				description:
					'Professional trading education and live trading rooms founded in 2018 for 18,000+ active traders.',
				inLanguage: 'en-US',
				mainEntity: {
					'@type': 'Organization',
					'@id': `${SITE_URL}/#organization`
				}
			},
			organizationSchema({
				name: 'Revolution Trading Pros',
				url: SITE_URL,
				id: `${SITE_URL}/#organization`,
				logo: `${SITE_URL}/logo.png`,
				description:
					'Institutional-grade trading education, live trading rooms, and professional tools for 18,000+ active traders.',
				sameAs: [
					'https://twitter.com/RevTradingPros',
					'https://facebook.com/RevTradingPros',
					'https://youtube.com/RevTradingPros'
				]
			}),
			breadcrumbSchema(
				[
					{ name: 'Home', url: SITE_URL },
					{ name: 'About', url: `${SITE_URL}/about` }
				],
				`${SITE_URL}/about#breadcrumb`
			)
		]
	};
	return { seo };
};

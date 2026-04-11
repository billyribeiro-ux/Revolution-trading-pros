import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';
import { breadcrumbSchema } from '$lib/seo/jsonld';

const SITE_URL = 'https://revolution-trading-pros.pages.dev';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Our Mission | The Institutional Bridge',
		description:
			'Dismantling the retail trader stereotype. We bridge the gap between gambling and institutional risk management through data, discipline, and transparency.',
		canonical: `${SITE_URL}/our-mission`,
		og: { type: 'website' },
		jsonld: [
			{
				'@context': 'https://schema.org',
				'@type': 'WebPage',
				'@id': `${SITE_URL}/our-mission#webpage`,
				url: `${SITE_URL}/our-mission`,
				name: 'Our Mission | Revolution Trading Pros',
				description:
					'Bridging retail and institutional trading with data, discipline, and transparency.',
				inLanguage: 'en-US',
				isPartOf: { '@type': 'WebSite', '@id': `${SITE_URL}/#website` },
				about: { '@type': 'Organization', '@id': `${SITE_URL}/#organization` }
			},
			breadcrumbSchema(
				[
					{ name: 'Home', url: SITE_URL },
					{ name: 'Our Mission', url: `${SITE_URL}/our-mission` }
				],
				`${SITE_URL}/our-mission#breadcrumb`
			)
		]
	};
	return { seo };
};

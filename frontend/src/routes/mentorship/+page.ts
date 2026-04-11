// +page.ts
import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';
import { breadcrumbSchema } from '$lib/seo/jsonld';

export const prerender = true;

const SITE_URL = 'https://revolution-trading-pros.pages.dev';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: '1-on-1 Trading Mentorship | Revolution Trading Pros',
		description:
			'Direct, private mentorship with senior traders. Personalized trade reviews, custom strategy design, and accountability to turn small accounts into disciplined ones.',
		canonical: `${SITE_URL}/mentorship`,
		og: { type: 'website' },
		jsonld: [
			{
				'@context': 'https://schema.org',
				'@type': 'Service',
				'@id': `${SITE_URL}/mentorship#service`,
				name: '1-on-1 Trading Mentorship',
				serviceType: 'Trading Mentorship',
				description: 'Private, personalized trading mentorship with senior institutional traders.',
				url: `${SITE_URL}/mentorship`,
				provider: {
					'@type': 'Organization',
					'@id': `${SITE_URL}/#organization`,
					name: 'Revolution Trading Pros'
				},
				areaServed: { '@type': 'Country', name: 'United States' },
				offers: {
					'@type': 'Offer',
					category: 'Paid',
					priceCurrency: 'USD',
					availability: 'https://schema.org/LimitedAvailability'
				}
			},
			{
				'@context': 'https://schema.org',
				'@type': 'Course',
				'@id': `${SITE_URL}/mentorship#course`,
				name: 'Revolution Trading Pros — Mentorship Course',
				description:
					'Structured 1-on-1 coaching covering risk management, strategy selection, trade journaling, and live-trade review.',
				provider: {
					'@type': 'Organization',
					'@id': `${SITE_URL}/#organization`,
					name: 'Revolution Trading Pros',
					sameAs: SITE_URL
				},
				hasCourseInstance: {
					'@type': 'CourseInstance',
					courseMode: 'Online',
					inLanguage: 'en-US'
				}
			},
			breadcrumbSchema(
				[
					{ name: 'Home', url: SITE_URL },
					{ name: 'Mentorship', url: `${SITE_URL}/mentorship` }
				],
				`${SITE_URL}/mentorship#breadcrumb`
			)
		]
	};
	return { seo };
};

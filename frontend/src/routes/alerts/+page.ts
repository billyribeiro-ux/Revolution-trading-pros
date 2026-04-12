import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';
import { breadcrumbSchema, faqSchema } from '$lib/seo/jsonld';

const SITE_URL = 'https://revolution-trading-pros.pages.dev';
const PAGE_URL = `${SITE_URL}/alerts`;

const faqs = [
	{
		question: 'How fast are Revolution Trading Pros alerts delivered?',
		answer:
			'Alerts are delivered sub-second via SMS, push notification, email, and Discord so you can act on the same setup our analysts are taking.'
	},
	{
		question: 'Do the alerts include entry, stop, and target?',
		answer:
			'Yes. Every alert includes a precise entry, a protective stop, and one or more profit targets. Risk parameters are always defined before the trade is called.'
	},
	{
		question: 'Are these alerts suitable for beginners?',
		answer:
			'The alerts are written so new traders can follow along, but all members are expected to manage their own positions and understand the risk of each trade.'
	}
];

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Trading Alert Services | Revolution Trading Pros',
		description:
			'Premium trading alerts delivered to your phone. Swing trading and day trading alerts with precise entries, exits, and risk management from institutional traders.',
		canonical: PAGE_URL,
		og: { type: 'website' },
		jsonld: [
			{
				'@context': 'https://schema.org',
				'@type': 'Service',
				'@id': `${PAGE_URL}#service`,
				name: 'Revolution Trading Pros Alert Services',
				serviceType: 'Trading Alert Service',
				description:
					'Real-time trading alerts with entries, stops, and targets delivered by institutional traders.',
				url: PAGE_URL,
				provider: {
					'@type': 'Organization',
					'@id': `${SITE_URL}/#organization`,
					name: 'Revolution Trading Pros'
				},
				areaServed: { '@type': 'Country', name: 'United States' }
			},
			faqSchema(faqs, `${PAGE_URL}#faq`),
			breadcrumbSchema(
				[
					{ name: 'Home', url: SITE_URL },
					{ name: 'Alerts', url: PAGE_URL }
				],
				`${PAGE_URL}#breadcrumb`
			)
		]
	};
	return { seo };
};

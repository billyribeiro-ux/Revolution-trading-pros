// +page.ts
import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';
import { breadcrumbSchema, faqSchema } from '$lib/seo/jsonld';

export const prerender = true;

const SITE_URL = 'https://revolution-trading-pros.pages.dev';
const PAGE_URL = `${SITE_URL}/live-trading-rooms/small-accounts`;

const faqs = [
	{
		question: 'What is the Small Accounts trading room?',
		answer:
			'The Growth Accelerator is built for traders with accounts under $25K. It focuses on disciplined position sizing, PDT-rule navigation, and strategies engineered for compounding smaller capital bases.'
	},
	{
		question: 'Is the Small Accounts room suitable for beginners?',
		answer:
			'Yes. It is designed for disciplined beginners and intermediate traders who want to learn institutional risk management while growing their accounts responsibly.'
	},
	{
		question: 'Can I grow a small account into a full-time trading account here?',
		answer:
			'Yes — the program provides a roadmap to $25K and beyond through cash-account-friendly strategies and strict risk rules, with live mentorship Q&A throughout the week.'
	}
];

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Small Accounts Growth Room | Sub-$25K Trading Mentorship',
		description:
			'Growth Accelerator live room for traders with accounts under $25K. Disciplined position sizing, PDT-friendly strategies, and a roadmap to $25K and beyond.',
		canonical: PAGE_URL,
		og: {
			type: 'website',
			title: 'Small Accounts Growth Room | Revolution Trading Pros',
			description:
				'Sub-$25K trading mentorship with disciplined position sizing and PDT-friendly strategies.'
		},
		twitter: {
			title: 'Small Accounts Growth Room | Revolution Trading Pros',
			description:
				'Sub-$25K trading mentorship with disciplined position sizing and PDT-friendly strategies.'
		},
		jsonld: [
			{
				'@context': 'https://schema.org',
				'@type': 'Service',
				'@id': `${PAGE_URL}#service`,
				name: 'Small Accounts Growth Accelerator',
				serviceType: 'Small Account Trading Mentorship',
				description:
					'Live trading mentorship for traders with accounts under $25K focused on disciplined growth and risk management.',
				url: PAGE_URL,
				provider: {
					'@type': 'Organization',
					'@id': `${SITE_URL}/#organization`,
					name: 'Revolution Trading Pros'
				},
				areaServed: { '@type': 'Country', name: 'United States' },
				offers: {
					'@type': 'Offer',
					price: 147,
					priceCurrency: 'USD',
					category: 'Subscription',
					availability: 'https://schema.org/InStock',
					url: PAGE_URL
				}
			},
			faqSchema(faqs, `${PAGE_URL}#faq`),
			breadcrumbSchema(
				[
					{ name: 'Home', url: SITE_URL },
					{ name: 'Live Trading Rooms', url: `${SITE_URL}/live-trading-rooms` },
					{ name: 'Small Accounts', url: PAGE_URL }
				],
				`${PAGE_URL}#breadcrumb`
			)
		]
	};
	return { seo };
};

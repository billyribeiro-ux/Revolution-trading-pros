// +page.ts
import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';
import { breadcrumbSchema, faqSchema } from '$lib/seo/jsonld';

export const prerender = true;

const SITE_URL = 'https://revolution-trading-pros.pages.dev';
const PAGE_URL = `${SITE_URL}/live-trading-rooms/swing-trading`;

const faqs = [
	{
		question: 'What is the Swing Alpha Room?',
		answer:
			'Swing Alpha Room is a multi-day-hold trading room with Sunday-night watchlists, weekly deep-dive strategy sessions, and institutional-grade risk/reward position sizing.'
	},
	{
		question: 'How long are trades typically held in the swing trading room?',
		answer:
			'Typical hold times are 3-7 trading days, with occasional multi-week core positions when structure supports a longer timeframe.'
	},
	{
		question: 'Is swing trading suitable for people with full-time jobs?',
		answer:
			'Yes — swing trading is designed for traders who cannot watch screens all day. Sunday-night prep and end-of-day alerts keep you positioned without constant monitoring.'
	}
];

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Swing Alpha Room | Multi-Day Swing Trading Mentorship',
		description:
			'Swing Alpha Room: 3-7 day swing trades with Sunday-night watchlists and risk/reward position sizing. Perfect for traders with full-time careers.',
		canonical: PAGE_URL,
		og: {
			type: 'website',
			title: 'Swing Alpha Room | Revolution Trading Pros',
			description:
				'Swing trading room for multi-day holds with Sunday-night watchlists and institutional risk management.'
		},
		twitter: {
			title: 'Swing Alpha Room | Revolution Trading Pros',
			description:
				'Swing trading room for multi-day holds with Sunday-night watchlists and institutional risk management.'
		},
		jsonld: [
			{
				'@context': 'https://schema.org',
				'@type': 'Service',
				'@id': `${PAGE_URL}#service`,
				name: 'Swing Alpha Room',
				serviceType: 'Swing Trading Live Room',
				description:
					'Multi-day swing trading mentorship with weekly strategy sessions and Sunday-night watchlists.',
				url: PAGE_URL,
				provider: {
					'@type': 'Organization',
					'@id': `${SITE_URL}/#organization`,
					name: 'Revolution Trading Pros'
				},
				areaServed: { '@type': 'Country', name: 'United States' },
				offers: {
					'@type': 'Offer',
					price: 197,
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
					{ name: 'Swing Trading', url: PAGE_URL }
				],
				`${PAGE_URL}#breadcrumb`
			)
		]
	};
	return { seo };
};

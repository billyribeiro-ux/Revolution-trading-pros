// +page.ts
import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';
import { breadcrumbSchema, faqSchema } from '$lib/seo/jsonld';

export const prerender = true;

const SITE_URL = 'https://revolution-trading-pros.pages.dev';
const PAGE_URL = `${SITE_URL}/live-trading-rooms/day-trading`;

const faqs = [
	{
		question: 'Do I need to be an expert to join the day trading room?',
		answer:
			'No. While trading is fast-paced, we prioritize education with a "New Member" onboarding video series covering levels, terminology, and execution before you take your first trade.'
	},
	{
		question: 'What platform is the day trading room hosted on?',
		answer:
			'The live room is hosted on a private Discord server with low-latency voice channels and 1080p screen share, accessible from desktop or mobile.'
	},
	{
		question: 'What instruments does the day trading room focus on?',
		answer:
			'We specialize in SPX 0DTE options with occasional SPY, QQQ, and ES/NQ futures trades when high-probability setups present.'
	},
	{
		question: 'How much capital do I need to start day trading with Revolution Trading Pros?',
		answer:
			'We recommend at least $2,000 to trade comfortably with proper risk management. Many members start with less using cash accounts to avoid Pattern Day Trader rules.'
	}
];

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Day Trading Command Live Room | SPX 0DTE Mentorship',
		description:
			'Live day trading room for SPX 0DTE options with voice-guided commentary, pre-market prep, and risk-managed trade calls from institutional day traders.',
		canonical: PAGE_URL,
		og: {
			type: 'website',
			title: 'Day Trading Command Live Room | Revolution Trading Pros',
			description:
				'Live day trading room with voice-guided SPX 0DTE commentary and risk-managed trade calls.'
		},
		twitter: {
			title: 'Day Trading Command Live Room',
			description:
				'Live day trading room with voice-guided SPX 0DTE commentary and risk-managed trade calls.'
		},
		jsonld: [
			{
				'@context': 'https://schema.org',
				'@type': 'Service',
				'@id': `${PAGE_URL}#service`,
				name: 'Day Trading Command Live Room',
				serviceType: 'Day Trading Live Room',
				description:
					'Live day trading room for SPX 0DTE options with real-time voice-guided commentary and risk-managed trade calls.',
				url: PAGE_URL,
				provider: {
					'@type': 'Organization',
					'@id': `${SITE_URL}/#organization`,
					name: 'Revolution Trading Pros'
				},
				areaServed: { '@type': 'Country', name: 'United States' },
				offers: {
					'@type': 'Offer',
					price: 247,
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
					{ name: 'Day Trading', url: PAGE_URL }
				],
				`${PAGE_URL}#breadcrumb`
			)
		]
	};
	return { seo };
};

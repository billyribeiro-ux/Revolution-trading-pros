/**
 * Live Trading Rooms - Server Load Function
 *
 * Proper SSR/SSG with the unified SEO layer:
 * - Page-level SEO is returned as `seo: SEOInput`, picked up by +layout.svelte
 * - Structured data (Service + FAQPage + BreadcrumbList) emitted as JSON-LD
 * - Legacy `structuredData` / manual head tags removed (owned by <Seo>)
 *
 * @module routes/live-trading-rooms/+page.server
 * @version 2.0.0 — unified SEO layer integration
 */

import type { PageServerLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';
import { breadcrumbSchema, faqSchema } from '$lib/seo/jsonld';

const SITE_URL =
	import.meta.env.VITE_SITE_URL || 'https://revolution-trading-pros.pages.dev';

// Static data that can be generated at build time
const tradingRoomsData = [
	{
		id: 'day-trading',
		iconType: 'volatility',
		name: 'Day Trading Command',
		tagline: 'Trade the Open with Professionals',
		description:
			'High-velocity execution. Join expert traders for the opening bell, real-time scanners, and rapid-fire trade calls.',
		liveCount: 842,
		features: [
			'Daily Live Session (9:30 AM - 12:00 PM ET)',
			'Institutional Order Flow Analysis',
			'Screen Share & Voice Comms',
			'Gap Scanner & Pre-Market Prep',
			'Private Discord Community'
		],
		price: { monthly: 247, quarterly: 597, annual: 1897 },
		accent: 'cyan',
		badge: 'MOST POPULAR'
	},
	{
		id: 'swing-trading',
		iconType: 'trend',
		name: 'Swing Alpha Room',
		tagline: 'Catch Multi-Day Moves',
		description:
			'Strategic positioning for 3-7 day holds. Perfect for those who cannot watch the screen all day but want institutional returns.',
		liveCount: 1250,
		features: [
			'Weekly Deep-Dive Strategy Session',
			'Sunday Night Watchlist Blueprint',
			'Risk/Reward Position Sizing',
			'Private Analyst Chat',
			'Weekend Market Prep'
		],
		price: { monthly: 197, quarterly: 497, annual: 1497 },
		accent: 'emerald',
		badge: ''
	},
	{
		id: 'small-accounts',
		iconType: 'growth',
		name: 'Growth Accelerator',
		tagline: 'Small Account → Big Future',
		description:
			'The disciplined path to $25K and beyond. Learn risk management and compounding strategies specifically for smaller capital bases.',
		liveCount: 430,
		features: [
			'Under $25K Specific Strategies',
			'Strict Risk Management Rules',
			'Account Builder Roadmap',
			'Live Mentorship Q&A',
			'Community Support'
		],
		price: { monthly: 147, quarterly: 397, annual: 1197 },
		accent: 'amber',
		badge: 'BEGINNER FRIENDLY'
	}
];

const benefitsData = [
	{
		id: 'institutional-mentorship',
		iconType: 'analysis',
		title: 'Institutional Mentorship',
		desc: 'Learn from professionals with decades of combined experience at major firms and prop desks.'
	},
	{
		id: 'sub-second-alerts',
		iconType: 'radar',
		title: 'Sub-Second Alerts',
		desc: 'Get trade alerts as they happen via SMS, email, and Discord. Never miss a breakout.'
	},
	{
		id: 'continuous-education',
		iconType: 'strategy',
		title: 'Continuous Education',
		desc: 'Access a library of strategy breakdowns, market analysis, and psychological training.'
	},
	{
		id: 'elite-community',
		iconType: 'network',
		title: 'Elite Community',
		desc: 'Join thousands of focused traders who help each other succeed, share alpha, and grow.'
	}
];

const tickerSymbols = [
	{ sym: 'SPY', price: '478.22', change: '+0.45%', up: true },
	{ sym: 'QQQ', price: '408.12', change: '+0.82%', up: true },
	{ sym: 'IWM', price: '198.40', change: '-0.12%', up: false },
	{ sym: 'AMD', price: '138.00', change: '+1.05%', up: true },
	{ sym: 'BTC', price: '42,100', change: '+1.2%', up: true }
];

const PAGE_URL = `${SITE_URL}/live-trading-rooms`;

const faqs = [
	{
		question: 'What are Revolution Trading Pros live trading rooms?',
		answer:
			'Live trading rooms are interactive online sessions where professional traders share real-time market analysis, trade alerts, and voice-guided commentary with members throughout the market day.'
	},
	{
		question: 'Who leads the live trading rooms?',
		answer:
			'Our rooms are led by institutional traders with 15+ years of experience at major firms, prop desks, and hedge funds. You learn directly from the people calling the trades.'
	},
	{
		question: 'Which trading room is right for me?',
		answer:
			'Day Trading Command is built for active intraday traders, Swing Alpha Room suits multi-day holds for busy professionals, and Growth Accelerator is designed for traders working with accounts under $25K.'
	}
];

const pageSeo: SEOInput = {
	title: 'Live Trading Rooms | Professional Trading Mentorship',
	description:
		'Join expert traders in our live trading rooms. Day trading, swing trading, and small account strategies with real-time alerts from institutional professionals.',
	canonical: PAGE_URL,
	og: {
		type: 'website',
		title: 'Live Trading Rooms | Revolution Trading Pros',
		description:
			'Live trading rooms with real-time voice-guided commentary from institutional traders. Day trading, swing trading, and small-account mentorship.',
		image: `${SITE_URL}/og-default.png`,
		imageAlt: 'Revolution Trading Pros Live Trading Rooms'
	},
	twitter: {
		title: 'Live Trading Rooms | Revolution Trading Pros',
		description:
			'Live trading rooms with real-time voice-guided commentary from institutional traders.'
	},
	jsonld: [
		{
			'@context': 'https://schema.org',
			'@type': 'Service',
			'@id': `${PAGE_URL}#service`,
			name: 'Live Trading Rooms',
			serviceType: 'Live Trading Education & Mentorship',
			description:
				'Voice-guided live trading rooms for day trading, swing trading, and small accounts, led by institutional traders.',
			url: PAGE_URL,
			provider: {
				'@type': 'Organization',
				'@id': `${SITE_URL}/#organization`,
				name: 'Revolution Trading Pros'
			},
			areaServed: { '@type': 'Country', name: 'United States' },
			offers: tradingRoomsData.map((room) => ({
				'@type': 'Offer',
				name: room.name,
				description: room.description,
				price: room.price.monthly,
				priceCurrency: 'USD',
				availability: 'https://schema.org/InStock',
				url: `${PAGE_URL}/${room.id}`,
				category: 'Subscription'
			}))
		},
		faqSchema(faqs, `${PAGE_URL}#faq`),
		breadcrumbSchema(
			[
				{ name: 'Home', url: SITE_URL },
				{ name: 'Live Trading Rooms', url: PAGE_URL }
			],
			`${PAGE_URL}#breadcrumb`
		)
	]
};

export const load: PageServerLoad = async ({
	setHeaders
}: {
	setHeaders: (headers: Record<string, string>) => void;
}) => {
	setHeaders({
		'Cache-Control': 'public, max-age=300, s-maxage=600, stale-while-revalidate=60',
		Vary: 'Accept-Encoding, Accept',
		'X-DNS-Prefetch-Control': 'on',
		'X-Content-Type-Options': 'nosniff',
		'X-Frame-Options': 'DENY',
		'Referrer-Policy': 'strict-origin-when-cross-origin'
	});

	return {
		rooms: tradingRoomsData,
		benefits: benefitsData,
		symbols: tickerSymbols,
		seo: pageSeo,
		generatedAt: new Date().toISOString()
	};
};

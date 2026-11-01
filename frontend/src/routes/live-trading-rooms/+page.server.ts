/**
 * Live Trading Rooms - Server Load Function
 *
 * Implements proper SSR/SSG patterns for Svelte 5:
 * - Static generation with dynamic data
 * - SEO metadata for server-side rendering
 * - Performance optimizations
 *
 * @see https://kit.svelte.dev/docs/load
 * @module routes/live-trading-rooms/+page.server
 * @version 1.0.0
 */

import type { PageServerLoad } from './$types';
import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildItemList } from '#lib/seo/schemas.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

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
		iconType: 'analysis',
		title: 'Institutional Mentorship',
		desc: 'Learn from professionals with decades of combined experience at major firms and prop desks.'
	},
	{
		iconType: 'radar',
		title: 'Sub-Second Alerts',
		desc: 'Get trade alerts as they happen via SMS, email, and Discord. Never miss a breakout.'
	},
	{
		iconType: 'strategy',
		title: 'Continuous Education',
		desc: 'Access a library of strategy breakdowns, market analysis, and psychological training.'
	},
	{
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

export const load: PageServerLoad = async ({
	setHeaders
}: {
	setHeaders: (headers: Record<string, string>) => void;
}) => {
	// Set caching headers for Google December 2025 Core Web Vitals compliance
	// Note: Content-Encoding is handled by the server/CDN at runtime, not during prerender
	setHeaders({
		'Cache-Control': 'public, max-age=300, s-maxage=600, stale-while-revalidate=60',
		Vary: 'Accept-Encoding, Accept',
		// Performance headers for LCP < 2.0s
		'X-DNS-Prefetch-Control': 'on',
		// Security headers for trustworthiness signals
		'X-Content-Type-Options': 'nosniff',
		'X-Frame-Options': 'DENY',
		'Referrer-Policy': 'strict-origin-when-cross-origin'
	});

	// SEO payload — shape must match SEOInput so the unified layer renders it.
	const seoMetadata: SEOInput = {
		title: 'Live Trading Rooms — Real-Time Mentorship',
		description:
			'Join expert traders in our live trading rooms. Learn day trading, swing trading, and small account strategies with real-time execution and voice guidance.',
		og: {
			type: 'website',
			title: 'Live Trading Rooms — Real-Time Mentorship',
			image: `${SITE}/revolution-trading-pros.png`,
			imageAlt: 'Professional Live Trading Rooms with Expert Mentors'
		},
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Live Trading Rooms', url: `${SITE}/live-trading-rooms` }
			]),
			buildItemList(
				'Live Trading Rooms',
				tradingRoomsData.map((room) => ({
					name: room.name,
					url: `${SITE}/live-trading-rooms/${room.id}`,
					description: room.description
				}))
			)
		]
	};

	return {
		// Static data for SSR/SSG
		rooms: tradingRoomsData,
		benefits: benefitsData,
		symbols: tickerSymbols,

		// SEO metadata (consumed by root +layout.svelte via page.data.seo)
		seo: seoMetadata,

		// Performance metrics
		generatedAt: new Date().toISOString(),
		buildTime: process.env.NODE_ENV === 'production' ? 'static' : 'dynamic'
	};
};

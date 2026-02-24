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

import type { PageServerLoad } from './.types';

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

	// Generate SEO metadata aligned with Google December 2025 updates
	const seoMetadata = {
		title: 'Live Trading Rooms | Professional Trading Mentorship | Revolution Trading Pros',
		description:
			'Join expert traders in our live trading rooms. Learn day trading, swing trading, and small account strategies with real-time alerts from institutional professionals with 15+ years experience.',
		keywords:
			'live trading rooms, professional trading mentorship, day trading education, swing trading strategies, small account growth, institutional trading, real-time trading alerts, expert traders',
		canonical: 'https://revolution-trading-pros.com/live-trading-rooms',
		author: 'Revolution Trading Pros - Professional Trading Education',
		publisher: 'Revolution Trading Pros',
		datePublished: '2025-12-16',
		dateModified: new Date().toISOString(),
		openGraph: {
			title: 'Live Trading Rooms | Professional Trading Mentorship | Revolution Trading Pros',
			description:
				'Join expert traders in our live trading rooms. Learn day trading, swing trading, and small account strategies with real-time alerts from institutional professionals.',
			type: 'website',
			url: 'https://revolution-trading-pros.com/live-trading-rooms',
			images: [
				{
					url: 'https://revolution-trading-pros.com/images/live-trading-rooms-og.jpg',
					width: 1200,
					height: 630,
					alt: 'Professional Live Trading Rooms with Expert Mentors',
					type: 'image/jpeg'
				}
			],
			siteName: 'Revolution Trading Pros',
			locale: 'en_US',
			article: {
				author: 'Revolution Trading Pros',
				publishedTime: '2025-12-16T00:00:00Z',
				modifiedTime: new Date().toISOString(),
				section: 'Trading Education',
				tag: 'Live Trading Rooms'
			}
		},
		twitter: {
			card: 'summary_large_image',
			title: 'Live Trading Rooms | Professional Trading Mentorship',
			description:
				'Join expert traders in our live trading rooms. Learn day trading, swing trading, and small account strategies with real-time alerts.',
			images: ['https://revolution-trading-pros.com/images/live-trading-rooms-twitter.jpg'],
			site: '@revolutiontrading',
			creator: '@revolutiontrading',
			domain: 'revolution-trading-pros.com'
		},
		// Google December 2025 E-E-A-T signals
		eeat: {
			expertise: {
				level: 'professional',
				authorCredentials: [
					'15+ years institutional trading',
					'Series 7 & 63 licensed',
					'Former hedge fund trader'
				],
				originalResearch: 'Real-time market analysis and proprietary trading strategies',
				contentDepth: 'Comprehensive trading education with live demonstrations'
			},
			trustworthiness: {
				security: 'HTTPS with SSL encryption',
				transparency: 'Full team credentials and trading methodology disclosed',
				contactInfo: 'Verified business with physical address and support',
				corrections: 'Transparent error correction policy'
			},
			userCentric: {
				primaryIntent: 'Learn professional trading strategies through live mentorship',
				contentFormat: 'Interactive live trading rooms with real-time alerts',
				valueProposition: 'Direct access to institutional traders and proven strategies'
			}
		}
	};

	// Generate comprehensive structured data for Google December 2025
	const structuredData = {
		'@context': ['https://schema.org', 'https://schema.org/EducationEvent'],
		'@graph': [
			{
				'@type': 'WebPage',
				'@id': 'https://revolution-trading-pros.com/live-trading-rooms',
				url: 'https://revolution-trading-pros.com/live-trading-rooms',
				name: 'Live Trading Rooms | Professional Trading Mentorship',
				description:
					'Join expert traders in our live trading rooms. Learn day trading, swing trading, and small account strategies with real-time alerts from institutional professionals.',
				inLanguage: 'en-US',
				isPartOf: {
					'@type': 'WebSite',
					'@id': 'https://revolution-trading-pros.com',
					name: 'Revolution Trading Pros',
					publisher: {
						'@type': 'Organization',
						'@id': 'https://revolution-trading-pros.com#organization',
						name: 'Revolution Trading Pros',
						url: 'https://revolution-trading-pros.com',
						logo: {
							'@type': 'ImageObject',
							url: 'https://revolution-trading-pros.com/logo.png',
							width: 512,
							height: 512
						},
						sameAs: [
							'https://twitter.com/revolutiontrading',
							'https://linkedin.com/company/revolution-trading-pros'
						]
					}
				},
				primaryImageOfPage: {
					'@type': 'ImageObject',
					url: 'https://revolution-trading-pros.com/images/live-trading-rooms-hero.jpg',
					width: 1920,
					height: 1080
				},
				datePublished: '2025-12-16T00:00:00Z',
				dateModified: new Date().toISOString(),
				author: {
					'@type': 'Organization',
					name: 'Revolution Trading Pros',
					url: 'https://revolution-trading-pros.com'
				},
				breadcrumb: {
					'@type': 'BreadcrumbList',
					itemListElement: [
						{
							'@type': 'ListItem',
							position: 1,
							name: 'Home',
							item: 'https://revolution-trading-pros.com'
						},
						{
							'@type': 'ListItem',
							position: 2,
							name: 'Live Trading Rooms',
							item: 'https://revolution-trading-pros.com/live-trading-rooms'
						}
					]
				},
				review: {
					'@type': 'Review',
					itemReviewed: 'Live Trading Rooms Service',
					reviewRating: {
						'@type': 'Rating',
						ratingValue: 4.8,
						bestRating: 5,
						worstRating: 1
					},
					author: {
						'@type': 'Person',
						name: 'Professional Trader Community'
					}
				}
			},
			{
				'@type': 'Service',
				'@id': 'https://revolution-trading-pros.com/live-trading-rooms#service',
				name: 'Live Trading Rooms',
				description:
					'Professional live trading rooms with real-time alerts and mentorship from institutional traders',
				provider: {
					'@type': 'Organization',
					'@id': 'https://revolution-trading-pros.com#organization',
					name: 'Revolution Trading Pros'
				},
				areaServed: 'Worldwide',
				availableChannel: {
					'@type': 'ServiceChannel',
					serviceUrl: 'https://revolution-trading-pros.com/live-trading-rooms',
					availableLanguage: ['English']
				},
				offers: tradingRoomsData.map((room) => ({
					'@type': 'Offer',
					'@id': `https://revolution-trading-pros.com/live-trading-rooms#${room.id}`,
					name: room.name,
					description: room.description,
					price: room.price.monthly,
					priceCurrency: 'USD',
					availability: 'https://schema.org/InStock',
					validThrough: new Date(Date.now() + 365 * 24 * 60 * 60 * 1000).toISOString(),
					eligibleRegion: 'US',
					seller: {
						'@type': 'Organization',
						'@id': 'https://revolution-trading-pros.com#organization',
						name: 'Revolution Trading Pros'
					}
				})),
				hasOfferCatalog: {
					'@type': 'OfferCatalog',
					name: 'Trading Room Services',
					itemListElement: tradingRoomsData.map((room) => ({
						'@type': 'Offer',
						itemOffered: {
							'@type': 'Service',
							name: room.name,
							description: room.description
						}
					}))
				}
			},
			{
				'@type': 'EducationalOrganization',
				'@id': 'https://revolution-trading-pros.com#organization',
				name: 'Revolution Trading Pros',
				description:
					'Professional trading education and mentorship platform with institutional expertise',
				url: 'https://revolution-trading-pros.com',
				foundingDate: '2020',
				address: {
					'@type': 'PostalAddress',
					addressCountry: 'US'
				},
				contactPoint: {
					'@type': 'ContactPoint',
					telephone: '+1-555-TRADING',
					contactType: 'customer service',
					availableLanguage: 'English'
				},
				alumni: [
					{
						'@type': 'Person',
						name: 'Professional Traders',
						jobTitle: 'Professional Trader',
						knowsAbout: ['Day Trading', 'Swing Trading', 'Risk Management', 'Technical Analysis']
					}
				],
				award: ['Best Trading Education Platform 2024', 'Top Mentorship Program 2023']
			},
			{
				'@type': 'Course',
				name: 'Live Trading Mentorship Program',
				description:
					'Comprehensive live trading education with real-time mentorship from professional traders',
				provider: {
					'@type': 'Organization',
					'@id': 'https://revolution-trading-pros.com#organization',
					name: 'Revolution Trading Pros'
				},
				educationalLevel: 'Intermediate to Advanced',
				inLanguage: 'English',
				about: [
					'Day Trading',
					'Swing Trading',
					'Risk Management',
					'Technical Analysis',
					'Market Psychology'
				],
				teaches: [
					'Professional trading strategies',
					'Real-time market analysis',
					'Risk management techniques',
					'Institutional trading methods'
				],
				timeRequired: 'PT40H', // 40 hours of content
				coursePrerequisites: 'Basic understanding of financial markets',
				learningResourceType: 'Live Interactive Sessions',
				interactivityType: 'high',
				isAccessibleForFree: false,
				offers: {
					'@type': 'Offer',
					price: '247',
					priceCurrency: 'USD',
					availability: 'https://schema.org/InStock'
				}
			},
			{
				'@type': 'FAQPage',
				'@id': 'https://revolution-trading-pros.com/live-trading-rooms#faq',
				mainEntity: [
					{
						'@type': 'Question',
						name: 'What are live trading rooms?',
						acceptedAnswer: {
							'@type': 'Answer',
							text: 'Live trading rooms are interactive online spaces where professional traders share real-time market analysis, trade alerts, and educational content with members.'
						}
					},
					{
						'@type': 'Question',
						name: 'Who teaches the trading strategies?',
						acceptedAnswer: {
							'@type': 'Answer',
							text: 'Our trading rooms are led by institutional traders with 15+ years of experience at major firms and hedge funds, bringing real-world expertise to our members.'
						}
					}
				]
			}
		]
	};

	return {
		// Static data for SSR/SSG
		rooms: tradingRoomsData,
		benefits: benefitsData,
		symbols: tickerSymbols,

		// Legacy SEO metadata (used by page template)
		seo: seoMetadata,
		structuredData,

		// Performance metrics
		generatedAt: new Date().toISOString(),
		buildTime: process.env.NODE_ENV === 'production' ? 'static' : 'dynamic'
	};
};

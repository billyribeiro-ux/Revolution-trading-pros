/**
 * SEO Plugin Layer - Site-Wide Defaults
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Single source of truth for all default SEO values.
 * Override per-route via +page.server.ts load functions.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since February 2026
 */

import type { SEODefaults } from './types';

const SITE_URL = import.meta.env.VITE_SITE_URL || 'https://revolution-trading-pros.pages.dev';

export const seoDefaults: SEODefaults = {
	siteUrl: SITE_URL,
	siteName: 'Revolution Trading Pros',
	titleTemplate: '%s | Revolution Trading Pros',
	defaultTitle: 'Revolution Trading Pros — Professional Trading Education & Tools',
	defaultDescription:
		'Institutional-grade trading education, live trading rooms, custom indicators, and professional tools for 18,000+ active traders.',
	defaultImage: `${SITE_URL}/og-default.png`,
	defaultImageAlt: 'Revolution Trading Pros — Professional Trading Education',
	defaultLocale: 'en_US',
	twitterSite: '@RevTradingPros',
	twitterCard: 'summary_large_image',

	canonical: {
		siteUrl: SITE_URL,
		forceHttps: true,
		trailingSlash: 'never',
		queryParamAllowlist: ['page'],
		queryParamDenylist: [
			'utm_source',
			'utm_medium',
			'utm_campaign',
			'utm_term',
			'utm_content',
			'gclid',
			'fbclid',
			'msclkid',
			'dclid',
			'twclid',
			'li_fat_id',
			'mc_cid',
			'mc_eid',
			'ref',
			'_ga',
			'_gl',
			'_hsenc',
			'_hsmi',
			'hsa_acc',
			'hsa_cam',
			'hsa_grp',
			'hsa_ad',
			'hsa_src',
			'hsa_tgt',
			'hsa_kw',
			'hsa_mt',
			'hsa_net',
			'hsa_ver'
		]
	},

	robots: {
		index: true,
		follow: true,
		'max-snippet': -1,
		'max-image-preview': 'large',
		'max-video-preview': -1
	},

	verification: {
		google: null,
		bing: null,
		yandex: null,
		pinterest: null
	},

	jsonld: [
		{
			'@context': 'https://schema.org',
			'@type': 'Organization',
			'@id': `${SITE_URL}/#organization`,
			name: 'Revolution Trading Pros',
			url: SITE_URL,
			logo: {
				'@type': 'ImageObject',
				url: `${SITE_URL}/logo.png`
			},
			description:
				'Professional trading education, custom indicators, and live trading rooms for 18,000+ active traders.',
			sameAs: [
				'https://twitter.com/RevTradingPros',
				'https://facebook.com/RevTradingPros',
				'https://youtube.com/RevTradingPros'
			]
		},
		{
			'@context': 'https://schema.org',
			'@type': 'WebSite',
			'@id': `${SITE_URL}/#website`,
			name: 'Revolution Trading Pros',
			url: SITE_URL,
			potentialAction: {
				'@type': 'SearchAction',
				target: {
					'@type': 'EntryPoint',
					urlTemplate: `${SITE_URL}/search?q={search_term_string}`
				},
				'query-input': 'required name=search_term_string'
			}
		},
		{
			'@context': 'https://schema.org',
			'@type': 'FinancialService',
			'@id': `${SITE_URL}/#financial-service`,
			name: 'Revolution Trading Pros',
			url: SITE_URL,
			description:
				'Professional trading education platform offering live trading rooms, institutional-grade indicators, and real-time alert services for active traders.',
			serviceType: 'Trading Education & Alert Services',
			areaServed: {
				'@type': 'Country',
				name: 'United States'
			},
			hasOfferCatalog: {
				'@type': 'OfferCatalog',
				name: 'Trading Services',
				itemListElement: [
					{
						'@type': 'Offer',
						itemOffered: {
							'@type': 'Service',
							name: 'Live Trading Rooms',
							description:
								'Real-time voice-guided trading rooms with professional traders'
						}
					},
					{
						'@type': 'Offer',
						itemOffered: {
							'@type': 'Service',
							name: 'Trading Alert Services',
							description:
								'Swing trading and 0DTE options alerts with precise entries and exits'
						}
					},
					{
						'@type': 'Offer',
						itemOffered: {
							'@type': 'Service',
							name: 'Custom Trading Indicators',
							description:
								'Institutional-grade indicators for ThinkorSwim and TradeStation'
						}
					}
				]
			},
			founder: {
				'@type': 'Person',
				name: 'Billy Ribeiro',
				jobTitle: 'Founder & Head Trader'
			},
			foundingDate: '2018',
			numberOfEmployees: {
				'@type': 'QuantitativeValue',
				value: '18000',
				unitText: 'active members'
			}
		}
	],

	privatePathPrefixes: [
		'/account',
		'/admin',
		'/auth',
		'/checkout',
		'/cart',
		'/crm',
		'/dashboard',
		'/embed',
		'/forgot-password',
		'/reset-password',
		'/verify-email',
		'/logout',
		'/my',
		'/api'
	],

	searchPathPrefixes: ['/search', '/blog?', '/courses?']
};

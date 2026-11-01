/**
 * SEO Plugin Layer - Site-Wide Defaults
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Single source of truth for all default SEO values.
 * Override per-route via +page.ts / +page.server.ts load functions.
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 * @since November 2026
 */

import type { SEODefaults } from './types';
import {
	PRODUCTION_ORIGIN,
	SITE_NAME,
	DEFAULT_OG_LOCALE,
	TRAILING_SLASH,
	QUERY_PARAM_ALLOWLIST,
	TRACKING_PARAM_DENYLIST,
	NOINDEX_PREFIXES
} from './config';

const SITE_URL = PRODUCTION_ORIGIN;

export const seoDefaults: SEODefaults = {
	siteUrl: SITE_URL,
	siteName: SITE_NAME,
	titleTemplate: '%s | Revolution Trading Pros',
	defaultTitle: 'Revolution Trading Pros — Live Trading Rooms & Education',
	defaultDescription:
		'Institutional-grade trading education, live trading rooms, custom indicators, and professional tools for 18,000+ active traders.',
	defaultImage: `${SITE_URL}/revolution-trading-pros.png`,
	defaultImageAlt: 'Revolution Trading Pros — Professional Trading Education',
	defaultLocale: DEFAULT_OG_LOCALE,
	twitterSite: '@RevTradingPros',
	twitterCard: 'summary_large_image',

	canonical: {
		siteUrl: SITE_URL,
		forceHttps: true,
		trailingSlash: TRAILING_SLASH,
		queryParamAllowlist: [...QUERY_PARAM_ALLOWLIST],
		queryParamDenylist: [...TRACKING_PARAM_DENYLIST]
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
			name: SITE_NAME,
			url: `${SITE_URL}/`,
			logo: {
				'@type': 'ImageObject',
				url: `${SITE_URL}/icon-512.png`
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
			name: SITE_NAME,
			url: `${SITE_URL}/`
		}
	],

	privatePathPrefixes: [...NOINDEX_PREFIXES],

	searchPathPrefixes: ['/search', '/blog?', '/courses?']
};

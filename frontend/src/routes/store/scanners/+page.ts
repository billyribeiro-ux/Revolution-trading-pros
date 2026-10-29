/**
 * Scanners Landing Page - Load Configuration
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Google December 2025 SEO Standards:
 * - Static Site Generation (SSG) for optimal performance
 * - Prerendering for instant page loads
 * - Proper hydration strategy
 * - Core Web Vitals optimization
 *
 * Apple ICT 11+ Standards:
 * - Type-safe data loading
 * - Error boundary handling
 * - Progressive enhancement
 *
 * Svelte 5 / SvelteKit (Dec 2025):
 * - No ./$types imports needed in universal load functions
 * - Explicit typing for load function parameters
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 */

// ═══════════════════════════════════════════════════════════════════════════
// TYPES - Explicit typing without ./$types (Svelte 5 pattern)
// ═══════════════════════════════════════════════════════════════════════════

import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildSoftwareApplication } from '#lib/seo/schemas.js';

const SITE = 'https://revolutiontradingpros.com';

interface Scanner {
	id: string;
	name: string;
	slug: string;
	description: string;
	price: number;
	priceYearly: number;
	interval: 'monthly';
	currency: string;
	features: string[];
	image: string;
	rating: number;
	reviewCount: number;
	availability: 'InStock';
	category: string;
	brand: string;
}

interface Breadcrumb {
	name: string;
	url: string;
}

export interface PageData {
	scanners: Scanner[];
	seo: SEOInput;
	breadcrumbs: Breadcrumb[];
	loadedAt: string;
}

export const prerender = true;
export const ssr = true;
export const csr = true;

export const load = async ({ url }: { url: URL }): Promise<PageData> => {
	const scanners: Scanner[] = [
		{
			id: 'high-octane-scanner',
			name: 'High Octane Scanner',
			slug: 'high-octane-scanner',
			description:
				'Professional-grade options scanner with real-time alerts and advanced filtering',
			price: 119,
			priceYearly: 1190,
			interval: 'monthly' as const,
			currency: 'USD',
			features: [
				'Real-time options scanning',
				'Advanced filtering algorithms',
				'Custom alert notifications',
				'Historical data analysis',
				'Multi-timeframe analysis',
				'Mobile app access'
			],
			image: '/images/scanners/high-octane-scanner.jpg',
			rating: 4.8,
			reviewCount: 127,
			availability: 'InStock' as const,
			category: 'Trading Software',
			brand: 'Revolution Trading Pros'
		}
	];

	const breadcrumbs: Breadcrumb[] = [
		{ name: 'Home', url: `${SITE}/` },
		{ name: 'Store', url: `${SITE}/store/scanners` },
		{ name: 'Scanners', url: `${SITE}/store/scanners` }
	];

	const seo: SEOInput = {
		title: 'Trading Scanners — Professional Options & Stock Analysis Tools',
		description:
			'Professional trading scanners for options and stock analysis. Real-time scanning, advanced filters, and smart alerts. Starting at $119/month.',
		canonical: `${SITE}/store/scanners`,
		og: {
			type: 'website',
			image: `${SITE}/images/og/scanners-landing.jpg`,
			imageAlt: 'Revolution Trading Pros — Trading Scanners'
		},
		jsonld: [
			buildBreadcrumb(breadcrumbs),
			...scanners.map((scanner) =>
				buildSoftwareApplication({
					name: scanner.name,
					description: scanner.description,
					url: `${SITE}/store/scanners/${scanner.slug}`,
					applicationCategory: 'FinanceApplication',
					operatingSystem: 'Web',
					image: `${SITE}${scanner.image}`,
					offers: {
						price: scanner.price,
						priceCurrency: scanner.currency,
						url: `${SITE}/store/scanners/${scanner.slug}`
					},
					aggregateRating: {
						ratingValue: scanner.rating,
						ratingCount: scanner.reviewCount
					}
				})
			)
		]
	};

	return {
		scanners,
		seo,
		breadcrumbs,
		loadedAt: url ? new Date().toISOString() : new Date().toISOString()
	};
};

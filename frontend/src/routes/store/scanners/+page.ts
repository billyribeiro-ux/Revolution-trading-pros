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

interface SEO {
	title: string;
	description: string;
	canonical: string;
	keywords: string[];
	ogImage: string;
	ogType: string;
	twitterCard: string;
}

interface Breadcrumb {
	name: string;
	url: string;
}

export interface PageData {
	scanners: Scanner[];
	seo: SEO;
	breadcrumbs: Breadcrumb[];
	loadedAt: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// SSR/SSG CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Enable prerendering (SSG) for this page
 * Google Dec 2025: Static pages rank higher and load faster
 */
export const prerender = true;

/**
 * Enable SSR for dynamic content fallback
 * Ensures page works even if prerendering fails
 */
export const ssr = true;

/**
 * Client-side routing for SPA navigation
 * Maintains fast navigation after initial load
 */
export const csr = true;

/**
 * Trailing slash handling for consistent URLs
 * Google Dec 2025: URL consistency is a ranking factor
 */
export const trailingSlash = 'always';

// ═══════════════════════════════════════════════════════════════════════════
// DATA LOADING - Svelte 5 universal load function
// ═══════════════════════════════════════════════════════════════════════════

export const load = async ({ url }: { url: URL }): Promise<PageData> => {
	// Structured data for SEO
	const scanners = [
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

	// SEO metadata
	const seo = {
		title: 'Trading Scanners - Professional Options & Stock Analysis Tools',
		description:
			'Professional trading scanners for options and stock analysis. Real-time scanning, advanced filters, and smart alerts. Starting at $119/month.',
		canonical: `${url.origin}/store/scanners/`,
		keywords: [
			'options scanner',
			'stock scanner',
			'trading scanner',
			'high octane scanner',
			'real-time scanner',
			'options trading tools',
			'stock analysis software'
		],
		ogImage: `${url.origin}/images/og/scanners-landing.jpg`,
		ogType: 'website',
		twitterCard: 'summary_large_image'
	};

	// Breadcrumb data for structured markup
	const breadcrumbs = [
		{ name: 'Home', url: url.origin },
		{ name: 'Store', url: `${url.origin}/store` },
		{ name: 'Scanners', url: `${url.origin}/store/scanners/` }
	];

	return {
		scanners,
		seo,
		breadcrumbs,
		// Timestamp for cache validation
		loadedAt: new Date().toISOString()
	};
};

/**
 * Indicators Page Configuration - Apple ICT 7
 *
 * SSG Configuration:
 * - prerender: true (inherited from root, explicit for clarity)
 * - ssr: true (server-side rendering for SEO)
 *
 * @version 1.0.0
 */

import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildItemList } from '$lib/seo/schemas';
import { indicators } from './data';

const SITE = 'https://revolutiontradingpros.com';

// Enable static site generation for SEO
export const prerender = true;

// Enable server-side rendering
export const ssr = true;

export const load = () => ({
	seo: {
		title: 'Best Technical Indicators for Day Trading (2026 Guide)',
		description:
			"Stop guessing. Master the RSI, VWAP, MACD, and Bollinger Bands. Get the exact 'Golden Setup' configurations used by professional traders.",
		og: { type: 'article' },
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Indicators', url: `${SITE}/indicators` }
			]),
			buildItemList(
				'Trading Indicators',
				indicators.map((i) => ({
					name: i.name,
					url: `${SITE}/indicators/${i.slug}`,
					description: i.description
				}))
			)
		]
	} satisfies SEOInput
});

import type { SEOInput } from '#lib/seo/types.js';
import { buildArticle, buildBreadcrumb } from '#lib/seo/schemas.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => {
	const url = `${SITE}/resources/stock-indexes-list`;
	return {
		seo: {
			title: 'Major U.S. Stock Indexes Guide',
			description:
				'Comprehensive guide to major U.S. stock market indexes including the S&P 500, Nasdaq-100, Dow Jones, and Russell 2000 for options and futures traders.',
			og: { type: 'article' },
			jsonld: [
				buildBreadcrumb([
					{ name: 'Home', url: `${SITE}/` },
					{ name: 'Trading Resources', url: `${SITE}/resources` },
					{ name: 'Major Stock Indexes Guide', url }
				]),
				buildArticle({
					type: 'Article',
					url,
					headline: 'Major U.S. Stock Market Indexes Guide',
					description:
						'Comprehensive guide to major U.S. stock market indexes including the S&P 500, Nasdaq-100, Dow Jones, and Russell 2000 for options and futures traders.',
					image: [`${SITE}/revolution-trading-pros.png`],
					datePublished: '2026-01-15T00:00:00Z',
					dateModified: '2026-05-25T00:00:00Z',
					author: {
						name: 'Billy Ribeiro',
						url: `${SITE}/authors/billy-ribeiro`
					}
				})
			]
		} satisfies SEOInput
	};
};

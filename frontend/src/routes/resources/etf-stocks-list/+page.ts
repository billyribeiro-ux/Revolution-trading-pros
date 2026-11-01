import type { SEOInput } from '#lib/seo/types.js';
import { buildArticle, buildBreadcrumb } from '#lib/seo/schemas.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => {
	const url = `${SITE}/resources/etf-stocks-list`;
	return {
		seo: {
			title: 'ETF & Stocks Master List',
			description:
				'Explore our complete ETF and stock reference list with detailed sector breakdowns, underlying holdings, and liquidity metrics for active traders.',
			og: { type: 'article' },
			jsonld: [
				buildBreadcrumb([
					{ name: 'Home', url: `${SITE}/` },
					{ name: 'Trading Resources', url: `${SITE}/resources` },
					{ name: 'ETF & Stocks Master List', url }
				]),
				buildArticle({
					type: 'Article',
					url,
					headline: 'ETF & Stocks Master List for Active Traders',
					description:
						'Explore our complete ETF and stock reference list with detailed sector breakdowns, underlying holdings, and liquidity metrics for active traders.',
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

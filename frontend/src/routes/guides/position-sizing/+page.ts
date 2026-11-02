import type { Load } from '@sveltejs/kit';
import type { SEOInput } from '#lib/seo/types.js';
import { buildArticle, buildBreadcrumb } from '#lib/seo/schemas.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load: Load = () => {
	const url = `${SITE}/guides/position-sizing`;
	const seo: SEOInput = {
		title: 'Position Sizing for Swing Trading',
		description:
			'Calculate optimal position sizes for stock and options swing trades using institutional risk-per-share formulas and portfolio heat limits.',
		og: { type: 'article' },
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Trading Guides', url: `${SITE}/resources` },
				{ name: 'Position Sizing', url }
			]),
			buildArticle({
				type: 'Article',
				url,
				headline: 'Position Sizing for Swing Trading',
				description:
					'Calculate optimal position sizes for stock and options swing trades using institutional risk-per-share formulas and portfolio heat limits.',
				image: [`${SITE}/revolution-trading-pros.png`],
				datePublished: '2026-01-15T00:00:00Z',
				dateModified: '2026-05-25T00:00:00Z',
				author: {
					name: 'Billy Ribeiro',
					url: `${SITE}/authors/billy-ribeiro`
				}
			})
		]
	};
	return { seo };
};

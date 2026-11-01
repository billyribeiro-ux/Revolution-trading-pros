import type { Load } from '@sveltejs/kit';
import type { SEOInput } from '#lib/seo/types.js';
import { buildArticle, buildBreadcrumb } from '#lib/seo/schemas.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load: Load = () => {
	const url = `${SITE}/guides/exit-strategies`;
	const seo: SEOInput = {
		title: 'Swing Trading Exit Strategies',
		description:
			'Master professional swing trading exit strategies: scaling out at multiple targets, breakeven stop management, trailing stops, and time-based exits.',
		og: { type: 'article' },
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Trading Guides', url: `${SITE}/resources` },
				{ name: 'Exit Strategies', url }
			]),
			buildArticle({
				type: 'Article',
				url,
				headline: 'Exit Strategies for Swing Trading',
				description:
					'Master professional swing trading exit strategies: scaling out at multiple targets, breakeven stop management, trailing stops, and time-based exits.',
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

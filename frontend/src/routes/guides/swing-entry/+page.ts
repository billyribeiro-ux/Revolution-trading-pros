import type { Load } from '@sveltejs/kit';
import type { SEOInput } from '#lib/seo/types.js';
import { buildArticle, buildBreadcrumb } from '#lib/seo/schemas.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load: Load = () => {
	const url = `${SITE}/guides/swing-entry`;
	const seo: SEOInput = {
		title: 'Swing Trading Entry Strategies',
		description:
			'Learn institutional swing trading entry strategies including consolidation breakouts, trend pullbacks, relative strength momentum, and earnings catalysts.',
		og: { type: 'article' },
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Trading Guides', url: `${SITE}/resources` },
				{ name: 'Swing Entry Setups', url }
			]),
			buildArticle({
				type: 'Article',
				url,
				headline: 'Swing Trading Entry Strategies',
				description:
					'Learn institutional swing trading entry strategies including consolidation breakouts, trend pullbacks, relative strength momentum, and earnings catalysts.',
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

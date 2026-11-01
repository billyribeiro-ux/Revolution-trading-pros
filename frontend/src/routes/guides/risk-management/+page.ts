import type { Load } from '@sveltejs/kit';
import type { SEOInput } from '#lib/seo/types.js';
import { buildArticle, buildBreadcrumb } from '#lib/seo/schemas.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load: Load = () => {
	const url = `${SITE}/guides/risk-management`;
	const seo: SEOInput = {
		title: 'Risk Management for Swing Trading',
		description:
			'Protect your trading capital with disciplined swing trading risk management: the 2% rule, technical and ATR stop placement, and minimum 2:1 risk/reward.',
		og: { type: 'article' },
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Trading Guides', url: `${SITE}/resources` },
				{ name: 'Risk Management', url }
			]),
			buildArticle({
				type: 'Article',
				url,
				headline: 'Risk Management for Swing Trading',
				description:
					'Protect your trading capital with disciplined swing trading risk management: the 2% rule, technical and ATR stop placement, and minimum 2:1 risk/reward.',
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

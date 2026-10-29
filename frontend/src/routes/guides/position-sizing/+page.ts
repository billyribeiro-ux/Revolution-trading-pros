import type { Load } from '@sveltejs/kit';
import type { SEOInput } from '#lib/seo/types.js';
import { breadcrumbSchema } from '#lib/seo/jsonld.js';

export const load: Load = () => {
	const seo: SEOInput = {
		title: 'Position Sizing for Swing Trading — Formula & Risk Tiers',
		description:
			'Calculate optimal position sizes for stock and options swing trades using institutional risk-per-share formulas and portfolio heat limits.',
		og: { type: 'article' },
		jsonld: [
			breadcrumbSchema(
				[
					{ name: 'Home', url: 'https://revolutiontradingpros.com' },
					{
						name: 'Position Sizing Guide',
						url: 'https://revolutiontradingpros.com/guides/position-sizing'
					}
				],
				'https://revolutiontradingpros.com/guides/position-sizing#breadcrumb'
			)
		]
	};
	return { seo };
};

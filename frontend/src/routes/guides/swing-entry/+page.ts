import type { Load } from '@sveltejs/kit';
import type { SEOInput } from '#lib/seo/types.js';
import { breadcrumbSchema } from '#lib/seo/jsonld.js';

export const load: Load = () => {
	const seo: SEOInput = {
		title: 'Swing Trading Entry Strategies — Breakouts, Pullbacks & Momentum',
		description:
			'Learn institutional swing trading entry strategies including consolidation breakouts, trend pullbacks, relative strength momentum, and earnings catalysts.',
		og: { type: 'article' },
		jsonld: [
			breadcrumbSchema(
				[
					{ name: 'Home', url: 'https://revolutiontradingpros.com' },
					{
						name: 'Swing Entry Guide',
						url: 'https://revolutiontradingpros.com/guides/swing-entry'
					}
				],
				'https://revolutiontradingpros.com/guides/swing-entry#breadcrumb'
			)
		]
	};
	return { seo };
};

import type { Load } from '@sveltejs/kit';
import type { SEOInput } from '#lib/seo/types.js';
import { breadcrumbSchema } from '#lib/seo/jsonld.js';

export const load: Load = () => {
	const seo: SEOInput = {
		title: 'Exit Strategies for Swing Trading — Scaling Out & Trailing Stops',
		description:
			'Master professional swing trading exit strategies: scaling out at multiple targets, breakeven stop management, trailing stops, and time-based exits.',
		og: { type: 'article' },
		jsonld: [
			breadcrumbSchema(
				[
					{ name: 'Home', url: 'https://revolutiontradingpros.com' },
					{
						name: 'Exit Strategies Guide',
						url: 'https://revolutiontradingpros.com/guides/exit-strategies'
					}
				],
				'https://revolutiontradingpros.com/guides/exit-strategies#breadcrumb'
			)
		]
	};
	return { seo };
};

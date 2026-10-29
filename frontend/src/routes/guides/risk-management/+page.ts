import type { Load } from '@sveltejs/kit';
import type { SEOInput } from '#lib/seo/types.js';
import { breadcrumbSchema } from '#lib/seo/jsonld.js';

export const load: Load = () => {
	const seo: SEOInput = {
		title: 'Risk Management for Swing Trading — The 2% Rule & R:R Ratios',
		description:
			'Protect your trading capital with disciplined swing trading risk management: the 2% rule, technical and ATR stop placement, and minimum 2:1 risk/reward setups.',
		og: { type: 'article' },
		jsonld: [
			breadcrumbSchema(
				[
					{ name: 'Home', url: 'https://revolutiontradingpros.com' },
					{
						name: 'Risk Management Guide',
						url: 'https://revolutiontradingpros.com/guides/risk-management'
					}
				],
				'https://revolutiontradingpros.com/guides/risk-management#breadcrumb'
			)
		]
	};
	return { seo };
};

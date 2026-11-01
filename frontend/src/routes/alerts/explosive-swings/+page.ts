import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildProduct } from '#lib/seo/schemas.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Explosive Swings Trading Alerts',
		description:
			'Catch 20%+ moves with 3-7 day swing trading alerts. Institutional dark pool analysis and precise options signals for busy traders.',
		og: {
			type: 'website',
			image: `${SITE}/revolution-trading-pros.png`,
			imageAlt: 'Explosive Swings Trading Alerts — Multi-Day Opportunities'
		},
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Trading Alerts', url: `${SITE}/alerts` },
				{ name: 'Explosive Swings', url: `${SITE}/alerts/explosive-swings` }
			]),
			buildProduct({
				name: 'Explosive Swings Trading Alerts',
				description:
					'Premium multi-day swing trading alerts service. Catch 3-7 day moves with precise entry and exit signals.',
				url: `${SITE}/alerts/explosive-swings`,
				image: `${SITE}/revolution-trading-pros.png`,
				offers: { price: 97, priceCurrency: 'USD' }
			})
		]
	} satisfies SEOInput
});

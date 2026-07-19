import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildService } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

// SEO Requirement: Prerender this page as static HTML for instant indexing.
export const prerender = true;
export const trailingSlash = 'always';

export const load = () => ({
	seo: {
		title: 'Explosive Swings Trading Alerts — High-Probability Multi-Day Signals',
		description:
			'Catch 20%+ moves with 3-7 day swing trading alerts. Institutional dark pool analysis and precise options signals for traders with day jobs. 82% historical win rate.',
		og: {
			type: 'website',
			image: `${SITE}/images/og-swings.jpg`,
			imageAlt: 'Explosive Swings Trading Alerts — Multi-Day Opportunities'
		},
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Alerts', url: `${SITE}/alerts` },
				{ name: 'Explosive Swings', url: `${SITE}/alerts/explosive-swings` }
			]),
			buildService({
				name: 'Explosive Swings Trading Alerts',
				description:
					'Premium multi-day swing trading alerts service. Catch 3-7 day moves with precise entry and exit signals. Institutional dark pool analysis.',
				url: `${SITE}/alerts/explosive-swings`,
				provider: { name: 'Revolution Trading Pros', url: SITE },
				serviceType: 'Trading Alerts',
				offers: { price: 97, priceCurrency: 'USD' }
			})
		]
	} satisfies SEOInput
});

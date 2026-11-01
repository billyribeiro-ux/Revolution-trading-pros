import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildItemList, buildProduct } from '#lib/seo/schemas.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Pricing — Membership Plans',
		description:
			'Simple, transparent pricing for every level of trader. Start with Starter ($97/mo), grow with Pro ($197/mo), or unlock everything with Elite ($397/mo).',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Pricing', url: `${SITE}/pricing` }
			]),
			buildProduct({
				name: 'Revolution Trading Pros Pro Membership',
				description:
					'Full access to live trading rooms, real-time swing & day trading alerts, and institutional trading education.',
				url: `${SITE}/pricing`,
				image: `${SITE}/revolution-trading-pros.png`,
				offers: { price: 197, priceCurrency: 'USD' }
			}),
			buildItemList('Revolution Trading Pros Membership Plans', [
				{
					name: 'Starter Plan ($97/month)',
					url: `${SITE}/pricing#starter`,
					description: 'Perfect for traders getting started with professional education.'
				},
				{
					name: 'Pro Plan ($197/month)',
					url: `${SITE}/pricing#pro`,
					description: 'For serious traders who want every edge available.'
				},
				{
					name: 'Elite Plan ($397/month)',
					url: `${SITE}/pricing#elite`,
					description: 'Full institutional access with weekly 1-on-1 mentorship and all indicators.'
				}
			])
		]
	} satisfies SEOInput
});

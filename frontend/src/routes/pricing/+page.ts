import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildItemList, buildWebPage } from '#lib/seo/schemas.js';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Pricing — Simple, Transparent Membership Plans',
		description:
			'Simple, transparent pricing for every level of trader. Start with Starter ($97/mo), grow with Pro ($197/mo), or unlock everything with Elite ($397/mo).',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Pricing', url: `${SITE}/pricing` }
			]),
			buildWebPage({
				url: `${SITE}/pricing`,
				name: 'Pricing — Revolution Trading Pros',
				description:
					'Simple, transparent pricing for every level of trader. Start with Starter, grow with Pro, or unlock everything with Elite.',
				dateModified: '2026-10-28'
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

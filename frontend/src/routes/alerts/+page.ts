import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildItemList, buildService } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Trading Alert Services',
		description:
			'Get premium trading alerts delivered to your phone. Swing trading and day trading alerts with precise entries, exits, and risk management.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Alerts', url: `${SITE}/alerts` }
			]),
			buildService({
				name: 'Revolution Trading Pros Alerts',
				description:
					'Premium trading alerts (swing + day trading) delivered via SMS and push notification, with precise entries, exits, and risk management.',
				url: `${SITE}/alerts`,
				provider: { name: 'Revolution Trading Pros', url: SITE },
				serviceType: 'Trading Alerts'
			}),
			buildItemList('Alert Services', [
				{
					name: 'Explosive Swings',
					url: `${SITE}/alerts/explosive-swings`,
					description: 'Multi-day swing trading alerts with 3-7 day holding periods.'
				},
				{
					name: 'SPX Profit Pulse',
					url: `${SITE}/alerts/spx-profit-pulse`,
					description: 'Real-time SPX 0DTE options alerts with SMS delivery.'
				}
			])
		]
	} satisfies SEOInput
});

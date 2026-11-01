import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildEvent, buildProduct } from '#lib/seo/schemas.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Swing Trading Room — Multi-Day Signals',
		description:
			'Swing trading room for part-time traders. Get high-precision stock and options alerts (3-7 day holds) sent via SMS and push notifications.',
		og: {
			type: 'website',
			image: `${SITE}/revolution-trading-pros.png`,
			imageAlt: 'Swing Trading Room — Multi-Day Trading Opportunities'
		},
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Live Trading Rooms', url: `${SITE}/live-trading-rooms` },
				{ name: 'Swing Trading Room', url: `${SITE}/live-trading-rooms/swing-trading` }
			]),
			buildProduct({
				name: 'Swing Trading Room',
				description:
					'Premium multi-day swing trading room. 3-7 day stock and options signals delivered via SMS and push notification.',
				url: `${SITE}/live-trading-rooms/swing-trading`,
				image: `${SITE}/revolution-trading-pros.png`,
				offers: { price: 97, priceCurrency: 'USD' }
			}),
			buildEvent({
				name: 'Swing Trading Room — Weekly Plan Session',
				description:
					'Recurring weekly planning session covering setups, position sizing, and exit plans for swing trades.',
				url: `${SITE}/live-trading-rooms/swing-trading`,
				startDate: '2026-05-31T19:00:00-04:00',
				eventAttendanceMode: 'OnlineEventAttendanceMode',
				organizer: { name: 'Revolution Trading Pros', url: SITE },
				isLiveBroadcast: true,
				location: { name: 'Revolution Trading Pros — Discord', url: SITE }
			})
		]
	} satisfies SEOInput
});
